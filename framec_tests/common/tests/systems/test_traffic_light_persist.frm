# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Removed backticks - import jsonpickle

system TrafficLight {

    operations:

        @staticmethod
        unmarshal(data): TrafficLight {
            return jsonpickle.decode(data)
        }

        marshal(): JSON {
            return jsonpickle.encode(self)
        }
    
    interface:

        tick()

    machine:

        $Red {
            $>() {
                print("Red")
            }

            tick() {
                -> $Green
            }
        }

        $Green {
            $>() {
                print("Green")
            }

            tick() {
                -> $Yellow
            }
        }

        $Yellow {
            $>() {
                print("Yellow")
            }

            tick() {
                -> $Red
            }
        }
}

fn main() {
    var tl = TrafficLight()
    tl.tick()
    tl.tick()
    tl.tick()
    tl.tick()
}