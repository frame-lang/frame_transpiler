# Test loading dictionaries from external sources

import json
import configparser
import os

fn test_json_operations() {
    print("=== JSON Operations ===")
    
    # From JSON string
    var json_str = "{\"name\": \"Alice\", \"age\": 30}"
    var d = json.loads(json_str)
    print("From JSON string: " + str(d))
    
    # To JSON string
    var data = {"user": "Bob", "score": 95}
    var json_output = json.dumps(data)
    print("To JSON string: " + json_output)
    
    # Pretty printing
    var pretty = json.dumps(data, indent=2)
    print("Pretty JSON:\n" + pretty)
}

fn test_config_file() {
    print("\n=== Configuration File Loading ===")
    
    # First create a test config file if it doesn't exist
    var config_exists = os.path.exists("config.ini")
    if not config_exists {
        print("Creating test config.ini file...")
        var f = open("config.ini", "w")
        f.write("[database]\n")
        f.write("host = localhost\n")
        f.write("port = 5432\n")
        f.write("user = admin\n")
        f.write("password = secret123\n")
        f.write("\n[application]\n")
        f.write("debug = true\n")
        f.write("log_level = INFO\n")
        f.write("max_connections = 100\n")
        f.write("\n[paths]\n")
        f.write("data = /var/data\n")
        f.write("logs = /var/logs\n")
        f.close()
    }
    
    # Create config parser
    var config = configparser.ConfigParser()
    config.read("config.ini")
    
    # Convert section to dictionary
    var db_config = dict(config["database"])
    print("Database config: " + str(db_config))
    print("DB Host: " + db_config["host"])
    print("DB Port: " + db_config["port"])
    
    # Access different sections
    var app_config = dict(config["application"])
    print("\nApplication config: " + str(app_config))
    print("Debug mode: " + app_config["debug"])
    
    # Get all sections
    var sections = config.sections()
    print("\nAll sections: " + str(sections))
}

fn test_environment_variables() {
    print("\n=== Environment Variables ===")
    
    # Get single environment variable
    var path = os.environ.get("PATH", "not set")
    print("PATH length: " + str(len(path)) + " chars")
    
    # Get HOME directory
    var home = os.environ.get("HOME", "not set")
    print("HOME: " + home)
    
    # Convert all env vars to dictionary (careful - can be large!)
    var env_dict = dict(os.environ)
    print("Total environment variables: " + str(len(env_dict)))
    
    # Get some common ones
    var user = os.environ.get("USER", "unknown")
    var shell = os.environ.get("SHELL", "unknown")
    print("Current user: " + user)
    print("Current shell: " + shell)
    
    # Check if a var exists (note: 'in' operator not supported yet)
    # var has_display = "DISPLAY" in os.environ
    # print("Has DISPLAY variable: " + str(has_display))
}

fn test_json_file_operations() {
    print("\n=== JSON File Operations ===")
    
    # Write JSON to file
    var test_data = {
        "project": "Frame v0.38",
        "features": ["json", "config", "env"],
        "test_date": "2025-01-23"
    }
    
    var f = open("test_data.json", "w")
    json.dump(test_data, f, indent=2)
    f.close()
    print("Wrote test data to test_data.json")
    
    # Read JSON from file
    var f2 = open("test_data.json", "r")
    var loaded_data = json.load(f2)
    f2.close()
    print("Read back: " + str(loaded_data))
    print("Project: " + loaded_data["project"])
}

fn main() {
    print("Frame v0.38 - Loading from External Sources")
    print("=" * 50)
    
    test_json_operations()
    test_config_file()
    test_environment_variables()
    test_json_file_operations()
    
    print("\n" + "=" * 50)
    print("Summary:")
    print("  [OK] JSON string loading/dumping")
    print("  [OK] JSON file operations")
    print("  [OK] Configuration file parsing")
    print("  [OK] Environment variable access")
    print("\nAll external loading methods work in Frame!")
}