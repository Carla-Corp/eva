cd interpreter
cargo build --release
cd ..
cp interpreter/target/release/libinterpreter.a test/libeva.a

cd test

gcc main.c -I../includes -L. -leva -o program
./program

cd ..
