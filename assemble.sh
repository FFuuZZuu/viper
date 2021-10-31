gcc -static tests/maths.S -o tests/maths
echo Assemble: $?
tests/maths
echo Run: $?
