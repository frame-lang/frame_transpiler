"""Core runtime primitives for generated Python Frame systems.

This module centralizes the shared Frame runtime classes so every generated
system reuses the same definitions instead of embedding per-file copies.
"""

from __future__ import annotations
from dataclasses import dataclass
from typing import Any, Dict, Optional


class FrameEvent:
    """Represents a message delivered to the Frame runtime."""

    __slots__ = ("_message", "_parameters")

    def __init__(self, message: str, parameters: Optional[Any]):
        self._message = message
        self._parameters = parameters


@dataclass
class FrameCompartment:
    """Encapsulates state machine context for the active compartment."""

    state: str
    forward_event: Optional[FrameEvent] = None
    exit_args: Optional[Any] = None
    enter_args: Optional[Any] = None
    parent_compartment: Optional["FrameCompartment"] = None
    state_vars: Optional[Dict[str, Any]] = None
    state_args: Optional[Dict[str, Any]] = None

    def __post_init__(self) -> None:
        # Ensure mutable defaults are unique per compartment
        if self.state_vars is None:
            self.state_vars = {}
        if self.state_args is None:
            self.state_args = {}


from . import socket

__all__ = ["FrameEvent", "FrameCompartment", "socket"]
