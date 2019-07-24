use crate::cell::Cell;
use rand;
use rand::Rng;

pub fn start_gen(mut cells: Vec<Cell>, width: usize, height: usize) -> Vec<Cell> {
  //make starting cell and set it parent to itself
  let start = cells[width + 1];
  cells[width + 1].parent_x = Some(1);
  cells[width + 1].parent_y = Some(1);
  let mut end = start;
  let mut rng = rand::thread_rng();
  loop {
    end = link(end, &mut cells, width, height, &mut rng);
    //when the generation backtracks all the way to the start, is generation finished
    if end.x == start.x && end.y == start.y {
      let done = make_entrances(cells, width, height);
      return done;
    }
  }
}

fn make_entrances(mut cells: Vec<Cell>, width: usize, height: usize) -> Vec<Cell> {
  cells[1].is_wall = false;
  cells[width * height - 2].is_wall = false;
  return cells;
}

fn link<R: Rng>(mut cell: Cell, 
                cells: &mut Vec<Cell>,
                width: usize, 
                height: usize,
                rng: &mut R) -> Cell {
  //coordinates of the cell under exploration
  let mut x: usize;
  let mut y: usize;
  while cell.dirs > 0 {
    let dir = 1 << rng.gen_range(0, 4);
    if cell.dirs & dir == 0 { 
      continue; //if direction is already explored, pick new direction
    }
    cell.dirs = cell.dirs & !dir; //mark the direction as explored
    match dir {
      1 => { //up
        if cell.y >= 2 {
          x = cell.x;
          y = cell.y - 2;
        } else {
          continue;
        }
      },
      2 => { //right
        if cell.x < width - 2 {
          x = cell.x + 2;
          y = cell.y;
        } else {
          continue;
        }
      },
      4 => { //down
        if cell.y < height - 2 {
          x = cell.x;
          y = cell.y + 2;
        } else {
          continue;
        }
      },
      8 => { //left
        if cell.x >= 2 {
          x = cell.x - 2;
          y = cell.y;
        } else {
          continue;
        }
      },
      _ => continue
    }
    let idx = x * height + y;
    if !cells[idx].is_wall {
      if cells[idx].parent_x.is_some() {
        continue; //if is already linked, abort and pick new direction
      }
      //else adopt the cell
      cells[idx].parent_x = Some(cell.x);
      cells[idx].parent_y = Some(cell.y);
      //remove the wall between the parent and child cells
      let x_diff = (x as i32) - (cell.x as i32);
      let y_diff = (y as i32) - (cell.y as i32);
      let wall_x = ((cell.x as i32) + x_diff / 2) as usize;
      let wall_y = ((cell.y as i32) + y_diff / 2) as usize;
      cells[wall_x * height + wall_y].is_wall = false;
      return cells[idx];
    }
  }
  //return parent of cell (go back) if no direction can be found
  return cells[cell.parent_x.unwrap() * height + cell.parent_y.unwrap()];
}