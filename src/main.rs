extern crate rsimpl;

use rsimpl::simplex;
use std::fs::File;
use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    let args = std::env::args().collect::<Vec<_>>();
    let conf_file = args.get(1).ok_or("Not enough argument".to_string())?;
    let file = File::open(conf_file)?;
    let mut simplex = simplex::load(file)?;
    println!("{}", simplex);
    let res = simplex.run().ok_or("No exist solution")?;
    println!("{}\nResult {:?}\n", simplex, res);
    Ok(())
}
