@target python

system MegaPythonIsland {
    actions:
        runmega() {
            # Begin native Python mega-syntax demo inside a Frame action
            import asyncio, math, os, sys, random, time
            from dataclasses import dataclass, field, asdict
            from typing import Protocol, runtime_checkable, TypeVar, Generic, Callable, Iterable, Iterator, Union, Optional
            from collections import Counter, defaultdict, deque, namedtuple

            VERSION: str = "1.0.0"
            PI = math.pi

            Point = namedtuple("Point", ["x", "y"])

            T = TypeVar("T")
            U = TypeVar("U")

            @runtime_checkable
            class HasArea(Protocol):
                def area(self) -> float: ...

            class Box(Generic[T]):
                def __init__(self, value: T) -> None:
                    self.value: T = value
                def map(self, f: Callable[[T], U]) -> U:
                    return f(self.value)

            class Color:
                RED = 1; GREEN = 2; BLUE = 3

            class NonNegative:
                def __set_name__(self, owner, name):
                    self.private_name = f"_{name}"
                def __get__(self, obj, objtype=None):
                    return getattr(obj, self.private_name)
                def __set__(self, obj, value):
                    if value < 0:
                        raise ValueError("non-negative only")
                    setattr(obj, self.private_name, value)

            class AttrLoggingMeta(type):
                def __new__(mcls, name, bases, ns):
                    ns["__created_with_meta__"] = True
                    return super().__new__(mcls, name, bases, ns)

            def trace(fn):
                def wrap(*a, **k):
                    print(f"[TRACE] {fn.__name__}")
                    return fn(*a, **k)
                return wrap

            @dataclass(slots=True)
            class Vec2:
                x: float
                y: float
                def __add__(self, o: "Vec2") -> "Vec2":
                    return Vec2(self.x + o.x, self.y + o.y)
                def __matmul__(self, o: "Vec2") -> float:
                    return self.x * o.x + self.y * o.y
                def norm(self) -> float:
                    return (self.x**2 + self.y**2) ** 0.5

            @dataclass(frozen=True)
            class ImmutablePair:
                a: int
                b: int

            class Account(metaclass=AttrLoggingMeta):
                balance = NonNegative()
                def __init__(self, owner: str, balance: float = 0.0) -> None:
                    self.owner = owner
                    self.balance = balance
                def deposit(self, amt: float) -> None:
                    self.balance += amt
                def __repr__(self) -> str:
                    return f"Account({self.owner!r},{self.balance:.2f})"

            @trace
            def funky_math(a: float, b: float, /, c: float = 1.0, *, op: str = "add") -> float:
                match op:
                    case "add":
                        return a + b + c
                    case "mul":
                        return a * b * c
                    case _:
                        raise ValueError("op")

            class Circle:
                def __init__(self, r: float) -> None: self.r = r
                def area(self) -> float: return math.pi * self.r * self.r

            def total_area(shapes: Iterable[HasArea]) -> float:
                return sum(s.area() for s in shapes)

            # Comprehensions and Counter/walrus
            data = [random.randint(0, 10) for _ in range(20)]
            squares = [x*x for x in data]
            evens = set([x for x in data if x % 2 == 0])
            freq = Counter(ch for ch in "abracadabra" if (n := "abracadabra".count(ch)) > 1)

            # Pattern matching
            def match_demo(obj):
                match obj:
                    case 0: return "zero"
                    case [x, y]: return f"pair({x},{y})"
                    case ImmutablePair(a=a, b=b) if a + b > 0: return "positive pair"
                    case _: return "other"

            # Context manager
            class Timer:
                def __enter__(self):
                    self.t = time.perf_counter(); return self
                def __exit__(self, *exc):
                    dt = (time.perf_counter()-self.t)*1000
                    print(f"[TIMER] {dt:.2f}ms")

            with Timer():
                _ = sum(it for it in range(5000))

            # Exceptions
            class CustomError(Exception): pass
            try:
                pass
            except Exception:
                print("should not see")
            else:
                pass

            # Protocol & total_area
            shapes = [Circle(1.0), Circle(2.0)]
            area_total = total_area(shapes)

            # Dataclasses and math
            v1, v2 = Vec2(3,4), Vec2(1,2)
            dot = (v1 @ v2)
            v3 = v1 + v2

            # Print summary so the runner sees activity
            acc = Account("alice", 100.0)
            acc.deposit(25.0)
            print("[OK] VERSION", VERSION)
            print("[OK] funky_math", funky_math(2,3, op="mul"))
            print("[OK] comp", len(data), len(squares), len(evens), dict(freq))
            print("[OK] match", match_demo([1,2]), match_demo(ImmutablePair(1,2)))
            print("[OK] area_total", round(area_total, 3))
            print("[OK] vec", v3, round(dot, 3))
        }
    machine:
        $Init {
            start() {
                runmega()
                -> $Done
            }
        }
        $Done {}
}
