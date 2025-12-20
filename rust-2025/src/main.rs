#![feature(range_into_bounds)]
pub mod days;

macro_rules! run {
    ($($day:ident),*) => {
        $(
            days::$day::main();
        )*
    };
}

fn main() {
    run!(
        day5
    );
}
