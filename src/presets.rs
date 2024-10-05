
/// Describes the different presets that can be used to configure the solvers.
#[derive(Debug, Clone)]
pub enum Preset {
    HeuristicsFocus,
    SeparatingFocus,
    Default,
    PseudoCostBranching,
    SettingsFile(String),
}