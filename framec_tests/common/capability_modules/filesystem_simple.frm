# Frame Capability Module: Filesystem Operations (Simplified)
# Provides universal file I/O across all target languages

module FileSystem {
    # Check if file exists
    fn exists(path) {
        print("Checking if path exists: " + path)
        return true
    }
    
    # Read entire file as string
    fn readFile(path) {
        print("Reading file: " + path)
        return "simulated file content from " + path
    }
    
    # Write string to file
    fn writeFile(path, content) {
        print("Writing to file: " + path + " (" + str(len(content)) + " chars)")
    }
    
    # Append string to file
    fn appendFile(path, content) {
        print("Appending to file: " + path + " (" + str(len(content)) + " chars)")
    }
    
    # Copy file
    fn copyFile(sourcePath, destPath) {
        print("Copied file from " + sourcePath + " to " + destPath)
    }
    
    # Delete file
    fn deleteFile(path) {
        print("Deleting file: " + path)
    }
    
    # Create directory
    fn createDirectory(path) {
        print("Creating directory: " + path)
    }
    
    # List directory contents
    fn listDirectory(path) {
        print("Listing directory: " + path)
        return ["file1.txt", "file2.txt", "subdirectory"]
    }
    
    # Get file size
    fn getFileSize(path) {
        print("Getting file size: " + path)
        return 1024
    }
    
    # Path utilities
    fn joinPath(parts) {
        # Simplified implementation - string join not working correctly in Frame modules
        return parts[0] + "/" + parts[1]
    }
    
    fn getFileName(path) {
        var parts = path.split("/")
        return parts[-1]
    }
    
    fn getCurrentDirectory() {
        return "/simulated/current/directory"
    }
}