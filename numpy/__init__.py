"""
Lightweight fallback for NumPy used in the Frame transpiler test suite.

If a real NumPy installation is available on sys.path (outside the project
root), this shim defers to it. Otherwise it provides a tiny subset sufficient
for the repository's positive tests that exercise the Python target.
"""
from __future__ import annotations

import importlib.util
import math
import pathlib
import sys
from typing import Iterable, List, Sequence, Tuple, Union

_REAL_NUMPY = None
_MODULE_PATH = pathlib.Path(__file__).resolve()
_PROJECT_ROOT = _MODULE_PATH.parent.parent

_original_sys_path = list(sys.path)
try:
    sys.path = [
        p
        for p in sys.path
        if pathlib.Path(p).resolve() != _PROJECT_ROOT
    ]
    spec = importlib.util.find_spec("numpy")
    if spec and spec.origin and pathlib.Path(spec.origin).resolve() != _MODULE_PATH:
        module = importlib.util.module_from_spec(spec)
        assert spec.loader is not None
        spec.loader.exec_module(module)  # type: ignore[misc]
        globals().update(module.__dict__)
        _REAL_NUMPY = module
finally:
    sys.path = _original_sys_path

if _REAL_NUMPY is None:
    Number = Union[int, float]
    ArrayData = Union[Number, Sequence[Number], Sequence[Sequence[Number]]]

    def _is_matrix(values: Sequence) -> bool:
        return values and isinstance(values[0], Sequence)

    def _is_matrix_data(data) -> bool:
        return isinstance(data, list) and data and isinstance(data[0], list)

    def _is_vector_data(data) -> bool:
        return isinstance(data, list) and not _is_matrix_data(data)

    def _clone(data: ArrayData):
        if isinstance(data, (int, float)):
            return data
        return [ _clone(item) for item in data ]  # type: ignore[arg-type]

    class ndarray:
        def __init__(self, data: ArrayData):
            if isinstance(data, (int, float)):
                self._data: Union[List[List[Number]], List[Number], Number] = data
            elif _is_matrix(list(data)):
                self._data = [list(row) for row in data]  # type: ignore[list-item]
            else:
                self._data = list(data)  # type: ignore[list-item]

        def __matmul__(self, other: "ndarray") -> Union["ndarray", Number]:
            other_data = other._data
            if _is_matrix_data(self._data):
                # Matrix @ Matrix
                if not _is_matrix_data(other_data):
                    raise TypeError("matrix @ operand requires 2D data")
                rows = len(self._data)
                cols = len(other_data[0])  # type: ignore[index]
                inner = len(self._data[0])
                if any(len(row) != inner for row in self._data):
                    raise ValueError("inconsistent row lengths")
                if any(len(row) != cols for row in other_data):  # type: ignore[arg-type]
                    raise ValueError("incompatible matrices")
                result = []
                for r in range(rows):
                    row = []
                    for c in range(cols):
                        value = 0
                        for k in range(inner):
                            value += self._data[r][k] * other_data[k][c]  # type: ignore[index]
                        row.append(value)
                    result.append(row)
                return ndarray(result)

            # Vector dot product
            left = self.flatten()
            right = ndarray(other_data).flatten()
            if len(left) != len(right):
                raise ValueError("incompatible vector sizes")
            return sum(l * r for l, r in zip(left, right))

        def __imatmul__(self, other: "ndarray") -> "ndarray":
            result = self @ other
            if isinstance(result, ndarray):
                self._data = _clone(result._data)
            else:
                self._data = result
            return self

        def __mul__(self, scalar: Number) -> "ndarray":
            if _is_matrix_data(self._data):
                return ndarray([[elem * scalar for elem in row] for row in self._data])  # type: ignore[index]
            if _is_vector_data(self._data):
                return ndarray([elem * scalar for elem in self._data])  # type: ignore[list-item]
            return ndarray(self._data * scalar)  # type: ignore[operator]

        def __rmul__(self, scalar: Number) -> "ndarray":
            return self * scalar

        def __add__(self, other: Union["ndarray", Number]) -> "ndarray":
            if isinstance(other, ndarray):
                if _is_matrix_data(self._data) and _is_matrix_data(other._data):
                    rows = len(self._data)
                    cols = len(self._data[0])
                    result = []
                    for r in range(rows):
                        row = [
                            self._data[r][c] + other._data[r][c]  # type: ignore[index]
                            for c in range(cols)
                        ]
                        result.append(row)
                    return ndarray(result)
                if _is_vector_data(self._data) and _is_vector_data(other._data):
                    if len(self._data) != len(other._data):  # type: ignore[arg-type]
                        raise ValueError("incompatible arrays")
                    return ndarray([a + b for a, b in zip(self._data, other._data)])  # type: ignore[list-comprehension]
                raise ValueError("unsupported array shapes for addition")
            if _is_matrix_data(self._data):
                return ndarray([[elem + other for elem in row] for row in self._data])  # type: ignore[index]
            if _is_vector_data(self._data):
                return ndarray([elem + other for elem in self._data])  # type: ignore[list-item]
            return ndarray(self._data + other)  # type: ignore[operator]

        def __radd__(self, other: Number) -> "ndarray":
            return self + other

        def flatten(self) -> List[Number]:
            if _is_matrix_data(self._data):
                return [item for row in self._data for item in row]  # type: ignore[misc]
            if _is_vector_data(self._data):
                return list(self._data)  # type: ignore[list-item]
            return [self._data]  # type: ignore[list-item]

        def __iter__(self):
            if isinstance(self._data, list):
                return iter(self._data)
            return iter([self._data])

        def __str__(self) -> str:
            if _is_matrix_data(self._data):
                rows = ["[" + " ".join(str(x) for x in row) + "]" for row in self._data]  # type: ignore[index]
                return "[{}]".format("\n ".join(rows))
            if _is_vector_data(self._data):
                return "[" + " ".join(str(x) for x in self._data) + "]"  # type: ignore[str-format]
            return str(self._data)

        __repr__ = __str__

    def array(data: ArrayData, dtype=None) -> ndarray:
        return ndarray(data)

    def zeros(shape: Tuple[int, ...], dtype=float) -> ndarray:
        if len(shape) == 1:
            return ndarray([0 for _ in range(shape[0])])
        if len(shape) == 2:
            rows, cols = shape
            return ndarray([[0 for _ in range(cols)] for _ in range(rows)])
        raise NotImplementedError("zeros stub supports 1D or 2D shapes")

    def dot(a: ndarray, b: ndarray) -> Union[ndarray, Number]:
        return a @ b

    pi = math.pi
    e = math.e

    __all__ = ["array", "ndarray", "zeros", "dot", "pi", "e"]
