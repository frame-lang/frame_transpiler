@target python_3

# Port of legacy test_class_decorators_args.frm to V3 syntax.

system PyDecorArgsV3 {
    actions:
        run() {
            from dataclasses import dataclass

            @dataclass(frozen=True)
            class ImmutablePoint:
                x: int
                y: int

            p = ImmutablePoint(5, 12)
            print("ImmutablePoint: (" + str(p.x) + ", " + str(p.y) + ")")
        }
}

def main():
    s = PyDecorArgsV3()
    s.run()

if __name__ == '__main__':
    main()
