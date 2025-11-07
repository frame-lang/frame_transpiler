# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test custom enum values

fn main() {
    var http = HttpServer()
    http.testStatusCodes()
    
    var logger = Logger()
    logger.testLogLevels()
}

system HttpServer {
    interface:
        testStatusCodes()
    
    machine:
        $Ready {
            testStatusCodes() {
                var ok = HttpStatus.Ok
                var notFound = HttpStatus.NotFound
                var serverError = HttpStatus.ServerError
                
                print("OK code: " + str(ok.value))
                print("NotFound code: " + str(notFound.value))
                print("ServerError code: " + str(serverError.value))
                
                if ok.value == 200 {
                    print("OK status is correctly 200")
                }
                
                if notFound.value == 404 {
                    print("NotFound status is correctly 404")
                }
                
                if serverError.value == 500 {
                    print("ServerError status is correctly 500")
                }
                
                return
            }
        }
    
    domain:
        # Integer enum with custom values
        enum HttpStatus {
            Ok = 200
            Created = 201
            BadRequest = 400
            NotFound = 404
            ServerError = 500
        }
}

system Logger {
    interface:
        testLogLevels()
    
    machine:
        $Active {
            testLogLevels() {
                # Test negative values
                var silent = Priority.Silent
                var low = Priority.Low
                var high = Priority.High
                
                print("Silent priority: " + str(silent.value))
                print("Low priority: " + str(low.value))
                print("High priority: " + str(high.value))
                
                # Test mixed explicit and auto values
                var unknown = MixedEnum.Unknown
                var idle = MixedEnum.Idle
                var running = MixedEnum.Running
                var complete = MixedEnum.Complete
                var error = MixedEnum.Error
                
                print("Unknown: " + str(unknown.value))  # -1
                print("Idle: " + str(idle.value))        # 0
                print("Running: " + str(running.value))  # 1 (auto)
                print("Complete: " + str(complete.value)) # 100
                print("Error: " + str(error.value))      # 101 (auto)
                
                return
            }
        }
    
    domain:
        # Enum with negative values
        enum Priority {
            Silent = -1
            Low = 1
            Medium = 5
            High = 10
        }
        
        # Mixed explicit and auto values
        enum MixedEnum {
            Unknown = -1
            Idle = 0
            Running      # 1 (auto continues from 0)
            Complete = 100
            Error        # 101 (auto continues from 100)
        }
}