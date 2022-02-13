# Intermediate representation (IR)

The IR is an intermediate representation of the code that lies between the AST and bytecode steps of the
compilation pipeline. The IR is structured to be a suitable target for running various checks and optimization algorithms.

During the IR step there are three primary things going on, type inference, type checking and optimization.
But first however, we need to outline the properties of this representation.

The IR combines multiple approaches from recent advancement in compiler theory and is designed to be a one-stop shop for most transformations. Traditional compilers usually use multiple IR's instead of just one. Zaia only uses a single IR to improve compiler latency and throughput. The classical approach works great for AoT-style compilation, but this approach is not ideal for dynamic language runtimes like Zaia.

The IR is based on a [Sea-of-Nodes](https://darksi.de/d.sea-of-nodes/) like structure which combines useful properties from control flow graphs and data flow graphs. The IR is organized into basic blocks, each basic block being a block non-branching statements which may have one branch at the end. To facilitate common optimizations, the IR is also in SSA form. This means that a variable is only assigned to once and reassignments are translated into new variabless.

## Type inference 

## Type checking

## Optimization

As mentioned earlier, the IR is basde on [Sea-of-Nodes](https://darksi.de/d.sea-of-nodes/) which combines properties of classical CFGs and DFGs. These graphs are used to implement most optimizations in modern compilers and it serves as a base in Zaia for common optimizations like inlining and constant evaluation.

On top of this graph lies a virtual IR similar to program expression graphs used in the concept of [Equality Saturation](https://rosstate.org/publications/eqsat/). This is a novel approach that relies on recent advancements to allow us to implement a basic set of rules that are explored to generate complex emergent optimizations without the need for explicitly defining complex optimization passes.

To scale equality saturation to a complex optimizations and real-time graphs, Zaia utilizes techniques from [Sketch-Guided Equality Saturation](https://arxiv.org/abs/2111.13040) to augment the equality saturation approach.
