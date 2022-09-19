use super::bord;

#[allow(unused)]
pub struct Grid {
    pub on_disc: bool,
    pub disc_color: Option<Color>,
}

pub enum Color {
    Black,
    White,
}

impl bord::Grid for Grid {
    fn flip(&mut self) {
        match self.disc_color.as_ref().unwrap() {
            Color::Black => {
                self.disc_color = Some(Color::White);
            }
            Color::White => {
                self.disc_color = Some(Color::Black);
            }
        }
    }
    fn put(&mut self, color: Color) -> bool {
        if self.on_disc {
            return false;
        };
        self.on_disc = true;
        self.disc_color = Some(color);
        return true;
    }
}
