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

use std::env;

mod rustdoku;

static NAME: &str = "rustdoku 1.1 by Toni Helminen";

fn print_help() {
  println!("[ # Hlp");
  println!("  Usage: rustdoku [OPTION]... [FILE]...");
  println!("  Solve 9x9 sudokus");
  println!("]\n");

  println!("[ # Cmds");
  println!("  -h(elp)        This help");
  println!("  -v(ersion)     Show version");
  println!("  -bench         Run benchmarks");
  println!("  -f(ile) [FILE] Solve [FILE] 9x9 sudoku");
  println!("                 Provide sudoku as file.txt. See empty.txt");
  println!("]\n");

  println!("[ # Src code, please see:");
  println!("  <https://github.com/SamuraiDangyo/rustdoku/>");
  println!("]");
}

fn commands() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    print_help();
    return;
  }

  let cmd = &args[1];

  if args.len() >= 3 {
    let s = &args[2];
    if cmd == "-f" || cmd == "-file" {
      rustdoku::solve_sudoku(s);
      return;
    }
    assert!(false, "Illegal arguments");
  }

  if args.len() >= 2 {
    if cmd == "-h" || cmd == "-help" {
      print_help();
    } else if cmd == "-v" || cmd == "-version" {
      println!("{}", NAME);
    } else if cmd == "-bench" {
      rustdoku::bench();
    } else {
      assert!(false, "Illegal arguments");
    }
  }
}

fn main() {
  commands();
}
