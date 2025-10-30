# Validates queued exit/enter ordering across chained transitions.

system TransitionQueueExit {
    interface:
        go()
        finish()

    machine:
        $Ready {
            go() {
                print("ready go")
                -> $Active
            }

            <$() {
                print("ready exit")
                self.ready_exit_count = self.ready_exit_count + 1
                return
            }
        }

        $Active {
            $>() {
                print("active enter")
                self.active_enter_count = self.active_enter_count + 1
                return
            }

            finish() {
                print("active finish")
                -> $Done
            }

            <$() {
                print("active exit")
                self.active_exit_count = self.active_exit_count + 1
                return
            }
        }

        $Done {
            $>() {
                print("done enter")
                self.done_enter_count = self.done_enter_count + 1
                return
            }

            finish() {
                print("done finish")
                print(self.ready_exit_count)
                print(self.active_enter_count)
                print(self.active_exit_count)
                print(self.done_enter_count)
            }
        }

    domain:
        var ready_exit_count: int = 0
        var active_enter_count: int = 0
        var active_exit_count: int = 0
        var done_enter_count: int = 0
}

fn main() {
    var runner = TransitionQueueExit()
    runner.go()
    runner.finish()
    runner.finish()
}
