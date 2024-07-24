use nalgebra::Vector3;
pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    background_color: u32,
    current_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Framebuffer {
        let buffer_size = width * height;
        let buffer = vec![0; buffer_size]; // Inicializa el buffer con 0 (representa color negro)
        Framebuffer {
            width,
            height,
            buffer,
            background_color: 0xFF000000, // Fondo negro transparente (RGBA: 255, 0, 0, 0)
            current_color: 0xFFFFFFFF,     // Color blanco opaco (RGBA: 255, 255, 255, 255)
        }
    }

    pub fn clear(&mut self) {
        for pixel in &mut self.buffer {
            *pixel = self.background_color;
        }
    }

    pub fn point(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.buffer[index] = self.current_color;
        }
    }

    pub fn point_vertex(&mut self, vertex: Vector3<f32>) {
        let x = vertex.x.round() as usize;
        let y = vertex.y.round() as usize;
        self.point(x, y);
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    pub fn flip_vertical(&mut self) {
        let temp_buffer = self.buffer.clone(); // Clonamos el buffer actual

        for y in 0..self.height {
            for x in 0..self.width {
                let index = y * self.width + x;
                let flipped_index =(self.height - 1 - y) * self.width + x;
                self.buffer[index] = temp_buffer[flipped_index];
            }
        }
    }
}
