#!/bin/bash
export FRAME_DEBUG=1
./target/debug/framec -l python_3 framec_tests/python/src/test_functions_simple.frm 2>&1 | head -100