# Frame TypeScript Language Support

**Document Version**: 1.0  
**Date**: 2025-10-30  
**Status**: Comprehensive Support Plan  
**Target**: TypeScript with Node.js runtime

This document addresses all aspects of TypeScript support in Frame beyond core syntax, including toolchains, libraries, external integrations, runtime requirements, and ecosystem considerations.

## Toolchain Requirements

### Core Tools
| Tool | Version | Purpose | Status |
|------|---------|---------|--------|
| **Node.js** | 18.0+ | JavaScript runtime | Required |
| **TypeScript** | 5.0+ | Type checking & compilation | Required |
| **npm/yarn/pnpm** | Latest | Package management | Required |
| **@types/node** | Latest | Node.js type definitions | Required |

### Development Dependencies
```json
{
  "devDependencies": {
    "@types/node": "^20.0.0",
    "typescript": "^5.0.0",
    "ts-node": "^10.9.0",
    "@typescript-eslint/eslint-plugin": "^6.0.0",
    "@typescript-eslint/parser": "^6.0.0",
    "prettier": "^3.0.0"
  }
}
```

### Compilation Pipeline
```bash
# Frame to TypeScript transpilation
framec -l typescript runtime_protocol.frm > RuntimeProtocol.ts

# TypeScript compilation
npx tsc --target es2020 --module commonjs RuntimeProtocol.ts

# Direct execution (development)
npx ts-node RuntimeProtocol.ts

# Bundle for distribution
npx webpack --config webpack.config.js
```

## Runtime Library Architecture

### Frame TypeScript Runtime Components

#### Core Runtime (`FrameRuntime`)
```typescript
export namespace FrameRuntime {
    // State machine runtime
    export class FrameEvent {
        constructor(public message: string, public parameters: any[]) {}
    }
    
    export class FrameCompartment {
        constructor(
            public state: string,
            public stateArgs: any[],
            public enterArgs: any[],
            public exitArgs: any[]
        ) {}
    }
    
    // Collection operations for Frame semantic consistency
    export function equals(left: any, right: any): boolean
    export function createSet(items?: any[]): Set<any>
    export function createMap(pairs?: [any, any][]): Map<any, any>
    
    // Type conversion utilities
    export function frameType(obj: any): string
    export function isinstance(obj: any, type: string): boolean
}
```

#### Async Operations (`FrameAsync`)
```typescript
export namespace FrameAsync {
    // Socket operations for Bug #055
    export class AsyncSocket {
        constructor(private socket: net.Socket) {}
        
        async readline(): Promise<string>
        async writeUtf8(data: string): Promise<void>
        async close(): Promise<void>
    }
    
    // HTTP operations
    export async function httpGet(url: string): Promise<Response>
    export async function httpPost(url: string, data: any): Promise<Response>
    
    // Concurrency utilities
    export async function parallel<T>(tasks: Promise<T>[]): Promise<T[]>
    export async function race<T>(tasks: Promise<T>[]): Promise<T>
    export async function timeout<T>(promise: Promise<T>, ms: number): Promise<T>
}
```

#### String Operations (`FrameString`)
```typescript
export namespace FrameString {
    // Python-equivalent string methods
    export function startswith(str: string, prefix: string): boolean
    export function endswith(str: string, suffix: string): boolean
    export function strip(str: string, chars?: string): string
    export function split(str: string, separator?: string, maxSplit?: number): string[]
    export function join(separator: string, items: string[]): string
    export function format(template: string, ...args: any[]): string
}
```

#### Collections (`FrameCollections`)
```typescript
export namespace FrameCollections {
    // Dictionary operations
    export function dictFromKeys<T>(keys: any[], defaultValue?: T): Map<any, T>
    export function dictUpdate<K, V>(target: Map<K, V>, source: Map<K, V>): void
    
    // Set operations
    export function setUnion<T>(a: Set<T>, b: Set<T>): Set<T>
    export function setIntersection<T>(a: Set<T>, b: Set<T>): Set<T>
    export function setDifference<T>(a: Set<T>, b: Set<T>): Set<T>
    
    // List operations
    export function listExtend<T>(target: T[], source: T[]): void
    export function listInsert<T>(list: T[], index: number, item: T): void
}
```

### Runtime Embedding Strategy

#### Inline Embedding (Current)
```typescript
// Generated Frame TypeScript includes embedded runtime
// File: RuntimeProtocol.ts (generated)

// === Frame TypeScript Runtime ===
namespace FrameRuntime {
    // ... complete runtime implementation
}

// === Generated Frame Code ===
export class RuntimeProtocol {
    // ... Frame system implementation
}
```

#### External Module (Future)
```typescript
// Package: @frame-lang/typescript-runtime
import { FrameRuntime, FrameAsync, FrameString } from '@frame-lang/typescript-runtime'

export class RuntimeProtocol {
    // ... Frame system implementation using imported runtime
}
```

## Library Ecosystem Integration

### Node.js Standard Library
| Module | Usage | Frame Integration |
|--------|--------|-------------------|
| **fs/promises** | Async file operations | Direct import and usage |
| **net** | Socket programming | FrameAsync.AsyncSocket wrapper |
| **http/https** | HTTP client/server | Direct usage with Frame async |
| **path** | File path manipulation | Direct usage |
| **crypto** | Cryptographic operations | Direct usage |
| **events** | Event emitters | Frame event system integration |
| **stream** | Stream processing | Buffer integration |
| **util** | Utility functions | Direct usage |

### Popular Third-Party Libraries
| Library | Purpose | Integration Notes |
|---------|---------|------------------|
| **express** | Web framework | Frame systems as middleware |
| **axios** | HTTP client | Async action integration |
| **lodash** | Utility functions | Direct usage in actions |
| **moment/dayjs** | Date manipulation | Direct usage |
| **joi/yup** | Validation | Interface method validation |
| **socket.io** | WebSocket communication | Frame event dispatching |
| **typeorm** | Database ORM | Domain variable persistence |
| **jest** | Testing framework | Frame system testing |

### Integration Examples
```typescript
@target typescript

import express, { Request, Response } from 'express'
import axios from 'axios'
import { validate } from 'joi'

system WebServer {
    interface:
        start(port: number): Promise<void>
        addRoute(path: string, handler: Function): void
    
    machine:
        $Idle {
            async start(port: number) {
                this.app = express()
                this.setupMiddleware()
                await new Promise(resolve => {
                    this.server = this.app.listen(port, resolve)
                })
                -> $Running(port)
            }
        }
        
        $Running(port: number) {
            addRoute(path: string, handler: Function) {
                this.app.get(path, handler)
            }
            
            async apiCall(endpoint: string) {
                try {
                    const response = await axios.get(endpoint)
                    this.handleResponse(response.data)
                } catch (error) {
                    this.handleError(error)
                }
            }
        }
    
    actions:
        setupMiddleware(): void {
            this.app.use(express.json())
            this.app.use(express.urlencoded({ extended: true }))
        }
        
        handleResponse(data: any): void {
            console.log('API Response:', data)
        }
        
        handleError(error: any): void {
            console.error('API Error:', error.message)
        }
    
    domain:
        var app: express.Application
        var server: any
}
```

## External System Integration

### Database Integration
```typescript
@target typescript

import { createConnection, Connection, Repository } from 'typeorm'
import { User } from './entities/User'

system DatabaseManager {
    interface:
        connect(): Promise<void>
        saveUser(userData: any): Promise<User>
        findUser(id: number): Promise<User | null>
    
    machine:
        $Disconnected {
            async connect() {
                this.connection = await createConnection()
                this.userRepository = this.connection.getRepository(User)
                -> $Connected
            }
        }
        
        $Connected {
            async saveUser(userData: any): Promise<User> {
                const user = this.userRepository.create(userData)
                return await this.userRepository.save(user)
            }
            
            async findUser(id: number): Promise<User | null> {
                return await this.userRepository.findOne({ where: { id } })
            }
        }
    
    domain:
        var connection: Connection
        var userRepository: Repository<User>
}
```

### Message Queue Integration
```typescript
@target typescript

import amqp, { Connection, Channel } from 'amqplib'

system MessageProcessor {
    interface:
        connect(url: string): Promise<void>
        publishMessage(queue: string, message: any): Promise<void>
        consumeMessages(queue: string): Promise<void>
    
    machine:
        $Idle {
            async connect(url: string) {
                this.connection = await amqp.connect(url)
                this.channel = await this.connection.createChannel()
                -> $Connected
            }
        }
        
        $Connected {
            async publishMessage(queue: string, message: any) {
                await this.channel.assertQueue(queue)
                this.channel.sendToQueue(queue, Buffer.from(JSON.stringify(message)))
            }
            
            async consumeMessages(queue: string) {
                await this.channel.assertQueue(queue)
                this.channel.consume(queue, (msg) => {
                    if (msg) {
                        const data = JSON.parse(msg.content.toString())
                        this.processMessage(data)
                        this.channel.ack(msg)
                    }
                })
                -> $Consuming(queue)
            }
        }
        
        $Consuming(queue: string) {
            // Handle consuming state
        }
    
    actions:
        processMessage(data: any): void {
            console.log('Processing message:', data)
        }
    
    domain:
        var connection: Connection
        var channel: Channel
}
```

## Testing Framework Integration

### Jest Integration
```typescript
// tests/RuntimeProtocol.test.ts
import { RuntimeProtocol } from '../src/RuntimeProtocol'

describe('RuntimeProtocol', () => {
    let protocol: RuntimeProtocol
    
    beforeEach(() => {
        protocol = new RuntimeProtocol()
    })
    
    test('should start in idle state', () => {
        expect(protocol.getState()).toBe('Idle')
    })
    
    test('should handle async operations', async () => {
        await protocol.connect('localhost', 3000)
        expect(protocol.getState()).toBe('Connected')
    })
    
    test('should process messages correctly', async () => {
        const mockData = { type: 'test', payload: 'hello' }
        const result = await protocol.processMessage(mockData)
        expect(result).toBeDefined()
    })
})
```

### Test Configuration
```json
// jest.config.js
module.exports = {
    preset: 'ts-jest',
    testEnvironment: 'node',
    roots: ['<rootDir>/src', '<rootDir>/tests'],
    testMatch: ['**/__tests__/**/*.ts', '**/?(*.)+(spec|test).ts'],
    transform: {
        '^.+\\.ts$': 'ts-jest'
    },
    collectCoverageFrom: [
        'src/**/*.ts',
        '!src/**/*.d.ts'
    ]
}
```

## Build and Deployment

### TypeScript Configuration
```json
// tsconfig.json
{
    "compilerOptions": {
        "target": "ES2020",
        "module": "commonjs",
        "lib": ["ES2020"],
        "outDir": "./dist",
        "rootDir": "./src",
        "strict": true,
        "esModuleInterop": true,
        "skipLibCheck": true,
        "forceConsistentCasingInFileNames": true,
        "resolveJsonModule": true,
        "declaration": true,
        "declarationMap": true,
        "sourceMap": true
    },
    "include": ["src/**/*"],
    "exclude": ["node_modules", "dist", "tests"]
}
```

### Build Scripts
```json
// package.json
{
    "scripts": {
        "build": "tsc",
        "build:watch": "tsc --watch",
        "dev": "ts-node src/index.ts",
        "start": "node dist/index.js",
        "test": "jest",
        "test:watch": "jest --watch",
        "lint": "eslint src/**/*.ts",
        "format": "prettier --write src/**/*.ts"
    }
}
```

### Docker Support
```dockerfile
# Dockerfile
FROM node:18-alpine

WORKDIR /app

# Copy package files
COPY package*.json ./
RUN npm ci --only=production

# Copy built TypeScript
COPY dist/ ./dist/

# Run the application
CMD ["node", "dist/index.js"]
```

## Performance Considerations

### Runtime Performance
- **Frame State Machine**: Minimal overhead, native object properties
- **Event Dispatching**: Direct method calls, no reflection
- **Type Checking**: Compile-time only, no runtime impact
- **Memory Usage**: Efficient object pooling for compartments

### Compilation Performance
- **Incremental Compilation**: TypeScript's `--incremental` flag
- **Build Caching**: Use `ts-loader` with webpack for caching
- **Code Splitting**: Dynamic imports for large Frame systems

### Optimization Strategies
```typescript
// Lazy loading of Frame systems
const LazySystem = () => import('./HeavySystem').then(m => m.HeavySystem)

// Connection pooling for database systems
class ConnectionPool {
    private static instance: ConnectionPool
    private connections: Connection[] = []
    
    static getInstance(): ConnectionPool {
        if (!ConnectionPool.instance) {
            ConnectionPool.instance = new ConnectionPool()
        }
        return ConnectionPool.instance
    }
}
```

## Security Considerations

### Input Validation
```typescript
@target typescript

import Joi from 'joi'

system APIServer {
    actions:
        validateRequest(data: any): boolean {
            const schema = Joi.object({
                username: Joi.string().alphanum().min(3).max(30).required(),
                email: Joi.string().email().required(),
                age: Joi.number().integer().min(18).max(100)
            })
            
            const { error } = schema.validate(data)
            return !error
        }
        
        sanitizeInput(input: string): string {
            return input
                .replace(/[<>]/g, '')
                .trim()
                .substring(0, 1000)
        }
}
```

### Async Security
```typescript
// Timeout protection for async operations
async function secureAsyncCall<T>(
    operation: Promise<T>, 
    timeoutMs: number = 5000
): Promise<T> {
    return Promise.race([
        operation,
        new Promise<never>((_, reject) => 
            setTimeout(() => reject(new Error('Operation timeout')), timeoutMs)
        )
    ])
}
```

## Debugging and Development Tools

### Source Maps
- Enable source maps in TypeScript compilation
- Frame-to-TypeScript source mapping integration
- Debugging at Frame level vs TypeScript level

### Development Server
```typescript
// Development hot reload
import { watch } from 'fs'
import { spawn } from 'child_process'

function startDevServer() {
    let process: any
    
    const restart = () => {
        if (process) process.kill()
        process = spawn('node', ['dist/index.js'])
    }
    
    watch('./src', { recursive: true }, (eventType, filename) => {
        if (filename?.endsWith('.frm')) {
            console.log('Frame file changed, recompiling...')
            // Trigger Frame recompilation
            restart()
        }
    })
}
```

## Future Enhancements

### Package Distribution
- **@frame-lang/typescript-runtime**: Standalone runtime package
- **@frame-lang/cli**: Frame CLI with TypeScript support
- **@frame-lang/vscode-extension**: VS Code integration

### Advanced Features
- **Frame to TypeScript Definition Files**: Generate `.d.ts` for Frame systems
- **Tree Shaking**: Remove unused runtime components
- **WebAssembly**: Compile Frame systems to WASM
- **Deno Support**: Alternative to Node.js runtime

### Integration Roadmap
- **GraphQL**: Frame systems as GraphQL resolvers
- **React/Vue**: Frame state management for frontend
- **Electron**: Desktop application framework
- **NestJS**: Enterprise framework integration

This comprehensive language support plan ensures Frame TypeScript integration is production-ready with full ecosystem compatibility, robust tooling, and scalable architecture for complex applications.