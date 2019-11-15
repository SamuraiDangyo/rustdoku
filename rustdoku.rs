/*
rustdoku, a 9x9 sudoku solver
Copyright (C) 2019 Toni Helminen

rustdoku is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

rustdoku is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <http://www.gnu.org/licenses/>.
*/

use std::fs;
use std::time::{Instant};

struct Sudoku {
  width:  usize,
  height: usize,
  board:  Vec<i32>
}

fn print_board(sudoku: Sudoku) {
  println!("~~~~ {}x{} ~~~~", sudoku.width, sudoku.height);
  println!("-------------");
  for (i, x) in sudoku.board.iter().enumerate() {
    if i % 9 == 0 && (i / 9 == 3 || i / 9 == 6) {println!("-------------");}
    if i % 9 == 0 || i % 9 == 3 || i % 9 == 6 {print!("|");}
    print!("{}", x);
    if (i + 1) % 9 == 0 {println!("|");}
  }
  println!("-------------");
}

fn is_legal(sudoku: &Sudoku, pos: i32, num: i32) -> bool {
  if sudoku.board[pos as usize] != 0 { return false; }

  let x: usize = (pos as usize) % sudoku.width;
  let y: usize = (pos as usize) / sudoku.width;

  for i in 0..9 {
    if sudoku.board[sudoku.height * y + i] == num { return false; } // >>>
    if sudoku.board[x + 9 * i] == num { return false; } // ^^^
  }

  let little_x: usize = x / 3;
  let little_y: usize = y / 3;

  for i in 0..3 {
    for j in 0..3 {
      let p: usize = (3 * little_y + i) * 9 + (3 * little_x + j);
      if sudoku.board[p] == num { return false; }
    }
  }

  return true;
}

fn is_legal_board(sudoku: &mut Sudoku) -> bool {
  for i in 0..81 {
    if sudoku.board[i] == 0 { continue; }

    let t = sudoku.board[i]; // hack to save lines
    sudoku.board[i] = 0;

    if ! is_legal(&sudoku, i as i32, t) {
      sudoku.board[i] = t;
      return false;
    }

    sudoku.board[i] = t;
  }

  return true;
}

// Just brute force it through...
fn fill_sudoku(sudoku: &mut Sudoku, pos: usize) -> bool {
  if pos >= 81 { return true; }

  if sudoku.board[pos] != 0 { return fill_sudoku(sudoku, pos + 1); }

  for n in 1..10 {
    if is_legal(&sudoku, pos as i32, n) {
      sudoku.board[pos] = n;
      let r: bool = fill_sudoku(sudoku, pos + 1);
      if r == true { return true; }
      sudoku.board[pos] = 0;
    }
  }

  return false;
}

fn read_sudoku(filename: &str, sudoku: &mut Sudoku) {
  let contents = fs::read_to_string(filename).expect("Error reading the file!");

  for c in contents.chars() {
    if c == '\n' || c == ' ' {
      // pass
    } else if c == '_' {
      sudoku.board.push(0);
    } else {
      let s = c.to_string();
      sudoku.board.push(s.parse::<i32>().unwrap());
    }
  }

  assert!(sudoku.board.len() == 81, "Illegal board");
}

pub fn bench() {
  let mut sudoku: Sudoku = Sudoku { width: 9, height: 9, board: vec![] };

  for _i in 0..81 { sudoku.board.push(0); }

  println!("> Benching ...");
  if fill_sudoku(&mut sudoku, 0) {
    println!("> Done!");
    let start = Instant::now();
    print_board(sudoku);
    let duration = start.elapsed();
    println!("> Time elapsed: {:?}", duration);
  }
}

pub fn solve_sudoku(filename: &str) {
  let mut sudoku: Sudoku = Sudoku { width: 9, height: 9, board: vec![] };

  read_sudoku(filename, &mut sudoku);

  assert!(is_legal_board(&mut sudoku), "Illegal board");

  println!("> Solving sudoku ...");
  if fill_sudoku(&mut sudoku, 0) {
    println!("> Done!");
    let start = Instant::now();
    print_board(sudoku);
    let duration = start.elapsed();
    println!("> Time elapsed: {:?}", duration);
  } else {
    assert!(false, "Unsolvable");
  }
}