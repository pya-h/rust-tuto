echo -e "\n1.2 - 1.0 | 1.2 + -1.0*1.0\n2.6 - 2.0 | 2.6 + -1.0*2.0:\n"
echo "- - - - - - - - - - - - - - - - - - - - - - Result In Different Compilers - - - - - - - - - - - - - -"
echo -e -n "C:\n\t"

gcc main.c -o c.out
./c.out

echo -e -n "\n\nC++:\n\t"
g++ main.cpp -o cpp.out
./cpp.out

echo -e -n "\n\nRust:\n\t"
rustc main.rs -o rust.out
./rust.out

echo -e -n "\n\nGO:\n\t"
go run main.go

echo -e -n "\n\nJava\n\t"
javac Program.java
java Program

echo -n -e "\n\nNodeJS:\n\t"
node app.js

echo -n -e "\n\nTypeScript just to make sure:\n\t"
tsx app.ts

echo -n -e "\n\nPython for crying out loud:\n\t"
python3 app.py


echo -e "\n"
