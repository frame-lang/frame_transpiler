// Torture test: maximally complex TS MixedBody + Frame semantics
module Alpha {
    // module-scope declarations to exercise scoping
    fn util_inc(x) { return x + 1 }
}

system TortureTS {
    interface:
        async runTest(): bool
        compute(a: number, b: number): number

    domain:
        var counter: number = 0
        var history: Array<number>
        var name: string = "start"

    actions:
        log(msg) {
            // native TS body with templates, regex, comments
            const user = { name: this.name, count: this.counter };
            const tpl = `Log: ${'$'}{user.name} (${ '$'}{`${'$'}{user.count}` })`;
            const re = new RegExp("\\bstate\\b", "i"); // regex via constructor
            if (re.test(tpl)) { this.name = tpl; }
        }
        async computeHeavy(x, y) {
            // arrow, destructuring, nested template
            const add = (a: number, b: number) => a + b;
            const [a,b] = [x,y];
            const s = `sum=${'$'}{`${'$'}{a}+${'$'}{b}`}`;
            this.counter = add(a,b);
            // Frame directive at SOL
            -> $Processing(this.counter, this.name)
        }

    operations:
        op_note(note: string) {
            // parent forward embedded
            // some native code
            const lines = note.split("\n");
            for (const l of lines) { this.name = l.trim(); }
            => $^
        }

    machine:
        $Init {
            async $>() {
                // start logic
                this.history = [];
                // push current state, then transition
                $$[+]
                -> $Idle("ready")
            }
        }

        $Idle(name: string) {
            runTest(): bool {
                // TS native with try/catch/finally
                try { this.counter = 1; } catch (e) { this.counter = -1; } finally { this.history.push(this.counter); }
                this.log(name);
                // transition with args pulling from native vars
                -> $Processing(this.counter, name)
                // unreachable after transition (checked elsewhere)
            }
        }

        $Processing(count: number, label: string) {
            $>() {
                // interleave directives
                if (count > 10) {
                    $$[-]
                    -> $Done(count)
                }
            }
            compute(a: number, b: number): number {
                const r = a + b;
                // parent forward
                => $^
                return r;
            }
        }

        $Done(total: number) {
            $>() { return }
        }
    }
}

