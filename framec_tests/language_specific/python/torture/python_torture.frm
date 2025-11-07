# Torture test: maximally complex Python MixedBody + Frame semantics

module Alpha {
    fn util_inc(x) {
        return x + 1
    }
}

system TorturePy {
    interface:
        async runTest(): bool
        compute(a: int, b: int) -> int

    domain:
        counter: int = 0
        history: list[int]
        name: str = "start"

    actions:
        log(msg):
            doc = """triple
            quote with { } braces
            """
            user = { 'name': self.name, 'count': self.counter }
            # f-string with nested braces
            tpl = f"Log: { '{' }{user['name']}{'}'} ({ '{' }{user['count']}{'}'})"
            if 'state' in tpl.lower():
                self.name = tpl
        async compute_heavy(x, y):
            # comprehensions, with/try/except/finally
            squares = [i*i for i in range(x)]
            with open(__file__) as f:
                _ = f.readline()
            try:
                self.counter = sum(squares) + y
            except Exception as e:
                self.counter = -1
            finally:
                self.history = [] if self.counter < 0 else squares
            # Frame directive at SOL
            -> $Processing(self.counter, self.name)

    operations:
        op_note(note: str):
            lines = note.split('\n')
            for l in lines:
                self.name = l.strip()
            => $^

    machine:
        $Init {
            async $>():
                # start logic
                self.history = []
                $$[+]
                -> $Idle("ready")
        }

        $Idle(name: str) {
            runTest(): bool:
                try:
                    self.counter = 1
                except Exception:
                    self.counter = -1
                finally:
                    self.history.append(self.counter)
                self.log(name)
                -> $Processing(self.counter, name)
        }

        $Processing(count: int, label: str) {
            $>():
                if count > 10:
                    $$[-]
                    -> $Done(count)
            compute(a: int, b: int) -> int:
                r = a + b
                => $^
                return r
        }

        $Done(total: int) {
            $>():
                return
        }
    }
}
