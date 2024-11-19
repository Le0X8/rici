use std::env;

/// Determines if the current environment is a CI environment.
/// 
/// RICI: Running In Continuous Integration
/// 
/// Currently this function only checks for GitHub Actions.
pub fn rici() -> bool {
    env::var("GITHUB_ACTIONS").map_or(false, |v| v == "true")
}