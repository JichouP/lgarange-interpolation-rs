use crate::{Config, Result};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

pub fn calc(config: &Config) -> Result<Vec<(f64, f64)>> {
    if config.source.len() != 4 {
        return Err("config.source.len() in calc is not 4".into());
    }

    let p = |x: f64| -> f64 {
        let x0 = config.source[0].0;
        let x1 = config.source[1].0;
        let x2 = config.source[2].0;
        let x3 = config.source[3].0;
        let y0 = config.source[0].1;
        let y1 = config.source[1].1;
        let y2 = config.source[2].1;
        let y3 = config.source[3].1;

        y0 * (x - x1) * (x - x2) * (x - x3) / (x0 - x1) / (x0 - x2) / (x0 - x3)
            + y1 * (x - x0) * (x - x2) * (x - x3) / (x1 - x0) / (x1 - x2) / (x1 - x3)
            + y2 * (x - x0) * (x - x1) * (x - x3) / (x2 - x0) / (x2 - x1) / (x2 - x3)
            + y3 * (x - x0) * (x - x1) * (x - x2) / (x3 - x0) / (x3 - x1) / (x3 - x2)
    };

    let result = config
        .x
        .par_iter()
        .map(|&x| {
            let y = p(x);

            (x, y)
        })
        .collect();

    Ok(result)
}
