#!/usr/bin/env python3
import os
import sys
import re
import ctypes
from glob import glob
from subprocess import check_call, check_output, CalledProcessError, STDOUT
import pytest


GCC_RUST_PATH = os.environ["GCC_RUST"]
RUSTC_PATH = os.environ.get("RUSTC", "rustc")

GCC_COMPILE_FLAGS = ["-g", "-O3"]


def demangle(names):
    return dict(
        zip(names, check_output(["c++filt", "--"] + names).decode().splitlines())
    )


def list_exported_symbols(so_path):
    return [
        line.split()[2]
        for line in check_output(
            ["nm", "--extern-only", "--defined-only", "--no-demangle", "--", so_path]
        )
        .decode()
        .splitlines()
    ]


def is_test_function(name):
    return re.match(r"^[^:]+::test[^:]*(?:::h[0-9a-f]+)?$", name)


def find_test_functions(so_path):
    syms = list_exported_symbols(so_path)
    demangled_syms = demangle(syms)
    return [sym for sym in syms if is_test_function(demangled_syms[sym])]


def compile_shared_library_with_gcc_rust(src_file, out_file):
    check_output(
        [GCC_RUST_PATH, src_file, "-o", out_file, "-shared", "-fPIC"]
        + GCC_COMPILE_FLAGS,
        stderr=STDOUT,
    )


def summarize_output(output):
    m = re.search(r"(?m)panicked at.+$", output)
    if m:
        return m.group()

    m = re.search(r"(?m)internal compiler error: .+$", output)
    if m:
        return m.group()

    return None


def compile_shared_library_with_rustc(src_file, out_file):
    # Not using cdylib to ensure that all our `pub` functions are exported.
    check_call([RUSTC_PATH, src_file, "-o", out_file, "--crate-type", "dylib"])


def call_test_function(so_path, func_name):
    so = ctypes.CDLL(os.path.abspath(so_path))
    func = getattr(so, func_name)
    func.restype = ctypes.c_int32
    return func()


my_dir = os.path.abspath(os.path.dirname(__file__))
src_files = glob(os.path.join(my_dir, "src", "*.rs"))
src_files.sort()


@pytest.mark.parametrize("src_file", src_files, ids=os.path.basename)
def test_compile_and_run(tmpdir, src_file):
    gcc_rust_so = str(tmpdir.join("gcc_rust_out.so"))
    rustc_so = str(tmpdir.join("rustc_out.so"))

    # Compile with rustc first to verify that the source code is ok. This way, if gcc-rust fails, we
    # know it's gcc-rust at fault, not the source code.
    try:
        compile_shared_library_with_rustc(src_file, rustc_so)
    except CalledProcessError as e:
        raise Exception(f"rustc failed to compile {src_file!r}") from e

    try:
        compile_shared_library_with_gcc_rust(src_file, gcc_rust_so)
    except CalledProcessError as e:
        output = e.output.decode()
        sys.stderr.write(output)

        summary = summarize_output(output)
        if not summary:
            summary = "gcc-rust compilation error"

        raise Exception(summary) from e

    test_funcs = find_test_functions(gcc_rust_so)
    assert test_funcs, "No test functions found!"
    assert test_funcs == find_test_functions(
        rustc_so
    ), "gcc_rust's output's test functions list doesn't match rustc's output's!"

    for f in test_funcs:
        gcc_rust_exit_code = call_test_function(gcc_rust_so, f)
        rustc_exit_code = call_test_function(rustc_so, f)
        assert (
            gcc_rust_exit_code == rustc_exit_code
        ), f"Exit code mismatch for {f!r}, got {gcc_rust_exit_code} instead of {rustc_exit_code}"


if __name__ == "__main__":
    sys.exit(pytest.main(sys.argv))
