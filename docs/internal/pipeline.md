# Compilation Pipeline

Zaia is targeted to towards maximizing runtime performance. The intended use case is for embedding within
other programs and running scripts for common operations very often. Due to this goal, Zaia features a complex compilation pipeline that is longer but facilitates more optimizations.

There are two primary pipelines, the initial pipeline that transforms source code into bytecode, this is designed to complete relatively quickly as to not waste time on compiling code that is rarely executed. The second (to be implemented) pipeline transforms bytecode into machine code with a JIT. This pipeline only executes for high-impact code that runs frequently.

Please see the index for the internal documentation for links detailing the various stages and representations used.

## Interpreter Pipeline

```
+---------------------+
|                     |
|     Source Code     |
|                     |
+---------------------+

           ↓

+------------------------------+
|                              |
|     Concrete Syntax Tree     |
|                              |
+------------------------------+

       ↓

+-------------+
|             |
|     HIR     |
|             |
+-------------+

        ↓

+------------------+
|                  |
|     Bytecode     |
|                  |
+------------------+
```

## JIT Pipeline
