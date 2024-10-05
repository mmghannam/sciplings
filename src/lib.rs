pub mod scipling;

use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;
use scipling::Scipling;

pub struct MainModel {
    ticks: usize,
    global_primal_bound: Arc<RwLock<f64>>,
    global_dual_bound: Arc<RwLock<f64>>,
    instance_path: String,
    controls: Vec<Arc<RwLock<bool>>>,
}


impl MainModel {
    pub fn new(instance_path: String) -> Self {
        Self {
            ticks: 0,
            global_primal_bound: Arc::new(RwLock::new(f64::INFINITY)),
            global_dual_bound: Arc::new(RwLock::new(-f64::INFINITY)),
            instance_path,
            controls: Vec::new(),
        }
    }


    pub fn solve(&mut self, n_solvers: usize) {
        for i in 0..n_solvers {
            let instance_path = self.instance_path.clone();
            let global_primal_bound = self.global_primal_bound.clone();
            let global_dual_bound = self.global_dual_bound.clone();
            let should_run = Arc::new(RwLock::new(true));
            self.controls.push(should_run.clone());
            rayon::spawn(move || {
                let model = russcip::Model::new()
                    .include_default_plugins()
                    .read_prob(instance_path.as_str()).unwrap()
                    .hide_output()
                    .set_int_param("randomization/permutationseed", i as i32).unwrap();
                let should_run = Arc::new(RwLock::new(true));
                let scipling = Scipling::new(i, model.clone_for_plugins(), global_primal_bound.clone(), global_dual_bound, should_run);
                model.include_eventhdlr(
                    format!("Scipling{}", i).as_str(),
                    "",
                    Box::new(scipling),
                ).solve();
            });
        }

        loop {
            self.ticks += 1;

            // self.controls.iter().for_each(|c| *c.write().unwrap() = false);
            thread::sleep(Duration::from_millis(10));
            let primal_bound = *self.global_primal_bound.read().unwrap();
            let dual_bound = *self.global_dual_bound.read().unwrap();
            let gap = primal_bound - dual_bound;
            let rel_gap = gap / primal_bound;
            // self.controls.iter().for_each(|c| *c.write().unwrap() = true);
            println!("{:.2}s | bounds: [{:.2}, {:.2}] | rel_gap: {:.2}%",
                     self.ticks as f64 / 100.0, primal_bound, dual_bound, rel_gap * 100.0);

            if rel_gap < 1e-6 {
                break;
            }
        }
    }
}