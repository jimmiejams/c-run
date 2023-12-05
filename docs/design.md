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
  * interpreter
* driver
  * reads command line arguments
  * instantiates the required front end and back end
  * starts the pipeline (front end)
* common structures
  * AST