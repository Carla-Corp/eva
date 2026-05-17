cd interpreter
cargo build --release
cd ..
cp interpreter/target/release/libinterpreter.a test_cpp/libeva.a

cd test_cpp
g++ main.cpp -I../includes -L. -leva -o program
./program

cd ..
