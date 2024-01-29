
pub mod solver;

use solver::Solver;

fn main() {

let s = Solver::load_problem("ini/config.ini".to_string());
println!("{}", s.solve());
}
