# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test enum iteration

fn main() {
    menu = MenuSystem()
    menu.displayAllOptions()
    menu.validateOption("SaveFile")
    menu.validateOption("InvalidOption")
    
    validator = Validator()
    validator.processAllStatuses()
}

system MenuSystem {
    interface:
        displayAllOptions()
        validateOption(name: string): bool
    
    machine:
        $Ready {
            displayAllOptions() {
                print("=== Menu Options ===")
                index = 1
                
                # Iterate over enum values
                for option in MenuOption:
                    print(str(index) + ". " + option.name)
                    index = index + 1
                
                return
            }
            
            validateOption(name: string): bool {
                # Check if name is valid enum member
                for option in MenuOption:
                    if option.name == name:
                        print(name + " is a valid menu option")
                        return true
                
                print(name + " is NOT a valid menu option")
                return false
            }
        }
    
    domain:
        enum MenuOption {
            NewFile
            OpenFile
            SaveFile
            SaveAs
            Exit
        }
}

system Validator {
    interface:
        processAllStatuses()
    
    machine:
        $Active {
            processAllStatuses() {
                print("=== Processing All Statuses ===")
                
                # Iterate with custom values
                for status in ProcessStatus:
                    print("Status: " + status.name + " (code: " + str(status.value) + ")")
                    
                    if status.value < 0:
                        print("  -> This is an error status")
                    elif status.value == 0:
                        print("  -> This is the idle status")
                    else:
                        print("  -> This is an active status")
                }
                
                # Count total statuses
                count = 0
                for s in ProcessStatus:
                    count = count + 1
                print("Total statuses: " + str(count))
                
                return
            }
        }
    
    domain:
        enum ProcessStatus {
            Error = -1
            Idle = 0
            Starting = 1
            Running = 10
            Stopping = 20
            Complete = 100
        }
}
