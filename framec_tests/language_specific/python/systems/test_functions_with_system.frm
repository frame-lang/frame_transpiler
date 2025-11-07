# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    counter = Counter()
    
    # Demonstrate system interaction  
    iterations = [1, 2, 3]
    for i in iterations {
        counter.increment()
    }
    
    print("Final count: " + str(counter.getCount()))
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
        count: int = 0
}