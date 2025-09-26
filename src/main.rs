mod framebuffer;
mod polygon_fill;

use framebuffer::Framebuffer;
use polygon_fill::fill_polygon;
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Polígono Relleno")
        .build();

    let mut framebuffer = Framebuffer::new(800, 600, Color::BLACK);
    framebuffer.set_current_color(Color::RED);


let poligono4 = [
    (413, 177),
    (448, 159),
    (502, 88),
    (553, 53),
    (535, 36),
    (676, 37),
    (660, 52),
    (750, 145),
    (761, 179),
    (672, 192),
    (659, 214),
    (615, 214),
    (632, 230),
    (580, 230),
    (597, 215),
    (552, 214),
    (517, 144),
    (466, 180),
];

framebuffer.set_current_color(Color::YELLOW);
fill_polygon(&mut framebuffer, &poligono4);



    while !rl.window_should_close() {
        framebuffer.swap_buffers(&mut rl, &thread);
    }
}
