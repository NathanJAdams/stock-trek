#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pymodule]
pub fn stock_trek(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let stats = super::stats::stats_module(py)?;

    m.add_submodule(&stats)?;

    Ok(())
}
