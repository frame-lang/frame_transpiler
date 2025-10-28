system Bug54Test {
    interface:
        generatePythonCode(): string
        testBooleanLiterals(): string
    
    machine:
        $Start {
            generatePythonCode(): string {
                var code = "import socket\n"
                code = code + "result = True\n"
                code = code + "status = False\n" 
                code = code + "if True:\n"
                code = code + "    return False\n"
                return code
            }
            
            testBooleanLiterals(): string {
                var pythonCode = "def test():\n"
                pythonCode = pythonCode + "    enabled = True\n"
                pythonCode = pythonCode + "    disabled = False\n"
                pythonCode = pythonCode + "    return True if enabled else False\n"
                return pythonCode
            }
        }
    
    actions:
        createMinimalPythonCode(): string {
            var code = "import socket\n"
            code = code + "        return True\n"
            code = code + "        return False\n"
            return code
        }
}