use std::{thread, time};

extern crate rand;
use rand::Rng;

const SLEEP_MILLIS: u64 = 100;

const LENGTH: usize = 80;
type Cells = [i32; LENGTH];

fn main() {
    let mut rng = rand::thread_rng();
    let mut cells = [0; LENGTH];
    for i in 0..LENGTH {
        cells[i] = rng.gen_range(0, 3) - 1;
    }

    print_cells(&cells);
    sleep();

    loop {
        cells = update(&cells);
        print_cells(&cells);
        sleep();
    }
}

fn sleep() {
    thread::sleep(time::Duration::from_millis(SLEEP_MILLIS));
}

fn print_cells(cells: &Cells) {
    for i in 0..LENGTH {
        print!("{:2}", cells[i]);
    }
    println!();
}

fn update(cells: &Cells) -> Cells {
    let mut new_cells = [0; LENGTH];
    new_cells[0] = new_cell((0, cells[0], cells[1]));
    for i in 1..(LENGTH - 1) {
        new_cells[i] = new_cell((cells[i - 1], cells[i], cells[i + 1]));
    }
    new_cells[LENGTH - 1] = new_cell((cells[LENGTH - 2], cells[LENGTH - 1], 0));

    new_cells
}

fn new_cell(neighbor_cells: (i32, i32, i32)) -> i32 {
    match neighbor_cells {
        (0, 0, -1) => -1,
        (0, 0, _) => 0,
        (0, 1, 1) => 1,
        (0, 1, _) => 0,
        (1, 0, -1) => 0,
        (1, 0, _) => 1,
        (1, 1, 1) => 1,
        (1, 1, _) => 0,
        (-1, 0, -1) => -1,
        (-1, 0, _) => 0,
        (-1, 1, 1) => 1,
        (-1, 1, _) => 0,
        (-1, -1, _) => -1,
        (_, -1, _) => 0,
        _ => unreachable!(),
    }
}

#[allow(unused)]
fn equal_cells(cells1: &Cells, cells2: &Cells) -> bool {
    for (c1, c2) in cells1.iter().zip(cells2.iter()) {
        if c1 != c2 {
            return false;
        }
    }
    true
}
