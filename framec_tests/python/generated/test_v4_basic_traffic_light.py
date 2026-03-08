class TrafficLight:
    def __init__(self):
        self._state_stack = []
        self._state_context = {}
        self._state = "Red"
        self._enter()

    def _transition(self, target_state, exit_args = None, enter_args = None):
        self._exit()
        self._state = target_state
        self._enter()

    def _change_state(self, target_state):
        self._state = target_state

    def _dispatch_event(self, event, *args):
        handler_name = f"_s_{self._state}_{event}"
        handler = getattr(self, handler_name, None)
        if handler:
            return handler(*args)

    def _enter(self):
        # No enter handlers
        pass

    def _exit(self):
        # No exit handlers
        pass

    def tick(self):
        self._dispatch_event("tick")

    def pedestrian(self):
        self._dispatch_event("pedestrian")

    def emergency(self):
        self._dispatch_event("emergency")

    def _s_Yellow_emergency(self):

        print("Emergency - going to flashing")
        self._transition("Emergency", None, None)


    def _s_Yellow_tick(self):

        print("Yellow -> Red")
        self._transition("Red", None, None)


    def _s_Green_emergency(self):

        print("Emergency - going to flashing")
        self._transition("Emergency", None, None)


    def _s_Green_pedestrian(self):

        print("Pedestrian - shortening green")
        self._transition("Yellow", None, None)


    def _s_Green_tick(self):

        print("Green -> Yellow")
        self._transition("Yellow", None, None)


    def _s_Emergency_tick(self):

        print("Emergency resolved -> Red")
        self._transition("Red", None, None)


    def _s_Red_emergency(self):

        print("Emergency - going to flashing")
        self._transition("Emergency", None, None)


    def _s_Red_tick(self):

        print("Red -> Green")
        self._transition("Green", None, None)


    def _s_Red_pedestrian(self):

        print("Pedestrian button - staying Red")


