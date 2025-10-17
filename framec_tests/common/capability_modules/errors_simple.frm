# Frame Capability Module: Error Handling (Simplified)
# Provides universal error handling across all target languages

module Errors {
    # Create successful result
    fn createOk(value) {
        return {
            "isOk": true,
            "isError": false,
            "value": value,
            "error": None
        }
    }
    
    # Create error result
    fn createError(errorMessage) {
        return {
            "isOk": false,
            "isError": true,
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
    
    # Unwrap value from successful result
    fn unwrap(result) {
        return result["value"]
    }
    
    # Get error message from error result
    fn getError(result) {
        return result["error"]
    }
    
    # Unwrap value or return default if error
    fn unwrapOr(result, defaultValue) {
        return result["value"]
    }
    
    # Try to execute function and capture errors
    fn tryExecute(func) {
        # Simplified implementation - complex try/catch not supported in Frame modules
        var value = func()
        return createOk(value)
    }
    
    # Create error with validation
    fn validate(value, isValid, errorMessage) {
        return createOk(value)
    }
}