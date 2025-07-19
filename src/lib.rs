use pyo3::prelude::*;

#[pyfunction]
fn increment_list(values: Vec<i32>) -> Vec<i32> {
    values.into_iter().map(|x| x + 1).collect()
}

#[pymodule]
fn rustpython(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(increment_list, m)?)?;
    Ok(())
}
