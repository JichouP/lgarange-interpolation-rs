/// Config for using Lagrange completion functions.
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    /// Source data for Lagrangian completion.
    pub source: Vec<(f64, f64)>,
    /// Vector of the x-coordinates of the point to be completed.
    /// The value must be between the minimum and maximum x-coordinate of the Source data.
    pub x: Vec<f64>,
}

impl Config {
    pub fn new(source: Vec<(f64, f64)>, x: Vec<f64>) -> Self {
        Self { source, x }
    }
}
