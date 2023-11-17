use polars::prelude::*;

fn main() {
    let s: Series = [1, 2, 3].iter().collect();
    eprintln!("{s}")
}
