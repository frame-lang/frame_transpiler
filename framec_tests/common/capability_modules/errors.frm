# Frame Capability Module: Error Handling
# Provides universal error handling across all target languages
# Python: uses try/except with custom Result class
# TypeScript: uses try/catch with Result union types
# C#: uses try/catch with Result<T,E> or exceptions
# Java: uses try/catch with Result<T,E> or exceptions
# Go: uses multiple return values (value, error)
# Rust: uses native Result<T,E> type
# C: uses error codes with output parameters

module Errors {
    # Create successful result
    fn createOk(value) {
        return {
            "isOk": True,
            "isError": False,
            "value": value,
            "error": None
        }
    }
    
    # Create error result
    fn createError(errorMessage) {
        return {
            "isOk": False,
            "isError": True,
            "value": None,
            "error": errorMessage
        }
    }
    
    # Check if result is successful
    fn isOk(result) {
        return result["isOk"]
    }
    
    # Check if result is error
    fn isError(result) {
        return result["isError"]
    }
    
    # Unwrap value from successful result (throws if error)
    fn unwrap(result) {
        if result["isOk"] {
            return result["value"]
        } else {
            raise RuntimeError("Attempted to unwrap error result: " + str(result["error"]))
        }
    }
    
    # Unwrap value or return default if error
    fn unwrapOr(result, defaultValue) {
        if result["isOk"] {
            return result["value"]
        } else {
            return defaultValue
        }
    }
    
    # Get error message from error result
    fn getError(result) {
        if result["isError"] {
            return result["error"]
        } else {
            return None
        }
    }
    
    # Map successful result to new value
    fn mapOk(result, mapFunc) {
        if result["isOk"] {
            var newValue = mapFunc(result["value"])
            return createOk(newValue)
        } else {
            return result  # Pass through error unchanged
        }
    }
    
    # Map error result to new error
    fn mapError(result, mapFunc) {
        if result["isError"] {
            var newError = mapFunc(result["error"])
            return createError(newError)
        } else {
            return result  # Pass through ok unchanged
        }
    }
    
    # Chain operations that return results
    fn andThen(result, nextFunc) {
        # If result is Ok, call nextFunc with the value
        # If result is Error, pass through the error
        if result["isOk"] {
            return nextFunc(result["value"])
        } else {
            return result
        }
    }
    
    # Provide alternative result if current is error
    fn orElse(result, altFunc) {
        if result["isError"] {
            return altFunc(result["error"])
        } else {
            return result
        }
    }
    
    # Try to execute function and capture errors
    fn tryExecute(func) {
        # Execute function and return Result
        # Python: try/except block
        # TypeScript: try/catch block
        # C#: try/catch block
        # Java: try/catch block
        # Go: check error return value
        # Rust: Result from function
        # C: check return code
        
        try {
            var value = func()
            return createOk(value)
        } except Exception as e {
            return createError(str(e))
        }
    }
    
    # Execute function with error handling
    async fn tryExecuteAsync(asyncFunc) {
        # Async version of tryExecute
        try {
            var value = await asyncFunc()
            return createOk(value)
        } except Exception as e {
            return createError(str(e))
        }
    }
    
    # Combine multiple results (fail if any fail)
    fn combineResults(results) {
        var values = []
        
        for result in results {
            if result["isError"] {
                return result  # Return first error encountered
            }
            values.append(result["value"])
        }
        
        return createOk(values)
    }
    
    # Execute function and retry on error
    fn retry(func, maxAttempts, delayMs) {
        var attempt = 0
        
        while attempt < maxAttempts {
            var result = tryExecute(func)
            
            if result["isOk"] {
                return result
            }
            
            attempt = attempt + 1
            
            if attempt < maxAttempts {
                # Wait before retrying (implementation depends on language)
                print("Retrying after " + str(delayMs) + "ms (attempt " + str(attempt) + ")")
                # Real implementation would use platform-specific sleep
            }
        }
        
        return createError("Max retry attempts exceeded (" + str(maxAttempts) + ")")
    }
    
    # Async version of retry
    async fn retryAsync(asyncFunc, maxAttempts, delayMs) {
        var attempt = 0
        
        while attempt < maxAttempts {
            var result = await tryExecuteAsync(asyncFunc)
            
            if result["isOk"] {
                return result
            }
            
            attempt = attempt + 1
            
            if attempt < maxAttempts {
                print("Retrying async after " + str(delayMs) + "ms (attempt " + str(attempt) + ")")
                # Would use AsyncSupport.sleep(delayMs) in real implementation
            }
        }
        
        return createError("Max async retry attempts exceeded (" + str(maxAttempts) + ")")
    }
    
    # Create error with context
    fn createContextError(message, context) {
        var fullMessage = message + " (Context: " + str(context) + ")"
        return createError(fullMessage)
    }
    
    # Add context to existing error
    fn addContext(result, context) {
        if result["isError"] {
            var newMessage = result["error"] + " (Context: " + str(context) + ")"
            return createError(newMessage)
        } else {
            return result
        }
    }
    
    # Convert exception to result
    fn fromException(exception) {
        return createError(str(exception))
    }
    
    # Convert result to exception (for compatibility)
    fn toException(result) {
        if result["isError"] {
            return RuntimeError(result["error"])
        } else {
            return None
        }
    }
    
    # Validate value and return result
    fn validate(value, validator, errorMessage) {
        if validator(value) {
            return createOk(value)
        } else {
            return createError(errorMessage)
        }
    }
    
    # Ensure value is not null/None
    fn requireNotNull(value, fieldName) {
        if value is None {
            return createError(fieldName + " cannot be null")
        } else {
            return createOk(value)
        }
    }
    
    # Ensure string is not empty
    fn requireNotEmpty(str_value, fieldName) {
        if str_value is None or str_value == "" {
            return createError(fieldName + " cannot be empty")
        } else {
            return createOk(str_value)
        }
    }
    
    # Ensure number is in range
    fn requireInRange(number, min_val, max_val, fieldName) {
        if number < min_val or number > max_val {
            return createError(fieldName + " must be between " + str(min_val) + " and " + str(max_val))
        } else {
            return createOk(number)
        }
    }
    
    # Pattern matching for results (similar to Rust)
    fn match(result, okHandler, errorHandler) {
        if result["isOk"] {
            return okHandler(result["value"])
        } else {
            return errorHandler(result["error"])
        }
    }
}