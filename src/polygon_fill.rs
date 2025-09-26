use crate::framebuffer::Framebuffer; // Asegúrate de que el path sea correcto

pub fn fill_polygon(framebuffer: &mut Framebuffer, vertices: &[(i32, i32)]) {
    if vertices.len() < 3 {
        return; // No es un polígono válido
    }

    // Encontrar el Y mínimo y máximo del polígono
    let mut min_y = vertices[0].1;
    let mut max_y = vertices[0].1;
    for &(_, y) in vertices.iter() {
        if y < min_y { min_y = y; }
        if y > max_y { max_y = y; }
    }

    // Para cada scanline
    for y in min_y..=max_y {
        let mut intersections: Vec<i32> = Vec::new();

        // Encontrar las intersecciones del scanline con cada arista
        for i in 0..vertices.len() {
            let (x0, y0) = vertices[i];
            let (x1, y1) = vertices[(i + 1) % vertices.len()];

            if (y0 <= y && y1 > y) || (y1 <= y && y0 > y) {
                let x = x0 + ((y - y0) * (x1 - x0)) / (y1 - y0);
                intersections.push(x);
            }
        }

        // Ordenar las intersecciones
        intersections.sort_unstable();

        // Dibujar píxeles entre pares de intersecciones
        let mut i = 0;
        while i + 1 < intersections.len() {
            for x in intersections[i]..=intersections[i + 1] {
                framebuffer.set_pixel(x, y);
            }
            i += 2;
        }
    }
}
