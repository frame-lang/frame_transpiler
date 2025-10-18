system SimpleTypeScript {
    operations:
        getValue(): str {
            return "hello"
        }

    machine:
        $Start {
            $>() {
                print("Test")
            }
        }
}