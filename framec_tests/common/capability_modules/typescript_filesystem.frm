# TypeScript-Optimized Filesystem Capability Module
# Leverages File System Access API, File API, and modern browser/Node.js patterns
# Supports both browser and Node.js environments with proper type safety

module TypeScriptFileSystem {
    # File API integration for browser environments
    fn withFileHandle(fileHandle, operation) {
        # TypeScript: (fileHandle: FileSystemFileHandle, operation: (handle: FileSystemFileHandle) => Promise<T>): Promise<Result<T, string>>
        
        try {
            var result = await operation(fileHandle)
            return {
                "kind": "ok",
                "isOk": True,
                "value": result,
                "error": None
            }
        } except Exception as e {
            return {
                "kind": "error",
                "isOk": False,
                "value": None,
                "error": "File handle operation failed: " + str(e)
            }
        }
    }
    
    # Safe file reading with multiple environment support
    async fn readFile(path) {
        # TypeScript: (path: string): Promise<Result<string, string>>
        # Browser: uses File System Access API or fetch
        # Node.js: uses fs.promises.readFile
        
        try {
            # Environment detection will be handled by TypeScript visitor
            if typeof(window) != "undefined" {
                # Browser environment - use fetch for public files
                var response = await fetch(path)
                if not response.ok {
                    return {
                        "kind": "error",
                        "isOk": False,
                        "value": None,
                        "error": "File not found: " + path + " (HTTP " + str(response.status) + ")"
                    }
                }
                
                var content = await response.text()
                return {
                    "kind": "ok",
                    "isOk": True,
                    "value": content,
                    "error": None
                }
            } else {
                # Node.js environment - use fs.promises
                var fs = require("fs").promises
                var content = await fs.readFile(path, "utf8")
                return {
                    "kind": "ok",
                    "isOk": True,
                    "value": content,
                    "error": None
                }
            }
        } except Exception as e {
            return {
                "kind": "error",
                "isOk": False,
                "value": None,
                "error": "Read file failed: " + str(e)
            }
        }
    }
    
    # Safe file writing with environment detection
    async fn writeFile(path, content) {
        # TypeScript: (path: string, content: string): Promise<Result<void, string>>
        
        try {
            if typeof(window) != "undefined" {
                # Browser environment - use File System Access API if available
                if hasattr(window, "showSaveFilePicker") {
                    var fileHandle = await window.showSaveFilePicker({
                        "suggestedName": path
                    })
                    
                    var writable = await fileHandle.createWritable()
                    await writable.write(content)
                    await writable.close()
                    
                    return {
                        "kind": "ok",
                        "isOk": True,
                        "value": None,
                        "error": None
                    }
                } else {
                    # Fallback: create downloadable blob
                    var blob = Blob([content], {"type": "text/plain"})
                    var url = URL.createObjectURL(blob)
                    
                    var a = document.createElement("a")
                    a.href = url
                    a.download = path
                    a.click()
                    
                    URL.revokeObjectURL(url)
                    
                    return {
                        "kind": "ok",
                        "isOk": True,
                        "value": None,
                        "error": None
                    }
                }
            } else {
                # Node.js environment
                var fs = require("fs").promises
                await fs.writeFile(path, content, "utf8")
                
                return {
                    "kind": "ok",
                    "isOk": True,
                    "value": None,
                    "error": None
                }
            }
        } except Exception as e {
            return {
                "kind": "error",
                "isOk": False,
                "value": None,
                "error": "Write file failed: " + str(e)
            }
        }
    }
    
    # Stream-based file operations for large files
    async fn createReadStream(path) {
        # TypeScript: (path: string): Promise<Result<ReadableStream<Uint8Array>, string>>
        
        try {
            if typeof(window) != "undefined" {
                # Browser: use fetch with streaming
                var response = await fetch(path)
                if not response.ok {
                    return {
                        "kind": "error",
                        "isOk": False,
                        "value": None,
                        "error": "Stream creation failed: HTTP " + str(response.status)
                    }
                }
                
                return {
                    "kind": "ok",
                    "isOk": True,
                    "value": response.body,
                    "error": None
                }
            } else {
                # Node.js: use fs.createReadStream
                var fs = require("fs")
                var stream = fs.createReadStream(path)
                
                return {
                    "kind": "ok",
                    "isOk": True,
                    "value": stream,
                    "error": None
                }
            }
        } except Exception as e {
            return {
                "kind": "error",
                "isOk": False,
                "value": None,
                "error": "Create read stream failed: " + str(e)
            }
        }
    }
    
    # File system watching (Node.js) / Service Worker caching (Browser)
    fn watchFile(path, callback) {
        # TypeScript: (path: string, callback: (eventType: string, filename: string) => void): Result<FileWatcher, string>
        
        try {
            if typeof(window) != "undefined" {
                # Browser: simulate with periodic checks or Service Worker
                var intervalId = setInterval(lambda: {
                    # Simplified simulation - real implementation would use Service Worker
                    callback("change", path)
                }, 5000)
                
                return {
                    "kind": "ok",
                    "isOk": True,
                    "value": {
                        "close": lambda: clearInterval(intervalId),
                        "path": path
                    },
                    "error": None
                }
            } else {
                # Node.js: use fs.watch
                var fs = require("fs")
                var watcher = fs.watch(path, callback)
                
                return {
                    "kind": "ok",
                    "isOk": True,
                    "value": watcher,
                    "error": None
                }
            }
        } except Exception as e {
            return {
                "kind": "error",
                "isOk": False,
                "value": None,
                "error": "File watching failed: " + str(e)
            }
        }
    }
    
    # Directory operations with proper permissions
    async fn createDirectory(path) {
        # TypeScript: (path: string): Promise<Result<void, string>>
        
        try {
            if typeof(window) != "undefined" {
                # Browser: use File System Access API
                if hasattr(window, "showDirectoryPicker") {
                    var dirHandle = await window.showDirectoryPicker()
                    # Browser directories are handled differently
                    return {
                        "kind": "ok",
                        "isOk": True,
                        "value": dirHandle,
                        "error": None
                    }
                } else {
                    return {
                        "kind": "error",
                        "isOk": False,
                        "value": None,
                        "error": "Directory creation not supported in this browser"
                    }
                }
            } else {
                # Node.js: use fs.promises.mkdir
                var fs = require("fs").promises
                await fs.mkdir(path, {"recursive": True})
                
                return {
                    "kind": "ok",
                    "isOk": True,
                    "value": None,
                    "error": None
                }
            }
        } except Exception as e {
            return {
                "kind": "error",
                "isOk": False,
                "value": None,
                "error": "Create directory failed: " + str(e)
            }
        }
    }
    
    # Safe file operations with RAII pattern
    async fn withFile(path, mode, operation) {
        # TypeScript: async <T>(path: string, mode: "r" | "w" | "a", operation: (file: FileHandle) => Promise<T>): Promise<Result<T, string>>
        
        var fileResult = None
        try {
            if typeof(window) != "undefined" {
                # Browser environment
                if mode == "r" {
                    var response = await fetch(path)
                    if not response.ok {
                        return {
                            "kind": "error",
                            "isOk": False,
                            "value": None,
                            "error": "File not found: " + path
                        }
                    }
                    
                    fileResult = {
                        "path": path,
                        "mode": mode,
                        "response": response,
                        "read": lambda: response.text(),
                        "close": lambda: None  # No cleanup needed for fetch
                    }
                } else {
                    # Write mode - use File System Access API or download
                    fileResult = {
                        "path": path,
                        "mode": mode,
                        "content": "",
                        "write": lambda data: fileResult["content"] = fileResult["content"] + data,
                        "close": lambda: None
                    }
                }
            } else {
                # Node.js environment
                var fs = require("fs").promises
                var fileHandle = await fs.open(path, mode)
                
                fileResult = {
                    "path": path,
                    "mode": mode,
                    "handle": fileHandle,
                    "read": lambda: fileHandle.readFile({"encoding": "utf8"}),
                    "write": lambda data: fileHandle.writeFile(data),
                    "close": lambda: fileHandle.close()
                }
            }
            
            var result = await operation(fileResult)
            
            return {
                "kind": "ok",
                "isOk": True,
                "value": result,
                "error": None
            }
        } except Exception as e {
            return {
                "kind": "error",
                "isOk": False,
                "value": None,
                "error": "File operation failed: " + str(e)
            }
        } finally {
            # Always cleanup
            if fileResult and hasattr(fileResult, "close") {
                try {
                    await fileResult.close()
                } except Exception {
                    # Ignore cleanup errors
                    pass
                }
            }
        }
    }
    
    # IndexedDB integration for browser persistent storage
    async fn createPersistentStorage(dbName, version) {
        # TypeScript: (dbName: string, version: number): Promise<Result<IDBDatabase, string>>
        
        try {
            if typeof(window) == "undefined" {
                return {
                    "kind": "error",
                    "isOk": False,
                    "value": None,
                    "error": "IndexedDB not available in Node.js environment"
                }
            }
            
            var request = indexedDB.open(dbName, version)
            
            var db = await Promise((resolve, reject) => {
                request.onsuccess = lambda event: resolve(event.target.result)
                request.onerror = lambda event: reject(event.target.error)
                request.onupgradeneeded = lambda event: {
                    var db = event.target.result
                    # Create object store for file-like storage
                    if not db.objectStoreNames.contains("files") {
                        db.createObjectStore("files", {"keyPath": "path"})
                    }
                }
            })
            
            return {
                "kind": "ok",
                "isOk": True,
                "value": db,
                "error": None
            }
        } except Exception as e {
            return {
                "kind": "error",
                "isOk": False,
                "value": None,
                "error": "Persistent storage creation failed: " + str(e)
            }
        }
    }
    
    # File type detection using MIME types
    fn detectFileType(path) {
        # TypeScript: (path: string): string
        var extension = path.split(".").pop().toLowerCase()
        
        var mimeTypes = {
            "txt": "text/plain",
            "html": "text/html", 
            "css": "text/css",
            "js": "application/javascript",
            "json": "application/json",
            "png": "image/png",
            "jpg": "image/jpeg",
            "jpeg": "image/jpeg",
            "gif": "image/gif",
            "svg": "image/svg+xml",
            "pdf": "application/pdf",
            "zip": "application/zip"
        }
        
        return mimeTypes.get(extension, "application/octet-stream")
    }
}