# Zaia

Zaia is an implementation of the Lua language that is designed to be easy and flexible,
promoting integration and extensibility within modern software.

## Ethos

Zaia's goal is first and foremost to be easy to use and deploy. This means providing additional tools that
other Lua implementations does not provide such as convenient language functionality and specialized libraries
providing datastructures and algorithms.

Performance does matter, but it is of secondary priority in contrast to the above goals as they are often
the limiting factors in adoption and widespread use.

## Conformance

Zaia currently targets Lua 5.4. We expect to support newer versions as they come out
with possible support for older versions if breaking changes are made to the reference.

We do not support the `goto` functionality included with PUC-Rio Lua for ethical reasons.

We use parts of the PUC-Rio test suite in combination with custom tests to ensure that the implementation is correct.
