use snake::snake::Snake;
use snake::dir::Dir;
use snake::pos::Pos;

use rand::{Rng, ThreadRng};

type Strawberries = Vec<Pos>;

pub struct Game {
    width: u32,
    height: u32,
    points: u32,
    snake: Snake,
    strawberries: Strawberries,

    rng: ThreadRng,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        Game {
            width: width,
            height: height,
            points: 0,
            snake: Snake::new(
                Pos {
                    x: width / 2,
                    y: height / 2,
                },
                Dir::Left,
            ),
            strawberries: vec![],
            rng: ::rand::thread_rng(),
        }
    }

    pub fn step(&mut self) {
        if self.is_game_over() {
            return;
        }

        self.snake.step();
        Self::check_strawberries(
            self.width(),
            self.height(),
            &self.snake,
            &mut self.strawberries,
            &mut self.points,
            &mut self.rng,
        );
    }

    // TODO shrink the arguments
    fn check_strawberries(
        width: u32,
        height: u32,
        snake: &Snake,
        strawberries: &mut Strawberries,
        points: &mut u32,
        rng: &mut ThreadRng,
    ) {
        let strawberry = strawberries.iter().position(|s| s == snake.pos());
        match strawberry {
            Some(index) => {
                strawberries.remove(index);
                *points += 1;
            }
            _ => {}
        }

        while strawberries.len() < 1 {
            // TODO more than one strawberry
            let new_pos = Pos {
                x: rng.gen_range(1, width - 1),
                y: rng.gen_range(1, height - 1),
            };

            if snake.pos().x == new_pos.x && snake.pos().y == new_pos.y ||
                strawberries.contains(&new_pos)
            {
                continue;
            }

            strawberries.push(new_pos);
        }
    }

    pub fn is_game_over(&self) -> bool {
        let head = self.snake.pos();
        if head.x == 0 || head.y == 0 || head.x == self.width() - 1 || head.y == self.height() - 1 {
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

    pub fn strawberries(&self) -> &Strawberries {
        &self.strawberries
    }
}
