# Regression test for Bug #52: TypeScript actions should use Node.js APIs

system NodeApiMappingRegression {
    interface:
        run()
    
    machine:
        $Init {
            run() {
                var success = True
                
                # Verify readFile returns actual file contents
                var content = self.readFile("package.json")
                if type(content) != "str" {
                    print("FAIL: readFile did not return string content")
                    success = False
                } else {
                    if content.find("\"name\"") >= 0 {
                        print("SUCCESS: readFile returned package.json contents")
                    } else {
                        print("FAIL: readFile result missing package name")
                        success = False
                    }
                }
                
                # Verify spawn returns a child process that can be terminated
                var proc = self.spawn("node", ["-e", "process.exit(0)"])
                if not proc {
                    print("FAIL: spawn returned falsy value")
                    success = False
                } else {
                    try {
                        proc.kill()
                        print("SUCCESS: spawn returned process handle")
                    } except Exception as e {
                        print("FAIL: spawn returned non-process: " + str(e))
                        success = False
                    }
                }
                
                # Verify createTcpServer returns server instance
                var server = self.createTcpServer(0)
                if not server {
                    print("FAIL: createTcpServer returned falsy value")
                    success = False
                } else {
                    try {
                        server.close()
                        print("SUCCESS: createTcpServer returned server instance")
                    } except Exception as e {
                        print("FAIL: createTcpServer returned invalid server: " + str(e))
                        success = False
                    }
                }
                
                system.return = success
            }
        }
    
    actions:
        readFile(path) {
            print(f"[Action] readFile: {path}")
        }
        
        spawn(command, args) {
            print(f"[Action] spawn: {command}")
        }
        
        createTcpServer(port) {
            print(f"[Action] createTcpServer: {port}")
        }
}

fn main() {
    print("Running Node API mapping regression test (Bug #52)")
    var tester = NodeApiMappingRegression()
    var passed = tester.run()
    if passed {
        print("SUCCESS: Node API mapping regression passed")
    } else {
        print("FAIL: Node API mapping regression failed")
    }
}
