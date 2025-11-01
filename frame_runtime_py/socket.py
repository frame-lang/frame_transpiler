"""
Async socket helpers for Frame-generated Python targets.

This module mirrors the runtime/socket declarations used by other targets,
providing coroutine-friendly helpers backed by asyncio streams.
"""

from __future__ import annotations

import asyncio
from dataclasses import dataclass
from typing import Optional


@dataclass
class FrameSocketClient:
    """Wrapper around asyncio streams used by Frame runtime helpers."""

    reader: asyncio.StreamReader
    writer: asyncio.StreamWriter

    async def read_line(self) -> str:
        """Read a single line (without trailing newline) from the stream."""
        data = await self.reader.readline()
        if not data:
            return ""
        line = data.decode("utf-8")
        return line.rstrip("\r\n")

    async def write_line(self, line: str) -> None:
        """Write a line to the stream, ensuring newline termination."""
        payload = line if line.endswith("\n") else f"{line}\n"
        self.writer.write(payload.encode("utf-8"))
        await self.writer.drain()

    def close(self) -> None:
        """Close the underlying writer."""
        try:
            self.writer.close()
        finally:
            # Python 3.7 requires awaiting wait_closed; guard for earlier versions.
            wait_closed = getattr(self.writer, "wait_closed", None)
            if callable(wait_closed):
                asyncio.create_task(wait_closed())  # fire-and-forget


async def frame_socket_client_connect(host: str, port: int) -> FrameSocketClient:
    """Connect to the given host/port and return a FrameSocketClient wrapper."""
    reader, writer = await asyncio.open_connection(host, port)
    return FrameSocketClient(reader=reader, writer=writer)


async def frame_socket_client_read_line(
    client: FrameSocketClient,
) -> str:
    """Delegate to the client's read_line coroutine."""
    return await client.read_line()


async def frame_socket_client_write_line(
    client: FrameSocketClient,
    line: str,
) -> None:
    """Delegate to the client's write_line coroutine."""
    await client.write_line(line)


def frame_socket_client_close(client: FrameSocketClient) -> None:
    """Close the socket client."""
    client.close()


__all__ = [
    "FrameSocketClient",
    "frame_socket_client_connect",
    "frame_socket_client_read_line",
    "frame_socket_client_write_line",
    "frame_socket_client_close",
]
