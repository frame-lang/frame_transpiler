@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Class decorators exercised within a native Python action body

system PyDecorAll {
    actions:
        run() {
            from dataclasses import dataclass, field
            from typing import ClassVar

            @dataclass
            class Point:
                def __init__(self, x, y):
                    self.x = x
                    self.y = y

            @dataclass
            @dataclass(frozen=True)
            class ImmutablePoint:
                def __init__(self, x, y):
                    self.x = x
                    self.y = y

            @dataclass(order=True, repr=False)
            class OrderedPoint:
                def __init__(self, x, y, z):
                    self.x = x
                    self.y = y
                    self.z = z

            def my_custom_decorator(cls):
                print(f"Decorating class {cls.__name__}")
                return cls

            @dataclass
            @my_custom_decorator
            class CustomPoint:
                class_counter: ClassVar[int] = 0

                def __init__(self, x, y):
                    self.x = x
                    self.y = y
                    CustomPoint.class_counter = CustomPoint.class_counter + 1

                def distance_to_origin(self):
                    return (self.x ** 2 + self.y ** 2) ** 0.5

            p1 = Point(3, 4)
            p2 = ImmutablePoint(5, 12)
            p3 = OrderedPoint(1, 2, 3)
            p4 = CustomPoint(6, 8)

            print(f"Point: ({p1.x}, {p1.y})")
            print(f"ImmutablePoint: ({p2.x}, {p2.y})")
            print(f"OrderedPoint: ({p3.x}, {p3.y}, {p3.z})")
            print(f"CustomPoint: ({p4.x}, {p4.y})")
            print(f"CustomPoint instances: {CustomPoint.class_counter}")
            print(f"Distance: {p4.distance_to_origin()}")
        }
}

def main():
    s = PyDecorAll()
    s._action_run()

if __name__ == '__main__':
    main()
