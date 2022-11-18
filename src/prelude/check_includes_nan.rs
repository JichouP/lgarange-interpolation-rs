use super::Config;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn check_includes_nan(config: &Config) -> Result<()> {
    let is_source_includes_nan = config
        .source
        .par_iter()
        .filter(|v| v.0.is_nan() || v.1.is_nan())
        .collect::<Vec<_>>()
        .len()
        != 0;

    if is_source_includes_nan {
        return Err("Error: Vector config.source includes NaN".into());
    }

    let is_x_includes_nan = config
        .x
        .par_iter()
        .filter(|v| v.is_nan())
        .collect::<Vec<_>>()
        .len()
        != 0;

    if is_x_includes_nan {
        return Err("Error: Vector config.x includes NaN".into());
    }

    Ok(())
}
