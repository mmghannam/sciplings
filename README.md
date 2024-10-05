# sciplings

This project is a concurrent solver based on the [SCIP optimization suite](https://scipopt.org/). It runs multiple SCIP instances in parallel,
each with a different set of presets.


## Usage from the command line
To use this project, you need to have Rust and Cargo installed. You can then run the project with
```sh
cargo r --release -- INSTANCE_PATH PRESETS   
```
where `INSTANCE_PATH` is the path to the instance file and `PRESETS` is a comma-separated list of presets to use.

currently, the following presets are available:
- `d`: the default SCIP settings
- `h`: heuristics focus
- `s`: separation focus
- `p`: pseudocost branching

by default presets is set to `d,h,s,p`.

## Usage as a library
You can also use this project as a library.

```rust
use sciplings::{Solver, Presets};

fn main() {
    let instance_path = "path/to/instance";
    let presets = vec![Presets::Default, Presets::Heuristics, Presets::Separation, Presets::Pseudocost];
    Solver::new(instance_path).solve();
}
```