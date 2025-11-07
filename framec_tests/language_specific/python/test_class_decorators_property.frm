# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Property decorators exercised inside a native Python action body

system PyDecorProperty {
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

fn main() {
    s = PyDecorProperty()
    s._action_run()
}
