use russcip::{ffi, Event, EventMask, Model, SCIPEventhdlr, Solving, WithSolvingStats};
use std::sync::{Arc, RwLock};

pub(crate) struct Scipling {
    id: usize,
    primal_bound: Arc<RwLock<f64>>,
    dual_bound: Arc<RwLock<f64>>,
    should_run: Arc<RwLock<bool>>,
}

impl Scipling {
    pub fn new(
        id: usize,
        primal_bound: Arc<RwLock<f64>>,
        dual_bound: Arc<RwLock<f64>>,
        should_run: Arc<RwLock<bool>>,
    ) -> Self {
        Scipling {
            id,
            primal_bound,
            dual_bound,
            should_run,
        }
    }
}

impl russcip::Eventhdlr for Scipling {
    fn get_type(&self) -> EventMask {
        EventMask::NODE_SOLVED
    }

    fn execute(&mut self, model: Model<Solving>, _eventhdlr: SCIPEventhdlr, _event: Event) {
        // if *self.should_run.read().unwrap() {
        // println!("running scipling {}", self.id);
        // return;
        // }

        let new_primal = model.obj_val();
        // println!("Scipling {} bound: {}", self.id, new_bound);
        if new_primal < *self.primal_bound.read().unwrap() {
            self.primal_bound.write().unwrap().clone_from(&new_primal);
        } else if new_primal > *self.primal_bound.read().unwrap() {
            unsafe {
                let scip_ptr = model.scip_ptr();
                ffi::SCIPsetObjlimit(scip_ptr, new_primal);
            }
        }

        let new_dual = model.best_bound();
        if new_dual > *self.dual_bound.read().unwrap() {
            self.dual_bound.write().unwrap().clone_from(&new_dual);
        }

        // self.should_run.write().unwrap().clone_from(&true);
    }
}
