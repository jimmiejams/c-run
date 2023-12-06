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

### AST

Notes on ASTs.

#### Optimising ASTs

https://stackoverflow.com/questions/1825298/your-favourite-abstract-syntax-tree-optimization

> Mostly you can't do interesting optimizations at the AST level, because you need information how how data flows from one part of the program to another. While data flow is implicit in the meaning of the AST, it isn't easily determined by inspecting just the AST, which is why people building compilers and optimizers build other program representations (including symbol tables, control flow graphs, reaching definitions, data flow and SSA forms, etc.).
>
> Having a parser for a language is the easy part of analyzing/manipulating that language. You need all that other stuff to do a good job.
>
> If you do have all those other representations, you can think about doing optimizations at the AST level. Most folks building compilers don't bother; they convert to a data flow representation and simply optimize that. But if you want to reproduce source code with changes, you need the AST. You'll also need a prettyprinter to enable you to regenerate the source code. If you go this far, you'll end up with a source-to-source program transformation system.
> 
> The DMS Software Reengineering Toolkit is a system that transforms ASTs, using all these other representations to enable the analyses needed by the transforms.

Counterpoint:

> An optimisation that is easiest to do on the AST (rather than, say, the CFG) is tail-call optimisation: if you see a subtree of the form:
>
>     RETURN
>         CALL f
>             ARGS x, y, ...
>
> You can replace it with a jump to f. If f(a, b) is the function that the tail-call appears in, the replacement is as simple as:
>
>     a = x; b = y
>     JUMP to root of tree
>
> I find it easiest to represent that jump as a special "restart" statement, which the AST->CFG translation treats as an edge back to the first node. Jumping to other functions is a bit trickier since you can't just set local variables, you need to actually think ahead how arguments are passed to them and prepare to translate this at a lower level. For example, the AST might need a special node that can instruct a later pass to overwrite the current stack frame with the arguments and jump accordingly.


https://2022.programming-conference.org/details/MoreVMs-2022-papers/3/Less-Is-More-Merging-AST-Nodes-To-Optimize-Interpreters

> The Truffle framework allows language implementations to reach state-of-the-art run time performance while only providing an abstract syntax tree (AST) interpreter; the AST is compiled to machine code using GraalVM’s JIT compiler.

### VMs

Simple Object Machine -- http://som-st.github.io/
* a minimal Smalltalk for teaching and research on VMs

TruffleSOM -- https://github.com/smarr/TruffleSOM
* SOM on Truffle

Truffle -- https://github.com/oracle/graal/tree/master/truffle
* framework for implementing languages on GraalVM

### Benchmarking

Are We Fast Yet? benchmark -- https://github.com/smarr/are-we-fast-yet