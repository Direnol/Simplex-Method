use std::fmt::{Display, Error, Formatter};
use std::ops::{Deref, Index, IndexMut};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum SimplexMethod {
    Min,
    Max,
    None,
}

impl SimplexMethod {
    pub fn new(x: &str) -> SimplexMethod {
        match x {
            "max" => SimplexMethod::Max,
            "min" => SimplexMethod::Min,
            _ => SimplexMethod::None
        }
    }
}

type Row = Vec<f64>;
type Solution = Option<Row>;

#[derive(Debug, Clone)]
pub struct Simplex {
    pub mat: Box<Vec<Row>>,
    pub action: SimplexMethod,
    pub res: Box<Row>,
}

#[derive(Default, Debug)]
struct SimplexSolution {
    // count of vars
    pub n: usize,
    pub names: HashMap<usize, usize>
}

impl Simplex {
    pub fn new() -> Simplex {
        Simplex {
            action: SimplexMethod::Max,
            mat: Box::new(Vec::new()),
            res: Box::new(Vec::new()),
        }
    }

    pub fn push(&mut self, v: Row) {
        self.mat.push(v);
    }

    pub fn pop(&mut self) -> Option<Row> {
        self.mat.pop()
    }

    /// return lead row and lead col
    fn lead(&self) -> (usize, usize) {
        let (min_col, _) = self.mat.last().unwrap().iter().enumerate()
            .min_by(|x, y| {
                x.1.partial_cmp(y.1).unwrap()
            }).ok_or((-1.0, -1.0)).unwrap();
        let (min_row, _) = self.res[..(self.res.len() - 1)].iter().enumerate()
            .map(
                |(i, x)| {
                    let i = self[i][min_col];
                    *x / i
                }).filter(|x| x.is_finite() && *x >= 0.0).enumerate()
            .min_by(|x, y| {
                let a = x.1;
                let b = y.1;
                a.partial_cmp(&b).unwrap()
            }).unwrap();
        (min_row, min_col)
    }

    fn iteration(&mut self) -> (usize, usize) {
        let (row, col) = self.lead();

        let lead = self[row][col];
        let new_lead: Row = self[row].iter().map(|x| x / lead).collect();
        self.res[row] /= lead;
        println!("{:?} {}", new_lead, self.res[row]);

        for r in 0..(*self).len() {
            if r == row { continue; }

            let tmp = -self[r][col];
            new_lead.iter().zip(self[r].iter_mut()).for_each(|(i, j)| {
                *j = i * tmp + *j
            });
            self.res[r] = self.res[row] * tmp + self.res[r];
        }
        let tmp = -self.mat.last().unwrap()[col];
        new_lead.iter().zip(self.mat.last_mut().unwrap().iter_mut()).for_each(
            |(i, j)| { *j = i * tmp + *j });
        self[row] = new_lead;
        (row, col)
    }

    fn maximization(&mut self) -> Solution {
//        let state = SimplexSolution::default();
        while self.mat.last().unwrap().iter().any(|x| x.is_sign_negative()) {
            println!("Before {}", self);
            self.iteration();
            println!("After {}", self);
        }
        Some(Vec::new())
    }

    fn minimization(&mut self) -> Solution {
//        let state = SimplexSolution::default();
//        Some(Vec::new())
        unimplemented!()
    }

    pub fn run(&mut self) -> Solution {
        match self.action {
            SimplexMethod::Max => self.maximization(),
            SimplexMethod::Min => self.minimization(),
            _ => Option::None
        }
    }
}

impl Display for Simplex {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        writeln!(f, "Action: {:?}", self.action);
        writeln!(f, "Matrix:");
        for (row, v) in self.mat.iter().zip(self.res.iter()).enumerate() {
            writeln!(f, "{}: {:>6.3?} | {:.3}", row, v.0, v.1);
        }
        Ok(())
    }
}

impl Deref for Simplex {
    type Target = Vec<Row>;

    fn deref(&self) -> &Vec<Row> {
        &self.mat
    }
}

impl Index<usize> for Simplex
{
    type Output = Row;

    fn index(&self, index: usize) -> &Row {
        &(&**self)[index]
    }
}

impl IndexMut<usize> for Simplex
{
    fn index_mut(&mut self, index: usize) -> &mut Row {
        let m = &mut *self.mat;
        &mut m[index]
    }
}
