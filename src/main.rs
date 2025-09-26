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


let poligono3 = [
    (377, 249),
    (411, 197),
    (436, 249),
];



framebuffer.set_current_color(Color::GREEN);
fill_polygon(&mut framebuffer, &poligono3);



    while !rl.window_should_close() {
        framebuffer.swap_buffers(&mut rl, &thread);
    }
}
