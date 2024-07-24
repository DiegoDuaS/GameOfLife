// Define la estructura del juego de la vida
pub struct GameOfLife {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<bool>>, // Matriz para almacenar el estado de las células
}

impl GameOfLife {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![false; width]; height];
        Self { width, height, grid }
    }

    // Función para inicializar el tablero con un patrón específico
    pub fn set_pattern(&mut self, pattern: &[(usize, usize)]) {
        for &(x, y) in pattern {
            if x < self.width && y < self.height {
                self.grid[y][x] = true;
            }
        }
    }

    pub fn update(&mut self) {
        let mut new_grid = self.grid.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let live_neighbors = self.live_neighbor_count(x, y);

                new_grid[y][x] = match (self.grid[y][x], live_neighbors) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
            }
        }

        self.grid = new_grid;
    }

    fn live_neighbor_count(&self, x: usize, y: usize) -> usize {
        let mut count = 0;

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x.wrapping_add(dx as usize);
                let ny = y.wrapping_add(dy as usize);

                if nx < self.width && ny < self.height && self.grid[ny][nx] {
                    count += 1;
                }
            }
        }

        count
    }
}
