use std::time::{Duration, Instant};

use snake::game::Game;

use rustty::{Terminal, Event, HasSize, CellAccessor};
use rustty::ui::{Painter, Dialog, Widget, Alignable, HorizontalAlign, VerticalAlign};

//const BLOCK: char = '\u{25AA}';
const BLOCK: char = '\u{2588}';

pub struct Ui {
    term: Terminal,
    canvas: Widget,
    game: Game,
}

impl Ui {
    pub fn new() -> Ui {
        let term = Terminal::new().unwrap();
        let width = term.size().0;
        let height = term.size().1 - 4;
        let mut canvas = Widget::new(width, height);
        canvas.align(&term, HorizontalAlign::Left, VerticalAlign::Top, 0);
        Ui {
            term: term,
            canvas: canvas,
            game: Game::new(width as u32, height as u32),
        }
    }

    pub fn run(&mut self) {
        let step_dur = Duration::from_millis(250);

        let mut timeout = Instant::now() + step_dur;
        'main: loop {
            let now = Instant::now();

            if timeout > now { // TODO fix this mess
                let timeout_diff = timeout - now;
                let evt = self.term.get_event(timeout_diff).unwrap();
                if let Some(Event::Key(ch)) = evt {
                    match ch {
                        'q' => break 'main,
                        'a' => self.game.left(),
                        'd' => self.game.right(),
                        'w' => self.game.up(),
                        's' => self.game.down(),
                        _ => {
                        }
                    }
                } else {
                    self.game.step();
                    timeout = Instant::now() + step_dur;
                }
            } else {
                self.game.step();
                timeout = Instant::now() + step_dur;
            }

            Self::clear(&mut self.canvas);
            Self::draw_snake(&mut self.canvas, &self.game);

            // draw the canvas, dialog window and swap buffers
            self.canvas.draw_into(&mut self.term);
            self.term.swap_buffers().unwrap();
        }
    }

    fn clear(canvas: &mut Widget) {
        let (cols, rows) = canvas.size();
        let (cols, rows) = (cols as isize, rows as isize);

        for i in 0..cols * rows {
            let y = i as isize / cols;
            let x = i as isize % cols;

            let mut cell = canvas.get_mut(x as usize, y as usize).unwrap();
            if x == 0 || y == 0 || x == cols - 1 || y == rows - 1 {
                cell.set_ch(BLOCK);
            } else {
                cell.set_ch(' ');
            }
        }
    }

    fn draw_snake(canvas: &mut Widget, game: &Game) {
        let snake = game.snake();
        let head = snake.pos();
        {
            Self::draw(canvas, head.x as usize, head.y as usize, BLOCK);
        }

        for pos in snake.tail() {
            Self::draw(canvas, pos.x as usize, pos.y as usize, BLOCK);
        }
    }

    fn draw(canvas: &mut Widget, x: usize, y: usize, c: char) {
        let (_, rows) = canvas.size();
        let draw_x = x;
        let draw_y = rows - y;
        let mut opt_cell = canvas.get_mut(draw_x, draw_y);
        match opt_cell {
            Some(ref mut cell) => { cell.set_ch(c); } ,
            None => println!("error mapping x:{} y:{} to dx:{} dy:{}", x, y, draw_x, draw_y),
        };
    }
}
