# Frame v4 Persistence and Snapshots

## Overview

Frame v4 provides a standardized persistence model that allows systems to be saved and restored, maintaining their complete state including:
- Current state and state parameters
- Domain variables
- State stack (for push/pop operations)
- Compartment data

## Persistence Philosophy: Native-First Approach

Frame v4 embraces a **native-first persistence strategy**, allowing developers to use their language's existing persistence ecosystem while Frame provides state machine-specific helpers. This means:

1. **Native annotations are supported** alongside Frame annotations
2. **Native serialization libraries** can be used directly
3. **Frame provides state access** but doesn't dictate persistence mechanism
4. **Each language uses its idiomatic patterns**

## Native Annotations Support

Frame v4 supports native annotations for all target languages, allowing seamless integration with existing persistence frameworks. Native annotations can be placed:
- Before system declarations
- Before interface methods  
- Before domain fields

### Annotation Syntax by Language

Frame's parser recognizes and preserves these annotation patterns:

| Language | Annotation Syntax | Example | Placement |
|----------|------------------|---------|-----------|
| **Python** | `@annotation` | `@dataclass` | Before system, methods, fields |
| **TypeScript** | `@annotation` or `@annotation()` | `@Injectable()` | Before system, methods, fields |
| **JavaScript** | `@annotation` (with Babel/TS) | `@serializable` | Before system, methods, fields |
| **Rust** | `#[annotation]` | `#[derive(Serialize)]` | Before system, fields |
| **C++** | `[[annotation]]` | `[[nodiscard]]` | Before system, methods |
| **Java** | `@Annotation` | `@Entity` | Before system, methods, fields |
| **C#** | `[Attribute]` | `[Serializable]` | Before system, methods, fields |
| **C** | `__attribute__((x))` | `__attribute__((packed))` | Limited use |
| **Go** | Struct tags | `` `json:"name"` `` | Field tags only |

### How Native Annotations Work with Frame

Frame's parser:
1. **Collects** native annotations as opaque strings
2. **Preserves** them without interpretation
3. **Positions** them correctly in generated code
4. **Lets native compilers** handle their semantics

Example parsing:
```python
# Input
@dataclass
@serialize
@@persist
system MySystem {
    domain:
        @Column()
        count: int = 0
}

# Parser sees:
# - Native annotations: ["@dataclass", "@serialize"]  
# - Frame annotations: ["@@persist"]
# - Domain field annotation: ["@Column()"]
```

## The @@persist Annotation

Systems opt into persistence support using the `@@persist` annotation, which can be combined with native persistence annotations:

```python
@@target python

@@persist system TrafficLight {
    interface:
        tick()
        getColor(): *str*
        save(): *str*      # Returns JSON snapshot
        restore(*json: str*)  # Restores from JSON
    
    machine:
        $Red {
            tick() {
                -> $Green()
            }
            getColor() {
                *return "red"*
            }
        }
        
        $Green {
            tick() {
                -> $Yellow()
            }
            getColor() {
                *return "green"*
            }
        }
        
        $Yellow {
            tick() {
                -> $Red()
            }
            getColor() {
                *return "yellow"*
            }
        }
    
    domain:
        *tickCount = 0*
        *startTime = now()*
}
```

## Snapshot Format

Frame uses a language-neutral JSON format for system snapshots:

```json
{
    "schemaVersion": 1,
    "systemName": "TrafficLight",
    "state": "Green",
    "stateArgs": {},
    "domainState": {
        "tickCount": 5,
        "startTime": "2024-01-15T10:30:00Z"
    },
    "stack": []
}
```

### Snapshot Schema

| Field | Type | Description |
|-------|------|-------------|
| `schemaVersion` | *number* | Schema version for compatibility |
| `systemName` | *string* | Name of the Frame system |
| `state` | *string* | Current state name (without `$`) |
| `stateArgs` | *object* | Current state's parameters |
| `domainState` | *object* | Domain variable values |
| `stack` | *array* | State stack for `$$[+]`/`$$[-]` |

## Generated Persistence Methods

When `@@persist` is present, Frame generates language-appropriate persistence methods:

### Python
```python
@@persist system DataManager {
    # System definition...
}

# Generated class methods
*system = DataManager()*
*json_snapshot = DataManager.save_to_json(system)*
*restored_system = DataManager.restore_from_json(json_snapshot)*
```

### TypeScript
```typescript
@@persist system DataManager {
    // System definition...
}

// Generated static methods
*const system = new DataManager()*
*const jsonSnapshot = DataManager.saveToJson(system)*
*const restoredSystem = DataManager.restoreFromJson(jsonSnapshot)*
```

### Rust
```rust
@@persist system DataManager {
    // System definition...
}

// Implements SnapshotableSystem trait
*let system = DataManager::new()*
*let json_snapshot = system.save_to_json()?*
*let restored_system = DataManager::restore_from_json(&json_snapshot)?*
```

## State Stack Persistence

Systems using state stack operations have their stack preserved:

```python
@@target python
@@persist system ModalDialog {
    machine:
        $MainScreen {
            openSettings() {
                $$[+]  # Push current state
                -> $Settings()
            }
        }
        
        $Settings {
            close() {
                $$[-]  # Pop back to previous state
            }
        }
}
```

Snapshot with stack:
```json
{
    "schemaVersion": 1,
    "systemName": "ModalDialog",
    "state": "Settings",
    "stateArgs": {},
    "domainState": {},
    "stack": [
        {
            "state": "MainScreen",
            "stateArgs": {}
        }
    ]
}
```

## State Parameters Persistence

State parameters passed during transitions are preserved:

```python
@@target python
@@persist system Processor {
    machine:
        $Idle {
            startProcessing(*data: dict, priority: int*) {
                -> $Processing(*data, priority*)
            }
        }
        
        $Processing {
            $>(*data: dict, priority: int*) {
                *self.currentData = data*
                *self.priority = priority*
            }
        }
}
```

Snapshot with state parameters:
```json
{
    "schemaVersion": 1,
    "systemName": "Processor",
    "state": "Processing",
    "stateArgs": {
        "data": {"key": "value"},
        "priority": 5
    },
    "domainState": {
        "currentData": {"key": "value"},
        "priority": 5
    },
    "stack": []
}
```

## Selective Domain Persistence

Not all domain variables need to be persisted. Frame provides control over what gets saved:

```python
@@target python
@@persist(domain=[tickCount, mode]) system Controller {
    domain:
        *tickCount = 0*        # Persisted
        *mode = "normal"*      # Persisted
        *tempBuffer = []*      # Not persisted
        *debugFlag = False*    # Not persisted
}
```

Alternatively, exclude specific fields:

```python
@@persist(exclude=[tempBuffer, debugFlag]) system Controller {
    domain:
        *tickCount = 0*        # Persisted
        *mode = "normal"*      # Persisted
        *tempBuffer = []*      # Not persisted
        *debugFlag = False*    # Not persisted
}
```

## Custom Serialization

For complex domain objects, custom serialization can be provided:

```python
@@target python
@@persist system ComplexSystem {
    operations:
        encodeDomain() {
            # Custom encoding logic
            *return {
                "config": self.config.to_dict(),
                "cache": list(self.cache.keys()),  # Only save keys
                "timestamp": self.timestamp.isoformat()
            }*
        }
        
        decodeDomain(*snapshot: dict*) {
            # Custom decoding logic
            *self.config = Config.from_dict(snapshot["config"])*
            *self.cache = {k: None for k in snapshot["cache"]}*  # Rebuild cache
            *self.timestamp = parse_datetime(snapshot["timestamp"])*
        }
    
    domain:
        *config: Config*
        *cache: dict*
        *timestamp: datetime*
}
```

## Persistence Patterns

### Pattern 1: Checkpoint/Restore

```python
@@target python
@@persist system LongRunningTask {
    interface:
        checkpoint(): *str*
        resumeFromCheckpoint(*checkpoint: str*)
    
    machine:
        $Processing {
            checkpoint() {
                # Save current progress
                *snapshot = self.__class__.save_to_json(self)*
                *save_to_file(f"checkpoint_{self.taskId}.json", snapshot)*
                *return snapshot*
            }
        }
    
    actions:
        resumeFromCheckpoint(*checkpoint: str*) {
            # Restore from checkpoint
            *restored = self.__class__.restore_from_json(checkpoint)*
            *self.__dict__.update(restored.__dict__)*
        }
}
```

### Pattern 2: Migration Between Versions

```python
@@target python
@@persist system VersionedSystem {
    operations:
        migrateSnapshot(*snapshot: dict*) {
            # Handle schema version differences
            *version = snapshot.get("schemaVersion", 1)*
            
            *if version == 1:*
                # Migrate from v1 to v2
                *snapshot["newField"] = "default"*
                *snapshot["schemaVersion"] = 2*
            
            *return snapshot*
        }
}
```

### Pattern 3: Cross-Process State Transfer

```python
@@target python
@@persist system DistributedWorker {
    interface:
        transferToWorker(*workerId: str*)
    
    actions:
        transferToWorker(*workerId: str*) {
            # Serialize current state
            *snapshot = self.__class__.save_to_json(self)*
            
            # Send to another process/worker
            *send_to_worker(workerId, snapshot)*
            
            # This instance can now shut down
            -> $Transferred()
        }
}
```

## Limitations

### What IS Persisted
- Current state identifier
- State parameters (from transitions)
- Selected domain variables
- State stack
- Compartment data

### What IS NOT Persisted
- Live resources (file handles, network connections, threads)
- Event handler local variables
- In-flight event data
- External references (unless explicitly handled)
- Private runtime bookkeeping

### Resource Reconstruction

After restoring from a snapshot, resources must be re-established:

```python
@@target python
@@persist system NetworkService {
    machine:
        $Connected {
            $>(*host: str, port: int*) {
                # Re-establish connection after restore
                *if not self.socket:*
                    *self.socket = connect(host, port)*
            }
            
            $<() {
                # Clean up before snapshot
                *if self.socket:*
                    *self.socket.close()*
                    *self.socket = None*
            }
        }
}
```

## Best Practices

### 1. Design for Persistence
- Keep domain state serializable
- Avoid storing live resources in domain
- Use state parameters for transient data
- Clean up resources in exit handlers

### 2. Version Your Schema
- Increment `schemaVersion` for breaking changes
- Provide migration logic for upgrades
- Test snapshot compatibility across versions

### 3. Test Persistence Thoroughly
```python
*# Test snapshot/restore cycle*
*original = MySystem()*
*original.process(data)*

*snapshot = MySystem.save_to_json(original)*
*restored = MySystem.restore_from_json(snapshot)*

*# Verify state is preserved*
*assert original.getState() == restored.getState()*
*assert original.getData() == restored.getData()*

*# Verify behavior continues correctly*
*original.nextStep()*
*restored.nextStep()*
*assert original.getResult() == restored.getResult()*
```

### 4. Handle Restoration Failures
```python
*try:*
    *system = MySystem.restore_from_json(snapshot)*
*catch error:*
    *# Fall back to fresh instance*
    *log_error("Failed to restore:", error)*
    *system = MySystem()*
```

### 5. Document Persistence Behavior
- Clearly indicate which systems support persistence
- Document what domain fields are persisted
- Explain any custom serialization logic
- Provide examples of save/restore usage

## Recommended Persistence Approaches by Language

### Python - Native Library Integration

```python
@@target python

import pickle
import json
import jsonpickle
from typing import Dict, Any

# Simple Frame system - no class-level decorators needed
@@persist
system UserSession {
    interface:
        save(): str
        restore(data: str)
        saveToFile(filename: str)
        loadFromFile(filename: str)
    
    machine:
        $Active {
            timeout() {
                -> $Expired()
            }
        }
        
        $Expired {
            reactivate() {
                -> $Active()
            }
        }
    
    actions:
        # Option 1: JSON serialization (human-readable)
        save() {
            state_dict = {
                'state': self._state,
                'domain': {
                    'username': self.username,
                    'session_id': self.session_id,
                    'login_time': self.login_time.isoformat() if self.login_time else None
                }
            }
            return json.dumps(state_dict)
        }
        
        restore(data: str) {
            state_dict = json.loads(data)
            self._state = state_dict['state']
            self.username = state_dict['domain']['username']
            self.session_id = state_dict['domain']['session_id']
            if state_dict['domain']['login_time']:
                self.login_time = datetime.fromisoformat(state_dict['domain']['login_time'])
        }
        
        # Option 2: Pickle (binary, Python-only)
        saveToFile(filename: str) {
            with open(filename, 'wb') as f:
                pickle.dump(self, f)
        }
        
        loadFromFile(filename: str) {
            with open(filename, 'rb') as f:
                restored = pickle.load(f)
                self.__dict__.update(restored.__dict__)
        }
    
    domain:
        username: str = "guest"
        session_id: str = ""
        login_time: datetime = None
}
```

### TypeScript - Native JSON and Libraries

```typescript
@@target typescript

// Using plain JSON - no decorators needed for basic persistence
@@persist
system UserSystem {
    interface:
        save(): string
        restore(json: string): void
        toJSON(): object
    
    machine:
        $Pending {
            approve() {
                -> $Active()
            }
        }
        
        $Active {
            suspend() {
                -> $Suspended()
            }
        }
    
    actions:
        // Simple JSON serialization
        save(): string {
            return JSON.stringify({
                state: this._state,
                domain: {
                    id: this.id,
                    email: this.email,
                    lastLogin: this.lastLogin
                }
            })
        }
        
        restore(json: string): void {
            const data = JSON.parse(json)
            this._state = data.state
            this.id = data.domain.id
            this.email = data.domain.email
            this.lastLogin = new Date(data.domain.lastLogin)
        }
        
        // Standard toJSON for JSON.stringify support
        toJSON(): object {
            return {
                state: this._state,
                id: this.id,
                email: this.email,
                lastLogin: this.lastLogin
            }
        }
    
    domain:
        let id: number = 0
        let email: string = ""
        let lastLogin: Date = new Date()
}
```

### Rust - Serde Integration

```rust
@@target rust

use serde::{Serialize, Deserialize};
use serde_json;

// Serde derives are the standard for Rust serialization
#[derive(Serialize, Deserialize)]
@@persist
system SessionManager {
    interface:
        save(&self) -> String
        restore(&mut self, data: &str) -> Result<(), String>
    
    machine:
        $Active {
            expire() {
                -> $Expired()
            }
        }
        
        $Expired {
            reactivate() {
                -> $Active()
            }
        }
    
    actions:
        save(&self) -> String {
            // Serde automatically handles the serialization
            serde_json::to_string(self).unwrap()
        }
        
        restore(&mut self, data: &str) -> Result<(), String> {
            // Deserialize and update self
            let restored: SessionManager = serde_json::from_str(data)
                .map_err(|e| e.to_string())?;
            *self = restored;
            Ok(())
        }
    
    domain:
        id: u64 = 0
        username: String = String::new()
        created_at: u64 = 0  // Unix timestamp
        
        #[serde(skip)]  // Excluded from serialization
        temp_cache: Vec<u8> = vec![]
}
```

### Java - Jackson or Gson

```java
@@target java

import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.annotation.JsonIgnore;

// Simple serialization with Jackson
@@persist
system WorkflowSystem {
    interface:
        String save()
        void restore(String json)
    
    machine:
        $Draft {
            submit() {
                -> $UnderReview()
            }
        }
        
        $UnderReview {
            approve() {
                -> $Approved()
            }
            
            reject() {
                -> $Draft()
            }
        }
    
    actions:
        String save() {
            ObjectMapper mapper = new ObjectMapper();
            try {
                return mapper.writeValueAsString(this);
            } catch (Exception e) {
                throw new RuntimeException(e);
            }
        }
        
        void restore(String json) {
            ObjectMapper mapper = new ObjectMapper();
            try {
                WorkflowSystem restored = mapper.readValue(json, WorkflowSystem.class);
                this._state = restored._state;
                this.id = restored.id;
                this.title = restored.title;
            } catch (Exception e) {
                throw new RuntimeException(e);
            }
        }
    
    domain:
        Long id = null
        String title = ""
        
        @JsonIgnore  // Excluded from JSON serialization
        transient String tempData = null
}
```

### C# - JSON.NET or System.Text.Json

```csharp
@@target csharp

using System;
using Newtonsoft.Json;

// Simple JSON serialization with Newtonsoft.Json
[Serializable]
@@persist
system ProcessSystem {
    interface:
        string Save()
        void Restore(string json)
    
    machine:
        $Running {
            Pause() {
                -> $Paused()
            }
        }
        
        $Paused {
            Resume() {
                -> $Running()
            }
        }
    
    actions:
        string Save() {
            return JsonConvert.SerializeObject(this);
        }
        
        void Restore(string json) {
            var restored = JsonConvert.DeserializeObject<ProcessSystem>(json);
            this._state = restored._state;
            this.Id = restored.Id;
            this.Name = restored.Name;
            this.Progress = restored.Progress;
        }
    
    domain:
        public int Id { get; set; } = 0
        public string Name { get; set; } = ""
        public int Progress { get; set; } = 0
        
        [JsonIgnore]  // Excluded from serialization
        public string TempBuffer { get; set; } = ""
}
```

### Go - JSON with Struct Tags

```go
@@target go

import (
    "encoding/json"
)

// Go uses struct tags for serialization control
@@persist
system TaskSystem {
    interface:
        Save() (string, error)
        Restore(data string) error
    
    machine:
        $Pending {
            Start() {
                -> $Running()
            }
        }
    
    actions:
        Save() (string, error) {
            data, err := json.Marshal(self)
            return string(data), err
        }
        
        Restore(data string) error {
            return json.Unmarshal([]byte(data), self)
        }
    
    domain:
        // Struct tags control JSON serialization
        ID        uint   `json:"id"`
        Name      string `json:"name"`
        State     string `json:"state"`
        
        // Excluded from JSON
        TempData  []byte `json:"-"`
}
```

### C++ - Manual or Library-Specific

```cpp
@@target cpp

#include <nlohmann/json.hpp>
using json = nlohmann::json;

// C++ typically uses library-specific approaches
@@persist
system StateMachine {
    interface:
        std::string save()
        void restore(const std::string& data)
    
    machine:
        $Idle {
            start() {
                -> $Running()
            }
        }
    
    actions:
        std::string save() {
            json j;
            j["state"] = _state;
            j["id"] = id;
            j["name"] = name;
            return j.dump();
        }
        
        void restore(const std::string& data) {
            json j = json::parse(data);
            _state = j["state"];
            id = j["id"];
            name = j["name"];
        }
    
    domain:
        int id = 0;
        std::string name = "";
        std::string _tempCache = "";  // Prefix convention for exclusion
}
```

## Compatibility Matrix

| Language | Native Annotations | Parser Support | Common Frameworks |
|----------|-------------------|----------------|-------------------|
| **Python** | ✅ Full support | `@decorator` | dataclasses, SQLAlchemy, Pydantic, marshmallow |
| **TypeScript** | ✅ Full support | `@decorator()` | TypeORM, class-transformer, class-validator |
| **JavaScript** | ✅ With transpiler | `@decorator` | Same as TypeScript (needs Babel/TS) |
| **Rust** | ✅ Full support | `#[attribute]` | serde, diesel, sqlx |
| **Java** | ✅ Full support | `@Annotation` | JPA/Hibernate, Jackson, JAXB |
| **C#** | ✅ Full support | `[Attribute]` | Entity Framework, Newtonsoft.Json, DataAnnotations |
| **Go** | ⚠️ Struct tags only | `` `tag:"value"` `` | encoding/json, gorm, validator |
| **C++** | ⚠️ Limited | `[[attribute]]` | Manual or library-specific macros |
| **C** | ⚠️ Very limited | `__attribute__` | Mostly manual serialization |

## Parser Implementation Strategy

### Phase 1: Annotation Collection
The Frame parser collects annotations without interpretation:

```rust
// Pseudocode for annotation parsing
fn parse_native_annotations() -> Vec<String> {
    let mut annotations = vec![];
    
    loop {
        match current_char() {
            '@' if peek() != '@' => {
                // Python/TypeScript/Java style
                annotations.push(consume_annotation());
            }
            '#' if peek() == '[' => {
                // Rust style
                annotations.push(consume_rust_attribute());
            }
            '[' if is_csharp_attribute() => {
                // C# style
                annotations.push(consume_csharp_attribute());
            }
            _ => break
        }
    }
    
    annotations
}
```

### Phase 2: AST Attachment
Annotations are attached to AST nodes:

```rust
struct System {
    native_annotations: Vec<String>,
    frame_annotations: Vec<FrameAnnotation>,
    // ...
}

struct DomainField {
    native_annotations: Vec<String>,
    name: String,
    field_type: Type,
    // ...
}
```

### Phase 3: Code Generation
Annotations are emitted in the correct position:

```rust
fn generate_system(system: &System) -> String {
    let mut code = String::new();
    
    // Emit native annotations
    for annotation in &system.native_annotations {
        code.push_str(annotation);
        code.push_str("\n");
    }
    
    // Generate class/struct
    code.push_str(&format!("class {} {{\n", system.name));
    // ...
}
```

## Annotation Compatibility

### Philosophy: Let Native Compilers Validate

Frame takes a **"preserve and pass-through"** approach to native annotations:
1. Frame preserves annotations without interpretation
2. Frame positions them in generated code
3. Native compiler validates compatibility
4. If incompatible, you get clear native error messages

### Annotations That Work Well

These annotations are commonly used with Frame systems:

| Language | Good Annotations | Purpose |
|----------|-----------------|---------|
| **Python** | `pickle`-compatible classes | No decoration needed |
| **TypeScript** | Plain classes | JSON works out-of-box |
| **Rust** | `#[derive(Serialize, Deserialize)]` | Standard serde support |
| **Java** | `@JsonIgnore` on fields | Control serialization |
| **C#** | `[Serializable]`, `[JsonIgnore]` | Mark class/control fields |
| **Go** | `` `json:"name"` `` struct tags | Field-level control |

### Potentially Problematic Annotations

These may conflict with Frame's code generation:

| Annotation | Language | Issue | Solution |
|------------|----------|-------|----------|
| `@dataclass` | Python | Generates conflicting `__init__` | Don't use on systems |
| `@attrs` | Python | Generates conflicting `__init__` | Don't use on systems |
| `@staticmethod` | Python | No `self` parameter | Frame needs instance methods |
| `@abstractmethod` | Python | Requires subclass implementation | Frame generates concrete code |
| `sealed` | TypeScript | Prevents extension | May conflict with Frame patterns |

**Important**: You don't need to memorize this list. If an annotation doesn't work, your native compiler will tell you exactly why.

### Best Practice Examples

#### ✅ DO: Simple, Compatible Approaches
```python
# Python - Just use pickle or json
@@persist
system Simple {
    actions:
        save(self):
            return pickle.dumps(self)
}
```

#### ✅ DO: Field-Level Control
```rust
// Rust - Control specific fields
#[derive(Serialize, Deserialize)]
@@persist  
system WithControl {
    domain:
        data: String = String::new()
        
        #[serde(skip)]  // Fine - field-level
        cache: Vec<u8> = vec![]
}
```

#### ❌ DON'T: Class-Level Constructor Conflicts
```python
# Python - Avoid @dataclass on systems
@dataclass  # BAD - conflicts with Frame's __init__
@@persist
system Bad { }
```

#### ✅ DO: Use Composition for Complex Cases
```python
# Python - Use composition for dataclasses
@@persist
system Good {
    operations:
        get_domain(self):
            @dataclass  # OK - separate class
            class DomainData:
                field1: str
                field2: int
            return DomainData(self.field1, self.field2)
}
```

### Recommendations
1. **Start simple** - Try without annotations first
2. **Add incrementally** - Add annotations as needed
3. **Trust compiler errors** - They tell you what's wrong
4. **Use composition** for complex serialization needs
5. **Test early** - Verify persistence works with your approach

## Runtime Library Support

Frame provides runtime libraries for each target language:

### Python: `frame_persistence_py`
```python
*from frame_persistence_py import snapshot_system, restore_system*

*# Low-level API*
*snapshot = snapshot_system(system)*
*json_text = json.dumps(snapshot)*
*snapshot_data = json.loads(json_text)*
*restored = restore_system(snapshot_data, MySystem)*
```

### TypeScript: `frame_persistence_ts`
```typescript
*import { snapshotSystem, restoreSystem } from 'frame_persistence_ts'*

*// Low-level API*
*const snapshot = snapshotSystem(system)*
*const jsonText = JSON.stringify(snapshot)*
*const snapshotData = JSON.parse(jsonText)*
*const restored = restoreSystem(snapshotData, MySystem)*
```

### Rust: `frame_persistence_rs`
```rust
*use frame_persistence_rs::{SnapshotableSystem, SystemSnapshot};*

*// Trait implementation*
*let snapshot = system.snapshot_system();*
*let json = serde_json::to_string(&snapshot)?;*
*let snapshot_data: SystemSnapshot = serde_json::from_str(&json)?;*
*let restored = MySystem::restore_system(snapshot_data);*
```

## Design Decision: Native Annotations

### Why Native Annotations?

Frame v4's decision to support native annotations for persistence is based on several key factors:

#### 1. **Alignment with v4 Philosophy**
Just as Frame v4 uses:
- Native imports instead of Frame-specific imports
- Native types instead of Frame-specific types
- Native code in blocks instead of Frame syntax

It naturally follows to use:
- **Native persistence annotations** instead of Frame-specific persistence syntax

#### 2. **Ecosystem Leverage**
Each language has mature persistence ecosystems:
- **Python**: 20+ years of serialization libraries (pickle, json, SQLAlchemy, etc.)
- **Java**: Decades of enterprise persistence (JPA, Hibernate, Jackson)
- **C#**: Rich .NET ecosystem (Entity Framework, DataContract, Json.NET)
- **Rust**: Modern serialization (serde is the de facto standard)

Creating Frame-specific persistence would ignore this wealth of tooling.

#### 3. **Parser Simplicity**
The parsing complexity is manageable because:
- Only **2-3 annotation patterns** to recognize (`@`, `#[]`, `[]`)
- **Limited placement locations** (before system, methods, fields)
- **No semantic interpretation** needed (just preserve and re-emit)
- **Native compiler validates** annotation semantics

#### 4. **Developer Familiarity**
Developers can use their existing knowledge:
- Python developers already know `@dataclass`, `@property`
- Java developers already know `@Entity`, `@Column`
- Rust developers already know `#[derive(Serialize)]`

No need to learn Frame-specific persistence API.

#### 5. **Integration Benefits**
Frame systems can integrate seamlessly with existing codebases:
- Inherit from ORM base classes
- Use company-standard serialization
- Work with existing database schemas
- Integrate with message queues and caches

### Implementation Complexity Analysis

| Aspect | Complexity | Solution |
|--------|------------|----------|
| **Parser changes** | Low | Add pattern recognition for 2-3 annotation styles |
| **AST changes** | Low | Add `native_annotations` field to nodes |
| **Code generation** | Low | Re-emit annotations before generated code |
| **Language variance** | Medium | Different annotation syntax per language |
| **Semantic conflicts** | Low | Document incompatible annotations |
| **Testing burden** | Medium | Test annotation preservation per language |

### Alternative Considered: Frame-Only Persistence

We considered a Frame-specific persistence syntax:
```
@@persist(include=[field1, field2], exclude=[field3])
system MySystem {
    domain:
        field1: int @persist
        field2: str @persist  
        field3: tmp @transient
}
```

**Rejected because:**
1. Creates a parallel persistence universe
2. Requires Frame to understand serialization deeply
3. Can't leverage existing tools
4. Forces adapter layers for integration
5. Less expressive than native solutions

### Decision Summary

**Native annotations are the right choice** because:
1. **Manageable complexity** - Only 2-3 patterns to parse
2. **High value** - Full ecosystem access
3. **Philosophical alignment** - Consistent with Frame v4's native-first approach
4. **Developer friendly** - Use existing knowledge
5. **Future proof** - New persistence libraries just work

The parser complexity is a small price for the enormous benefit of native ecosystem integration.

## Summary

Frame v4 persistence provides:
- **Language-neutral** JSON snapshot format
- **Opt-in** via `@@persist` annotation
- **Generated methods** for save/restore
- **Complete state** preservation including stack
- **Selective** domain variable persistence
- **Custom** serialization support
- **Cross-platform** compatibility
- **Version migration** capabilities

This enables Frame systems to be checkpointed, transferred between processes, stored in databases, and restored after restarts while maintaining their exact state machine position and data.