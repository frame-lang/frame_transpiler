> **⚠️ DEPRECATED - DO NOT READ UNLESS INSTRUCTED ⚠️**
>
> This document is archived and may contain outdated or incorrect information about Frame syntax.
> For current Frame V4 syntax, see `frame_v4_lang_reference.md` in the parent directory.

---

# Frame v4 System Initialization and Parameters

## System Instantiation with @@system

The `@@system` annotation is Frame's mechanism for declaring and instantiating Frame systems. This annotation enables Frame to track system instances and provide semantic validation of all system interactions.

### Basic Syntax

```python
@@target python

# System definition
system TrafficLight {
    interface:
        start()
        stop()
        getColor(): *str*
    
    machine:
        $Red {
            # states...
        }
}

# System instantiation - creates an instance that Frame tracks
@@system *light = TrafficLight()*

# Now Frame validates all calls to 'light'
*light.start()*      # Valid - in interface
*light.getColor()*   # Valid - in interface  
*light.helper()*     # ERROR - not in interface (caught at compile time)
```

### Instantiation with Parameters

```python
@@target python

# System with parameters
system ConfigurableServer(*host: str, port: int, debug: bool = False*) {
    # system implementation...
}

# Instantiate with arguments
@@system *server = ConfigurableServer("localhost", 8080, True)*

# Multiple instances with different configurations
@@system *prodServer = ConfigurableServer("prod.example.com", 443, False)*
@@system *testServer = ConfigurableServer("test.example.com", 8080, True)*
```

### Why @@system Matters

The `@@system` annotation provides several critical benefits:

1. **Semantic Validation**: Frame validates that all method calls on the system instance match the interface
2. **Type Safety**: Frame ensures parameter types and return types match the interface definition
3. **Access Control**: Frame prevents direct access to private actions, operations, and domain variables
4. **Debug Support**: Frame Debug Adapter can track and visualize system state transitions
5. **Documentation**: Makes it clear which variables are Frame systems vs regular objects

### Examples of @@system Usage

```typescript
@@target typescript

// System definition
system DataProcessor {
    interface:
        process(*data: any*): *Promise<any>*
        getStatus(): *string*
        reset(): *void*
}

// Create tracked instance
@@system *processor = DataProcessor()*

// Frame validates all interactions
async function main() {
    *const result = await processor.process(data)*  // Valid
    *const status = processor.getStatus()*          // Valid
    *processor.reset()*                              // Valid
    
    // These would be compile-time errors:
    // processor.internalHelper()  // ERROR: not in interface
    // processor.domain_var         // ERROR: domain is private
}
```

### Module-Level Systems

Systems can be instantiated at module level:

```python
@@target python

# Module-level system instantiation
@@system *globalController = SystemController()*

fn main() {
    # Use the module-level system
    *globalController.start()*
    *result = globalController.process(data)*
}
```

### System Arrays and Collections

Multiple system instances can be managed in collections:

```python
@@target python

# Create multiple system instances
@@system *server1 = Server("host1", 8080)*
@@system *server2 = Server("host2", 8081)*
@@system *server3 = Server("host3", 8082)*

# Manage in a collection
*servers = [server1, server2, server3]*

# Iterate over systems
*for server in servers:*
    *server.start()*
```

## System Parameters

Frame systems can accept parameters during instantiation, allowing for configurable, reusable system definitions.

### System Parameter Syntax

```python
@@target python

# System definition with parameters
system ConfigurableServer(*host: str, port: int, debug: bool = False*) {
    operations:
        getAddress() {
            *return f"{self.host}:{self.port}"*
        }
    
    interface:
        start()
        stop()
        getStatus(): *str*
    
    machine:
        $Stopped {
            start() {
                *if self.debug:*
                    *print(f"Starting server on {self.getAddress()}")*
                -> $Running()
            }
            
            getStatus() {
                *return "stopped"*
            }
        }
        
        $Running {
            stop() {
                *if self.debug:*
                    *print("Stopping server")*
                -> $Stopped()
            }
            
            getStatus() {
                *return "running"*
            }
        }
    
    domain:
        # System parameters become instance variables
        *host = host*
        *port = port*
        *debug = debug*
}

# System instantiation with parameters
@@system *server = ConfigurableServer("localhost", 8080, True)*
```

### TypeScript Example

```typescript
@@target typescript

system DataProcessor(*batchSize: number, timeout: number, retries: number = 3*) {
    operations:
        shouldRetry(): *boolean* {
            *return this.currentRetries < this.retries*
        }
    
    machine:
        $Ready {
            process(*data: any[]*) {
                *if (data.length > this.batchSize) {*
                    -> $BatchProcessing(*data*)
                *} else {*
                    -> $Processing(*data*)
                *}*
            }
        }
    
    domain:
        *let batchSize: number = batchSize*
        *let timeout: number = timeout*
        *let retries: number = retries*
        *let currentRetries: number = 0*
}
```

## Default Initialization

Frame supports default values at multiple levels to reduce boilerplate and provide sensible defaults.

### Interface Method Defaults

Interface methods can specify default return values that are used when an event handler doesn't explicitly return a value:

```python
@@target python

system Calculator {
    interface:
        # Methods with default return values
        getValue(): *int* = *0*
        getName(): *str* = *"unnamed"*
        isReady(): *bool* = *False*
        getConfig(): *dict* = *{}*
        calculate(*x: int, y: int*): *int*  # No default - must be explicitly returned
    
    machine:
        $Idle {
            getValue() {
                # Can override the default
                *if self.has_value:*
                    *return self.current_value*
                # Otherwise returns interface default (0)
            }
            
            getName() {
                # No return statement - uses interface default ("unnamed")
            }
            
            calculate(*x: int, y: int*) {
                # Must return explicitly - no default provided
                *return x + y*
            }
        }
}
```

### Enter Handler Initialization

Enter handlers (`$>`) are the primary mechanism for state initialization:

```python
@@target python

system StatefulSystem {
    machine:
        $Uninitialized {
            start() {
                -> $Active(*default_config()*)
            }
        }
        
        $Active {
            # Enter handler with parameter and default initialization
            $>(*config: dict*) {
                # Initialize state-specific data
                *self.config = config*
                *self.start_time = now()*
                *self.counter = 0*
                *self.buffer = []*
                
                # Perform startup actions
                *self.connect_to_service()*
                *self.load_resources()*
            }
            
            $<() {
                # Cleanup on exit
                *self.disconnect_from_service()*
                *self.release_resources()*
            }
        }
}
```

### Action Default Implementations

Actions can provide default behavior while allowing override:

```python
@@target python

system BaseProcessor {
    actions:
        # Actions with default implementations
        preProcess(*data*) {
            # Default: validate and normalize
            *if not self.validate(data):*
                -> $Error("Invalid data")
            *return self.normalize(data)*
        }
        
        postProcess(*result*) {
            # Default: log and return
            *self.log(f"Processed: {result}")*
            *return result*
        }
        
        handleError(*error*) {
            # Default error handling
            *self.log_error(error)*
            -> $Error(*error*)
        }
}
```

## Initialization Patterns

### Pattern 1: Configuration Object

```python
@@target python

# Define configuration structure
*ConfigType = {
    "database": {"host": str, "port": int},
    "cache": {"enabled": bool, "ttl": int},
    "logging": {"level": str, "file": str}
}*

system Application(*config: ConfigType*) {
    operations:
        connectDatabase() {
            *db_config = self.config["database"]*
            *return connect(db_config["host"], db_config["port"])*
        }
    
    machine:
        $Init {
            $>() {
                # Use configuration
                *self.db = self.connectDatabase()*
                *if self.config["cache"]["enabled"]:*
                    *self.cache = create_cache(self.config["cache"]["ttl"])*
                -> $Ready()
            }
        }
    
    domain:
        *config = config*
        *db = None*
        *cache = None*
}
```

### Pattern 2: Builder Pattern

```python
@@target python

system ServerBuilder {
    interface:
        withHost(*host: str*): *self*
        withPort(*port: int*): *self*
        withSSL(*enabled: bool*): *self*
        build(): *Server*
    
    machine:
        $Building {
            withHost(*host: str*) {
                *self.host = host*
                *return self*
            }
            
            withPort(*port: int*) {
                *self.port = port*
                *return self*
            }
            
            withSSL(*enabled: bool*) {
                *self.ssl_enabled = enabled*
                *return self*
            }
            
            build() {
                *server = Server(self.host, self.port, self.ssl_enabled)*
                -> $Complete()
                *return server*
            }
        }
    
    domain:
        *host = "localhost"*  # Defaults
        *port = 8080*
        *ssl_enabled = False*
}
```

### Pattern 3: Factory Pattern

```python
@@target python

system ProcessorFactory {
    interface:
        createProcessor(*type: str*): *Processor*
    
    operations:
        makeProcessor(*type: str, config: dict*) {
            *if type == "batch":*
                *return BatchProcessor(config)*
            *elif type == "stream":*
                *return StreamProcessor(config)*
            *else:*
                *return DefaultProcessor(config)*
        }
    
    machine:
        $Ready {
            createProcessor(*type: str*) {
                *config = self.getConfig(type)*
                *processor = self.makeProcessor(type, config)*
                *return processor*
            }
        }
}
```

## Domain Variable Initialization

Domain variables can have default values and initialization expressions:

```python
@@target python

system DataManager {
    domain:
        # Simple defaults
        *counter: int = 0*
        *name: str = "default"*
        *active: bool = True*
        
        # Complex initialization
        *items: list = []*
        *cache: dict = {}*
        *config = load_config("config.json")*
        
        # Computed initialization
        *id = generate_uuid()*
        *timestamp = now()*
        *logger = get_logger(__name__)*
}
```

### TypeScript Domain Initialization

```typescript
@@target typescript

system DataManager {
    domain:
        # TypeScript requires let/const
        *let counter: number = 0*
        *let name: string = "default"*
        *let active: boolean = true*
        
        # Complex types
        *let items: string[] = []*
        *let cache: Map<string, any> = new Map()*
        *const config = loadConfig("config.json")*
        
        # Computed initialization
        *const id: string = generateUUID()*
        *const timestamp: Date = new Date()*
        *const logger = getLogger(__name__)*
}
```

## Lazy Initialization

Some resources should be initialized only when needed:

```python
@@target python

system LazyLoader {
    operations:
        getDatabase() {
            # Lazy initialization
            *if self.db is None:*
                *self.db = connect_database()*
            *return self.db*
        }
        
        getCache() {
            # Initialize on first use
            *if not hasattr(self, 'cache'):*
                *self.cache = create_cache()*
            *return self.cache*
        }
    
    machine:
        $Ready {
            query(*sql: str*) {
                # Database initialized only when needed
                *db = self.getDatabase()*
                *return db.execute(sql)*
            }
        }
    
    domain:
        *db = None*
}
```

## Multi-Stage Initialization

Complex systems may require multi-stage initialization:

```python
@@target python

system ComplexSystem {
    machine:
        $Uninitialized {
            bootstrap() {
                -> $Bootstrapping()
            }
        }
        
        $Bootstrapping {
            $>() {
                # Stage 1: Core initialization
                *self.load_core_modules()*
                *self.verify_environment()*
                -> $Configuring()
            }
        }
        
        $Configuring {
            $>() {
                # Stage 2: Configuration
                *self.load_configuration()*
                *self.validate_settings()*
                -> $Connecting()
            }
        }
        
        $Connecting {
            $>() {
                # Stage 3: External connections
                *self.connect_to_services()*
                *self.register_handlers()*
                -> $Ready()
            }
            
            $<() {
                # Cleanup if initialization fails
                *self.disconnect_all()*
            }
        }
        
        $Ready {
            # Fully initialized and operational
        }
}
```

## Best Practices

### 1. Use System Parameters for Configuration
- Pass configuration through system parameters
- Avoid hard-coded values in domain
- Support both required and optional parameters with defaults

### 2. Initialize in Enter Handlers
- Use `$>()` for state-specific initialization
- Clean up in `$<()` exit handlers
- Pass initialization data through transition parameters

### 3. Provide Sensible Defaults
- Interface methods should have default returns where appropriate
- Domain variables should have safe initial values
- Optional parameters should have reasonable defaults

### 4. Lazy Initialize Expensive Resources
- Defer database connections, file handles, network connections
- Initialize on first use rather than system creation
- Cache initialized resources

### 5. Validate During Initialization
- Check parameters in enter handlers
- Transition to error states for invalid configuration
- Provide clear error messages

### 6. Support Reinitialization
- Design states to be re-enterable
- Clean up properly in exit handlers
- Allow configuration changes at runtime

## Summary

Frame v4's initialization system provides:
- **System parameters** for configurable, reusable systems
- **Default values** at interface, domain, and action levels
- **Enter handlers** for state-specific initialization
- **Exit handlers** for cleanup
- **Flexible patterns** for complex initialization scenarios
- **Native language support** for all initialization code
