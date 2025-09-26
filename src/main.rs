mod framebuffer;
mod polygon_fill;
use framebuffer::Framebuffer;
use polygon_fill::fill_polygon;
use raylib::prelude::*;
use std::fs::File;
use std::io::{Write, Result};

// BMP file header structure
#[repr(C, packed)]
struct BMPFileHeader {
    file_type: u16,      // "BM"
    file_size: u32,      // Size of the file in bytes
    reserved1: u16,      // Reserved
    reserved2: u16,      // Reserved
    offset_data: u32,    // Offset to start of pixel data
}

#[repr(C, packed)]
struct BMPInfoHeader {
    size: u32,           // Size of this header
    width: i32,          // Width of the image
    height: i32,         // Height of the image
    planes: u16,         // Number of color planes
    bit_count: u16,      // Bits per pixel
    compression: u32,    // Compression method
    size_image: u32,     // Size of the image data
    x_pixels_per_m: i32, // Horizontal resolution
    y_pixels_per_m: i32, // Vertical resolution
    colors_used: u32,    // Number of colors in palette
    colors_important: u32, // Important colors
}

impl Framebuffer {
    pub fn save_as_bmp(&mut self, filename: &str) -> Result<()> {
        let width = self.width;
        let height = self.height;
        let row_padded = (width * 3 + 3) & !3; // Row must be multiple of 4 bytes
        let image_size = row_padded * height;
        let file_size = 54 + image_size; // 54 = sizeof(BMPFileHeader) + sizeof(BMPInfoHeader)

        let file_header = BMPFileHeader {
            file_type: 0x4D42, // "BM" in little endian
            file_size: file_size as u32,
            reserved1: 0,
            reserved2: 0,
            offset_data: 54,
        };

        let info_header = BMPInfoHeader {
            size: 40,
            width,
            height,
            planes: 1,
            bit_count: 24, // 24 bits per pixel (RGB)
            compression: 0, // No compression
            size_image: image_size as u32,
            x_pixels_per_m: 2835, // 72 DPI
            y_pixels_per_m: 2835, // 72 DPI
            colors_used: 0,
            colors_important: 0,
        };

        let mut file = File::create(filename)?;

        // Write file header
        unsafe {
            let header_bytes = std::slice::from_raw_parts(
                &file_header as *const BMPFileHeader as *const u8,
                std::mem::size_of::<BMPFileHeader>(),
            );
            file.write_all(header_bytes)?;
        }

        // Write info header
        unsafe {
            let header_bytes = std::slice::from_raw_parts(
                &info_header as *const BMPInfoHeader as *const u8,
                std::mem::size_of::<BMPInfoHeader>(),
            );
            file.write_all(header_bytes)?;
        }

        // Write pixel data (BMP format stores pixels bottom-to-top)
        let padding = vec![0u8; (row_padded - width * 3) as usize];

        for y in (0..height).rev() {
            for x in 0..width {
                // Get pixel color from the raylib Image
                let color = self.color_buffer.get_color(x, y);
                // BMP uses BGR format instead of RGB
                file.write_all(&[color.b, color.g, color.r])?;
            }
            // Write padding to make row length multiple of 4
            if !padding.is_empty() {
                file.write_all(&padding)?;
            }
        }

        Ok(())
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Polígono Relleno")
        .build();

    let mut framebuffer = Framebuffer::new(800, 600, Color::BLACK);
    framebuffer.set_current_color(Color::RED);

    let poligono1 = [(165, 380), (185, 360), (180, 330), (207, 345), (233, 330), (230, 360), (250, 380), (220, 385), (205, 410), (193, 383),];

    framebuffer.set_current_color(Color::RED);
    fill_polygon(&mut framebuffer, &poligono1);

    // Save the framebuffer as BMP file
    match framebuffer.save_as_bmp("out.bmp") {
        Ok(()) => println!("BMP file saved successfully as 'out.bmp'"),
        Err(e) => eprintln!("Error saving BMP file: {}", e),
    }

    while !rl.window_should_close() {
        framebuffer.swap_buffers(&mut rl, &thread);
    }
}
