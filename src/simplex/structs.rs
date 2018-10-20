use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};
use std::ops::{Deref, Index, IndexMut};

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
            _ => SimplexMethod::None,
        }
    }
}

type Row = Vec<f64>;
type Solution = Option<HashMap<usize, f64>>;

#[derive(Debug, Clone)]
pub struct Simplex {
    pub mat: Box<Vec<Row>>,
    pub action: SimplexMethod,
    pub res: Box<Row>,
    // count of vars in equation
    pub n: usize,
    // count of conditions
    pub k: usize,
    // [x1] = row in matrix
    pub names: Box<Vec<usize>>,
}

impl Simplex {
    pub fn new() -> Simplex {
        Simplex {
            action: SimplexMethod::Max,
            mat: Box::new(Vec::new()),
            res: Box::new(Vec::new()),
            n: 0,
            k: 0,
            names: Box::new(Vec::new()),
        }
    }

    pub fn push(&mut self, v: Row) {
        self.mat.push(v);
    }

    pub fn pop(&mut self) -> Option<Row> {
        self.mat.pop()
    }

    /// return lead row and lead col
    fn lead<F, P>(&self, f: F, p: P) -> (usize, usize)
        where F: FnMut((usize, f64), (usize, f64)) -> (usize, f64),
              P: FnMut((usize, f64), (usize, &f64)) -> (usize, f64) {
        let acc = (0usize, self.res.first().unwrap().clone());

        let (min_col, _) = self.mat.last().unwrap().iter().enumerate()
            .fold(acc, p);
        let (min_row, _) = self.res[..(self.res.len() - 1)].iter().enumerate()
            .map(|(i, x)| {
                let i = self[i][min_col];
                *x / i
            })
            .filter(|x| {
                x.is_finite() && *x >= 0.0
            })
            .enumerate().fold(acc, f);
        (min_row, min_col)
    }

    fn iteration<F, P>(&mut self, f: F, p: P) -> (usize, usize)
        where F: FnMut((usize, f64), (usize, f64)) -> (usize, f64),
              P: FnMut((usize, f64), (usize, &f64)) -> (usize, f64) {
        let (row, col) = self.lead(f, p);

        let lead = self[row][col];
        let new_lead: Row = self[row].iter().map(|x| x / lead).collect();
        self.res[row] /= lead;

        for r in 0..(*self).len() {
            if r == row {
                continue;
            }

            let tmp = -self[r][col];
            new_lead.iter().zip(self[r].iter_mut())
                .for_each(|(i, j)| *j = i * tmp + *j);

            self.res[r] = self.res[row] * tmp + self.res[r];
        }

        let tmp = -self.mat.last().unwrap()[col];
        new_lead.iter().zip(self.mat.last_mut().unwrap().iter_mut())
            .for_each(|(i, j)| *j = i * tmp + *j);
        self[row] = new_lead;
        (row, col)
    }

    fn maximization(&mut self) -> Solution {
        let f = |x: (usize, f64), y: (usize, f64)| {
            match x.1.partial_cmp(&y.1).unwrap() {
                Ordering::Greater => y,
                _ => x
            }
        };
        let p = |(i, x): (usize, f64), (j, y): (usize, &f64)| {
            match x.partial_cmp(y).unwrap() {
                Ordering::Greater => (j, *y),
                _ => (i, x)
            }
        };

        while self.mat.last().unwrap().iter().any(|x| x.is_sign_negative()) {
            let change = self.iteration(f, p);
            self.names[change.0] = change.1;
        }

        let mut res = HashMap::new();
        for (i, v) in self.names.iter().enumerate() {
            if 1 <= *v && *v <= self.n {
                res.insert(v.clone(), self.res[i]);
            }
        }

        for i in 1..self.n + 1 {
            res.entry(i).or_insert(0.0);
        }
        res.insert(0usize, *self.res.last().unwrap());
        Some(res)
    }

    fn minimization(&mut self) -> Solution {
        let f: fn((usize, f64), (usize, f64)) -> (usize, f64) = |x: (usize, f64), y: (usize, f64)| {
            match x.1.partial_cmp(&y.1).unwrap() {
                Ordering::Greater => x,
                _ => y
            }
        };
        let p = |(i, x): (usize, f64), (j, y): (usize, &f64)| {
            match x.partial_cmp(y).unwrap() {
                Ordering::Greater => (i, x),
                _ => (j, *y)
            }
        };
        while self.mat.last().unwrap().iter().any(|x| x.is_sign_positive()) {
            let change = self.iteration(f, p);
            self.names[change.0] = change.1;
        }

        let mut res = HashMap::new();
        for (i, v) in self.names.iter().enumerate() {
            if 1 <= *v && *v <= self.n {
                res.insert(v.clone(), self.res[i]);
            }
        }

        for i in 1..self.n + 1 {
            res.entry(i).or_insert(0.0);
        }
        res.insert(0usize, *self.res.last().unwrap());
        Some(res)
    }

    pub fn run(&mut self) -> Solution {
        match self.action {
            SimplexMethod::Max => self.maximization(),
            SimplexMethod::Min => self.minimization(),
            _ => Option::None,
        }
    }
}

impl Display for Simplex {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        writeln!(f, "Action: {:?}, N: {}, K: {}", self.action, self.n, self.k);
        write!(f, "X:   {: ^6}", 'z');
        for i in 1..self.n + self.k + 1 {
            write!(f, "{:^8}", format!("x{}", i));
        }
        let zero = 0usize;
        writeln!(f, " | {: ^5}", "res");
        for (row, v) in self.mat.iter().zip(self.res.iter()).enumerate() {
            writeln!(
                f,
                "{}: {:>6.3?} | {:.3}",
                self.names.get(row).or(Some(&zero)).unwrap(),
                v.0,
                v.1
            );
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

impl Index<usize> for Simplex {
    type Output = Row;

    fn index(&self, index: usize) -> &Row {
        &(&**self)[index]
    }
}

impl IndexMut<usize> for Simplex {
    fn index_mut(&mut self, index: usize) -> &mut Row {
        let m = &mut *self.mat;
        &mut m[index]
    }
}
