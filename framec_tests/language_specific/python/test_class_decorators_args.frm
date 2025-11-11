@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test class decorators with arguments (v0.58) — executed inside a native action

system PyDecorArgs {
    actions:
        run() {
            from dataclasses import dataclass

            @dataclass(frozen=True)
            class ImmutablePoint:
                def __init__(self, x, y):
                    self.x = x
                    self.y = y

            p = ImmutablePoint(5, 12)
            print("ImmutablePoint: (" + str(p.x) + ", " + str(p.y) + ")")
        }
}

fn main() {
    s = PyDecorArgs()
    s._action_run()
}
