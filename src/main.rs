use clap::Parser;
use sciplings::MainModel;

#[derive(Debug, Parser)]
struct Args {
    instance_path: String,
    n_solvers: usize,
}

fn main() {
    let args = Args::parse();
    let mut model = MainModel::new(args.instance_path);
    model.solve(args.n_solvers);
}
