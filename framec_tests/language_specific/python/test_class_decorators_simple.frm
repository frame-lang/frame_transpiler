@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test class decorators pass-through for Python (v0.58)
# Simplified to run inside a native Python action body

system PyDecorSimple {
    actions:
        run() {
            from dataclasses import dataclass

            @dataclass
            class Point:
                def __init__(self, x, y):
                    self.x = x
                    self.y = y

            p1 = Point(3, 4)
            print("Point: (" + str(p1.x) + ", " + str(p1.y) + ")")
        }
}

fn main() {
    s = PyDecorSimple()
    s._action_run()
}
