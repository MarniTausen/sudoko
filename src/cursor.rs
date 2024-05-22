
#[derive(Debug, Hash, Clone, Copy)]
pub struct Cursor {
    pub row: usize,
    pub col: usize
}

impl Cursor {
    pub fn new() -> Cursor {
        Cursor {
            row: 1,
            col: 1
        }
    }

    pub fn move_right(&mut self) {
        self.col = match self.col {
            1..=8 => self.col + 1,
            9 => self.col,
            _ => 9
        }
    }

    pub fn move_left(&mut self) {
        match self.col {
            2..=9 => self.col = self.col - 1,
            _ => (),
        }
    }

    pub fn move_down(&mut self) {
        match self.row {
            1..=8 => self.row = self.row + 1,
            _ => ()
        }
    }

    pub fn move_up(&mut self) {
        match self.row {
            2..=9 => self.row = self.row - 1,
            _ => ()
        }
    }

}