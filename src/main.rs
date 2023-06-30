// Please see the README file for an overview of this program.
//
// Copyright 2023 by Curtis Whitley
// 
// MIT License
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::fs;
use std::io::Write;
use std::{env, vec};
use std::collections::HashMap;
use image::Rgb;

const IMG_R: usize = 0;
const IMG_G: usize = 1;
const IMG_B: usize = 2;
const IMG_A: usize = 3;

#[derive(Debug, Clone)]
struct DirParameters {
    pub width: usize,
    pub height: usize,
    pub bpp: u8,
    pub no_output: bool,
    pub vapor: bool,
    pub path: String
}

impl DirParameters {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            bpp: 0,
            no_output: false,
            vapor: false,
            path: String::new()        
        }
    }

    pub fn current_dir() -> Self {
        let mut params = DirParameters::new();
        params.path = "./".to_string();
        params
    }
}

#[derive(Debug, Clone)]
struct FileParameters {
    pub width: usize,
    pub height: usize,
    pub bpp: u8,
    pub no_output: bool,
    pub vapor: bool,
    pub path: String,
    pub size: usize,
    pub max_colors: usize,
    pub colors: HashMap<Rgb<u8>, u8>
}

impl FileParameters {
    pub fn new(params: &DirParameters) -> Self {
        Self {
            width: params.width,
            height: params.height,
            bpp: params.bpp,
            no_output: params.no_output,
            vapor: params.vapor,
            path: params.path.clone(),
            size: 0,
            max_colors: 0,
            colors: HashMap::new()
        }
    }
}

#[derive(Debug, Default)]
struct Expectations {
    pub width: bool,
    pub height: bool,
    pub file: bool,
    pub bpp: bool
}

impl Expectations {
    pub fn new() -> Self {
        Expectations::default()
    }

    pub fn expect_file(&mut self) {
        *self = Expectations::new();
        self.file = true;
    }

    pub fn anything(&self) -> bool {
        self.width || self.height || self.bpp
    }
}

fn main() {
    println!("Image to Agon (PNG-to-Agon-binary file convertor) V1.5");

    // Determine which directories to use.
    let mut directories: Vec<DirParameters> = vec![];

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        // No command arguments given; use current directory only.
        directories.push(DirParameters::current_dir());
    } else {
        // Traverse command arguments.
        let mut params = DirParameters::new();
        let mut expect = Expectations::new();
        expect.expect_file();

        for a in 1..args.len() {
            let arg = args[a].clone().to_ascii_lowercase();
            if arg.starts_with("-") && expect.anything() {
                println!("ERROR: Missing parameter value");
                return;
            } else if arg.eq("-w") | arg.eq("-width") {
                expect.width = true;
            } else if arg.eq("-h") || arg.eq("-height") {
                expect.height = true;
            } else if arg.eq("-b") || arg.eq("-bpp") {
                expect.bpp = true;
            } else if arg.eq("-n") || arg.eq("-nooutput") {
                params.no_output = true;
            } else if expect.width {
                match arg.parse::<usize>() {
                    Ok(number) => {
                        params.width = number;
                        expect.expect_file();
                    },
                    Err(err) => {
                        println!("ERROR: Invalid width: {}", err.to_string());
                        return;
                    }
                }
            } else if expect.height {
                match arg.parse::<usize>() {
                    Ok(number) => {
                        params.height = number;
                        expect.expect_file();
                    },
                    Err(err) => {
                        println!("ERROR: Invalid width: {}", err.to_string());
                        return;
                    }
                }
            } else if expect.bpp {
                match arg.parse::<u8>() {
                    Ok(number) => {
                        if number == 1 || number == 2 || number == 3 ||
                            number == 4 || number == 6 || number == 8 {
                            params.bpp = number;
                            expect.expect_file();    
                        } else {
                            println!("ERROR: Invalid bits-per-pixel");
                            return;
                        }
                    },
                    Err(err) => {
                        println!("ERROR: Invalid bits-per-pixel: {}", err.to_string());
                        return;
                    }
                }
            } else {
                params.path = arg;
                directories.push(params);
                params = DirParameters::new();
                expect = Expectations::new();
            }
        }        

        if expect.anything() {
            println!("ERROR: Missing parameter value");
            return;
        } else if expect.file {
            params.path = "./".to_string();
            directories.push(params);
        }
    }

    // Make sure we have something to do.
    if directories.len() == 0 {
        println!("ERROR: No directories to process.");
        return;
    }

    // Determine the paths to all files to process.
    let mut files: Vec<FileParameters> = vec![];

    for directory in &mut directories {
        // Validate certain options.
        if directory.bpp == 0 {
            directory.bpp = 8;
        }

        // Skip virtual data, as there is no directory or file.
        if directory.vapor {
            let mut params = FileParameters::new(&directory);
            params.size = directory.width * directory.height * 2;
            files.push(params);
            continue;
        }

        println!("Reading: {}", directory.path);

        // Check for accessing a single file, rather than a directory.
        if directory.path.to_ascii_lowercase().ends_with(".png") {
            match fs::metadata(directory.path.clone()) {
                Ok(metadata) => {
                    if metadata.is_file() {
                        let img = image::open(directory.path.clone()).unwrap();
                        let mut params = FileParameters::new(&directory);
                        if directory.width == 0 {
                            params.width = img.width() as usize;
                        }
                        if directory.height == 0 {
                            params.height = img.height() as usize;
                        }

                        let mut width = params.width;
                        match params.bpp {
                            1 => {
                                width = (width + 7) / 8; // 8 pixels per byte
                            },
                            2 => {
                                width = (width + 3) / 4; // 4 pixels per byte
                            },
                            3 => {
                                width = (width + 1) / 2; // 2 pixels per byte
                            },
                            4 => {
                                width = (width + 1) / 2; // 2 pixels per byte
                            },
                            6 => {
                                // 1 pixel per byte
                            },
                            8 => {
                                // 1 pixel per byte
                            },
                            _ => {}
                        }
                        params.size = width * params.height;

                        files.push(params);
                    } else {
                        println!("ERROR: Specified file is not a file: {}", directory.path);
                        return;
                    }
                },
                Err(_) => {
                    println!("ERROR: Cannot read the specified file: {}", directory.path);
                    return;
                }
            }
            continue;
        }

        // We must be accessing a whole directory.
        let paths = match fs::read_dir(&directory.path) {
            Ok(path) => path,
            Err(_) => {
                println!("ERROR: Cannot read the specified directory: {}", directory.path);
                return;
            }
        };
        for path in paths {
            match path {
                Ok(dir_entry) => {
                    match dir_entry.file_type() {
                        Ok(file_type) => {
                            if file_type.is_file() {
                                let pathname = dir_entry.path().as_os_str().to_str().unwrap().to_string();
                                if pathname.to_ascii_lowercase().ends_with(".png") {
                                    let img = image::open(pathname.clone()).unwrap();
                                    let mut params = FileParameters::new(&directory);
                                    if directory.width == 0 {
                                        params.width = img.width() as usize;
                                    }
                                    if directory.height == 0 {
                                        params.height = img.height() as usize;
                                    }

                                    let mut width = params.width;
                                    match params.bpp {
                                        1 => {
                                            width = (width + 7) / 8; // 8 pixels per byte
                                        },
                                        2 => {
                                            width = (width + 3) / 4; // 4 pixels per byte
                                        },
                                        3 => {
                                            width = (width + 1) / 2; // 2 pixels per byte
                                        },
                                        4 => {
                                            width = (width + 1) / 2; // 2 pixels per byte
                                        },
                                        6 => {
                                            // 1 pixel per byte
                                        },
                                        8 => {
                                            // 1 pixel per byte
                                        },
                                        _ => {}
                                    }
                                    params.size = width * params.height;
                                    params.path = pathname;
                                                                        
                                    files.push(params);
                                }
                            }
                        },
                        Err(_err) => {}
                    }
                },
                Err(_err) => {}
            }
        }
    }

    // Make sure we have something to do.
    if files.len() == 0 {
        println!("ERROR: No files to process.");
        return;
    }

    // Read the contents of all files, and determine their unique pixel colors.
    for img_file in &mut files {
        // Determine the maximum number of colors, not including transparent
        match img_file.bpp {
            0 => {
                img_file.bpp = 8;
                img_file.max_colors = 64;
            },
            1 => {
                img_file.max_colors = 1;
            },
            2 => {
                img_file.max_colors = 3;
            },
            3 => {
                img_file.max_colors = 7;
            },
            4 => {
                img_file.max_colors = 15;
            },
            6 => {
                img_file.max_colors = 63;
            },
            8 => {
                img_file.max_colors = 64;
            },
            _ => {}
        }

        // Check for needing to read the file
        if img_file.vapor {
            continue; // skip it
        }

        // Read the file contents
        let img = image::open(img_file.path.clone()).unwrap();
        let width = img.width();
        let height = img.height();
        println!("{}, {}x{}, {:?}", img_file.path, width, height, img.color());
    
        match img {
            image::DynamicImage::ImageRgb8(rgba) => {
                for y in 0..height {
                    for x in 0..width {
                        let pixel = rgba.get_pixel(x, y);
                        let r = convert_color(pixel[IMG_R]);
                        let g = convert_color(pixel[IMG_G]);
                        let b = convert_color(pixel[IMG_B]);
                        let color = Rgb::<u8>([r, g, b]);
                        if !img_file.colors.contains_key(&color) {
                            if img_file.colors.len() >= img_file.max_colors {
                                println!("ERROR: File {} contains too many colors (over {})",
                                    img_file.path, img_file.max_colors);
                                return;
                            }
                            if img_file.bpp == 8 {
                                let index = (b<<4)|(g<<2)|r;
                                img_file.colors.insert(color, index);    
                            } else {
                                let index = (img_file.colors.len() + 1) as u8;
                                img_file.colors.insert(color, index);    
                            }
                        }
                    }
                }
            },
            image::DynamicImage::ImageRgba8(rgba) => {
                for y in 0..height {
                    for x in 0..width {
                        let pixel = rgba.get_pixel(x, y);
                        //let a = convert_color(pixel[IMG_A]);
                        /*if a > 0*/ {
                            let r = convert_color(pixel[IMG_R]);
                            let g = convert_color(pixel[IMG_G]);
                            let b = convert_color(pixel[IMG_B]);
                            let color = Rgb::<u8>([r, g, b]);
                            if !img_file.colors.contains_key(&color) {
                                if img_file.colors.len() >= img_file.max_colors {
                                    println!("ERROR: File {} contains too many colors (over {})",
                                        img_file.path, img_file.max_colors);
                                    return;
                                }
                                if img_file.bpp == 8 {
                                    let index = (b<<4)|(g<<2)|r;
                                    img_file.colors.insert(color, index);    
                                } else {
                                    let index = (img_file.colors.len() + 1) as u8;
                                    img_file.colors.insert(color, index);    
                                }
                            }
                        }
                    }
                }
            },
            _ => {
                println!("ERROR: Unhandled image format ({}). Must be RGB8 or RGBA8!", img_file.path);
                return;
            }
        }

        println!("File {} has {} unique colors (maximum is {}).",
            img_file.path, img_file.colors.len(), img_file.max_colors);
    }

    // Use the colors of all files, and consolidate their palettes.

    let mut palette_map: HashMap<Rgb<u8>, Vec<u8>> = HashMap::new();
    let mut palette_array: Vec<Option<Rgb::<u8>>> = vec![];

    for _index in 0..64 {
        palette_array.push(None);
    }

    // Find indexes for colors.
    let mut dump_palette = false;
    let next_index: usize = 1;
    for img_file in &mut files {
        dump_palette |= img_file.bpp != 8;
        for (color, index) in &img_file.colors {
            if !palette_map.contains_key(&color) {
                if dump_palette {
                    let mut found = false;
                    for palette_index in next_index..256 {
                        if palette_array[palette_index].is_none() {
                            palette_array[palette_index] = Some(color.clone());
                            let mut indexes: Vec<u8> = vec![];
                            indexes.push(palette_index as u8);
                            palette_map.insert(color.clone(), indexes);            
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        println!("ERROR: Could not insert all colors into palette (please reduce colors)");
                        return;
                    }
                } else {
                    palette_array[(*index) as usize] = Some(color.clone());
                    let mut indexes: Vec<u8> = vec![];
                    indexes.push(*index);
                    palette_map.insert(color.clone(), indexes);            
                }
            } 
        }    
    }

    if dump_palette {
        // Dump the palette to the console, for documentation purposes.
        println!("; Palette entries by index:");
        println!(";           Agon            Dec Hex:   R G B");
        println!(";");
        println!("begin_palette_table:");
        for index in 0..palette_array.len() {
            let color: Rgb<u8>;        
            let free = match palette_array[index] {
                Some(c) => {
                    color = c.clone();
                    ""
                },
                None => {
                    color = Rgb::<u8>([0,0,0]); // black
                    " (FREE)"
                }
            };
            let wcolor = widen_color(&color);
            println!("    DB    0{:02X}H,0{:02X}H,0{:02X}H  ; {:03} 0{:02x}H:  {:x} {:x} {:x}{}",
                wcolor[0], wcolor[1], wcolor[2], // R G B
                index, index,
                color[0], color[1], color[2], // R G B
                free);
        }
        println!("end_palette_table:\n");
    }

    // For each PNG file, convert its pixels to palette indexes, and write to binary output file.
    // Also, write the widened RGB colors to a separate file.
    //
    for img_file in &mut files {
        if img_file.vapor || img_file.no_output {
            continue; // skip it
        }
        println!("\n---{}---\n", img_file.path);
        let img = image::open(img_file.path.clone()).unwrap();

        // Get dimensions for input image.
        let img_width = img.width() as i32;
        let img_height = img.height() as i32;
        let img_center_x = img_width / 2;
        let img_center_y = img_height / 2;

        // Get dimensions for output image.
        let out_width = img_file.width as i32;
        let out_height = img_file.height as i32;
        let out_center_x = out_width / 2;
        let out_center_y = out_height / 2;

        // Compute necessary rectangles.
        let out_start_x: i32;
        let out_end_x: i32;
        let out_start_y: i32;
        let out_end_y: i32;

        out_start_x = 0;
        out_end_x = out_width;
    
        out_start_y = 0;
        out_end_y = out_height;
    
        match img {
            image::DynamicImage::ImageRgb8(rgb) => {
                // Convert pixel colors into indexes.
                let mut output_data: Vec<u8> = vec![];
                let mut output_data_rgb: Vec<u8> = vec![];

                for out_y in out_start_y..out_end_y {
                    let mut bits_used: u8 = 0;
                    let mut output_byte: u8 = 0;
        
                    let img_y = img_center_y - (out_center_y - out_y);
                    if img_y < 0 || img_y >= img_height {
                        for _out_x in out_start_x..out_end_x {
                            // output transparent color index (zero)
                            output_data_rgb.push(0);
                            output_data_rgb.push(0);
                            output_data_rgb.push(0);

                            if img_file.bpp > 4 {
                                output_data.push(0);
                            } else {
                                output_byte <<= img_file.bpp;
                                bits_used += img_file.bpp;
                                if bits_used >= 8 || 8 - bits_used < img_file.bpp {
                                    output_data.push(output_byte);
                                    output_byte = 0;
                                    bits_used = 0;
                                }    
                            }
                        }
                    } else {
                        for out_x in out_start_x..out_end_x {
                            let img_x = img_center_x - (out_center_x - out_x);
                            if img_x < 0 || img_x >= img_width {
                                // output transparent color index (zero)
                                output_data_rgb.push(0);
                                output_data_rgb.push(0);
                                output_data_rgb.push(0);

                                if img_file.bpp > 4 {
                                    output_data.push(0);
                                } else {
                                    output_byte <<= img_file.bpp;
                                    bits_used += img_file.bpp;
                                    if bits_used >= 8 || 8 - bits_used < img_file.bpp {
                                        output_data.push(output_byte);
                                        output_byte = 0;
                                        bits_used = 0;
                                    }    
                                }
                            } else {
                                let pixel = rgb.get_pixel(img_x as u32, img_y as u32);
                                let r = convert_color(pixel[IMG_R]);
                                let g = convert_color(pixel[IMG_G]);
                                let b = convert_color(pixel[IMG_B]);
                                let color = Rgb::<u8>([r, g, b]);
                                //if img_x==10 && img_y==10 {
                                //    println!("{},{}: {:?} {:?}", img_x, img_y, pixel, color);
                                //}

                                let wcolor = widen_color(&color);
                                output_data_rgb.push(wcolor[0]);
                                output_data_rgb.push(wcolor[1]);
                                output_data_rgb.push(wcolor[2]);

                                let indexes = palette_map.get(&color).unwrap();
                                let index = indexes[0];
                                //print!("<{} {} {} / {} {} {} {}> ",
                                //pixel[IMG_R],pixel[IMG_G],pixel[IMG_B],
                                //r,g,b,index);

                                // output some color index or color value
                                if img_file.bpp > 4 {
                                    output_data.push(index);
                                } else {
                                    output_byte = (output_byte << img_file.bpp) | index;
                                    bits_used += img_file.bpp;
                                    if bits_used >= 8 || 8 - bits_used < img_file.bpp {
                                        output_data.push(output_byte);
                                        output_byte = 0;
                                        bits_used = 0;
                                    }    
                                }
                            }
                        }    
                    }
                    // finish the pixel row
                    if bits_used > 0 {
                        while bits_used < 8 {
                            output_byte <<= img_file.bpp;
                            bits_used += img_file.bpp;
                        }
                        output_data.push(output_byte);
                    }
                }

                // Write the output data to a file.
                let uc_path = upcase_filename(&img_file.path);
                match fs::File::create(uc_path.clone()) {
                    Ok(mut file) => {
                        match file.write_all(&output_data[..]) {
                            Ok(()) => {
                                println!("Wrote file ({}) as {} bytes.", uc_path, output_data.len());
                            },
                            Err(err) => {
                                println!("ERROR: Cannot write output file ({}): {}", uc_path, err.to_string());
                            }
                        }
                    },
                    Err(err) => {
                        println!("ERROR: Cannot open output file ({}): {}", uc_path, err.to_string());
                    }
                }

                // Write the output RGB data to a file.
                let uc_path = upcase_filename(&img_file.path) + ".RGB";
                match fs::File::create(uc_path.clone()) {
                    Ok(mut file) => {
                        match file.write_all(&output_data_rgb[..]) {
                            Ok(()) => {
                                println!("Wrote RGB file ({}) as {} bytes.", uc_path, output_data_rgb.len());
                            },
                            Err(err) => {
                                println!("ERROR: Cannot write RGB output file ({}): {}", uc_path, err.to_string());
                            }
                        }
                    },
                    Err(err) => {
                        println!("ERROR: Cannot open RGB output file ({}): {}", uc_path, err.to_string());
                    }
                }
            },
            image::DynamicImage::ImageRgba8(rgba) => {
                // Convert pixel colors into indexes.
                let mut output_data: Vec<u8> = vec![];
                let mut output_data_rgb: Vec<u8> = vec![];

                for out_y in out_start_y..out_end_y {
                    let mut bits_used: u8 = 0;
                    let mut output_byte: u8 = 0;
        
                    let img_y = img_center_y - (out_center_y - out_y);
                    if img_y < 0 || img_y >= img_height {
                        for _out_x in out_start_x..out_end_x {
                            // output transparent color index (zero)
                            output_data_rgb.push(0);
                            output_data_rgb.push(0);
                            output_data_rgb.push(0);

                            if img_file.bpp > 4 {
                                output_data.push(0);
                            } else {
                                output_byte <<= img_file.bpp;
                                bits_used += img_file.bpp;
                                if bits_used >= 8 || 8 - bits_used < img_file.bpp {
                                    output_data.push(output_byte);
                                    output_byte = 0;
                                    bits_used = 0;
                                }    
                            }
                        }
                    } else {
                        for out_x in out_start_x..out_end_x {
                            let img_x = img_center_x - (out_center_x - out_x);
                            if img_x < 0 || img_x >= img_width {
                                // output transparent color index (zero)
                                output_data_rgb.push(0);
                                output_data_rgb.push(0);
                                output_data_rgb.push(0);

                                if img_file.bpp > 4 {
                                    output_data.push(0);
                                } else {
                                    output_byte <<= img_file.bpp;
                                    bits_used += img_file.bpp;
                                    if bits_used >= 8 || 8 - bits_used < img_file.bpp {
                                        output_data.push(output_byte);
                                        output_byte = 0;
                                        bits_used = 0;
                                    }    
                                }
                            } else {
                                let pixel = rgba.get_pixel(img_x as u32, img_y as u32);
                                let a = convert_color(pixel[IMG_A]);
                                /*if a > 0*/ {
                                    let r = convert_color(pixel[IMG_R]);
                                    let g = convert_color(pixel[IMG_G]);
                                    let b = convert_color(pixel[IMG_B]);
                                    let color = Rgb::<u8>([r, g, b]);
                                    //if img_x==10 && img_y==10 {
                                    //    println!("{},{}: {:?} {:?} {:?}", img_x, img_y, a, pixel, color);
                                    //}
    
                                    let wcolor = widen_color(&color);
                                    output_data_rgb.push(wcolor[0]);
                                    output_data_rgb.push(wcolor[1]);
                                    output_data_rgb.push(wcolor[2]);
    
                                    let indexes = palette_map.get(&color).unwrap();
                                    let index = indexes[0];
                                    //print!("({} {} {} / {} {} {} {}) ",
                                    //pixel[IMG_R],pixel[IMG_G],pixel[IMG_B],
                                    //r,g,b,index);
    
                                    // output some color index
                                    if img_file.bpp == 8 {
                                        let transparency = a << 6;
                                        output_data.push(index|transparency);
                                    } else if img_file.bpp > 4 {
                                        output_data.push(index);
                                    } else {
                                        output_byte = (output_byte << img_file.bpp) | index;
                                        bits_used += img_file.bpp;
                                        if bits_used >= 8 || 8 - bits_used < img_file.bpp {
                                            output_data.push(output_byte);
                                            output_byte = 0;
                                            bits_used = 0;
                                        }    
                                    }
                                } /*else {
                                    // output transparent color index (zero)
                                    output_data_rgb.push(0);
                                    output_data_rgb.push(0);
                                    output_data_rgb.push(0);

                                    if img_file.bpp > 4 {
                                        output_data.push(0);
                                    } else {
                                        output_byte <<= img_file.bpp;
                                        bits_used += img_file.bpp;
                                        if bits_used >= 8 || 8 - bits_used < img_file.bpp {
                                            output_data.push(output_byte);
                                            output_byte = 0;
                                            bits_used = 0;
                                        }    
                                    }
                                }*/
                            }
                        }    
                    }
                    // finish the pixel row
                    if bits_used > 0 {
                        while bits_used < 8 {
                            output_byte <<= img_file.bpp;
                            bits_used += img_file.bpp;
                        }
                        output_data.push(output_byte);
                    }
                }

                // Write the output data to a file.
                let uc_path = upcase_filename(&img_file.path);
                match fs::File::create(uc_path.clone()) {
                    Ok(mut file) => {
                        match file.write_all(&output_data[..]) {
                            Ok(()) => {
                                println!("Wrote file ({}) as {} bytes.", uc_path, output_data.len());
                            },
                            Err(err) => {
                                println!("ERROR: Cannot write output file ({}): {}", uc_path, err.to_string());
                            }
                        }
                    },
                    Err(err) => {
                        println!("ERROR: Cannot open output file ({}): {}", uc_path, err.to_string());
                    }
                }

                // Write the output RGB data to a file.
                let uc_path = upcase_filename(&img_file.path) + ".RGB";
                match fs::File::create(uc_path.clone()) {
                    Ok(mut file) => {
                        match file.write_all(&output_data_rgb[..]) {
                            Ok(()) => {
                                println!("Wrote RGB file ({}) as {} bytes.", uc_path, output_data_rgb.len());
                            },
                            Err(err) => {
                                println!("ERROR: Cannot write RGB output file ({}): {}", uc_path, err.to_string());
                            }
                        }
                    },
                    Err(err) => {
                        println!("ERROR: Cannot open RGB output file ({}): {}", uc_path, err.to_string());
                    }
                }
            },
            _ => {
                panic!("ERROR: Unhandled image format. Must be RGBA8!");
            }
        }
    }

    if dump_palette {
        // Write the palette data to a file.
        let mut palette_bytes: Vec<u8> = vec![];

        // standard and custom colors
        for index in 0..palette_array.len() {
            match palette_array[index] {
                Some(color) => {
                    palette_bytes.push(color[0]); // R
                    palette_bytes.push(color[1]); // G
                    palette_bytes.push(color[2]); // B
                },
                None => {
                    palette_bytes.push(0);
                    palette_bytes.push(0);
                    palette_bytes.push(0);
                }
            }
        }

        let uc_path = "PALETTE.BIN".to_string();
        match fs::File::create(uc_path.clone()) {
            Ok(mut file) => {
                match file.write_all(&palette_bytes[..]) {
                    Ok(()) => {
                        println!("Wrote file ({}) as {} bytes.", uc_path, palette_bytes.len());
                    },
                    Err(err) => {
                        println!("ERROR: Cannot write palette file ({}): {}", uc_path, err.to_string());
                    }
                }
            },
            Err(err) => {
                println!("ERROR: Cannot open palette file ({}): {}", uc_path, err.to_string());
            }
        }
    }

    show_memory_map(&mut files);
}

fn convert_color(color: u8) -> u8 {
    color >> 6
}

fn upcase_filename(path: &str) -> String {
    let parts = path.split("/").collect::<Vec<&str>>();
    let mut output_path = String::new();
    for i in 0..parts.len()-1 {
        output_path.push_str(parts[i]);
        output_path.push_str("/");
    }

    let parts2 = parts[parts.len()-1].split(".").collect::<Vec<&str>>();
    for i in 0..parts2.len()-1 {
        output_path.push_str(&parts2[i].to_ascii_uppercase());
        output_path.push_str(".");
    }
    output_path.push_str("BIN");

    output_path
}

fn show_memory_map(files: &mut Vec<FileParameters>) {
    println!("\nRelative Memory Map\n");
    println!("Start  End    Size   Width Height Path/Name");
    println!("------ ------ ------ ----- ------ ----------------------------------");

    let mut address: usize = 0;
    loop {
        if files.len() == 0 {
            break; // no more files to arrange
        }

        let file = files[0].clone();

        if file.no_output {
            files.remove(0);
            continue; // skip it
        }

        let last_address = address + file.size - 1;
        println!("{:05x}H {:05x}H {:6} {:5} {:5}  {}",
            address,
            last_address,
            file.size,
            file.width,
            file.height,
            file.path);

        files.remove(0);
        address += file.size;
    }
}

fn widen_color(color: &Rgb<u8>) -> Rgb<u8> {
    Rgb([widen_component(color[0]), widen_component(color[1]), widen_component(color[2])])
}

fn widen_component(component: u8) -> u8 {
    component << 6 | component << 4 | component << 2 | component
}