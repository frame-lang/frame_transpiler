# String utilities module for complex multi-file test

module StringModule {
    fn createGreeting(name) {
        return "Hello, " + name + "! Welcome to Frame v0.57"
    }
    
    fn reverseString(text) {
        var result = ""
        var i = len(text) - 1
        while i >= 0 {
            result = result + text[i]
            i = i - 1
        }
        return result
    }
    
    fn uppercase(text) {
        return text.upper()
    }
    
    fn lowercase(text) {
        return text.lower()
    }
}