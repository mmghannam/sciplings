use std::sync::{Arc, RwLock};
use russcip::{EventMask, ffi, WithSolvingStats};
use russcip::HasScipPtr;

pub(crate) struct Scipling {
    id: usize,
    model: russcip::ModelSolving,
    primal_bound: Arc<RwLock<f64>>,
    dual_bound: Arc<RwLock<f64>>,
    should_run: Arc<RwLock<bool>>,
}


impl Scipling {
    pub fn new(
        id: usize,
        model: russcip::ModelSolving,
        primal_bound: Arc<RwLock<f64>>,
        dual_bound: Arc<RwLock<f64>>,
        should_run: Arc<RwLock<bool>>,
    ) -> Self {
        Scipling {
            id,
            model,
            primal_bound,
            dual_bound,
            should_run,
        }
    }
}


impl russcip::Eventhdlr for Scipling {
    fn get_type(&self) -> EventMask {
        EventMask::LP_EVENT | EventMask::NODE_SOLVED
    }

    fn execute(&mut self) {
        // if *self.should_run.read().unwrap() {
            // println!("running scipling {}", self.id);
            // return;
        // }

        let new_primal = self.model.obj_val();
        // println!("Scipling {} bound: {}", self.id, new_bound);
        if new_primal < *self.primal_bound.read().unwrap() {
            self.primal_bound.write().unwrap().clone_from(&new_primal);
        } else if new_primal > *self.primal_bound.read().unwrap() {
            unsafe {
                let scip_ptr = self.model.scip_ptr();
                ffi::SCIPsetObjlimit(scip_ptr, new_primal);
            }
        }


        let new_dual = self.model.best_bound();
        if new_dual > *self.dual_bound.read().unwrap() {
            self.dual_bound.write().unwrap().clone_from(&new_dual);
        }

        // self.should_run.write().unwrap().clone_from(&true);
    }
}


