use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3_ffi::c_str;
use std::ffi::CString;
use std::fs;
use std::path::Path;

fn main() -> PyResult<()> {
    let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/python_app"));
    let py_app = CString::new(fs::read_to_string(path.join("test.py"))?)?;
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let syspath = py
            .import("sys")?
            .getattr("path")?
            .downcast_into::<PyList>()?;
        syspath.insert(0, path)?;
        let app: Py<PyAny> = PyModule::from_code(py, py_app.as_c_str(), c_str!(""), c_str!(""))?
            .getattr("run")?
            .into();
        app.call0(py)
    });

    println!("py: {}", from_python?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        assert!(main().is_ok());
    }
}
