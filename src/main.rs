use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
mod framebuffer;
use framebuffer::Framebuffer;

mod game_of_life;
use game_of_life::GameOfLife;

fn render(framebuffer: &mut Framebuffer, game: &GameOfLife) {
    framebuffer.set_background_color(0x333355);
    framebuffer.clear();

    let cell_size = 4; // Tamaño de cada célula en píxeles

    // Pintar todo el grid
    for y in 0..game.height {
        for x in 0..game.width {
            if game.grid[y][x] {
                // Pintar células vivas
                framebuffer.set_current_color(0xFFDDDD); // Color para células vivas
                for dy in 0..cell_size {
                    for dx in 0..cell_size {
                        let px = x * cell_size + dx;
                        let py = y * cell_size + dy;
                        if px < framebuffer.width && py < framebuffer.height {
                            framebuffer.point(px, py);
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let window_width = 540; // Ajustado para coincidir con el tamaño del framebuffer
    let window_height = 380; // Ajustado para coincidir con el tamaño del framebuffer

    let cell_size = 4; // Tamaño de cada célula en píxeles
    let framebuffer_width = 70 * cell_size; // Número de celdas por tamaño de celda
    let framebuffer_height = 100 * cell_size; // Número de celdas por tamaño de celda

    let frame_delay = Duration::from_millis(100);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let grid_width = framebuffer_width / cell_size;
    let grid_height = framebuffer_height / cell_size;

    let mut game = GameOfLife::new(grid_width, grid_height);

    // Definir el patrón específico
    let mut pattern = vec![
        (26, 7), (27, 7),
        (26, 8), (27, 8),
        (6, 9), (11, 9),(42, 9), (47, 9),
        (4, 10), (5, 10), (7,10), (8,10), (9,10),(10,10),(12,10),(13,10), (40,10), (41,10), (43,10),(44,10),(45,10),(46,10), (48,10), (49,10),
        (6, 11), (11, 11),(42, 11), (47, 11),
        
        (25,16),(28,16),
        (23,17),(25,17),(28,17),(30,17),
        (24,18),(25,18),(28,18),(29,18),
        (2,19),(5,19),(10,19),(13,19),(40,19),(43,19),(48,19),(51,19),
        (0,20), (1,20),(2,20),(5,20),(6,20),(7,20),(8,20),(9,20),(10,20),(13,20),(14,20),(15,20), (38,20), (39,20), (40,20), (43,20), (44,20), (45,20), (46,20), (47,20), (48,20), (51,20), (52,20), (53,20),
        (2,21),(5,21),(10,21),(13,21),(26,21),(27,21),(40,21),(43,21),(48,21),(51,21),
        (26,22),(27,22),
        (21,26),(32,26),
        (21,27),(32,27),
        (20,28),(22,28),(31,28),(33,28),
        (21,29),(32,29),
        (21,30),(32,30),
        (21,31),(32,31),
        (21,32),(32,32),
        (20,33),(22,33),(31,33),(33,33),
        (21,34),(32,34),
        (21,35),(32,35),
    ];

    for i in 0..54{
        pattern.push((i,60));
    }

    // Establecer el patrón
    game.set_pattern(&pattern);

    let mut window = Window::new(
        "Rust Graphics - Game of Life",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        game.update();

        render(&mut framebuffer, &game);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
