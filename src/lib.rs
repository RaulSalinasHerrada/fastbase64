#![deny(warnings)]
extern crate base64;
extern crate pyo3;

use base64::decoded_len_estimate;
use base64::encoded_len;
use base64::engine::general_purpose;
use base64::prelude::*;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::{types::PyBytes, Bound};
use tinyvec::TinyVec;

const SIZE: usize = 4096;

#[pymodule]
mod fastbase64 {

    use super::*;

    fn get_buf_with_capacity(capacity: usize) -> TinyVec<[u8; SIZE]> {
        let mut buf: TinyVec<[u8; 4096]> = TinyVec::new();
        buf.resize(capacity, 0);
        buf
    }

    #[pyfunction]
    fn standard_b64encode(py: Python, s: &Bound<PyBytes>) -> PyResult<Py<PyBytes>> {
        let input = s.as_bytes();

        let capacity = match encoded_len(input.len(), true) {
            Some(elen) => elen,
            None => return Err(PyTypeError::new_err("Cannot infer usize")),
        };

        let mut buf = get_buf_with_capacity(capacity);

        match general_purpose::STANDARD.encode_slice(input, &mut buf) {
            Ok(written) => Ok(PyBytes::new(py, &buf[..written]).into()),
            Err(e) => return Err(PyTypeError::new_err(e.to_string())),
        }
    }

    #[pyfunction]
    fn urlsafe_b64encode(py: Python, s: &Bound<PyBytes>) -> PyResult<Py<PyBytes>> {
        let input = s.as_bytes();

        let capacity = match encoded_len(input.len(), true) {
            Some(elen) => elen,
            None => return Err(PyTypeError::new_err("Cannot infer usize")),
        };

        let mut buf = get_buf_with_capacity(capacity);

        match general_purpose::URL_SAFE.encode_slice(input, &mut buf) {
            Ok(written) => Ok(PyBytes::new(py, &buf[..written]).into()),
            Err(e) => return Err(PyTypeError::new_err(e.to_string())),
        }
    }

    #[pyfunction]
    fn standard_b64decode(py: Python, s: &Bound<PyBytes>) -> PyResult<Py<PyBytes>> {
        let input = s.as_bytes();

        let capacity = decoded_len_estimate(input.len());

        let mut buf = get_buf_with_capacity(capacity);

        match general_purpose::STANDARD.decode_slice(input, &mut buf) {
            Ok(written) => Ok(PyBytes::new(py, &buf[..written]).into()),
            Err(e) => return Err(PyTypeError::new_err(e.to_string())),
        }
    }

    #[pyfunction]
    fn urlsafe_b64decode(py: Python, s: &Bound<PyBytes>) -> PyResult<Py<PyBytes>> {
        let input = s.as_bytes();

        let capacity = decoded_len_estimate(input.len());

        let mut buf = get_buf_with_capacity(capacity);

        match general_purpose::URL_SAFE.decode_slice(input, &mut buf) {
            Ok(written) => Ok(PyBytes::new(py, &buf[..written]).into()),
            Err(e) => return Err(PyTypeError::new_err(e.to_string())),
        }
    }
}
