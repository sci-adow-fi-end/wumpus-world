
pub mod solver;

use solver::Solver;

fn main() {

let s = Solver::load_problem("ini/knowledge_base.ini".to_string());
println!("{}", s.solve());
}
