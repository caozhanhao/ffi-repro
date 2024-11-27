rustc --crate-type=staticlib callee.rs -O
gcc caller.c -c -mabi=aapcs -O2
gcc --static caller.o libcallee.a -mabi=aapcs -O2
objdump -d a.out > result.S
