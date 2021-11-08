gcc -static tests/test.S -o tests/test
echo Assemble: $?
tests/test
echo Run: $?
