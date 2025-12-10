@target python_3

# Port of legacy test_class_decorators_simple.frm to V3 syntax.

system PyDecorSimpleV3 {
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

def main():
    s = PyDecorSimpleV3()
    s.run()

if __name__ == '__main__':
    main()
