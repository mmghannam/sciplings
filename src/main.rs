use sciplings::Preset;
use clap::{ArgMatches, Command, CommandFactory, Error, FromArgMatches, Parser};
use sciplings::MainModel;

#[derive(Debug, Parser)]
struct Args {
    instance_path: String,
    #[clap(default_value = "d-h-s-p")]
    presets_input: Option<String>,
}

fn parse_presets_input(input: String) -> Vec<Preset> {
    let mut presets = Vec::new();
    for preset in input.split('-') {
        match preset {
            "h" => presets.push(Preset::HeuristicsFocus),
            "s" => presets.push(Preset::SeparatingFocus),
            "d" => presets.push(Preset::Default),
            "p" => presets.push(Preset::PseudoCostBranching),
            _ => panic!("Unknown preset: {}", preset),
        }
    }
    presets
}

fn main() {
    let args = Args::parse();
    let mut model = MainModel::new(args.instance_path);
    let presets = parse_presets_input(args.presets_input.unwrap());
    model.solve(presets);
}
