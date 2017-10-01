use snake::snake::Snake;
use snake::dir::Dir;
use snake::pos::Pos;

pub struct Game {
    width: u32,
    height: u32,
    snake: Snake,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Game {
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

    pub fn step(&mut self) {
        if self.is_game_over() {
            return;
        }

        self.snake.step();
    }

    pub fn is_game_over(&self) -> bool {
        let head = self.snake.pos();
        if head.x == 0 ||
            head.y == 0 ||
            head.x == self.width() - 1 ||
            head.y == self.height() - 1 {
            panic!("x:{} y:{}", head.x, head.y);
            return true;
        }

        for seg in self.snake.tail() {
            if seg == head {
                return true;
            }
        }

        false
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
        self.snake.down()
    }

    pub fn left(&mut self) {
        self.snake.left()
    }

    pub fn right(&mut self) {
        self.snake.right()
    }

    pub fn snake(&self) -> &Snake {
        &self.snake
    }
}
