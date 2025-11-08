# Frame Async/Await Design Proposal

## Overview

Frame's state machine paradigm is naturally suited for asynchronous operations. State transitions already represent potentially long-running operations, making async/await a natural extension of the language.

## Design Philosophy

1. **State Machines Are Inherently Async**: Each state transition can be viewed as an async boundary
2. **Python-First Implementation**: Direct mapping to Python's async/await as our 1st class language
3. **Guide-Based for 2nd Class Languages**: Document patterns for other languages without visitor implementation

## Proposed Syntax

### Async Interface Methods

```frame
system HttpService {
    interface:
        async fetch(url: string): Response
        async process(data: Data): Result
        
    machine:
        $Idle {
            // Async interface method automatically makes handler async
            fetch(url) {
                var response = await http.get(url)
                self.lastResponse = response
                -> $Processing(response)
            }
        }
        
        $Processing(response) {
            $>(response) {
                if response.ok {
                    var data = await response.json()
                    -> $Success(data)
                } else {
                    -> $Error(response.status)
                }
            }
        }
}
```

### Async Operations

```frame
system DataProcessor {
    operations:
        // Async operation
        async transformData(input: Data): Data {
            var result = await external.process(input)
            return result
        }
        
    machine:
        $Ready {
            process(input) {
                // Await async operation
                var transformed = await self.transformData(input)
                -> $Complete(transformed)
            }
        }
}
```

### Async Functions

```frame
// Module-level async function
async fn fetchUserData(userId: int): User {
    var response = await api.get("/users/" + str(userId))
    var user = await response.json()
    return user
}

fn main() {
    // Cannot await at top level - must be in async context
    // This is a design decision to discuss
}

async fn asyncMain() {
    var user = await fetchUserData(123)
    print("User: " + user.name)
}
```

## State Machine Async Semantics

### Automatic Async Propagation

When an interface method is marked `async`, the corresponding event handlers become async:

```frame
interface:
    async getData(): Data  // Makes all getData handlers async

machine:
    $StateA {
        getData() {  // Implicitly async
            var result = await fetch()
            return result
        }
    }
```

### Async State Transitions

State transitions with async operations complete before the transition:

```frame
$Fetching {
    dataReady(url) {
        var data = await http.get(url)  // Completes before transition
        -> $Processing(data)  // Transition happens after await
    }
}
```

### Concurrent Async in States

Multiple async operations can run concurrently within a state:

```frame
$Loading {
    loadAll() {
        // These could run concurrently (language-dependent)
        var users = await loadUsers()
        var posts = await loadPosts()
        var comments = await loadComments()
        
        -> $Ready(users, posts, comments)
    }
}
```

## Python Generation (1st Class)

Direct mapping to Python's async/await:

```python
class HttpService:
    async def fetch(self, url: str) -> Response:
        response = await http.get(url)
        self.lastResponse = response
        self._transition_to_Processing(response)
        return response
        
    async def _Idle_fetch(self, url):
        return await self.fetch(url)
```

## 2nd Class Language Patterns

### JavaScript (Promise-based)

Generation guide would show:

```javascript
class HttpService {
    async fetch(url) {
        const response = await http.get(url);
        this.lastResponse = response;
        this.transitionToProcessing(response);
        return response;
    }
}
```

### Go (Goroutines and Channels)

Generation guide would demonstrate:

```go
func (s *HttpService) Fetch(url string) <-chan Response {
    ch := make(chan Response)
    go func() {
        response := <-http.Get(url)  // Blocking channel read
        s.lastResponse = response
        s.transitionToProcessing(response)
        ch <- response
    }()
    return ch
}
```

### Rust (Future-based)

Generation guide pattern:

```rust
impl HttpService {
    async fn fetch(&mut self, url: String) -> Response {
        let response = http::get(url).await;
        self.last_response = Some(response.clone());
        self.transition_to_processing(response.clone());
        response
    }
}
```

### C# (Task-based)

Generation guide:

```csharp
public class HttpService {
    public async Task<Response> FetchAsync(string url) {
        var response = await Http.GetAsync(url);
        LastResponse = response;
        TransitionToProcessing(response);
        return response;
    }
}
```

### C/C++ (Callback-based)

Generation guide would show callback patterns:

```c
typedef struct {
    void* context;
    void (*callback)(Response*, void*);
} AsyncContext;

void HttpService_fetch(HttpService* self, const char* url, AsyncContext* ctx) {
    http_get_async(url, &on_fetch_complete, ctx);
}

void on_fetch_complete(Response* response, void* context) {
    AsyncContext* ctx = (AsyncContext*)context;
    HttpService* self = (HttpService*)ctx->context;
    self->last_response = response;
    transition_to_processing(self, response);
    ctx->callback(response, ctx->context);
}
```

## Critical Design Decisions

### 1. Async Contagion

**Question**: Should async propagate through the call chain automatically?

**Recommendation**: Yes, similar to Python and JavaScript. If a method calls async operations, it must be async.

### 2. Top-Level Await

**Question**: Should Frame allow top-level await in modules?

**Recommendation**: No, require explicit async context (like Python pre-3.7). This maintains cleaner semantics.

### 3. Async Constructors

**Question**: Can system constructors be async?

**Recommendation**: No, keep constructors synchronous. Use async factory functions if needed:

```frame
system DataService {
    // Constructor remains sync
    new(config: Config) { ... }
    
    // Async factory method
    operations:
        async create(config: Config): DataService {
            var service = DataService(config)
            await service.initialize()
            return service
        }
}
```

### 4. Async Actions

**Question**: Should actions support async?

**Recommendation**: Yes, actions often perform I/O or other async operations:

```frame
actions:
    async loadData() {
        self.data = await database.query("...")
    }
```

### 5. Event Loop Integration

**Question**: How does Frame handle event loops?

**Recommendation**: Delegate to the target language runtime:
- Python: asyncio event loop
- JavaScript: native event loop
- Go: goroutine scheduler
- Others: language-specific patterns

## Implementation Phases

### Phase 1: Parser Support
- Add `async` keyword to scanner
- Support `async` modifier on functions, operations, interface methods
- Parse `await` expressions

### Phase 2: AST Representation
- Add `is_async` flag to relevant AST nodes
- Add `AwaitExprNode` for await expressions

### Phase 3: Python Visitor Implementation
- Generate `async def` for async methods
- Generate `await` for await expressions
- Handle async state transitions

### Phase 4: Documentation
- Create generation guides for each 2nd class language
- Document async patterns and best practices
- Migration guide for adding async to existing Frame code

## Benefits

1. **Natural Fit**: State machines and async operations complement each other
2. **Modern Patterns**: Aligns with contemporary async programming models
3. **Performance**: Enables non-blocking I/O in generated code
4. **Flexibility**: Works across different language async models

## Risks and Mitigations

### Risk: Complexity in State Semantics
**Mitigation**: Clear documentation on when transitions occur relative to async operations

### Risk: Language Model Mismatch
**Mitigation**: Provide detailed guides showing how Frame async maps to each language's model

### Risk: Debugging Difficulty
**Mitigation**: Ensure generated code includes appropriate async stack traces and error handling

## Example: Complete Async System

```frame
system WebCrawler {
    domain:
        var urls: list = []
        var results: dict = {}
        
    interface:
        async crawl(startUrl: string)
        async getResults(): dict
        
    machine:
        $Idle {
            crawl(startUrl) {
                self.urls.append(startUrl)
                -> $Crawling
            }
            
            getResults() {
                return self.results
            }
        }
        
        $Crawling {
            $>() {
                self.processNext()
            }
            
            getResults() {
                return self.results  // Can return partial results
            }
        }
        
    actions:
        async processNext() {
            if self.urls.length > 0 {
                var url = self.urls.pop(0)
                var content = await self.fetchUrl(url)
                self.results[url] = content
                
                var links = self.extractLinks(content)
                self.urls.extend(links)
                
                if self.urls.length == 0 {
                    -> $Complete
                }
            }
        }
        
    operations:
        async fetchUrl(url: string): string {
            var response = await http.get(url)
            return await response.text()
        }
        
        extractLinks(html: string): list {
            // Link extraction logic
            return links
        }
}
```

## Conclusion

Async/await support in Frame would:
1. Leverage the natural async boundaries in state machines
2. Provide clean syntax for modern async programming
3. Map directly to Python's async/await (1st class)
4. Offer clear patterns for 2nd class languages
5. Enable high-performance, non-blocking generated code

The implementation should focus on Python first, with comprehensive guides for other languages, maintaining Frame's philosophy of state-machine-centric design while embracing modern async patterns.
