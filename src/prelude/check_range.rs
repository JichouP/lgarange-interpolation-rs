use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::{Config, Result};

pub fn check_range(config: &Config) -> Result<()> {
    let source_max = config
        .source
        .par_iter()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .unwrap();
    let source_min = config
        .source
        .par_iter()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .unwrap();
    let x_max = config
        .x
        .par_iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let x_min = config
        .x
        .par_iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    if *x_min < source_min.0 || source_max.0 < *x_max {
        return Err("Error: x out of range".into());
    }

    Ok(())
}
