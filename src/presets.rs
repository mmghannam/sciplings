
/// Describes the different presets that can be used to configure the solvers.
#[derive(Debug, Clone)]
pub enum Preset {
    /// Focus on heuristics.
    HeuristicsFocus,
    /// Focus on separating.
    SeparatingFocus,
    /// Default settings.
    Default,
    /// Pseudo cost branching.
    PseudoCostBranching,
    /// Without Presolving
    WithoutPresolving,
    /// Load settings from a file.
    SettingsFile(String),
}