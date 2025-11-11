@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test type annotations in Frame v0.43
# This test demonstrates Frame's Python type hint support

# Function with typed parameters and return type
fn add(a: int, b: int) : int {
    return a + b
}

# Function with float types
fn calculate_area(width: float, height: float) : float {
    return width * height
}

# Function with string type (both Frame and Python style work)
fn greet(name: string) : string {
    return "Hello, " + name
}

fn greet_python(name: str) : str {
    return "Hi, " + name
}

# Function with collection types
fn process_list(items: list) : int {
    return len(items)
}

fn get_config() : dict {
    return {"debug": false, "port": 8080}
}

fn unique_items(data: list) : set {
    # Convert list to set
    return set(data)
}

# Function with tuple return type
fn get_coordinates() : tuple {
    return (10, 20)
}

# Function with any type
fn flexible_function(value: any) : any {
    return value
}

# Async function with type hints
async fn async_fetch(url: str) : str {
    # Simulated async operation
    return "Data from " + url
}

# Test variable type annotations
fn test_variables() {
    # Basic types
    count: int = 0
    price: float = 19.99
    message: string = "Frame v0.43"
    is_active: bool = true
    
    # Python-style types
    name: str = "Python"
    numbers: list = [1, 2, 3, 4, 5]
    config: dict = {"key": "value"}
    unique: set = {1, 2, 3}
    coords: tuple = (100, 200)
    anything: any = None
    
    # Variables without type annotations (still valid)
    auto = "inferred"
    
    print("Type annotations test complete")
}

# Test function calls with typed returns
fn main() {
    sum: int = add(5, 3)
    area: float = calculate_area(10.5, 20.3)
    greeting: string = greet("Frame")
    items: set = unique_items([1, 2, 2, 3, 3, 3])
    
    print("Sum: " + str(sum))
    print("Area: " + str(area))
    print("Greeting: " + greeting)
    print("Unique items: " + str(items))
    
    test_variables()
}

# Module initialization
