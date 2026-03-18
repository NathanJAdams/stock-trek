#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pyfunction]
fn add(a: f64, b: f64) -> f64 {
    crate::stats::add(a, b)
}

#[cfg(feature = "python")]
pub fn stats_module(py: Python<'_>) -> PyResult<Bound<'_, PyModule>> {
    let m = PyModule::new(py, "stats")?;

    m.add_function(wrap_pyfunction!(add, &m)?)?;

    Ok(m)
}
