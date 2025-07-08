use clap::Parser;
use sciplings::{Preset, Solver};

#[derive(Debug, Parser)]
struct Args {
    instance_path: String,
    #[clap(default_value = "def,heur,sep,pseu")]
    presets_input: Option<String>,
}

fn parse_presets_input(input: String) -> Vec<Preset> {
    let mut presets = Vec::new();
    for preset in input.split(',') {
        match preset {
            "heur" => presets.push(Preset::HeuristicsFocus),
            "sep" => presets.push(Preset::SeparatingFocus),
            "def" => presets.push(Preset::Default),
            "pseu" => presets.push(Preset::PseudoCostBranching),
            "nopr" => presets.push(Preset::WithoutPresolving),
            _ => panic!("Unknown preset: {}", preset),
        }
    }
    presets
}

fn main() {
    let args = Args::parse();
    let mut solver = Solver::new(args.instance_path);
    let presets = parse_presets_input(args.presets_input.unwrap());
    solver.solve(presets);
}
