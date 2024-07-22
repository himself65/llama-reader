use pyo3::prelude::*;
use llamareader;

#[pyfunction]
fn read_epub(path: String) -> PyResult<llamareader::Epub> {
	Ok(llamareader::read_epub(path))
}

#[pymodule]
fn llama_reader(m: &Bound<'_, PyModule>) -> PyResult<()> {
	m.add_function(wrap_pyfunction!(read_epub, m)?)
}
