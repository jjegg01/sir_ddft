use sir_ddft::ode::{StopCondition, ODEIVP, RKF45Solver, ExplicitODESolver};

struct TestODE {
    pub result : Vec<(f64,f64)>
}

struct TestODE2 {
    pub result : Vec<(f64,f64)>
}

struct TestODE3 {
    pub result : Vec<(f64,f64)>
}

// Simple test ODE for arctan(t)
impl<S> ODEIVP<S> for TestODE {
    fn rhs(&mut self, t : f64, _: &[f64]) -> Vec<f64> {
        vec![1./(1. + t*t)]
    }
    fn initial_state(&mut self) -> (f64, Vec<f64>) {
        (0., vec![0.])
    }
    fn end_step(&mut self, t : f64, y: &[f64], _: &S) -> sir_ddft::ode::StopCondition {
        self.result.push((t, y[0]));
        StopCondition::ContinueUntil(10.)
    }

    fn final_state(&mut self, _: f64, _: Vec<f64>) {}
}

// Simple test ODE for harmonic oscillator
impl<S> ODEIVP<S> for TestODE2 {
    fn rhs(&mut self, _ : f64, y: &[f64]) -> Vec<f64> {
        vec![y[1], -y[0]]
    }
    fn initial_state(&mut self) -> (f64, Vec<f64>) {
        (0., vec![1., 0.])
    }
    fn end_step(&mut self, t : f64, y: &[f64], _: &S) -> sir_ddft::ode::StopCondition {
        self.result.push((t, y[0]));
        StopCondition::ContinueUntil(10.)
    }

    fn final_state(&mut self, _: f64, _: Vec<f64>) {}
}

// Simple exponential growth ode
impl<S> ODEIVP<S> for TestODE3 {
    fn rhs(&mut self, _ : f64, y: &[f64]) -> Vec<f64> {
        vec![y[0]]
    }
    fn initial_state(&mut self) -> (f64, Vec<f64>) {
        (0., vec![1.])
    }
    fn end_step(&mut self, t : f64, y: &[f64], _: &S) -> StopCondition {
        self.result.push((t, y[0]));
        StopCondition::ContinueUntil(1.)
    }

    fn final_state(&mut self, _: f64, _: Vec<f64>) {}
}

pub fn main() {
    let mut ode = TestODE {
        result: vec!()
    };
    let mut ode2 = TestODE2 {
        result: vec!()
    };
    let mut ode3 = TestODE3 {
        result: vec!()
    };
    let solver = RKF45Solver::new();
    let solver2 = RKF45Solver::new();
    let solver3 = RKF45Solver::new();
    solver.integrate(&mut ode);
    solver2.integrate(&mut ode2);
    solver3.integrate(&mut ode3);
    println!("Integration result for arctan:");
    // for (t,y) in &ode.result {
    //     println!("{} {}", t, y);
    // }
    println!("Exact result: {}", f64::atan(10.));
    println!("ODE result: {}", ode.result.last().unwrap().1);
    println!("Integration result for harmonic oscillator:");
    println!("Exact result: {}", f64::cos(10.));
    println!("ODE result: {}", ode2.result.last().unwrap().1);
    // for (t,y) in &ode2.result {
    //     println!("{} {}", t, y);
    // }
    println!("Integration result for exponential growth:");
    println!("Exact result: {}", f64::exp(1.));
    println!("ODE result: {}", ode3.result.last().unwrap().1);
    // for (t,y) in &ode3.result {
    //     println!("{} {}", t, y);
    // }
}