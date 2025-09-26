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


let poligono2 = [
    (321, 335),
    (288, 286),
    (339, 251),
    (374, 302),
];



framebuffer.set_current_color(Color::BLUE);
fill_polygon(&mut framebuffer, &poligono2);



    while !rl.window_should_close() {
        framebuffer.swap_buffers(&mut rl, &thread);
    }
}
