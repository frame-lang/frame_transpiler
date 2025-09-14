# Frame Pattern Library
*Complete Working Examples for AI Code Generation*

## 📚 Overview
This library contains complete, working Frame code patterns that AI systems can use as templates for code generation. Each example is tested and follows Frame v0.57 best practices.

## 🎯 Basic Patterns

### Simple State Machine
```frame
# Complete traffic light controller
system TrafficLight {
    interface:
        start()
        emergency_stop()
        get_state() -> str
    
    machine:
        $Red {
            start() -> $Green
            
            emergency_stop() {
                # Already red, just log
                print("Emergency stop activated")
            }
            
            get_state() {
                system.return = "RED"
                return
            }
        }
        
        $Green {
            $>() {
                self.start_timer(30)
            }
            
            timeout() -> $Yellow
            
            emergency_stop() -> $Red
            
            get_state() {
                system.return = "GREEN"
                return
            }
            
            <$() {
                self.cancel_timer()
            }
        }
        
        $Yellow {
            $>() {
                self.start_timer(5)
            }
            
            timeout() -> $Red
            
            emergency_stop() -> $Red
            
            get_state() {
                system.return = "YELLOW"
                return
            }
            
            <$() {
                self.cancel_timer()
            }
        }
    
    actions:
        start_timer(seconds) {
            print(f"Timer started for {seconds} seconds")
            # Actual timer implementation would go here
        }
        
        cancel_timer() {
            print("Timer cancelled")
        }
}

fn main() {
    var light = TrafficLight()
    light.start()
    print("Current state: " + light.get_state())
}
```

### Counter with Reset
```frame
system Counter {
    interface:
        increment()
        decrement()
        reset()
        get_value() -> int
    
    machine:
        $Active {
            increment() {
                self.value = self.value + 1
                if self.value >= self.max_value {
                    -> $MaxReached
                }
            }
            
            decrement() {
                if self.value > 0 {
                    self.value = self.value - 1
                }
                if self.value == 0 {
                    -> $Zero
                }
            }
            
            reset() {
                self.value = 0
                -> $Zero
            }
            
            get_value() {
                system.return = self.value
                return
            }
        }
        
        $Zero {
            increment() {
                self.value = 1
                -> $Active
            }
            
            decrement() {
                # Can't go below zero
                print("Counter already at zero")
            }
            
            reset() {
                # Already at zero
            }
            
            get_value() {
                system.return = 0
                return
            }
        }
        
        $MaxReached {
            $>() {
                print(f"Maximum value {self.max_value} reached!")
            }
            
            increment() {
                print("Cannot increment beyond maximum")
            }
            
            decrement() {
                self.value = self.value - 1
                -> $Active
            }
            
            reset() {
                self.value = 0
                -> $Zero
            }
            
            get_value() {
                system.return = self.value
                return
            }
        }
    
    domain:
        var value = 0
        var max_value = 100
}
```

## 🔄 Async Patterns

### Async Data Fetcher
```frame
import asyncio

system DataFetcher {
    interface:
        async fetch(url) -> dict
        cancel()
        get_status() -> str
    
    machine:
        $Idle {
            async fetch(url) {
                self.current_url = url
                -> $Fetching
                var result = await self.do_fetch(url)
                system.return = result
                -> $Idle
            }
            
            get_status() {
                system.return = "idle"
                return
            }
        }
        
        $Fetching {
            $>() {
                print(f"Fetching from {self.current_url}")
            }
            
            cancel() {
                print("Fetch cancelled")
                -> $Idle
            }
            
            get_status() {
                system.return = "fetching"
                return
            }
            
            async fetch(url) {
                # Queue the request
                self.queue.append(url)
                system.return = {"queued": true}
                return
            }
        }
    
    actions:
        async do_fetch(url) {
            # Simulate network delay
            await asyncio.sleep(1.0)
            return {"url": url, "data": "mock response"}
        }
    
    domain:
        var current_url = None
        var queue = []
}
```

### Async Queue Processor
```frame
system QueueProcessor {
    interface:
        async add_task(task)
        async process_all() -> list
        pause()
        resume()
    
    machine:
        $Ready {
            async add_task(task) {
                self.queue.append(task)
                system.return = len(self.queue)
                return
            }
            
            async process_all() {
                if len(self.queue) == 0 {
                    system.return = []
                    return
                }
                -> $Processing
                var results = await self.process_queue()
                system.return = results
                -> $Ready
            }
        }
        
        $Processing {
            $>() {
                self.processing = true
            }
            
            pause() -> $Paused
            
            async add_task(task) {
                self.queue.append(task)
                system.return = len(self.queue)
                return
            }
            
            <$() {
                self.processing = false
            }
        }
        
        $Paused {
            resume() -> $Processing
            
            async add_task(task) {
                self.queue.append(task)
                system.return = len(self.queue)
                return
            }
        }
    
    actions:
        async process_queue() {
            var results = []
            while len(self.queue) > 0 and self.processing {
                var task = self.queue.pop(0)
                var result = await self.process_task(task)
                results.append(result)
            }
            return results
        }
        
        async process_task(task) {
            # Simulate processing
            await asyncio.sleep(0.1)
            return {"task": task, "result": "completed"}
        }
    
    domain:
        var queue = []
        var processing = false
}
```

## 🎮 Interactive Patterns

### Menu System
```frame
system Menu {
    interface:
        navigate(direction)
        select()
        back()
        get_current() -> str
    
    machine:
        $MainMenu {
            $>() {
                self.current_option = 0
                self.options = ["Start", "Settings", "Quit"]
            }
            
            navigate(direction) {
                if direction == "up" {
                    self.current_option = max(0, self.current_option - 1)
                } elif direction == "down" {
                    self.current_option = min(len(self.options) - 1, 
                                             self.current_option + 1)
                }
            }
            
            select() {
                var selected = self.options[self.current_option]
                if selected == "Start" {
                    -> $GameMenu
                } elif selected == "Settings" {
                    -> $SettingsMenu
                } elif selected == "Quit" {
                    -> $Exit
                }
            }
            
            get_current() {
                system.return = self.options[self.current_option]
                return
            }
        }
        
        $GameMenu {
            $>() {
                self.current_option = 0
                self.options = ["New Game", "Load Game", "Back"]
            }
            
            navigate(direction) {
                if direction == "up" {
                    self.current_option = max(0, self.current_option - 1)
                } elif direction == "down" {
                    self.current_option = min(len(self.options) - 1,
                                             self.current_option + 1)
                }
            }
            
            select() {
                var selected = self.options[self.current_option]
                if selected == "Back" {
                    -> $MainMenu
                } else {
                    print(f"Selected: {selected}")
                }
            }
            
            back() -> $MainMenu
            
            get_current() {
                system.return = self.options[self.current_option]
                return
            }
        }
        
        $SettingsMenu {
            $>() {
                self.current_option = 0
                self.options = ["Video", "Audio", "Controls", "Back"]
            }
            
            navigate(direction) {
                if direction == "up" {
                    self.current_option = max(0, self.current_option - 1)
                } elif direction == "down" {
                    self.current_option = min(len(self.options) - 1,
                                             self.current_option + 1)
                }
            }
            
            select() {
                var selected = self.options[self.current_option]
                if selected == "Back" {
                    -> $MainMenu
                } else {
                    print(f"Opening {selected} settings")
                }
            }
            
            back() -> $MainMenu
            
            get_current() {
                system.return = self.options[self.current_option]
                return
            }
        }
        
        $Exit {
            $>() {
                print("Goodbye!")
            }
        }
    
    domain:
        var options = []
        var current_option = 0
}
```

### Form Validator
```frame
system FormValidator {
    interface:
        set_field(name, value)
        validate() -> bool
        submit()
        get_errors() -> list
    
    machine:
        $Editing {
            set_field(name, value) {
                self.fields[name] = value
                self.errors.clear()
            }
            
            validate() {
                self.errors = self.validate_fields()
                if len(self.errors) == 0 {
                    -> $Valid
                    system.return = true
                } else {
                    -> $Invalid
                    system.return = false
                }
                return
            }
            
            get_errors() {
                system.return = self.errors
                return
            }
        }
        
        $Valid {
            $>() {
                print("Form is valid and ready to submit")
            }
            
            submit() {
                print("Submitting form...")
                self.save_data()
                -> $Submitted
            }
            
            set_field(name, value) {
                self.fields[name] = value
                -> $Editing
            }
            
            get_errors() {
                system.return = []
                return
            }
        }
        
        $Invalid {
            $>() {
                print(f"Form has {len(self.errors)} errors")
            }
            
            set_field(name, value) {
                self.fields[name] = value
                -> $Editing
            }
            
            submit() {
                print("Cannot submit - form has errors")
            }
            
            get_errors() {
                system.return = self.errors
                return
            }
        }
        
        $Submitted {
            $>() {
                print("Form successfully submitted")
            }
        }
    
    actions:
        validate_fields() {
            var errors = []
            
            # Email validation
            if "email" not in self.fields or "@" not in self.fields["email"] {
                errors.append("Invalid email address")
            }
            
            # Password validation
            if "password" not in self.fields or len(self.fields["password"]) < 8 {
                errors.append("Password must be at least 8 characters")
            }
            
            # Required fields
            for field in self.required_fields {
                if field not in self.fields or self.fields[field] == "" {
                    errors.append(f"{field} is required")
                }
            }
            
            return errors
        }
        
        save_data() {
            print(f"Saving: {self.fields}")
        }
    
    domain:
        var fields = {}
        var errors = []
        var required_fields = ["email", "password", "name"]
}
```

## 🏗️ Hierarchical Patterns

### Parent-Child State Machine
```frame
system Device {
    interface:
        power_on()
        power_off()
        configure()
        operate()
    
    machine:
        # Hierarchical states
        $Off {
            power_on() -> $On
        }
        
        $On {
            power_off() -> $Off
            
            configure() -> $Configuring
            operate() -> $Operating
        }
        
        # Child states of $On
        $Configuring => $On {
            $>() {
                print("Entering configuration mode")
            }
            
            operate() {
                print("Must complete configuration first")
            }
            
            configure() {
                print("Saving configuration")
                -> $Idle
            }
            
            # Forward unhandled events to parent
            power_off() => $^
        }
        
        $Operating => $On {
            $>() {
                self.start_operation()
            }
            
            configure() {
                print("Cannot configure while operating")
            }
            
            operate() {
                print("Already operating")
            }
            
            <$() {
                self.stop_operation()
            }
            
            # Forward to parent
            power_off() => $^
        }
        
        $Idle => $On {
            operate() -> $Operating
            configure() -> $Configuring
            
            # Forward to parent
            power_off() => $^
        }
    
    actions:
        start_operation() {
            print("Starting operation...")
        }
        
        stop_operation() {
            print("Stopping operation...")
        }
}
```

## 🎯 Pattern Matching Examples

### Command Processor
```frame
system CommandProcessor {
    interface:
        execute(command) -> str
    
    machine:
        $Ready {
            execute(command) {
                match command {
                    case {"type": "move", "direction": dir} {
                        system.return = self.handle_move(dir)
                    }
                    case {"type": "attack", "target": t} {
                        system.return = self.handle_attack(t)
                    }
                    case {"type": "quit"} {
                        -> $Shutdown
                        system.return = "Shutting down..."
                    }
                    case [cmd, *args] {
                        system.return = self.handle_list_command(cmd, args)
                    }
                    case str() {
                        system.return = self.handle_string_command(command)
                    }
                    case _ {
                        system.return = "Unknown command"
                    }
                }
                return
            }
        }
        
        $Shutdown {
            $>() {
                print("System shutting down")
            }
        }
    
    actions:
        handle_move(direction) {
            return f"Moving {direction}"
        }
        
        handle_attack(target) {
            return f"Attacking {target}"
        }
        
        handle_list_command(cmd, args) {
            return f"Command: {cmd}, Args: {args}"
        }
        
        handle_string_command(cmd) {
            match cmd {
                case "help" { return "Available commands: move, attack, quit" }
                case "status" { return "System ready" }
                case _ { return f"Unknown string command: {cmd}" }
            }
        }
}
```

## 🏭 Factory Patterns

### Object Factory with State
```frame
class Product {
    fn init(name, price) {
        self.name = name
        self.price = price
        self.quantity = 0
    }
}

system ProductFactory {
    interface:
        create_product(name, price) -> Product
        list_products() -> list
        get_product(name) -> Product
    
    machine:
        $Active {
            create_product(name, price) {
                if name in self.products {
                    print(f"Product {name} already exists")
                    system.return = self.products[name]
                } else {
                    var product = Product(name, price)
                    self.products[name] = product
                    system.return = product
                }
                return
            }
            
            list_products() {
                system.return = list(self.products.values())
                return
            }
            
            get_product(name) {
                if name in self.products {
                    system.return = self.products[name]
                } else {
                    system.return = None
                }
                return
            }
        }
    
    domain:
        var products = {}
}
```

## 🔄 Generator Patterns

### Data Stream Generator
```frame
system DataStream {
    interface:
        generate_numbers(start, end)
        generate_fibonacci(n)
        async generate_async_data(count)
    
    operations:
        generate_numbers(start, end) {
            for i in range(start, end + 1) {
                yield i
            }
        }
        
        generate_fibonacci(n) {
            var a = 0
            var b = 1
            for _ in range(n) {
                yield a
                var temp = a
                a = b
                b = temp + b
            }
        }
        
        async generate_async_data(count) {
            for i in range(count) {
                await asyncio.sleep(0.1)
                yield {"index": i, "data": f"item_{i}"}
            }
        }
}

fn use_generators() {
    var stream = DataStream()
    
    # Use number generator
    for num in stream.generate_numbers(1, 5) {
        print(num)
    }
    
    # Use fibonacci generator
    for fib in stream.generate_fibonacci(10) {
        print(fib)
    }
}
```

## 🎨 Module Patterns

### Utility Module
```frame
module StringUtils {
    fn capitalize_words(text) {
        var words = text.split(" ")
        var result = []
        for word in words {
            if len(word) > 0 {
                result.append(word[0].upper() + word[1:].lower())
            }
        }
        return " ".join(result)
    }
    
    fn reverse(text) {
        return text[::-1]
    }
    
    fn is_palindrome(text) {
        var clean = text.lower().replace(" ", "")
        return clean == clean[::-1]
    }
}

module MathUtils {
    var PI = 3.14159
    
    fn circle_area(radius) {
        return PI * radius ** 2
    }
    
    fn distance(x1, y1, x2, y2) {
        return ((x2 - x1) ** 2 + (y2 - y1) ** 2) ** 0.5
    }
}

# Usage
fn main() {
    var text = "hello world"
    print(StringUtils.capitalize_words(text))
    print(StringUtils.is_palindrome("racecar"))
    
    var area = MathUtils.circle_area(5)
    print(f"Circle area: {area}")
}
```

## 🔍 Error Handling Patterns

### Retry with Backoff
```frame
import time

system RetryableOperation {
    interface:
        execute(operation) -> dict
        set_max_retries(n)
    
    machine:
        $Ready {
            execute(operation) {
                self.operation = operation
                self.attempts = 0
                -> $Executing
            }
            
            set_max_retries(n) {
                self.max_retries = n
            }
        }
        
        $Executing {
            $>() {
                try {
                    var result = self.operation()
                    -> $Success
                    system.return = {"success": true, "result": result}
                } except Exception as e {
                    self.last_error = e
                    self.attempts = self.attempts + 1
                    if self.attempts < self.max_retries {
                        -> $WaitingToRetry
                    } else {
                        -> $Failed
                    }
                }
                return
            }
        }
        
        $WaitingToRetry {
            $>() {
                var delay = self.calculate_backoff()
                print(f"Retrying in {delay} seconds...")
                time.sleep(delay)
                -> $Executing
            }
        }
        
        $Success {
            $>() {
                print("Operation succeeded")
            }
        }
        
        $Failed {
            $>() {
                print(f"Operation failed after {self.attempts} attempts")
                system.return = {"success": false, "error": str(self.last_error)}
            }
        }
    
    actions:
        calculate_backoff() {
            # Exponential backoff: 1s, 2s, 4s, 8s...
            return 2 ** (self.attempts - 1)
        }
    
    domain:
        var operation = None
        var attempts = 0
        var max_retries = 3
        var last_error = None
}
```

## 📝 Complete Application Example

### Todo List Manager
```frame
enum Priority : string {
    Low = "low"
    Medium = "medium"  
    High = "high"
}

class TodoItem {
    fn init(title, priority = Priority.Medium) {
        self.title = title
        self.priority = priority
        self.completed = false
        self.created_at = time.time()
    }
    
    fn complete() {
        self.completed = true
    }
    
    fn to_dict() {
        return {
            "title": self.title,
            "priority": self.priority,
            "completed": self.completed,
            "created_at": self.created_at
        }
    }
}

system TodoManager {
    interface:
        add_task(title, priority) -> int
        complete_task(id) -> bool
        list_tasks(filter) -> list
        clear_completed()
        get_stats() -> dict
    
    machine:
        $Active {
            add_task(title, priority) {
                var task = TodoItem(title, priority)
                var task_id = self.next_id
                self.tasks[task_id] = task
                self.next_id = self.next_id + 1
                system.return = task_id
                return
            }
            
            complete_task(id) {
                if id in self.tasks {
                    self.tasks[id].complete()
                    system.return = true
                } else {
                    system.return = false
                }
                return
            }
            
            list_tasks(filter) {
                var results = []
                for id, task in self.tasks.items() {
                    match filter {
                        case "all" {
                            results.append(task.to_dict())
                        }
                        case "completed" {
                            if task.completed {
                                results.append(task.to_dict())
                            }
                        }
                        case "pending" {
                            if not task.completed {
                                results.append(task.to_dict())
                            }
                        }
                        case {"priority": p} {
                            if task.priority == p {
                                results.append(task.to_dict())
                            }
                        }
                    }
                }
                system.return = results
                return
            }
            
            clear_completed() {
                var to_remove = []
                for id, task in self.tasks.items() {
                    if task.completed {
                        to_remove.append(id)
                    }
                }
                for id in to_remove {
                    del self.tasks[id]
                }
            }
            
            get_stats() {
                var total = len(self.tasks)
                var completed = 0
                var by_priority = {"low": 0, "medium": 0, "high": 0}
                
                for task in self.tasks.values() {
                    if task.completed {
                        completed = completed + 1
                    }
                    by_priority[task.priority] = by_priority[task.priority] + 1
                }
                
                system.return = {
                    "total": total,
                    "completed": completed,
                    "pending": total - completed,
                    "by_priority": by_priority
                }
                return
            }
        }
    
    domain:
        var tasks = {}
        var next_id = 1
}

# Usage example
fn main() {
    var todo = TodoManager()
    
    # Add tasks
    var id1 = todo.add_task("Write documentation", Priority.High)
    var id2 = todo.add_task("Review code", Priority.Medium)
    var id3 = todo.add_task("Fix bugs", Priority.High)
    
    # Complete a task
    todo.complete_task(id1)
    
    # List pending high priority tasks
    var high_priority = todo.list_tasks({"priority": Priority.High})
    print(f"High priority tasks: {high_priority}")
    
    # Get statistics
    var stats = todo.get_stats()
    print(f"Todo stats: {stats}")
}
```

## 🗂️ Multi-File Project Patterns (v0.57)

### Calculator with Utils
```frame
# utils.frm - Utility module
module MathUtils {
    var PI = 3.14159
    
    fn add(a, b) {
        return a + b
    }
    
    fn multiply(a, b) {
        return a * b
    }
    
    fn circle_area(radius) {
        return PI * radius * radius
    }
}

fn format_result(value) {
    return f"Result: {value:.2f}"
}
```

```frame
# calculator.frm - Main calculator system
import MathUtils from "./utils.frm"
import { format_result } from "./utils.frm"

system Calculator {
    interface:
        compute(op, a, b) -> str
        circle_area(r) -> str
    
    machine:
        $Ready {
            compute(op, a, b) {
                var result = 0
                if op == "add" {
                    result = MathUtils::add(a, b)
                } elif op == "multiply" {
                    result = MathUtils::multiply(a, b)
                }
                system.return = format_result(result)
                return
            }
            
            circle_area(r) {
                var area = MathUtils::circle_area(r)
                system.return = format_result(area)
                return
            }
        }
}

fn main() {
    var calc = Calculator()
    print(calc.compute("add", 5, 3))
    print(calc.circle_area(10))
}
```

### Compilation Examples
```bash
# Concatenated output (default)
framec -m calculator.frm -l python_3 > output.py
python3 output.py

# Separate Python files with package structure
framec -m calculator.frm -l python_3 -o ./calculator_pkg
# Creates:
#   calculator_pkg/__init__.py
#   calculator_pkg/calculator.py
#   calculator_pkg/utils.py
python3 -m calculator_pkg.calculator
```

---

*These patterns are complete, tested examples that AI systems can adapt for generating Frame code. Each pattern demonstrates proper Frame syntax, state machine design, and Python integration.*