
use snake::pos::Pos;
use snake::dir::Dir;

pub struct Snake {
    pos: Pos,
    max_len: usize,

    tail: Vec<Pos>,
    dir: Dir,
}

impl Snake {
    pub fn new(pos: Pos, dir: Dir) -> Snake {
        Snake {
            pos: pos,
            max_len: 4,
            dir: dir,

            tail: vec![],
        }
    }

    pub fn grow(&mut self, amount: usize) {
        self.max_len += amount
    }

    pub fn up(&mut self) {
        self.dir = Dir::Up
    }

    pub fn down(&mut self) {
        self.dir = Dir::Down
    }

    pub fn left(&mut self) {
        self.dir = Dir::Left
    }

    pub fn right(&mut self) {
        self.dir = Dir::Right
    }

    pub fn step(&mut self) {
        self.tail.push(self.pos.clone());
        match self.dir {
            Dir::Up => self.pos.y += 1,
            Dir::Down => self.pos.y -= 1,
            Dir::Left => self.pos.x -= 1,
            Dir::Right => self.pos.x += 1,
        }

        while self.tail.len() > self.max_len {
            self.tail.remove(0);
        }
    }

    pub fn pos(&self) -> &Pos {
        &self.pos
    }

    pub fn tail(&self) -> &Vec<Pos> {
        &self.tail
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_snake() {
        let s = Snake::new(Pos { x: 10, y: 11 }, Dir::Left);
        assert_eq!(10, s.pos().x);
        assert_eq!(11, s.pos().y);
        assert_eq!(0, s.tail().len());
    }

    #[test]
    fn move_up() {
        let mut s = Snake::new(Pos { x: 10, y: 11 }, Dir::Left);
        s.up();
        s.step();
        assert_eq!(10, s.pos().x);
        assert_eq!(12, s.pos().y);
    }

    #[test]
    fn move_down() {
        let mut s = Snake::new(Pos { x: 10, y: 11 }, Dir::Left);
        s.down();
        s.step();
        assert_eq!(10, s.pos().x);
        assert_eq!(10, s.pos().y);
    }

    #[test]
    fn move_left() {
        let mut s = Snake::new(Pos { x: 10, y: 11 }, Dir::Left);
        s.left();
        s.step();
        assert_eq!(9, s.pos().x);
        assert_eq!(11, s.pos().y);
    }

    #[test]
    fn move_right() {
        let mut s = Snake::new(Pos { x: 10, y: 11 }, Dir::Left);
        s.right();
        s.step();
        assert_eq!(11, s.pos().x);
        assert_eq!(11, s.pos().y);
    }

    #[test]
    fn growing() {
        let mut s = Snake::new(Pos { x: 10, y: 11 }, Dir::Left);
        s.max_len(3);
        assert_eq!(0, s.tail().len());

        s.step();
        assert_eq!(1, s.tail().len());

        s.step();
        assert_eq!(2, s.tail().len());

        s.step();
        assert_eq!(3, s.tail().len());

        s.step();
        assert_eq!(3, s.tail().len());
    }
}
