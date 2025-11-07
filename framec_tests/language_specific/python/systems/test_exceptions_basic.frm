# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
{-- Test basic exception handling in Frame
    This test covers try/except/else/finally blocks and raise statements --}

fn test_basic_try_except() {
    print("Testing basic try/except")
    
    try:
        print("In try block")
        x = 10 / 2
        print("Division succeeded: " + str(x))
    except :
        print("Exception caught")
    
    print("After try/except")

fn test_specific_exception() {
    print("Testing specific exception types")
    
    try:
        list = []
        item = list[10]  {-- IndexError --}
    except IndexError :
        print("Caught IndexError")
    
    try:
        x = int("not a number")  {-- ValueError --}
    except ValueError as e :
        print("Caught ValueError: " + str(e))

fn test_multiple_exceptions() {
    print("Testing multiple exception types")
    
    try:
        {-- Some operation that could raise different errors --}
        x = 1 / 0
    except (ZeroDivisionError, ValueError) as err :
        print("Caught exception: " + str(err))

fn test_else_clause() {
    print("Testing else clause")
    
    try:
        x = 10 / 2
    except ZeroDivisionError :
        print("Division by zero")
    else:
        print("No exception occurred")

fn test_finally_clause() {
    print("Testing finally clause")
    
    try:
        print("Try block")
        x = 10 / 0
    except ZeroDivisionError :
        print("Caught division by zero")
    finally:
        print("Finally block always executes")

fn test_nested_try() {
    print("Testing nested try blocks")
    
    try:
        print("Outer try")
        try:
            print("Inner try")
            raise ValueError("Inner error")
        except ValueError as e :
            print("Inner except: " + str(e))
            raise  {-- Re-raise the exception --}
    except ValueError :
        print("Outer except caught re-raised exception")

fn test_raise_statement() {
    print("Testing raise statement")
    
    try:
        raise ValueError("Custom error message")
    except ValueError as e :
        print("Caught: " + str(e))
    
    {-- Test raise with from clause --}
    try:
        try:
            x = 1 / 0
        except ZeroDivisionError as e :
            raise ValueError("Wrapped error") from e
    except ValueError as e :
        print("Caught wrapped error: " + str(e))

fn test_bare_raise() {
    print("Testing bare raise (re-raise)")
    
    try:
        try:
            raise RuntimeError("Original error")
        except RuntimeError :
            print("Catching and re-raising")
            raise  {-- Re-raise the same exception --}
    except RuntimeError as e :
        print("Caught re-raised: " + str(e))

fn test_all_clauses() {
    print("Testing all clauses together")
    
    try:
        print("Try block")
        x = 10 / 2
    except ZeroDivisionError :
        print("Exception handler")
    else:
        print("Else block - no exception")
    finally:
        print("Finally block - always runs")

{-- Main function to run all tests --}
fn main() {
    print("=== Frame Exception Handling Tests ===")
    print("")
    
    test_basic_try_except()
    print("")
    
    test_specific_exception()
    print("")
    
    test_multiple_exceptions()
    print("")
    
    test_else_clause()
    print("")
    
    test_finally_clause()
    print("")
    
    test_nested_try()
    print("")
    
    test_raise_statement()
    print("")
    
    test_bare_raise()
    print("")
    
    test_all_clauses()
    print("")
    
    print("=== All tests completed ===")

{-- Run the tests --}
