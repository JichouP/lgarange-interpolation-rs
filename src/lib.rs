//! # Lagrange Interpolation
//!
//! This crate provides a function for Lagrange completion.
//!
//! Usually, this function performs a 3rd order Lagrange completion from source data containing 4 or more arbitrary coordinates, based on the 2 points from the front and rear side the point to be completed.
//!
//! However, if the point to be complemented is close to the edge of the source data and 2 points cannot be taken from the front and rear sides respectively, then 3rd order Lagrangian completion is performed using 1 point from the front side - 3 points from the rear side or 3 points from the front side - 1 point from the rear side.

mod prelude;
use prelude::*;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// # Lagrange Interpolation
///
/// Returns the Vec of values after Lagrangian completion.
///
/// ## Error
///
/// If `source` or `x` of `Config` includes `NaN`, returns `Err`.
pub fn lagrange_interpolation(config: Config) -> Result<Vec<(f64, f64)>> {
    // Check if an element of Config includes NaN.
    // If included, returns Err.
    check_includes_nan(&config)?;
    check_range(&config)?;

    let config = &mut config.clone();

    // Sort by x
    config
        .source
        .sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
    config.x.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let result = calc(config)?;
    Ok(result)
}
