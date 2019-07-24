mod cell;
mod maze_gen;
use std::io::stdin;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

fn get_dimension() -> usize {
    loop {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<usize>() {
                    Ok(n) if n % 2 == 0 => println!("Enter an odd number. Try again:"),
                    Ok(n) => return n,
                    Err(_) => println!("Invalid input given. Try again:")
                }
            },
            Err(e) => panic!("Error getting input: {}", e)
        }
    }
}

fn get_fileinput() -> Option<String> {
    loop {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim() == "" {
                    return None; //if given empty input
                } else {
                    return Some(input); //some input given
                }
            },
            Err(e) => panic!("Error getting input: {}", e)
        }
    }
}

fn get_input() -> (usize, usize, Option<String>) {
    println!("Enter dimensions of maze:");
    println!("Enter width:");
    let width = get_dimension();
    println!("Enter height:");
    let height = get_dimension();
    println!("Enter filename to save the maze to or leave empty to not save:");
    let filename = get_fileinput();
    return (width, height, filename);
}

fn print_maze(cells: Vec<cell::Cell>, width: usize, height: usize) -> String {
    let mut maze_string = String::with_capacity((width + 1) * height);
    println!("");
    for i in 0..width {
        for j in 0..height {
            if cells[i * height + j].is_wall {
                maze_string.push('#');
            } else {
                maze_string.push(' ');
            }
        }
        maze_string.push('\n');
    }
    println!("{}", maze_string);
    return maze_string;
}

fn save_maze(maze_string: String, filename: &str) -> Result<()> {
    let mut file = File::create(filename.trim())?;
    file.write_all(maze_string.as_bytes())?;
    return Ok(());
}

fn main() {
    let (width, height, filename) = get_input();
    println!("Starting generation with width: {} and height: {}", width, height);
    let mut default_cell = cell::Cell::new();
    let mut cells: Vec<cell::Cell> = Vec::with_capacity(width * height);
    for i in 0..width {
        for j in 0..height {
            default_cell.x = i;
            default_cell.y = j;
            if (i * j) % 2 == 0 {
                default_cell.is_wall = true;
            } else {
                default_cell.is_wall = false;
            }
            cells.push(default_cell);
        }
    }
    let maze = maze_gen::start_gen(cells, width, height);
    let maze_string = print_maze(maze, width, height);
    match filename {
        Some(s) => match save_maze(maze_string, &s) {
            Ok(_) => println!("Saved maze to {}", s),
            Err(e) => println!("Failed to save maze. Error: {}", e)
        },
        None => ()
    }
}