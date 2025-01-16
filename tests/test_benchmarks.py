import base64
from random import randbytes

import pytest

import fastbase64


@pytest.fixture(scope="session")
def example():
    return randbytes(128)


@pytest.fixture(scope="session")
def standard_encoded():
    return base64.standard_b64encode(randbytes(128))


@pytest.fixture(scope="session")
def urlsafe_encoded():
    return base64.urlsafe_b64encode(randbytes(128))


@pytest.mark.benchmark(group="standardb64_encode")
def test_base_standard(benchmark, example):
    benchmark(base64.standard_b64encode, example)


@pytest.mark.benchmark(group="standardb64_encode")
def test_fast_standard(benchmark, example):
    benchmark(fastbase64.standard_b64encode, example)


@pytest.mark.benchmark(group="urlsafeb64_encode")
def test_base_urlsafe(benchmark, example):
    benchmark(base64.urlsafe_b64encode, example)


@pytest.mark.benchmark(group="urlsafeb64_encode")
def test_fast_urlsafe(benchmark, example):
    benchmark(fastbase64.urlsafe_b64encode, example)


@pytest.mark.benchmark(group="standardb64_decode")
def test_base_standard_decode(benchmark, standard_encoded):
    benchmark(base64.standard_b64decode, standard_encoded)


@pytest.mark.benchmark(group="standardb64_decode")
def test_fast_standard_decode(benchmark, standard_encoded):
    benchmark(fastbase64.standard_b64decode, standard_encoded)


@pytest.mark.benchmark(group="urlsafeb64_decode")
def test_base_urlsafe_decode(benchmark, urlsafe_encoded):
    benchmark(base64.urlsafe_b64decode, urlsafe_encoded)


@pytest.mark.benchmark(group="urlsafeb64_decode")
def test_fast_urlsafe_decode(benchmark, urlsafe_encoded):
    benchmark(fastbase64.urlsafe_b64decode, urlsafe_encoded)
