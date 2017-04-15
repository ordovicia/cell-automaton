use std::{thread, time};

extern crate rand;
use rand::Rng;

const TRANSITION: [i32; 8] = [0, 1, 1, 0, 1, 1, 1, 0];

const SLEEP_MILLIS: u64 = 100;

const LENGTH: usize = 160;
type Cells = [i32; LENGTH];

fn main() {
    let mut rng = rand::thread_rng();
    let mut cells = [0; LENGTH];
    for i in 0..LENGTH {
        cells[i] = rng.gen_range(0, 2);
    }

    print_cells(&cells);
    sleep();

    loop {
        cells = update(&cells);
        print_cells(&cells);
        sleep();
    }
}

#[inline]
fn sleep() {
    thread::sleep(time::Duration::from_millis(SLEEP_MILLIS));
}

fn print_cells(cells: &Cells) {
    for i in 0..LENGTH {
        print!("{}", if cells[i] == 1 { "x" } else { " " });
    }
    println!();
}

fn update(cells: &Cells) -> Cells {
    let mut new_cells = *cells;
    for i in 1..(LENGTH - 1) {
        new_cells[i] = new_cell((cells[i - 1], cells[i], cells[i + 1]));
    }

    new_cells
}

fn new_cell(neighbor_cells: (i32, i32, i32)) -> i32 {
    match neighbor_cells {
        (1, 1, 1) => TRANSITION[0],
        (1, 1, 0) => TRANSITION[1],
        (1, 0, 1) => TRANSITION[2],
        (1, 0, 0) => TRANSITION[3],
        (0, 1, 1) => TRANSITION[4],
        (0, 1, 0) => TRANSITION[5],
        (0, 0, 1) => TRANSITION[6],
        (0, 0, 0) => TRANSITION[7],
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
