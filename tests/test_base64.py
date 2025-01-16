import base64

from hypothesis import given
from hypothesis.strategies import binary

import fastbase64


@given(binary(min_size=128, max_size=4096))
def test_standard_b64encode(expected):
    base = base64.standard_b64encode(expected)
    fast = fastbase64.standard_b64encode(expected)

    assert base == fast


@given(binary(min_size=128, max_size=4096))
def test_urlsafe_b64encode(expected):
    base = base64.urlsafe_b64encode(expected)
    fast = fastbase64.urlsafe_b64encode(expected)

    assert base == fast


@given(binary(min_size=128, max_size=4096))
def test_standard_b64decode(expected):
    encoded = base64.standard_b64encode(expected)

    base = base64.standard_b64decode(encoded)
    fast = fastbase64.standard_b64decode(encoded)

    assert base == fast


@given(binary(min_size=128, max_size=4096))
def test_urlsafe_b64decode(expected):
    encoded = base64.urlsafe_b64encode(expected)

    base = base64.urlsafe_b64decode(encoded)
    fast = fastbase64.urlsafe_b64decode(encoded)

    assert base == fast
