use anyhow::{Context, Result};
use pico_args::Arguments;
use std::time::Instant;

use crate::days::*;

mod days;

macro_rules! day {
    ($d:tt) => {
        || {
            $d::solve();
            $d::solve_extra();
        }
    };
}

fn main() -> Result<()> {
    let mut args = Arguments::from_env();

    let day = args
        .value_from_fn("--day", |val| val.parse::<u32>())
        .context("Did not get valid --day parameter value")?;

    let task = {
        match day {
            1 => day!(day_1),
            2 => day!(day_2),
            3 => day!(day_3),
            4 => day!(day_4),
            5 => day!(day_5),
            6 => day!(day_6),
            _ => anyhow::bail!("this day is not yet implemented"),
        }
    };
    println!("Running day {}", day);
    println!("¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯");
    solver(task);

    Ok(())
}

fn solver<F>(f: F)
where
    F: Fn(),
{
    let start = Instant::now();
    f();
    let time = start.elapsed();
    println!("__________________________________________");
    println!("Time it took: {:03} seconds.", time.as_secs_f32());
}
