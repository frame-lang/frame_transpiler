# Negative: inline #[target: python] directives must error

system InlineTargetDirectivePython {
    interface:
        start()

    machine:
        $Init {
            start() {
                #[target: python]
                print("should not be allowed")
                return
            }
        }
}

