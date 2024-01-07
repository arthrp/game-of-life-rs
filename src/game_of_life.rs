const WIDTH: usize = 20;
const HEIGHT: usize = 20;

pub struct GameOfLife {
    board: [[bool;WIDTH];HEIGHT]
}

impl GameOfLife {
    pub fn new() -> GameOfLife {
        GameOfLife {
            board: [[false;WIDTH];HEIGHT]
        }
    }

    pub fn init(&mut self){
        self.board[2][2] = true;
        self.board[2][3] = true;
        self.board[2][4] = true;
    }

    pub fn print(&self){
        let mut str = "".to_string();
        for row in self.board.iter() {
            for &elem in row.iter() {
                let new_str = if elem { "■ "} else { "□ " };
                str.push_str(new_str);
            }
            str.push_str("\n");
        }

        println!("{}", str);
    }

    fn count_neighbors(&self, cell_x: i32, cell_y: i32) -> i32 {
        let mut count = 0;
        let h = HEIGHT as i32;
        let w = WIDTH as i32;

        for dy in -1..=1 {
            let y = (cell_y + dy + h) % h;
            for dx in -1..=1 {
                if dx == 0 && dy == 0 { continue; } //Not including the cell itself
                let x = (cell_x + dx + w) % w;
                count += if self.board[x as usize][y as usize] { 1 } else { 0 };
            }
        }

        count
    }

    pub fn make_turn(&mut self){
        let mut new_board = [[false;WIDTH];HEIGHT];
        
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let n = self.count_neighbors(x as i32, y as i32);
                let cell = self.board[x][y];

                new_board[x][y] = if (cell && (n == 2 || n == 3)) || (!cell && n == 3) { true } else { false };
            }
        }

        self.board = new_board;
    }
}