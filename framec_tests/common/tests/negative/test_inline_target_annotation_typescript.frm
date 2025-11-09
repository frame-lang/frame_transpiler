# Negative: inline #[target: typescript] directives must error

system InlineTargetDirectiveTS {
    interface:
        start()

    machine:
        $Init {
            start() {
                #[target: typescript]
                console.log("should not be allowed");
                return
            }
        }
}

