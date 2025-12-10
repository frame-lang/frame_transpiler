@target python_3

# Port of legacy test_class_decorators_property.frm to V3 syntax.

system PyDecorPropertyV3 {
    actions:
        run() {
            class Temperature:
                def __init__(self, celsius):
                    self._celsius = celsius

                @property
                def fahrenheit(self):
                    return self._celsius * 9.0 / 5.0 + 32.0

                @property
                def celsius(self):
                    return self._celsius

            temp = Temperature(25.0)
            print("Celsius: " + str(temp.celsius))
            print("Fahrenheit: " + str(temp.fahrenheit))
        }
}

def main():
    s = PyDecorPropertyV3()
    s.run()

if __name__ == '__main__':
    main()
