
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
    fn growing_left() {
        let mut s = Snake::new(Pos { x: 10, y: 11 }, Dir::Left);

        assert_eq!(0, s.tail().len());
        assert_eq!(s.pos().x, 10);
        assert_eq!(s.pos().y, 11);

        s.step();
        s.step();
        s.step();
        s.step();
        s.step();

        assert_eq!(4, s.tail().len());
        assert_eq!(s.pos().x,  5);
        assert_eq!(s.pos().y, 11);
        assert_eq!(s.tail()[0], Pos {x: 9, y: 11});
        assert_eq!(s.tail()[1], Pos {x: 8, y: 11});
        assert_eq!(s.tail()[2], Pos {x: 7, y: 11});
        assert_eq!(s.tail()[3], Pos {x: 6, y: 11});
    }

    #[test]
    fn growing_right() {
        let mut s = Snake::new(Pos { x: 10, y: 11 }, Dir::Right);

        assert_eq!(0, s.tail().len());
        assert_eq!(s.pos().x, 10);
        assert_eq!(s.pos().y, 11);

        s.step();
        s.step();
        s.step();
        s.step();
        s.step();

        assert_eq!(4, s.tail().len());
        assert_eq!(s.pos().x, 15);
        assert_eq!(s.pos().y, 11);
        assert_eq!(s.tail()[0], Pos {x: 11, y: 11});
        assert_eq!(s.tail()[1], Pos {x: 12, y: 11});
        assert_eq!(s.tail()[2], Pos {x: 13, y: 11});
        assert_eq!(s.tail()[3], Pos {x: 14, y: 11});
    }
}
