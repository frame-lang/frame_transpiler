@target python

system UnicodeStringsPy {
    interface:
        run(): None

    machine:
        $Idle {
            run(): None {
                # Unicode content in strings and f-strings
                x = "emoji: 😀👩🏻‍🚀 café naïve résumé"
                y = f"combo αβγ: {x}"
                long = """multi-line 🧠📦
spanning
several
lines"""
                # Ensure parser tolerates heavy unicode without Frame statements
                return
            }
        }
}
