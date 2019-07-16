mod solver;
use solver::sudoku;
use std::env;

fn main() -> std::io::Result<()> {
	let args: Vec<String> = env::args().collect();

	let filename = args[1].clone();

	let sudoku = sudoku::Sudoku::load_from_file(filename)?;

	match sudoku.solve() {
		Some(solution) => println!("{}", solution),
		None => println!("Hmmm, I'm having a hard time figuring that one out!"),
	}
	Ok(())
}
