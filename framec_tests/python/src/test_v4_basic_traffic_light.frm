@@target python_3

@@system TrafficLight {
    interface:
        tick()
        pedestrian()
        emergency()

    machine:
        $Red {
            tick() {
                print("Red -> Green")
                -> $Green
            }

            pedestrian() {
                print("Pedestrian button - staying Red")
            }

            emergency() {
                print("Emergency - going to flashing")
                -> $Emergency
            }
        }

        $Green {
            tick() {
                print("Green -> Yellow")
                -> $Yellow
            }

            pedestrian() {
                print("Pedestrian - shortening green")
                -> $Yellow
            }

            emergency() {
                print("Emergency - going to flashing")
                -> $Emergency
            }
        }

        $Yellow {
            tick() {
                print("Yellow -> Red")
                -> $Red
            }

            emergency() {
                print("Emergency - going to flashing")
                -> $Emergency
            }
        }

        $Emergency {
            tick() {
                print("Emergency resolved -> Red")
                -> $Red
            }
        }
}
