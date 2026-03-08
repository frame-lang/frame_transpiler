# Frame TypeScript Grammar Specification

**Document Version**: 1.1  
**Date**: 2025-10-30  
**Status**: Enhanced Native Support Grammar  
**Target**: TypeScript with Node.js runtime

This document defines the TypeScript-specific grammar extensions for Frame programs using the `@target typescript` declaration, with first-class support for native TypeScript constructs like anonymous functions, template literals, destructuring, and modern syntax.

## Overview

Frame TypeScript programs combine universal Frame state machine syntax with first-class native TypeScript expressions, imports, and type annotations. The grammar extends Frame's core semantics with full TypeScript language support including:

- **Anonymous Functions**: Arrow functions, lambdas, and function expressions as first-class values
- **Template Literals**: String interpolation with embedded expressions
- **Destructuring**: Object and array destructuring in parameters and assignments
- **Modern Operators**: Optional chaining, nullish coalescing, spread/rest operators
- **Async Expressions**: Await expressions and async immediately invoked functions
- **Type System**: Full TypeScript type annotations, guards, and assertions

## Target Declaration

```bnf
typescript_file ::= "@target" "typescript" import_block* frame_module

frame_module ::= (enum_decl | class_decl | var_decl | function | async_function | system)*
```

## Import System

### TypeScript Import Syntax
```bnf
typescript_import ::= es6_import | commonjs_import | namespace_import | dynamic_import
es6_import ::= "import" import_clause "from" string_literal
import_clause ::= default_import | namespace_import | named_imports | mixed_import

default_import ::= identifier
namespace_import ::= "*" "as" identifier  
named_imports ::= "{" import_list "}"
mixed_import ::= identifier "," ("{" import_list "}" | namespace_import)
import_list ::= import_item ("," import_item)*
import_item ::= identifier ("as" identifier)?

commonjs_import ::= "const" identifier "=" "require" "(" string_literal ")"
dynamic_import ::= "await" "import" "(" string_literal ")"
```

### Import Examples
```typescript
// ES6 imports (preferred)
import * as fs from 'fs'
import { readFile, writeFile } from 'fs/promises'
import net from 'net'
import express, { Request, Response } from 'express'

// CommonJS imports (legacy support)
const path = require('path')
const { EventEmitter } = require('events')

// Dynamic imports (async)
const module = await import('./dynamic-module.js')
```

## Type System

### Type Annotations
```bnf
typescript_type ::= primitive_type | union_type | object_type | array_type 
                  | generic_type | function_type | literal_type

primitive_type ::= "string" | "number" | "boolean" | "void" | "any" | "unknown" | "never"
union_type ::= typescript_type ("|" typescript_type)+
object_type ::= "{" property_list? "}"
array_type ::= typescript_type "[" "]" | "Array" "<" typescript_type ">"
generic_type ::= identifier "<" type_args ">"
function_type ::= "(" param_types? ")" "=>" typescript_type
literal_type ::= string_literal | number_literal | boolean_literal

property_list ::= property ("," property)*
property ::= identifier "?"? ":" typescript_type
type_args ::= typescript_type ("," typescript_type)*
param_types ::= typescript_type ("," typescript_type)*
```

### Type Examples
```typescript
// Primitive types
var name: string = "Frame"
var count: number = 42
var active: boolean = true

// Union types  
var result: string | number = getValue()
var status: "pending" | "complete" | "error" = "pending"

// Object types
var config: { host: string, port: number } = { host: "localhost", port: 3000 }

// Array types
var items: string[] = ["a", "b", "c"]
var numbers: Array<number> = [1, 2, 3]

// Generic types
var promise: Promise<string> = fetch("/api/data")
var map: Map<string, number> = new Map()

// Function types
var handler: (data: string) => void = (d) => console.log(d)
```

## Function Definitions

### Function Syntax (Enhanced for Native Support)
```bnf
typescript_function ::= async_function | sync_function | arrow_function | function_expression | generator_function
sync_function ::= "function" identifier "(" param_list? ")" type_annotation? "{" statements "}"
async_function ::= "async" "function" identifier "(" param_list? ")" type_annotation? "{" statements "}"
arrow_function ::= "(" param_list? ")" type_annotation? "=>" (expression | "{" statements "}")
function_expression ::= "function" "(" param_list? ")" type_annotation? "{" statements "}"
generator_function ::= "function" "*" identifier "(" param_list? ")" type_annotation? "{" statements "}"
async_generator ::= "async" "function" "*" identifier "(" param_list? ")" type_annotation? "{" statements "}"

param_list ::= param ("," param)* | rest_param
param ::= destructured_param | identifier "?"? type_annotation? ("=" expression)?
rest_param ::= "..." identifier type_annotation?
destructured_param ::= object_destructure_param | array_destructure_param
type_annotation ::= ":" typescript_type
```

### Function Examples (Enhanced Native Support)
```typescript
// Regular function
function processData(input: string): Promise<string> {
    return Promise.resolve(input.toUpperCase())
}

// Async function
async function fetchData(url: string): Promise<any> {
    const response = await fetch(url)
    return response.json()
}

// Arrow functions (first-class support in Frame domain/actions)
var transform = (data: string[]): string => data.join(", ")
var asyncProcessor = async (item: any) => await processItem(item)
var validator = (input: unknown): input is string => typeof input === 'string'

// Function expressions
var callback = function(error: Error | null, result: any) {
    if (error) handleError(error)
    else handleResult(result)
}

// Generator functions (first-class Frame support)
function* generateIds(): Generator<number> {
    var id = 1
    while (true) yield id++
}

// Async generators
async function* processStream(): AsyncGenerator<ProcessedItem> {
    for (var item of stream) {
        yield await processItem(item)
    }
}

// Destructured parameters (first-class Frame support)
function connect({ host, port = 3000 }: { host: string, port?: number }): void {
    // Connection logic
}

// Rest parameters (first-class Frame support)
function processItems(first: string, ...rest: string[]): string[] {
    return [first.toUpperCase(), ...rest.map(r => r.toLowerCase())]
}
```

## Control Flow

### Conditional Statements
```bnf
typescript_if ::= "if" "(" expression ")" statement ("else" "if" "(" expression ")" statement)* ("else" statement)?
statement ::= "{" statements "}" | expression_statement
```

### Loop Statements
```bnf
typescript_for ::= for_in_loop | for_of_loop | c_style_for
for_in_loop ::= "for" "(" "const" identifier "in" expression ")" statement
for_of_loop ::= "for" "(" "const" identifier "of" expression ")" statement  
c_style_for ::= "for" "(" init_expr? ";" condition_expr? ";" update_expr? ")" statement

typescript_while ::= "while" "(" expression ")" statement
```

### Control Flow Examples
```typescript
// If statements
if (condition) {
    doSomething()
} else if (otherCondition) {
    doOther()
} else {
    doDefault()
}

// For loops
for (const item of items) {
    process(item)
}

for (const key in object) {
    console.log(key, object[key])
}

for (let i = 0; i < array.length; i++) {
    array[i] = transform(array[i])
}

// While loop
while (isRunning) {
    tick()
}
```

## Expressions

### Object and Array Operations (Enhanced Native Support)
```bnf
object_literal ::= "{" property_assignments? "}"
property_assignments ::= property_assignment ("," property_assignment)*
property_assignment ::= property_key ":" expression | "[" expression "]" ":" expression | method_definition | spread_element
property_key ::= identifier | string_literal | computed_property
method_definition ::= ("async" | "*")? identifier "(" param_list? ")" type_annotation? "{" statements "}"
spread_element ::= "..." expression

array_literal ::= "[" array_elements? "]"
array_elements ::= array_element ("," array_element)*
array_element ::= expression | spread_element

member_access ::= expression "." identifier | expression "[" expression "]"
optional_chaining ::= expression "?." identifier | expression "?." "[" expression "]" | expression "?." "(" arguments? ")"
nullish_coalescing ::= expression "??" expression
```

### Async/Promise Operations (Enhanced Native Support)
```bnf
await_expression ::= "await" expression
promise_chain ::= expression ("." "then" "(" callback ")")* ("." "catch" "(" callback "))?
callback ::= arrow_function | function_reference
async_iife ::= "await" "(" "async" "(" param_list? ")" "=>" "{" statements "}" ")" "(" arguments? ")"
```

### Expression Examples (Enhanced Native Support)
```typescript
// Object literals with methods and spread (first-class Frame support)
var config = {
    host: "localhost",
    port: 3000,
    ssl: false,
    [computedKey]: "dynamic",
    async connect() { return await this.establishConnection() },
    *generateRetries() { for (var i = 0; i < 3; i++) yield i },
    ...defaultSettings
}

// Array literals with spread (first-class Frame support)
var numbers = [1, 2, 3, 4, 5]
var combined = [...array1, ...array2, newItem]
var mixed = ["string", 42, true, null]

// Member access and optional chaining (first-class Frame support)
var host = config.host
var port = config?.["port"]
var value = obj?.property?.method?.()
var safeResult = response?.data?.users?.[0]?.name

// Nullish coalescing (first-class Frame support)
var timeout = config?.timeout ?? 5000
var name = user.name ?? "Anonymous"

// Async operations and expressions (first-class Frame support)
var result = await apiCall()
var processed = (await fetchData()).transform()
var values = await Promise.all([op1(), op2(), op3()])

// Async immediately invoked functions (first-class Frame support)
var data = await (async () => {
    var response = await fetch('/api')
    return response.json()
})()

// Promise chains
var data = fetch("/api")
    .then(response => response.json())
    .catch(error => handleError(error))
```

## Advanced TypeScript Features

### Generics
```bnf
generic_function ::= "function" identifier "<" type_params ">" "(" param_list? ")" type_annotation? "{" statements "}"
type_params ::= type_param ("," type_param)*
type_param ::= identifier ("extends" typescript_type)?
```

### Interfaces and Types
```bnf
interface_decl ::= "interface" identifier "{" interface_body "}"
type_alias ::= "type" identifier "=" typescript_type
interface_body ::= interface_member*
interface_member ::= property_signature | method_signature
property_signature ::= identifier "?"? ":" typescript_type
method_signature ::= identifier "(" param_list? ")" ":" typescript_type
```

### Destructuring and Advanced Assignments (Enhanced Native Support)
```bnf
destructuring_assignment ::= object_destructuring | array_destructuring
object_destructuring ::= "{" destructure_props "}" (":" typescript_type)? "=" expression
array_destructuring ::= "[" destructure_items "]" (":" typescript_type)? "=" expression

destructure_props ::= destructure_prop ("," destructure_prop)*
destructure_prop ::= identifier (":" identifier)? ("=" expression)? | "..." identifier

destructure_items ::= destructure_item ("," destructure_item)*
destructure_item ::= identifier ("=" expression)? | "..." identifier

object_destructure_param ::= "{" param_destructure_props "}"
array_destructure_param ::= "[" param_destructure_items "]"
param_destructure_props ::= param_destructure_prop ("," param_destructure_prop)*
param_destructure_prop ::= identifier (":" identifier)? type_annotation? ("=" expression)?
param_destructure_items ::= identifier type_annotation? ("=" expression)?
```

### Advanced Examples (Enhanced Native Support)
```typescript
// Generic function
function identity<T>(arg: T): T {
    return arg
}

// Interface
interface User {
    id: number
    name: string
    email?: string
}

// Type alias
type Status = "pending" | "complete" | "error"

// Destructuring with defaults and rest (first-class Frame support)
var { host, port = 3000, ssl = false } = config
var { data: { users, total } } = response
var [first, second, ...rest] = array
var [error, result] = await handleAsync()

// Type guards (first-class Frame support)
function isString(value: any): value is string {
    return typeof value === 'string'
}

// Type assertions (first-class Frame support)
var element = document.getElementById('myId') as HTMLInputElement
var data = response as ApiResponse<User>
var value = maybeNull!.property  // Non-null assertion
```

## Template Literals

### Template Literal Syntax
```bnf
template_literal ::= "`" template_elements "`"
template_elements ::= (template_chars | template_substitution)*
template_chars ::= [^`$]+
template_substitution ::= "${" expression "}"
tagged_template ::= identifier template_literal
```

### Template Literal Examples
```typescript
// Basic template literals
var message = `Hello ${user.name}, you have ${count} messages`
var query = `
    SELECT * FROM users 
    WHERE id = ${userId} 
    AND status = '${status}'
`

// Multi-line templates
var html = `
    <div class="${className}">
        <h1>${title}</h1>
        <p>${description}</p>
    </div>
`

// Tagged templates
var styled = css`
    color: ${theme.primary};
    font-size: ${fontSize}px;
    margin: ${margin || '0px'};
`

var query = sql`
    SELECT * FROM ${table} 
    WHERE ${field} = ${value}
`
```

## Enhanced Native TypeScript Support

### Frame Domain Variables with Native TypeScript
```typescript
@target typescript

system DataProcessor {
    domain:
        // Arrow function variables (first-class)
        var transform = (data: string) => data.toUpperCase()
        var validator = async (input: any) => await validateInput(input)
        var filter = (items: any[]) => items.filter(item => item.active)
        
        // Template literal variables
        var connectionString = `mongodb://${host}:${port}/${database}`
        var logMessage = `[${new Date().toISOString()}] Processing ${taskName}`
        
        // Destructured assignments
        var { host, port, database } = await getConfig()
        var [primary, ...fallbacks] = servers
        
        // Optional chaining with defaults
        var timeout = config?.network?.timeout ?? 5000
        var retries = settings?.resilience?.maxRetries ?? 3
        
        // Spread operations
        var defaultConfig = { host: 'localhost', port: 3000 }
        var userConfig = { ...defaultConfig, ...userSettings }
}
```

### Enhanced Event Handlers with Native Syntax
```typescript
@target typescript

system ApiHandler {
    machine:
        $Processing {
            // Destructured event parameters (first-class)
            handleRequest({ method, url, headers }: RequestEvent) {
                var { authorization } = headers
                var token = authorization?.split(' ')[1] ?? null
                
                // Template literals in handlers
                var logEntry = `Processing ${method} ${url} from ${headers?.['user-agent'] ?? 'unknown'}`
                console.log(logEntry)
                
                // Async immediately invoked function expressions
                var result = await (async () => {
                    if (!token) return this.unauthorized()
                    var user = await this.validateToken(token)
                    return this.processRequest(user, { method, url })
                })()
                
                -> $Complete(result)
            }
            
            // Rest parameters (first-class)
            handleBatch(primary: Request, ...additional: Request[]) {
                var allRequests = [primary, ...additional]
                var processed = allRequests.map(async req => await this.processOne(req))
                var results = await Promise.all(processed)
                -> $BatchComplete(results)
            }
            
            // Arrow functions in event handlers
            handleStreamData(stream: AsyncIterable<DataChunk>) {
                var processor = async (chunk: DataChunk) => {
                    var transformed = await this.transform(chunk)
                    return { ...chunk, processed: transformed }
                }
                
                var results = []
                for await (var chunk of stream) {
                    results.push(await processor(chunk))
                }
                
                -> $StreamComplete(results)
            }
        }
}
```

### Enhanced Actions with Generators and Advanced Features
```typescript
@target typescript

system WebServer {
    actions:
        // Template literals in actions
        processMessage(message: string): void {
            var timestamp = new Date().toISOString()
            console.log(`[${timestamp}] Received: ${message}`)
        }
        
        // Generator methods (first-class)
        *generateResponses(requests: Request[]): Generator<Response> {
            for (var req of requests) {
                yield this.processRequest(req)
            }
        }
        
        // Async generator methods
        async *processStreamAsync(items: AsyncIterable<Item>): AsyncGenerator<ProcessedItem> {
            for await (var item of items) {
                var processed = await this.processItem(item)
                yield { ...item, result: processed }
            }
        }
        
        // Destructuring with type guards
        processUser(input: unknown): User {
            if (!this.isUserData(input)) {
                throw new Error(`Invalid user data: ${JSON.stringify(input)}`)
            }
            
            var { id, name, email = null } = input
            return { id, name, email }
        }
        
        // Arrow function assignments
        createValidator(): (data: any) => boolean {
            var rules = this.getValidationRules()
            return (data: any) => rules.every(rule => rule.test(data))
        }
}
```

## Frame Integration

### Frame System with TypeScript Actions
```typescript
@target typescript

import * as net from 'net'
import { promisify } from 'util'

system SocketServer {
    interface:
        start(port: number): Promise<void>
        stop(): Promise<void>
        
    machine:
        $Idle {
            async start(port: number) {
                this.server = net.createServer()
                const listen = promisify(this.server.listen.bind(this.server))
                await listen(port)
                -> $Running(port)
            }
        }
        
        $Running(port: number) {
            async stop() {
                if (this.server) {
                    const close = promisify(this.server.close.bind(this.server))
                    await close()
                }
                -> $Idle
            }
            
            handleConnection(socket: net.Socket) {
                socket.on('data', (data: Buffer) => {
                    const message = data.toString('utf8')
                    this.processMessage(message)
                })
            }
        }
    
    actions:
        processMessage(message: string): void {
            console.log(`Received: ${message}`)
        }
    
    domain:
        var server: net.Server | null = null
}
```

### Frame Functions with TypeScript
```typescript
@target typescript

interface APIResponse<T> {
    data: T
    status: number
    message: string
}

async fn fetchUser(id: number): Promise<APIResponse<User>> {
    const response = await fetch(`/api/users/${id}`)
    const data = await response.json()
    
    return {
        data: data as User,
        status: response.status,
        message: response.statusText
    }
}

fn validateEmail(email: string): boolean {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
    return emailRegex.test(email)
}
```

## Grammar Summary

### Core Frame (Universal)
- System architecture: `system`, `interface:`, `machine:`, `actions:`, `domain:`
- State definitions: `$State` with event handlers
- Transitions: `-> $Target` and parent dispatch `=> $^`
- Domain access: `self.variable`

### TypeScript-Specific Extensions (First-Class Native Support)
- **Import system**: ES6/CommonJS imports with type support
- **Type annotations**: Comprehensive TypeScript type system
- **Anonymous functions**: Arrow functions, lambdas, function expressions as Frame values
- **Template literals**: String interpolation with embedded expressions in Frame context
- **Destructuring**: Object/array destructuring in parameters, assignments, and domain variables
- **Modern operators**: Optional chaining (?.), nullish coalescing (??), spread (...) operators
- **Async expressions**: Await expressions, async IIFEs, Promise operations as Frame constructs
- **Generators**: Generator functions and async generators as Frame methods
- **Type system**: Type guards, assertions, generics fully integrated with Frame semantics
- **Advanced features**: Rest parameters, computed properties, method definitions in Frame objects

### Compilation Strategy
1. **Parse Frame structure** with universal Frame parser
2. **Parse TypeScript content** in action bodies and imports
3. **Generate TypeScript code** with proper type annotations
4. **Include runtime helpers** for Frame semantics

This grammar enables Frame programs to leverage TypeScript's type system and ecosystem while maintaining Frame's state machine semantics and structure.