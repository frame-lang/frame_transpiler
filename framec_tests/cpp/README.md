# Application README

## Setting up "GoogleTest" Library

To setup the "GoogleTest" library, follow these commands in the terminal:

1. Install the necessary dependencies:
sudo apt-get install -y libgtest-dev
sudo apt-get install cmake # install cmake


2. Navigate to the GoogleTest source directory:
cd /usr/src/gtest


3. Create a build directory and navigate into it:
sudo mkdir build
cd build

4. Generate the build files using CMake:
sudo cmake ..

5. Build the library:
sudo make

6. Install the library:
sudo make install

7. Clone the GoogleTest repository:
git clone https://github.com/google/googletest
cd googletest

8. Configure and build GoogleTest:
cmake -DBUILD_SHARED_LIBS=ON .
make
cd googlemock
sudo cp ./libgmock_main.so ./gtest/libgtest.so gtest/libgtest_main.so ./libgmock.so /usr/lib/
sudo ldconfig


## Transpiling a File

To transpile a file, run the following command in the Transpiler folder of the project ("/frame/frame_transpiler"):

cargo run -- -l <Language> <frame file path> > <final (transpiled) file>


## Compiling C++ Files

To compile a C++ file, use the following command:

g++ -std=c++17 ./<file.cpp>


## Generating Test Cases

To generate test cases, compile the test file using the following command:

g++ -o <test_file> <File.cpp> <test_file.cpp> -lgtest -lgtest_main -lpthread

Then, run the test cases:

./test_file

