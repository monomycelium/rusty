use good_lp::{constraint, default_solver, variables, Solution, SolverModel};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    variables! {
        vars:
            x <= 500;
            y <= 500;
    }

    let solution = vars
        .maximise(x * 25 + y * 30)
        .using(default_solver)
        .with(constraint!(x + y <= 800))
        .with(constraint!(x >= y / 3))
        .solve()?;

    println!(
        "number of chocolate chip cookies to bake:\t{}
number of oatmeal raisin cookies to bake:\t{}
total profit ($):\t\t\t\t{:2}",
        solution.value(x),
        solution.value(y),
        solution.eval(x * 0.25 + y * 0.30)
    );

    Ok(())
}
