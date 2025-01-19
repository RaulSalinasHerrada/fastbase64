#![deny(warnings)]
extern crate base64;
extern crate pyo3;

use base64::engine::general_purpose::GeneralPurpose;
use base64::prelude::{Engine, BASE64_STANDARD, BASE64_URL_SAFE};
use base64::{decoded_len_estimate, encoded_len};
use pyo3::exceptions::{PyOverflowError, PyTypeError};
use pyo3::prelude::*;
use pyo3::{types::PyBytes, Bound};

#[pymodule]
mod fastbase64 {

    use super::*;

    type PyBytesResult = PyResult<Py<PyBytes>>;

    fn get_buf_with_capacity(capacity: usize) -> Vec<u8> {
        let mut buf = Vec::with_capacity(capacity);
        buf.resize_with(capacity, Default::default);
        buf
    }

    fn get_encoded_length(input: &[u8]) -> PyResult<usize> {
        match encoded_len(input.len(), true) {
            Some(elen) => Ok(elen),
            None => Err(PyOverflowError::new_err("Cannot infer usize")),
        }
    }

    fn encode_with_engine(
        py: Python,
        engine: GeneralPurpose,
        input: &[u8],
        buf: &mut Vec<u8>,
    ) -> PyBytesResult {
        match engine.encode_slice(input, buf) {
            Ok(written) => Ok(PyBytes::new(py, &buf[..written]).into()),
            Err(e) => Err(PyTypeError::new_err(e.to_string())),
        }
    }

    fn decode_with_engine(
        py: Python,
        engine: GeneralPurpose,
        input: &[u8],
        buf: &mut Vec<u8>,
    ) -> PyBytesResult {
        match engine.decode_slice(input, buf) {
            Ok(written) => Ok(PyBytes::new(py, &buf[..written]).into()),
            Err(e) => Err(PyTypeError::new_err(e.to_string())),
        }
    }

    #[pyfunction]
    fn standard_b64encode(py: Python, s: &Bound<PyBytes>) -> PyBytesResult {
        let input = s.as_bytes();

        let capacity = get_encoded_length(input)?;
        let mut buf = get_buf_with_capacity(capacity);

        encode_with_engine(py, BASE64_STANDARD, input, &mut buf)
    }

    #[pyfunction]
    fn urlsafe_b64encode(py: Python, s: &Bound<PyBytes>) -> PyBytesResult {
        let input = s.as_bytes();

        let capacity = get_encoded_length(input)?;
        let mut buf = get_buf_with_capacity(capacity);

        encode_with_engine(py, BASE64_URL_SAFE, input, &mut buf)
    }

    #[pyfunction]
    fn standard_b64decode(py: Python, s: &Bound<PyBytes>) -> PyBytesResult {
        let input = s.as_bytes();

        let capacity = decoded_len_estimate(input.len());
        let mut buf = get_buf_with_capacity(capacity);

        decode_with_engine(py, BASE64_STANDARD, input, &mut buf)
    }

    #[pyfunction]
    fn urlsafe_b64decode(py: Python, s: &Bound<PyBytes>) -> PyBytesResult {
        let input = s.as_bytes();

        let capacity = decoded_len_estimate(input.len());
        let mut buf = get_buf_with_capacity(capacity);

        decode_with_engine(py, BASE64_URL_SAFE, input, &mut buf)
    }
}
