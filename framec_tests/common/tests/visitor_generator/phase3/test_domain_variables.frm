# Phase 3: Domain variables

system DataProcessor {
    interface:
        initialize(value: int): bool
        getValue(): int
        process(): string
    
    machine:
        $Ready {
            initialize(value: int) {
                data = value
                count = 1
                system.return = true
                return
            }
            
            getValue() {
                system.return = data
                return  
            }
            
            process() {
                var result = performCalculation()
                system.return = result
                return
            }
        }
    
    actions:
        performCalculation(): string {
            count = count + 1
            return "Processed " + str(data) + " times: " + str(count)
        }
    
    domain:
        var data: int = 0
        var count: int = 0
        var status: string = "idle"
}