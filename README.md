# Zaia

Zaia is an implementation of the Lua language that is designed to be easy and flexible,
promoting integration and extensibility within modern software.

## Ethos

Zaia's goal is first and foremost to be easy to use and deploy. This means providing additional tools that
other Lua implementations does not provide such as convenient language functionality and specialized libraries
providing datastructures and algorithms.

Performance does matter, but it is of secondary priority in contrast to the above goals as they are often
the limiting factors in adoption and widespread use.

## Where are we today?

Zaia currently provides a mostly compliant Lua 5.4 implementation that will run the majority of scripts.
It provides an intuitive and easy to use API makes extending the language and integrating it into Rust projects a breeze
and there is also a CLI tool available for running standalone Lua scripts from the terminal.

## Long term goals

- Extend the Lua language in meaningful ways with new features and built-in libraries.
- Provide acceptable performance for the majority of use cases with a bytecode virtual machine.
- Provide a widely accepted source code formatter.
- Provide a widely accepted source code linter.
- Provide bindings to C and C++.

## Conformance

Zaia currently targets Lua 5.4. We expect to support newer versions as they come out
with possible support for older versions if breaking changes are made to the reference.

We always require parenthesis for function calls to promote consistent and readable code.
We do not support the `goto` functionality included with PUC-Rio Lua for ethical reasons.

We use parts of the PUC-Rio test suite in combination with custom tests to ensure that the implementation is correct.
