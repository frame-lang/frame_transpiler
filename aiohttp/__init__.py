"""
Minimal aiohttp stub for offline Frame transpiler testing.

If a real aiohttp installation exists outside the project root this module
delegates to it; otherwise it exposes a tiny subset of the API used by the
Python positive tests (async context managers for HTTP GET/POST).
"""
from __future__ import annotations

import importlib.util
import pathlib
import sys
from typing import Any, Dict, Optional

_REAL_AIOHTTP = None
_MODULE_PATH = pathlib.Path(__file__).resolve()
_PROJECT_ROOT = _MODULE_PATH.parent.parent

_original_sys_path = list(sys.path)
try:
    sys.path = [
        p
        for p in sys.path
        if pathlib.Path(p).resolve() != _PROJECT_ROOT
    ]
    spec = importlib.util.find_spec("aiohttp")
    if spec and spec.origin and pathlib.Path(spec.origin).resolve() != _MODULE_PATH:
        module = importlib.util.module_from_spec(spec)
        assert spec.loader is not None
        spec.loader.exec_module(module)  # type: ignore[misc]
        globals().update(module.__dict__)
        _REAL_AIOHTTP = module
finally:
    sys.path = _original_sys_path

if _REAL_AIOHTTP is None:

    class ClientSession:
        def __init__(self, *args: Any, **kwargs: Any):
            self._closed = False

        async def __aenter__(self) -> "ClientSession":
            return self

        async def __aexit__(self, exc_type, exc, tb) -> Optional[bool]:
            await self.close()
            return None

        async def close(self) -> None:
            self._closed = True

        def get(self, url: str, *args: Any, **kwargs: Any) -> "_StubResponse":
            return _StubResponse(url, method="GET", data=kwargs.get("data"))

        def post(self, url: str, *args: Any, **kwargs: Any) -> "_StubResponse":
            return _StubResponse(url, method="POST", data=kwargs.get("data"))

    class _StubResponse:
        def __init__(self, url: str, method: str = "GET", data: Any = None):
            self._url = url
            self._method = method
            self._data = data
            self.status = 200
            self.headers: Dict[str, str] = {
                "Content-Type": "application/json"
            }

        async def __aenter__(self) -> "_StubResponse":
            return self

        async def __aexit__(self, exc_type, exc, tb) -> Optional[bool]:
            return None

        async def text(self) -> str:
            return f"stubbed {self._method} response from {self._url}"

        async def json(self) -> Dict[str, Any]:
            return {
                "url": self._url,
                "method": self._method,
                "data": self._data,
            }

    __all__ = ["ClientSession"]
