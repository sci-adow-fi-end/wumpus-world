
pub mod solver;

use solver::Solver;
use std::time::Instant;

fn main() {

    let s = Solver::load_problem("ini/problem.ini".to_string());
    let start_time = Instant::now();
    for _i in 1..10000{
        s.solve();
    }
    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("{}", elapsed_time.as_micros()/10000);
}
