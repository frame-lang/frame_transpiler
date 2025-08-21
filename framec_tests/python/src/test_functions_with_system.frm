fn main() {
    var counter = Counter()
    
    // Demonstrate system interaction  
    var iterations = [1, 2, 3]
    for i in iterations {
        counter.increment()
    }
    
    print("Final count: " + counter.getCount())
}

system Counter {
    interface:
        increment()
        getCount(): int

    machine:
        $Start {
            increment() {
                count = count + 1
            }
            
            getCount(): int {
                return count
            }
        }

    domain:
        var count: int = 0
}