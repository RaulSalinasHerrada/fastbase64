# fastbase64 (WIP)

FastBase64 is a library which provides CPython bindings to the Rust's Base64 library. The provided API is exactly as Python's builtin base64 class.

Currently supported on Python 3.10, 3.11, 3.12, & 3.13.


# Current limitations

For now the library only supports bytes on standard/urlsafe base64 encoding and decoding.


# Usage

```python
import fastbase64
import base64 

example = b'hello world!'

assert base64.standard_b64encode(example) == fastbase64.standard_b64encode(example)

encoded = "SXQgY2FuIGRlY29kZSB0b28h"

assert fastbase64.standard_b64decode(encoded) == b"It can decode too!"
```