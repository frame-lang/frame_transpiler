# Frame Tests for Python

### Prerequisite
- Python3
- Rust

### Working directory
 ```framec_test/python```
 
### Python Virtual Environment
1. Install Virtualenv package
```pip install virtualenv``` or ```pip3 install virtualenv```
2. Create virtual Environment 
```python -m virtualenv venv``` or ```python3 -m virtualenv venv```
3. Activate virtual Environment
```source venv/bin/activate```

### Install requirements

```pip install -r requirements.txt``` or ```pip3 install -r requirements.txt```

### Testing
-  Change directory to python testing project.
    ```cd framec_test/python```
-  Transpile Frame files in Go
    ```cargo test```
-  To test all test cases
    ```pytest -v```

### Reference
- [Pytest](https://docs.pytest.org/en/7.1.x/)