"""
V3 persistence helpers for Python Frame systems (Stage 14).

This module implements the minimal snapshot / restore API described in
`docs/framepiler_design/architecture_v3/14_persistence_and_snapshots.md`
for the Python target.

The snapshot format is a plain Python dict that matches the conceptual
`SystemSnapshot` JSON shape:

    {
        "schemaVersion": 1,
        "systemName": "S",
        "state": "__S_state_A",
        "stateArgs": { ... },
        "domainState": { ... },
        "stack": [
            { "state": "__S_state_B", "stateArgs": { ... } },
            ...
        ]
    }

Notes:
- `state` and the `stack[*].state` values currently use the concrete
  `FrameCompartment.state` strings emitted by the Python V3 generator
  (e.g. `"__S_state_A"`). A future refinement may expose logical state
  identifiers separately, but this representation is stable for round‑trips.
- `domainState` is populated from non‑private attributes on the system
  instance (those whose names do not start with "_") unless an explicit
  domain key list is provided.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Callable, Dict, Iterable, List, MutableMapping, Optional, Tuple, TypeVar
import json

from frame_runtime_py import FrameCompartment


SystemT = TypeVar("SystemT")


@dataclass
class SystemSnapshot:
    """In‑memory representation of a system snapshot for Python."""

    schemaVersion: int
    systemName: str
    state: str
    # `stateArgs` is stored in the same structural shape as the runtime
    # `FrameCompartment.state_args` attribute. It is typically a dict for
    # start-state parameters and may be a list for transitions that were
    # emitted positionally.
    stateArgs: Any
    domainState: Dict[str, Any]
    stack: List[Dict[str, Any]]

    def to_dict(self) -> Dict[str, Any]:
        return {
            "schemaVersion": self.schemaVersion,
            "systemName": self.systemName,
            "state": self.state,
            "stateArgs": self.stateArgs,
            "domainState": dict(self.domainState),
            "stack": [dict(frame) for frame in self.stack],
        }

    @staticmethod
    def from_dict(data: MutableMapping[str, Any]) -> "SystemSnapshot":
        return SystemSnapshot(
            schemaVersion=int(data.get("schemaVersion", 1)),
            systemName=str(data.get("systemName", "")),
            state=str(data.get("state", "")),
            stateArgs=data.get("stateArgs", {}) or {},
            domainState=dict(data.get("domainState", {}) or {}),
            stack=[dict(frame) for frame in data.get("stack", []) or []],
        )


def _default_domain_keys(system: Any) -> List[str]:
    """Infer domain field names from a generated Python V3 system instance.

    By default we treat any non‑callable attribute whose name does not
    start with "_" as part of the domain state. This is intentionally
    conservative; callers may override via the domain_keys parameter.
    """
    keys: List[str] = []
    for name in dir(system):
        if name.startswith("_"):
            continue
        # Avoid dunder attributes and imported modules/functions.
        if name.startswith("__") and name.endswith("__"):
            continue
        try:
            value = getattr(system, name)
        except Exception:
            continue
        if callable(value):
            continue
        keys.append(name)
    return keys


def snapshot_system(
    system: Any,
    *,
    system_name: Optional[str] = None,
    domain_keys: Optional[Iterable[str]] = None,
    domain_encoder: Optional[Callable[[Any], MutableMapping[str, Any]]] = None,
) -> SystemSnapshot:
    """Create a `SystemSnapshot` for a generated Python V3 system.

    Args:
        system: Instance of a generated V3 system (e.g. `S()` from a
            `@target python` module).
        system_name: Optional override for the system name. Defaults to
            `type(system).__name__`.
        domain_keys: Optional iterable of attribute names to treat as
            domain state. If omitted, non‑private, non‑callable attributes
            are inferred automatically.
    """
    # Determine system name
    sys_name = system_name or type(system).__name__

    # Current compartment
    try:
        compartment = getattr(system, "_compartment")
    except AttributeError as exc:
        raise ValueError("snapshot_system expects a V3 Python system with a '_compartment' attribute") from exc

    state = getattr(compartment, "state", "")
    raw_state_args = getattr(compartment, "state_args", {}) or {}
    # Preserve the structural shape of state_args (dict, list, etc.) while
    # avoiding aliasing mutable containers from the live system.
    if isinstance(raw_state_args, dict):
        state_args: Any = dict(raw_state_args)
    elif isinstance(raw_state_args, (list, tuple)):
        state_args = list(raw_state_args)
    else:
        state_args = raw_state_args

    # Domain state: either delegate to a custom encoder or infer attributes.
    if domain_encoder is not None:
        raw_domain = domain_encoder(system)
        domain = dict(raw_domain)
    else:
        if domain_keys is None:
            domain_keys = _default_domain_keys(system)
        domain = {}
        for name in domain_keys:
            try:
                domain[name] = getattr(system, name)
            except AttributeError:
                # Skip missing keys to keep the snapshot tolerant
                continue

    # Stack of prior compartments (if any)
    stack_snapshots: List[Dict[str, Any]] = []
    stack = getattr(system, "_stack", []) or []
    for comp in stack:
        comp_state = getattr(comp, "state", "")
        raw_comp_state_args = getattr(comp, "state_args", {}) or {}
        if isinstance(raw_comp_state_args, dict):
            comp_state_args: Any = dict(raw_comp_state_args)
        elif isinstance(raw_comp_state_args, (list, tuple)):
            comp_state_args = list(raw_comp_state_args)
        else:
            comp_state_args = raw_comp_state_args
        stack_snapshots.append(
            {
                "state": comp_state,
                "stateArgs": comp_state_args,
            }
        )

    return SystemSnapshot(
        schemaVersion=1,
        systemName=sys_name,
        state=str(state),
        stateArgs=state_args,
        domainState=domain,
        stack=stack_snapshots,
    )


def restore_system(
    snapshot: SystemSnapshot,
    system_factory: Callable[[], SystemT],
    *,
    domain_keys: Optional[Iterable[str]] = None,
    domain_decoder: Optional[Callable[[SystemSnapshot, SystemT], None]] = None,
) -> SystemT:
    """Restore a system instance from a `SystemSnapshot`.

    The caller is responsible for providing `system_factory`, which must
    construct a fresh instance of the corresponding generated system with
    an appropriate constructor call (e.g. `lambda: S()`).

    The restored instance has:
    - `_compartment` set to a new `FrameCompartment` with the snapshot
      state and stateArgs.
    - `_stack` rebuilt from the snapshot stack entries.
    - Domain fields populated from `snapshot.domainState` (restricted to
      `domain_keys` when provided, or all keys otherwise).

    No `$enter` or other events are fired as part of restoration.
    """
    sys = system_factory()

    # Rebuild current compartment
    comp = FrameCompartment(
        snapshot.state,
        enter_args=None,
        state_args=snapshot.stateArgs,
    )
    setattr(sys, "_compartment", comp)

    # Rebuild stack
    stack: List[FrameCompartment] = []
    for entry in snapshot.stack:
        state = str(entry.get("state", ""))
        state_args = entry.get("stateArgs", {}) or {}
        stack.append(FrameCompartment(state, enter_args=None, state_args=state_args))
    setattr(sys, "_stack", stack)

    # Restore domain fields using the generic attribute mapper, then allow an
    # optional domain_decoder to perform additional reconstruction for complex
    # object graphs.
    domain = snapshot.domainState
    keys = list(domain_keys) if domain_keys is not None else list(domain.keys())
    for name in keys:
        if name in domain:
            setattr(sys, name, domain[name])

    if domain_decoder is not None:
        domain_decoder(snapshot, sys)

    return sys


def snapshot_to_json(snapshot: SystemSnapshot, *, indent: Optional[int] = None) -> str:
    """Encode a `SystemSnapshot` to a JSON string."""
    return json.dumps(snapshot.to_dict(), indent=indent, sort_keys=True)


def snapshot_from_json(data: str) -> SystemSnapshot:
    """Decode a JSON string into a `SystemSnapshot`."""
    raw = json.loads(data)
    if not isinstance(raw, dict):
        raise ValueError("snapshot_from_json expected a JSON object at the top level")
    return SystemSnapshot.from_dict(raw)


__all__ = [
    "SystemSnapshot",
    "snapshot_system",
    "restore_system",
    "snapshot_to_json",
    "snapshot_from_json",
    "compare_snapshots",
]


def compare_snapshots(
    a: SystemSnapshot, b: SystemSnapshot
) -> Tuple[bool, List[str]]:
    """Compare two SystemSnapshot instances for equality.

    Returns (equal, differences), where `equal` is True when all fields match
    structurally and `differences` contains human-readable descriptions of
    any mismatched fields. This is intended for tests and tooling (including
    potential cross-language snapshot comparisons)."""
    differences: List[str] = []

    if a.schemaVersion != b.schemaVersion:
        differences.append(
            f"schemaVersion: {a.schemaVersion!r} != {b.schemaVersion!r}"
        )
    if a.systemName != b.systemName:
        differences.append(
            f"systemName: {a.systemName!r} != {b.systemName!r}"
        )
    if a.state != b.state:
        differences.append(f"state: {a.state!r} != {b.state!r}")
    if a.stateArgs != b.stateArgs:
        differences.append(
            f"stateArgs: {a.stateArgs!r} != {b.stateArgs!r}"
        )
    if a.domainState != b.domainState:
        differences.append(
            f"domainState: {a.domainState!r} != {b.domainState!r}"
        )
    if a.stack != b.stack:
        differences.append(f"stack: {a.stack!r} != {b.stack!r}")

    return not differences, differences
