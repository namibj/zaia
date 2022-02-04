# Zaia

Zaia is a dialect of the Lua language that is designed to be easy and flexible,
promoting integration and extensibility within modern software.

## Ethos

Zaia's goal is first and foremost to be easy to use and deploy. This means providing additional tools that
other Lua implementations does not provide such as convenient language functionality and specialized libraries
providing datastructures and algorithms.

## Where are we today?

Zaia currently provides a mostly compliant Lua 5.4 implementation that will run the majority of scripts.
It provides an intuitive and easy to use API makes extending the language and integrating it into Rust projects a breeze
and there is also a CLI tool available for running standalone Lua scripts from the terminal.

## Conformance

Zaia currently targets a base feature-set from Lua 5.4. We may support newer versions in the future.

We do not support the following Lua 5.4 features:
- `goto` statements and labels
- `\z` string literal escapes
- function calls without parentheses

## License

Zaia is licensed under the Apache v2.0 license.
See LICENSE for more information.
