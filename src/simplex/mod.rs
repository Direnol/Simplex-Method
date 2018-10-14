//!
//!
//!
//!
//!
//!
//!
//!

use serde_json;
use simplex::structs::Simplex;
use simplex::structs::SimplexMethod;
use std::collections::HashMap;
use std::fs::File;
use std::io::Result;

pub mod structs;

#[derive(Deserialize, Debug)]
struct Cond { vars: HashMap<String, f64>, sign: String, assign: f64 }

#[derive(Deserialize, Debug)]
struct Parameters {
    pub method: String,
    pub x: Vec<f64>,
    pub cond: Vec<Cond>,
}

pub fn load(f: File) -> Result<Simplex> {
    let conf = serde_json::from_reader::<File, Parameters>(f)?;
    let count_x = conf.x.len() as usize;
    let count_all_vars = conf.cond.len() + count_x;
    let mut matrix: Box<Vec<Vec<f64>>> = Box::new(
        Vec::with_capacity(count_all_vars)
    );
    let mut res = Vec::with_capacity(conf.cond.len());

    let mut func = vec![1.0];
    for i in conf.x.iter().map(|x| -x) { func.push(i) }

//    if cfg!(debug_assertions) {
//        println!("Method: {:?}\nxs: {:?}\n", conf.method, conf.x);
//    }
    for (line, c) in conf.cond.iter().enumerate() {
        if c.vars.len() == 0 {
            panic!("Not enough vars in {} row: {:?}", line, c.vars);
        }
        func.push(0.0);

//        if cfg!(debug_assertions) { println!("row {}: {:?}", line, c); }

        let mut row = Vec::with_capacity((count_x + 1) as usize);
        row.push(0.0);
        for i in 1..(count_all_vars + 1) {
            row.push(match c.vars.get(&format!("{}", i)) {
                Option::Some(k) => *k,
                Option::None => 0f64
            });
        }
        row[line + count_x + 1] = match c.sign.as_ref() {
            ">" | ">=" => -1.0,
            "<" | "<=" => 1.0,
            _ => panic!("Bad row {}: {:?}", line, c)
        };
        res.push(c.assign);
        matrix.push(row);
    }
    matrix.push(func);
    res.push(0.0);
    Ok(Simplex {
        mat: matrix,
        action: SimplexMethod::new(&conf.method),
        res: Box::new(res),
    })
}