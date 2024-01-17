# C-run little C compiler and interpreter

## Design

The overall structure of C-run is:
* front end
  * start of the pipeline 
  * parser
  * AST optimiser
* back end
  * generic back end interface
  * LLVM interface
  * interpreter (from AST)
  * other bytecode e.g. JVM or GraalVM
* driver
  * reads command line arguments
  * instantiates the required front end and back end
  * starts the pipeline (front end)
* common structures
  * AST

## Frontend

### Parser


### Language and grammar

The aim is to create a simplified C grammar.  In particular:
* it will have no C preprocessor
* there will be no automatic type coercing (but there will be explicit coercion)
* there will be pointers
* standard types will be i8, i16, i32, i64 (and unsigned versions), bool, char

Questions:
* how will we do forward references (inside the compilation unit we will just do two passes)
* how will we do external references, especially with no header files (we'll need a db of symbols defined by other files inside the same workspace)
* how will file i/o work without FILE* (which requires the CPP)
* will we support varargs (for printf)
* what will the runtime be like?  will it be general purpose, or embedded into another program?

### AST

### VMs

### Benchmarking

## Backend

### LLVM

### Cranelift