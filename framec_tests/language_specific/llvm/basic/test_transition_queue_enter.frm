# Validates that queued transitions run exit before the new state's enter handler.

system TransitionQueueEnter {
    interface:
        start()
        check()

    machine:
        $Ready {
            start() {
                print("ready handling")
                -> $Active
            }

            <$() {
                print("ready exit")
                self.exit_count = self.exit_count + 1
                return
            }
        }

        $Active {
            $>() {
                print("active enter")
                self.enter_count = self.enter_count + 1
                return
            }

            check() {
                print("active check")
                print(self.enter_count)
                print(self.exit_count)
            }
        }

    domain:
        var enter_count: int = 0
        var exit_count: int = 0
}

fn main() {
    var runner = TransitionQueueEnter()
    runner.start()
    runner.check()
}
