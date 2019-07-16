use std::io::*;
use std::fs::File;

#[derive(Debug, Copy, Clone)]
pub struct Sudoku {
	pub grid: [[u32; 9]; 9],
}

impl std::fmt::Display for Sudoku {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for y in 0..9 {
			for x in 0..9 {
				match self.grid[y][x] {
					42 => write!(f, "  .")?,
					num => write!(f, "{:3}", num)?,
				}
				if x % 3 == 2 {
					write!(f, "   ")?;
				}
			}
			if y % 3 == 2 {
				write!(f, "\n")?;
			}
			write!(f, "\n")?;
		}
		Ok(())
	}
}

impl Sudoku {
	fn find_first_tile(&self) -> (usize, usize) {
		for y in 0..9 {
			for x in 0..9 {
				if let 42 = self.grid[y][x] {
					return (x, y);
				}
			}
		}
		(42, 42)
	}

	fn place(&self, option: u32, (x, y): (usize, usize)) -> Option<Self> {
		for xi in 0..9 {
			if xi == x {
				continue;
			}
			if self.grid[y][xi] == option {
				return None;
			}
		}
		for yi in 0..9 {
			if yi == y {
				continue;
			}
			if self.grid[yi][x] == option {
				return None;
			}
		}
		for xi in 0..3 {
			for yi in 0..3 {
				if xi + x / 3 * 3 == x && yi + y / 3 * 3 == y {
					continue;
				}
				if self.grid[yi + y / 3 * 3][xi + x / 3 * 3] == option {
					return None;
				} 
			}
		}
		let mut copy = self.clone();
		copy.grid[y][x] = option;
		Some(copy)
	}

	pub fn solve(&self) -> Option<Self> {
		let first_tile = self.find_first_tile();

		if first_tile == (42, 42) {
			return Some(*self);
		}

		for option in 1..10 {
			if let Some(res) = self.place(option, first_tile) {
				if let Some(solution) = res.solve() {
					return Some(solution);
				}
			}
		}
		None
	}

	pub fn load_from_file(filename: String) -> std::io::Result<Self> {
		let mut file = File::open(filename)?;
		let mut buf: String = String::new();
		file.read_to_string(&mut buf)?;

		match Sudoku::load_from_string(buf) {
			Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
			Ok(sudoku) => Ok(sudoku), 
		}
	}

	pub fn load_from_string(s: String) -> std::result::Result<Self, String> {
		let mut sudoku = Sudoku {
			grid: [[0; 9]; 9],
		};
		let mut x = 0;
		let mut y = 0;
		for c in s.chars() {
			if c == '.' {
				if y == 9 {
					return Err(String::from("Too many characters in sudoku"));
				}
				sudoku.grid[y][x] = 42;
			} else if let Some(n) = c.to_digit(10) {
				if y == 9 {
					return Err(String::from("Too many characters in sudoku"));
				}
				sudoku.grid[y][x] = n;
			} else {
				continue;
			}
			x += 1;
			if x == 9 {
				x = 0;
				y += 1;
			}
		}
		Ok(sudoku)
	}
}