use anyhow::anyhow;
use argmin::core::{CostFunction, Error, Executor};
use argmin::solver::simulatedannealing::{Anneal, SimulatedAnnealing};
use rand::distributions::Uniform;
use rand::rngs::ThreadRng;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::{Arc, Mutex};

#[derive(PartialEq, Copy, Clone, Debug, Deserialize, Serialize)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    fn distance(&self, other: &Point) -> f64 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
    }
}

#[derive(Clone, Debug)]
struct BiscuitPacking {
    n: usize,
    w: f64,
    l: f64,
    // todo use a better one
    rng: Arc<Mutex<ThreadRng>>,
}

impl BiscuitPacking {
    fn init(&self) -> Vec<Point> {
        // todo start with poisson disc sample
        // https://www.jasondavies.com/poisson-disc/
        let mut out = vec![];
        for _ in 0..self.n {
            let mut rng = self.rng.lock().unwrap();
            let x = rng.sample(Uniform::new(0.0, self.w));
            let y = rng.sample(Uniform::new(0.0, self.l));
            out.push(Point::new(x, y));
        }
        out
    }
}

impl fmt::Display for BiscuitPacking {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let biscuits = self.n;
        let width = self.w;
        let length = self.l;
        write!(f, "{biscuits} biscuits in a {width} X {length} pan.")
    }
}

impl Anneal for BiscuitPacking {
    type Param = Vec<Point>;

    type Output = Vec<Point>;

    type Float = f64;

    // example: https://github.com/argmin-rs/argmin/blob/main/argmin/examples/simulatedannealing.rs
    fn anneal(&self, param: &Self::Param, temp: Self::Float) -> Result<Self::Output, Error> {
        let mut param_n = param.clone();
        let mut rng = self.rng.lock().unwrap();
        let idxs = Uniform::from(0..param.len());
        let step = Uniform::new_inclusive(-0.1, 0.1);
        for _ in 0..(temp.floor() as u64 + 1) {
            let idx = rng.sample(idxs);
            let val = rng.sample(step);
            let x = rng.sample(step) >= 0.0;
            if x {
                let x_next = param_n[idx].x + val;
                // don't update if it would cross the bound
                if x_next < self.w || x_next <= 0.0 {
                    param_n[idx] = Point {
                        x: param_n[idx].x + val,
                        y: param_n[idx].y,
                    };
                }
            } else {
                let y_next = param_n[idx].y;
                // don't update if it would cross the bound
                if y_next + val < self.l || y_next <= 0.0 {
                    param_n[idx] = Point {
                        x: param_n[idx].x,
                        y: param_n[idx].y + val,
                    };
                }
            }
        }
        Ok(param_n)
    }
}

impl CostFunction for BiscuitPacking {
    type Param = Vec<Point>;
    type Output = f64;

    fn cost(&self, param: &Self::Param) -> Result<Self::Output, Error> {
        let mut ds = vec![];
        // put the closest distance to pan edge into distances vector
        let edges = param
            .clone()
            .into_iter()
            .map(|p| vec![p.x, p.y])
            .collect::<Vec<Vec<f64>>>()
            .concat();
        let min = edges.into_iter().reduce(|x, y| x.min(y)).unwrap();
        ds.push(min);

        for p0 in param {
            if p0.x > self.w {
                return Err(anyhow!("point outside width"));
            } else if p0.y > self.l {
                return Err(anyhow!("point outside length"));
            }
            for p1 in param {
                if p0 != p1 {
                    ds.push(p0.distance(p1));
                }
            }
        }
        let max = ds.clone().into_iter().reduce(|x, y| x.max(y)).unwrap();
        let avg = (ds.clone().into_iter().sum::<f64>()) / (ds.len() as f64);
        if max <= 0.0 {
            Err(anyhow!("all zeros"))
        } else {
            // todo remove print
            // println!("distances {:?}", ds);
            // println!("max {max}");
            // println!("avg {avg}");
            // println!("score {}", max - avg);
            // println!("-------------");
            Ok(max - avg)
        }
    }
}

fn main() {
    let problem = BiscuitPacking {
        n: 2,
        l: 2.0,
        w: 3.0,
        rng: Arc::new(Mutex::new(rand::thread_rng())),
    };
    let init = problem.init();
    let solver = SimulatedAnnealing::new(100.0).unwrap();
    let res = Executor::new(problem.clone(), solver)
        .configure(|state| {
            state
                // Set initial parameters (depending on the solver,
                // this may be required)
                .param(init)
                // Set maximum iterations to 10
                // (optional, set to `std::u64::MAX` if not provided)
                .max_iters(100000)
                // Set target cost. The solver stops when this cost
                // function value is reached (optional)
                .target_cost(0.0)
        })
        // run the solver on the defined problem
        .run()
        .unwrap();

    println!("{}", problem);
    println!("{}", res);
}
