#[derive(Clone, Copy)]
pub struct Cell {
  pub x: usize,
  pub y: usize,
  pub is_wall: bool,
  pub parent_x: Option<usize>,
  pub parent_y: Option<usize>,
  pub dirs: usize
}

impl Cell {
  pub fn new() -> Cell {
    //directions are in this order: up, right, down, left
    let default_dirs = 15; //4 least significant bit set
    let default_cell = Cell {
        x: 0,
        y: 0,
        is_wall: false,
        parent_x: None,
        parent_y: None,
        dirs: default_dirs
    };
    return default_cell;
  }
}
