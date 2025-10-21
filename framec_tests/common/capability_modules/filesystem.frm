# Frame Capability Module: Filesystem Operations
# Provides universal file I/O across all target languages
# Python: uses pathlib, os, and io modules
# TypeScript: uses fs module (Node.js) or File API (browser)
# C#: uses System.IO namespace
# Java: uses java.nio.file package
# Go: uses os and io/ioutil packages
# Rust: uses std::fs module
# C: uses POSIX file operations or platform-specific APIs

module FileSystem {
    # Check if file or directory exists
    fn exists(path) {
        # Python: Path(path).exists()
        # TypeScript: fs.existsSync(path)
        # C#: File.Exists(path) or Directory.Exists(path)
        # Java: Files.exists(Paths.get(path))
        # Go: _, err := os.Stat(path); err == nil
        # Rust: std::path::Path::new(path).exists()
        # C: access(path, F_OK) == 0
        
        print("Checking if path exists: " + path)
        # Simulation - real implementation handled by visitor
        return True
    }
    
    # Check if path is a file
    fn isFile(path) {
        print("Checking if path is file: " + path)
        return True
    }
    
    # Check if path is a directory
    fn isDirectory(path) {
        print("Checking if path is directory: " + path)
        return False
    }
    
    # Read entire file as string
    fn readFile(path) {
        # Python: Path(path).read_text()
        # TypeScript: fs.readFileSync(path, 'utf8')
        # C#: File.ReadAllText(path)
        # Java: Files.readString(Paths.get(path))
        # Go: ioutil.ReadFile(path)
        # Rust: std::fs::read_to_string(path)
        # C: fopen/fread/fclose sequence
        
        if not exists(path) {
            return {
                "isOk": False,
                "isError": True,
                "value": None,
                "error": "File not found: " + path
            }
        }
        
        print("Reading file: " + path)
        return {
            "isOk": True,
            "isError": False,
            "value": "simulated file content from " + path,
            "error": None
        }
    }
    
    # Write string to file with Result error handling
    fn writeFile(path, content) {
        # Python: Path(path).write_text(content)
        # TypeScript: fs.writeFileSync(path, content, 'utf8')
        # C#: File.WriteAllText(path, content)
        # Java: Files.writeString(Paths.get(path), content)
        # Go: ioutil.WriteFile(path, []byte(content), 0644)
        # Rust: std::fs::write(path, content)
        # C: fopen/fwrite/fclose sequence
        
        try {
            print("Writing to file: " + path + " (" + str(len(content)) + " chars)")
            # Simulate potential write errors
            if len(content) > 1000000 {
                return {
                    "isOk": False,
                    "isError": True,
                    "value": None,
                    "error": "File too large to write: " + str(len(content)) + " chars"
                }
            }
            
            return {
                "isOk": True,
                "isError": False,
                "value": None,
                "error": None
            }
        } except Exception as e {
            return {
                "isOk": False,
                "isError": True,
                "value": None,
                "error": "Write failed: " + str(e)
            }
        }
    }
    
    # Append string to file
    fn appendFile(path, content) {
        print("Appending to file: " + path + " (" + str(len(content)) + " chars)")
    }
    
    # Read file as lines
    fn readLines(path) {
        var content = readFile(path)
        return content.split("\n")
    }
    
    # Write lines to file
    fn writeLines(path, lines) {
        var content = "\n".join(lines)
        writeFile(path, content)
    }
    
    # Read file as bytes
    fn readBytes(path) {
        # Python: Path(path).read_bytes()
        # TypeScript: fs.readFileSync(path)
        # C#: File.ReadAllBytes(path)
        # Java: Files.readAllBytes(Paths.get(path))
        # Go: ioutil.ReadFile(path)
        # Rust: std::fs::read(path)
        # C: binary file reading
        
        print("Reading file as bytes: " + path)
        return b"simulated binary content"
    }
    
    # Write bytes to file
    fn writeBytes(path, data) {
        print("Writing bytes to file: " + path + " (" + str(len(data)) + " bytes)")
    }
    
    # Copy file
    fn copyFile(sourcePath, destPath) {
        # Python: shutil.copy2(sourcePath, destPath)
        # TypeScript: fs.copyFileSync(sourcePath, destPath)
        # C#: File.Copy(sourcePath, destPath)
        # Java: Files.copy(source, dest)
        # Go: custom copy implementation
        # Rust: std::fs::copy(sourcePath, destPath)
        # C: custom copy implementation
        
        var content = readFile(sourcePath)
        writeFile(destPath, content)
        print("Copied file from " + sourcePath + " to " + destPath)
    }
    
    # Move/rename file
    fn moveFile(sourcePath, destPath) {
        copyFile(sourcePath, destPath)
        deleteFile(sourcePath)
        print("Moved file from " + sourcePath + " to " + destPath)
    }
    
    # Delete file
    fn deleteFile(path) {
        # Python: Path(path).unlink()
        # TypeScript: fs.unlinkSync(path)
        # C#: File.Delete(path)
        # Java: Files.delete(Paths.get(path))
        # Go: os.Remove(path)
        # Rust: std::fs::remove_file(path)
        # C: unlink(path)
        
        print("Deleting file: " + path)
    }
    
    # Create directory
    fn createDirectory(path) {
        # Python: Path(path).mkdir(parents=True, exist_ok=True)
        # TypeScript: fs.mkdirSync(path, { recursive: true })
        # C#: Directory.CreateDirectory(path)
        # Java: Files.createDirectories(Paths.get(path))
        # Go: os.MkdirAll(path, 0755)
        # Rust: std::fs::create_dir_all(path)
        # C: mkdir with recursive creation
        
        print("Creating directory: " + path)
    }
    
    # Remove directory
    fn removeDirectory(path) {
        print("Removing directory: " + path)
    }
    
    # List directory contents
    fn listDirectory(path) {
        # Python: list(Path(path).iterdir())
        # TypeScript: fs.readdirSync(path)
        # C#: Directory.GetFileSystemEntries(path)
        # Java: Files.list(Paths.get(path))
        # Go: ioutil.ReadDir(path)
        # Rust: std::fs::read_dir(path)
        # C: opendir/readdir/closedir
        
        print("Listing directory: " + path)
        return ["file1.txt", "file2.txt", "subdirectory"]
    }
    
    # Get file size
    fn getFileSize(path) {
        # Python: Path(path).stat().st_size
        # TypeScript: fs.statSync(path).size
        # C#: new FileInfo(path).Length
        # Java: Files.size(Paths.get(path))
        # Go: info.Size() from os.Stat()
        # Rust: metadata.len() from std::fs::metadata()
        # C: stat() system call
        
        print("Getting file size: " + path)
        return 1024  # Simulated size
    }
    
    # Get file modification time
    fn getModificationTime(path) {
        print("Getting modification time: " + path)
        return "2025-10-16T12:00:00Z"  # Simulated timestamp
    }
    
    # Path manipulation utilities
    fn joinPath(parts) {
        # Join path components with platform-appropriate separator
        # Python: Path(*parts)
        # TypeScript: path.join(...parts)
        # C#: Path.Combine(parts)
        # Java: Paths.get(parts[0], ...parts[1:])
        # Go: filepath.Join(parts...)
        # Rust: PathBuf::from().join() chain
        # C: custom implementation with separators
        
        return "/".join(parts)  # Unix-style for simulation
    }
    
    fn getParentDirectory(path) {
        # Get parent directory of path
        var parts = path.split("/")
        if len(parts) > 1 {
            parts.pop()
            return "/".join(parts)
        } else {
            return "."
        }
    }
    
    fn getFileName(path) {
        # Get filename from path
        var parts = path.split("/")
        return parts[-1]
    }
    
    fn getFileExtension(path) {
        # Get file extension
        var filename = getFileName(path)
        var parts = filename.split(".")
        if len(parts) > 1 {
            return "." + parts[-1]
        } else {
            return ""
        }
    }
    
    fn getBaseName(path) {
        # Get filename without extension
        var filename = getFileName(path)
        var parts = filename.split(".")
        if len(parts) > 1 {
            parts.pop()
            return ".".join(parts)
        } else {
            return filename
        }
    }
    
    # Working directory operations
    fn getCurrentDirectory() {
        # Python: Path.cwd()
        # TypeScript: process.cwd()
        # C#: Directory.GetCurrentDirectory()
        # Java: Paths.get("").toAbsolutePath()
        # Go: os.Getwd()
        # Rust: std::env::current_dir()
        # C: getcwd()
        
        return "/simulated/current/directory"
    }
    
    fn changeDirectory(path) {
        # Python: os.chdir(path)
        # TypeScript: process.chdir(path)
        # C#: Directory.SetCurrentDirectory(path)
        # Java: System.setProperty("user.dir", path)
        # Go: os.Chdir(path)
        # Rust: std::env::set_current_dir(path)
        # C: chdir(path)
        
        print("Changing directory to: " + path)
    }
    
    # Temporary file operations
    fn createTempFile(prefix, suffix) {
        # Create temporary file
        # Python: tempfile.NamedTemporaryFile()
        # TypeScript: os.tmpdir() + unique name
        # C#: Path.GetTempFileName()
        # Java: Files.createTempFile()
        # Go: ioutil.TempFile()
        # Rust: tempfile crate
        # C: mkstemp() or platform-specific
        
        var tempPath = "/tmp/" + prefix + "_temp_" + suffix
        print("Creating temporary file: " + tempPath)
        return tempPath
    }
    
    fn createTempDirectory(prefix) {
        # Create temporary directory
        var tempDir = "/tmp/" + prefix + "_temp_dir"
        createDirectory(tempDir)
        return tempDir
    }
    
    # File streaming for large files
    fn openFileForReading(path) {
        # Open file handle for reading
        # Returns a handle that can be used with readChunk
        print("Opening file for reading: " + path)
        return {
            "path": path,
            "position": 0,
            "isOpen": True
        }
    }
    
    fn openFileForWriting(path) {
        # Open file handle for writing
        print("Opening file for writing: " + path)
        return {
            "path": path,
            "position": 0,
            "isOpen": True
        }
    }
    
    fn readChunk(fileHandle, size) {
        # Read chunk of data from file
        if not fileHandle["isOpen"] {
            raise RuntimeError("File handle is closed")
        }
        
        print("Reading " + str(size) + " bytes from " + fileHandle["path"])
        fileHandle["position"] = fileHandle["position"] + size
        return "chunk_data_" + str(size)
    }
    
    fn writeChunk(fileHandle, data) {
        # Write chunk of data to file
        if not fileHandle["isOpen"] {
            raise RuntimeError("File handle is closed")
        }
        
        print("Writing " + str(len(data)) + " bytes to " + fileHandle["path"])
        fileHandle["position"] = fileHandle["position"] + len(data)
    }
    
    fn closeFile(fileHandle) {
        # Close file handle
        fileHandle["isOpen"] = False
        print("Closed file: " + fileHandle["path"])
    }
    
    # ============== SAFE FILE OPERATIONS WITH RAII ===============
    
    # Safe file operations using RAII pattern from Memory module
    fn withFile(path, mode, operation) {
        # Universal RAII pattern for safe file operations
        # Automatically handles cleanup even if exceptions occur
        # Python: uses 'with' statement
        # TypeScript: uses try/finally
        # C#: uses 'using' statement
        # Java: uses try-with-resources
        # Go: uses defer
        # Rust: automatic Drop trait
        # C: explicit cleanup with error handling
        
        var fileHandleResult = _openSafeFileHandle(path, mode)
        
        if fileHandleResult["isError"] {
            return fileHandleResult
        }
        
        var fileHandle = fileHandleResult["value"]
        
        try {
            var result = operation(fileHandle)
            return {
                "isOk": True,
                "isError": False,
                "value": result,
                "error": None
            }
        } except Exception as e {
            return {
                "isOk": False,
                "isError": True,
                "value": None,
                "error": "File operation failed: " + str(e)
            }
        } finally {
            # Always cleanup, even if operation fails
            if fileHandle["isOpen"] {
                _closeSafeFileHandle(fileHandle)
            }
        }
    }
    
    fn _openSafeFileHandle(path, mode) {
        # Private function to create a safe file handle
        # Returns Result<FileHandle, Error>
        
        if not exists(path) and mode == "r" {
            return {
                "isOk": False,
                "isError": True,
                "value": None,
                "error": "File not found: " + path
            }
        }
        
        try {
            var handle = {
                "path": path,
                "mode": mode,
                "isOpen": True,
                "position": 0,
                "close": lambda: _closeSafeFileHandle(handle)
            }
            
            print("Safely opened file: " + path + " (mode: " + mode + ")")
            
            return {
                "isOk": True,
                "isError": False,
                "value": handle,
                "error": None
            }
        } except Exception as e {
            return {
                "isOk": False,
                "isError": True,
                "value": None,
                "error": "Failed to open file: " + str(e)
            }
        }
    }
    
    fn _closeSafeFileHandle(handle) {
        # Private function to safely close file handle
        if handle["isOpen"] {
            handle["isOpen"] = False
            print("Safely closed file: " + handle["path"])
        }
    }
    
    # Convenience functions using safe pattern
    fn safeReadFile(path) {
        # Read entire file safely
        return withFile(path, "r", lambda handle: {
            var content = "simulated content from " + handle["path"]
            return content
        })
    }
    
    fn safeWriteFile(path, content) {
        # Write entire file safely
        return withFile(path, "w", lambda handle: {
            print("Writing " + str(len(content)) + " chars to " + handle["path"])
            return True
        })
    }
    
    fn safeAppendFile(path, content) {
        # Append to file safely
        return withFile(path, "a", lambda handle: {
            print("Appending " + str(len(content)) + " chars to " + handle["path"])
            return True
        })
    }
}