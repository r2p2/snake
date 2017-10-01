use snake::Snake;
use dir::Dir;
use pos::Pos;

pub struct Game {
    width: u32,
    height: u32,
    snake: Snake,
}

impl Game {
    pub fn new_game(width: u32, height: u32) -> Game {
        Game {
            width: width,
            height: height,
            snake: Snake::new(
                Pos {
                    x: width / 2,
                    y: height / 2,
                },
                Dir::Left,
            ),
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn up(&mut self) {
        self.snake.up()
    }

    pub fn down(&mut self) {
        self.snake.up()
    }

    pub fn left(&mut self) {
        self.snake.up()
    }

    pub fn right(&mut self) {
        self.snake.up()
    }
}
