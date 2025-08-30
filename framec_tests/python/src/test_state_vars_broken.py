#!/usr/bin/env python3

# Test to demonstrate state vars are broken
import sys
import os
sys.path.append(os.path.dirname(__file__))

from state_vars.state_vars import StateVars

def main():
    print("Creating StateVars instance...")
    sv = StateVars()
    print("Calling X() to trigger state_vars usage...")
    try:
        sv.X()
        print("SUCCESS: X() completed without error")
    except AttributeError as e:
        print(f"ERROR: {e}")
        print("CONFIRMED: state_vars are broken - FrameCompartment missing state_vars attribute")
        return False
    return True

if __name__ == "__main__":
    success = main()
    if not success:
        sys.exit(1)