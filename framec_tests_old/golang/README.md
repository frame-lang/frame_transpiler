# Frame Tests for Go

### Prerequisite
- Go
- Rust

### Testing
-  Change directory to golang testing project.
    ```cd framec_test/golang```
-  Transpile Frame files in Go
    ```cargo test```
-  To test all test cases
    ```go test ./... -v | grep -v framelang```
-  To test specfic test case file
    ```go test ./<folder name> -v```
### Reference
- [Testify](https://github.com/stretchr/testify)
- [Go Testing](https://pkg.go.dev/testing)