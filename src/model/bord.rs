use super::grid;

#[allow(unused)]
pub struct Bord {
    list: Vec<Vec<Box<dyn Grid>>>,
}

pub trait Grid {
    fn flip(&mut self);
    fn put(&mut self, color: grid::Color) -> bool;
}

pub fn build_bord() -> Bord {
    let mut list: Vec<Vec<Box<dyn Grid>>> = Vec::new();
    for _ in 0..8 {
        list.push(Vec::new());
        for _ in 0..8 {
            list.last_mut().unwrap().push(Box::new(grid::Grid {
                on_disc: false,
                disc_color: None,
            }));
        }
    }
    list[3][3].as_mut().put(grid::Color::White);
    list[4][4].as_mut().put(grid::Color::White);
    list[3][4].as_mut().put(grid::Color::Black);
    list[4][3].as_mut().put(grid::Color::Black);
    Bord { list }
}

impl Bord {
    fn put(&mut self, horizontal: usize, vertical: usize, color: grid::Color) -> bool {
        self.list[horizontal][vertical].put(color)
    }

    fn put_as_name(&mut self, horizontal: char, vertical: usize, color: grid::Color) -> bool {
        self.list[horizontal as usize][vertical].put(color)
    }
}
