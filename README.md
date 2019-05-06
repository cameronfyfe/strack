# strack
Static stack analysis tool for embedded C.  
Currently supports analysis of plain C compiled with arm-none-eabi-gcc toolchain.
Support for C++ and/or other compiler toolchains may be added in the future.  Difficulties of analyzing C++ outline in C++ section

## Usage
python strack.py object1.o object2.o ...

## C++
C++ is not currently supported.  Challenges needing to be overcome in order to support C++ are listed below:  
 - Name-mangling (a result of supporting namespaces and method overloading)
   - .o files contain name-mangled symbols, while .su files contain non-name-mangled method names with arg lists and return types.
   - This is further complicated because the method names in the .su files have user-declared data types before typedefs are applied so just applying name-mangling algorithms to these will not match them to their name-mangled counterparts in the .o files.
 - Virtual methods
   - Runtime polymorphism through virtual methods requires the use of function pointers which are not friendly to static analysis.
   - Extra logic would have to be added to categorize virtual methods so when a generic virtual method shows up in a call graph the analyzer can make a worst case judgement based on the possible virtual methods being invoked there.
