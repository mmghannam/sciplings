pub mod scipling;
pub mod presets;

pub use presets::Preset;

use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use scipling::Scipling;
use russcip::ffi;


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


    pub fn solve(&mut self, presets: Vec<Preset>) {
        for (i, preset) in presets.into_iter().enumerate() {
            let instance_path = self.instance_path.clone();
            let global_primal_bound = self.global_primal_bound.clone();
            let global_dual_bound = self.global_dual_bound.clone();
            let should_run = Arc::new(RwLock::new(true));
            self.controls.push(should_run.clone());
            rayon::spawn(move || {
                let mut model = russcip::Model::new()
                    .include_default_plugins()
                    .read_prob(instance_path.as_str()).unwrap()
                    .hide_output()
                    .set_int_param("randomization/permutationseed", i as i32).unwrap();

                match preset.clone() {
                    Preset::HeuristicsFocus => {
                        unsafe {
                            let scip_ptr = model.scip_ptr();
                            ffi::SCIPsetHeuristics(scip_ptr, ffi::SCIP_ParamSetting_SCIP_PARAMSETTING_AGGRESSIVE, 0);
                            ffi::SCIPsetEmphasis(scip_ptr, ffi::SCIP_ParamEmphasis_SCIP_PARAMEMPHASIS_FEASIBILITY, 0);
                        }
                    }
                    Preset::SeparatingFocus => {
                        unsafe {
                            let scip_ptr = model.scip_ptr();
                            ffi::SCIPsetSeparating(scip_ptr, ffi::SCIP_ParamSetting_SCIP_PARAMSETTING_AGGRESSIVE, 0);
                            ffi::SCIPsetEmphasis(scip_ptr, ffi::SCIP_ParamEmphasis_SCIP_PARAMEMPHASIS_OPTIMALITY, 0);
                        }
                    }
                    Preset::Default => {}
                    Preset::PseudoCostBranching => {
                        model = model.set_int_param("branching/pscost/priority", 10000000).unwrap();
                    }
                    Preset::SettingsFile(path) => {
                        todo!()
                    }
                }

                let should_run = Arc::new(RwLock::new(true));
                let scipling = Scipling::new(i, model.clone_for_plugins(), global_primal_bound.clone(), global_dual_bound, should_run);
                model.include_eventhdlr(
                    format!("Scipling{}", i).as_str(),
                    "",
                    Box::new(scipling),
                ).solve();
            });
        }

        let mut prev_gap = f64::INFINITY;
        loop {
            self.ticks += 1;

            // self.controls.iter().for_each(|c| *c.write().unwrap() = false);
            thread::sleep(Duration::from_millis(10));
            let primal_bound = *self.global_primal_bound.read().unwrap();
            let dual_bound = *self.global_dual_bound.read().unwrap();
            let gap = primal_bound - dual_bound;
            let rel_gap = gap / primal_bound;

            if gap < prev_gap || self.ticks % 1000 == 0 {
                let has_improvement = gap < prev_gap;
                prev_gap = gap;
                let primal_bound = match primal_bound {
                    f if f > 1000000000000.0 => "inf".to_string(),
                    f => format!("{:.2}", f),
                };
                let dual_bound = match dual_bound {
                    f if f < -1000000000000.0 => "-inf".to_string(),
                    f => format!("{:.2}", f),
                };
                let has_improvement = if has_improvement { "*" } else { "-" };
                println!("{} {:.2}s | bounds: [{}, {}] | rel_gap: {:.2}%",
                         has_improvement, self.ticks as f64 / 100.0, dual_bound, primal_bound, rel_gap * 100.0);
            }


            if rel_gap < 1e-6 {
                break;
            }
        }
    }
}