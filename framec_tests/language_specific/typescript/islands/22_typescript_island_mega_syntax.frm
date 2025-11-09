 

system MegaTSIsland {
    operations:
    mega() {
        // Declarations
        let x: number = 42;
        const y = { a: 1, b: [1, 2, 3] };
        type Pair<T, U> = { first: T; second: U };
        interface HasArea { area(): number }
        class Box<T> { constructor(public value: T) {} map<U>(f: (t: T) => U): U { return f(this.value) } }

        // Functions and arrows
        function add(a: number, b: number): number { return a + b }
        const mul = (a: number, b: number) => a * b;
        const asyncFn = async (n: number) => { await Promise.resolve(); return n }

        // Control flow
        if (x > 0) { x = add(x, 1) } else { x = mul(x, 2) }
        for (let i = 0; i < 3; i++) { x += i }
        while (x < 50) { x++ }
        try { JSON.parse("{}") } catch (e) { /* ignore */ } finally { /* done */ }

        // Template literals with nested ${}
        const name = "world";
        const msg = `hello ${name.toUpperCase()} ${`${1+1}`}`;

        // Decorator-like comment (no real decorators to avoid tsconfig)
        // @example
        const arr: Array<number> = [1, 2, 3];
        const doubled = arr.map(n => n * 2);

        // Comments containing Frame-statement-like tokens must not segment
        // -> $ShouldNotSegment
        /* $$[+] */

        // Frame statements at SOL
        -> $Next
        $$[+]
        $$[-]
        => $^
    }

    machine:
        $Init { mega() { } }
        $Parent {
            $Child {}
        }
        $Next {}
}
