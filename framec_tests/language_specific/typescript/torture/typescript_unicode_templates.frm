fn main() {
    var u = UnicodeTs()
    u.run()
}

system UnicodeTs {
    interface:
        run(): None

    machine:
        $Idle {
            run(): None {
                // Template with emoji and unicode
                const x = `emoji: 😀👩🏻‍🚀 café naïve résumé`;
                const y = `nested ${`inner ${`deep`}`} ok`;
                const z = `multiline 🧪
second line
third`;
                return;
            }
        }

    actions:
        helper(): None {
            const msg = `αβγ δει ${42}`;
            return;
        }
}

