# TypeScript-Optimized Error Handling Capability Module
# Leverages TypeScript's union types, generics, and strict type checking
# Generates clean TypeScript Result<T, E> patterns

module TypeScriptErrors {
    # TypeScript Result type mapping:
    # Frame: { "isOk": true, "value": T, "error": null }
    # TypeScript: { readonly kind: "ok"; readonly value: T } | { readonly kind: "error"; readonly error: E }
    
    # Create successful result (TypeScript: Result<T, never>)
    fn createOk(value) {
        return {
            "kind": "ok",
            "isOk": True,
            "isError": False,
            "value": value,
            "error": None
        }
    }
    
    # Create error result (TypeScript: Result<never, E>)
    fn createError(errorMessage) {
        return {
            "kind": "error", 
            "isOk": False,
            "isError": True,
            "value": None,
            "error": errorMessage
        }
    }
    
    # Type-safe result checking (TypeScript: value is Result<T, never>)
    fn isOk(result) {
        return result["kind"] == "ok"
    }
    
    # Type-safe error checking (TypeScript: value is Result<never, E>)
    fn isError(result) {
        return result["kind"] == "error"
    }
    
    # Type-safe unwrap with TypeScript exhaustiveness checking
    fn unwrap(result) {
        # TypeScript will enforce that result is checked before unwrapping
        if result["kind"] == "ok" {
            return result["value"]
        } else {
            # TypeScript Error type with stack trace
            throw Error("Attempted to unwrap error result: " + str(result["error"]))
        }
    }
    
    # Safe unwrap with default (TypeScript: (result: Result<T, E>, defaultValue: T) => T)
    fn unwrapOr(result, defaultValue) {
        return result["kind"] == "ok" ? result["value"] : defaultValue
    }
    
    # Map over successful results (TypeScript: map<T, U>(result: Result<T, E>, fn: (value: T) => U): Result<U, E>)
    fn map(result, mapFunc) {
        if result["kind"] == "ok" {
            var newValue = mapFunc(result["value"])
            return createOk(newValue)
        } else {
            return result  # Pass through error unchanged
        }
    }
    
    # FlatMap for chaining (TypeScript: flatMap<T, U>(result: Result<T, E>, fn: (value: T) => Result<U, E>): Result<U, E>)
    fn flatMap(result, nextFunc) {
        if result["kind"] == "ok" {
            return nextFunc(result["value"])
        } else {
            return result
        }
    }
    
    # Promise integration for async operations
    fn fromPromise(promiseFunc) {
        # TypeScript: async function fromPromise<T>(fn: () => Promise<T>): Promise<Result<T, Error>>
        return promiseFunc().then(
            lambda value: createOk(value),
            lambda error: createError(str(error))
        )
    }
    
    # Convert Result to Promise (TypeScript: toPromise<T>(result: Result<T, E>): Promise<T>)
    fn toPromise(result) {
        if result["kind"] == "ok" {
            return Promise.resolve(result["value"])
        } else {
            return Promise.reject(Error(result["error"]))
        }
    }
    
    # Combine multiple results (TypeScript: all<T>(results: Result<T, E>[]): Result<T[], E>)
    fn all(results) {
        var values = []
        
        for result in results {
            if result["kind"] == "error" {
                return result  # Return first error encountered
            }
            values.append(result["value"])
        }
        
        return createOk(values)
    }
    
    # Return first successful result (TypeScript: any<T>(results: Result<T, E>[]): Result<T, E[]>)
    fn any(results) {
        var errors = []
        
        for result in results {
            if result["kind"] == "ok" {
                return result  # Return first success
            }
            errors.append(result["error"])
        }
        
        return createError(errors)
    }
    
    # TypeScript-specific validation patterns
    fn validateString(value, fieldName) {
        # TypeScript: (value: unknown, fieldName: string): Result<string, string>
        if typeof(value) != "string" {
            return createError(fieldName + " must be a string, got: " + typeof(value))
        }
        
        if value == "" {
            return createError(fieldName + " cannot be empty")
        }
        
        return createOk(value)
    }
    
    fn validateNumber(value, fieldName) {
        # TypeScript: (value: unknown, fieldName: string): Result<number, string>
        if typeof(value) != "number" {
            return createError(fieldName + " must be a number, got: " + typeof(value))
        }
        
        if isNaN(value) {
            return createError(fieldName + " cannot be NaN")
        }
        
        return createOk(value)
    }
    
    fn validateArray(value, fieldName) {
        # TypeScript: (value: unknown, fieldName: string): Result<unknown[], string>
        if not Array.isArray(value) {
            return createError(fieldName + " must be an array, got: " + typeof(value))
        }
        
        return createOk(value)
    }
    
    # Generic validation with type predicate functions
    fn validate(value, predicate, errorMessage) {
        # TypeScript: <T>(value: unknown, predicate: (v: unknown) => v is T, errorMessage: string): Result<T, string>
        if predicate(value) {
            return createOk(value)
        } else {
            return createError(errorMessage)
        }
    }
    
    # Option-like pattern for nullable values
    fn fromNullable(value) {
        # TypeScript: <T>(value: T | null | undefined): Result<T, "null_or_undefined">
        if value is None or value == None {
            return createError("null_or_undefined")
        } else {
            return createOk(value)
        }
    }
    
    # Safe JSON parsing for TypeScript
    fn parseJson(jsonString) {
        # TypeScript: (jsonString: string): Result<unknown, string>
        try {
            var parsed = JSON.parse(jsonString)
            return createOk(parsed)
        } except Exception as e {
            return createError("JSON parse error: " + str(e))
        }
    }
    
    # Safe property access for TypeScript objects
    fn getProperty(obj, propertyName) {
        # TypeScript: (obj: Record<string, unknown>, propertyName: string): Result<unknown, string>
        if obj.hasOwnProperty(propertyName) {
            return createOk(obj[propertyName])
        } else {
            return createError("Property '" + propertyName + "' not found")
        }
    }
    
    # Retry with exponential backoff (async)
    async fn retryAsync(asyncFunc, maxAttempts, baseDelayMs) {
        # TypeScript: async <T>(asyncFunc: () => Promise<T>, maxAttempts: number, baseDelayMs: number): Promise<Result<T, string>>
        var attempt = 0
        
        while attempt < maxAttempts {
            var result = await fromPromise(asyncFunc)
            
            if result["kind"] == "ok" {
                return result
            }
            
            attempt = attempt + 1
            
            if attempt < maxAttempts {
                var delayMs = baseDelayMs * (2 ** (attempt - 1))  # Exponential backoff
                print("Retrying async operation in " + str(delayMs) + "ms (attempt " + str(attempt) + ")")
                await sleep(delayMs)
            }
        }
        
        return createError("Max retry attempts exceeded (" + str(maxAttempts) + ")")
    }
}