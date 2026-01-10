class TrafficLight:
    def __init__(self):
        self._state = None
        self._state_stack = []
        self._transition_to_Red()

    def timer(self):
        self._system_return = None
        self._dispatch('timer', locals())
        return self._system_return

    def get_color(self) -> str:
        self._system_return = None
        self._dispatch('get_color', locals())
        return self._system_return

    def _dispatch(self, event, args):
        handler = getattr(self, f'_handle_{self._state}_{event}', None)
        if handler:
            # Remove 'self' from args
            args = {k: v for k, v in args.items() if k != 'self'}
            handler(**args)

    def _transition_to_Red(self):
        if self._state:
            exit_handler = getattr(self, f'_exit_{self._state}', None)
            if exit_handler:
                exit_handler()
        self._state = 'Red'

    def _handle_Red_timer(self):
        "Red light timing out"

    def _handle_Red_get_color(self):
        "red"

    def _transition_to_Green(self):
        if self._state:
            exit_handler = getattr(self, f'_exit_{self._state}', None)
            if exit_handler:
                exit_handler()
        self._state = 'Green'

    def _handle_Green_timer(self):
        "Green light timing out"

    def _handle_Green_get_color(self):
        "green"

    def _transition_to_Yellow(self):
        if self._state:
            exit_handler = getattr(self, f'_exit_{self._state}', None)
            if exit_handler:
                exit_handler()
        self._state = 'Yellow'

    def _handle_Yellow_timer(self):
        "Yellow light timing out"

    def _handle_Yellow_get_color(self):
        "yellow"

