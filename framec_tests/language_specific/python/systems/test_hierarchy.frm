# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test complex module hierarchies

module Company {
    name = "Frame Corp"
    
    module Engineering {
        team_size = 10
        
        module Frontend {
            fn getStack() {
                return "React, TypeScript, Frame"
            }
            
            fn getTeamSize() {
                return 4
            }
        }
        
        module Backend {
            fn getStack() {
                return "Python, FastAPI, PostgreSQL"
            }
            
            fn getTeamSize() {
                return 6
            }
        }
        
        fn getTotalSize() {
            return Frontend::getTeamSize() + Backend::getTeamSize()
        }
    }
    
    module Sales {
        team_size = 5
        
        fn getQuota() {
            return 1000000
        }
    }
    
    fn getInfo() {
        return name + " has " + str(Engineering::team_size + Sales::team_size) + " employees"
    }
}

fn main() {
    print("Company: " + Company::name)
    print("Engineering team: " + str(Company::Engineering::team_size))
    print("Frontend stack: " + Company::Engineering::Frontend::getStack())
    print("Backend stack: " + Company::Engineering::Backend::getStack())
    print("Total engineering: " + str(Company::Engineering::getTotalSize()))
    print("Sales quota: $" + str(Company::Sales::getQuota()))
    print("Company info: " + Company::getInfo())
}
