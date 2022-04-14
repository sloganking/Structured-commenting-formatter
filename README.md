# Structured commenting formatter (scfmt)

``scfmt`` is a formatter designed to enable [structured commenting](https://github.com/sloganking/Structured-Commenting) in your code. Running ``scfmt`` after traditional code formatters, will ensure your structured comments are indented correctly and not mangled. This repository provides both a CLI for quickly formatting files, and a rust library for adding scfmt support to other projects.

## Quick Links

[FAQ](./FAQ.md)


## Features

### Formatting
``scfmt`` will format strings with bracketed structured comments, so that their contents inside them are correctly indented.

### Adding Brackets
Adding brackets converts the bracketless method of structured commenting to the bracketed version. Strucutred comments must be bracketed before being run through the formatter, or else no change will take effect.

### Removing Brackets
Removing brackets converts the bracketed method of structured commenting to the bracketless version. Bracketless structured comments are less verbose, but have no method of being recovered if their whitespace gets messed up. Say after running them through a traditional code formatter.

### Nullifying Brackets
Nullifying brackets puts a ``_`` character in front of any existing bracketed comments. This allows for more easily converting existing projects to being compatible with structured commenting.


## What is scfmt allowed to do?

The only things ``scfmt`` is allowed to do are: 
- Create and delete ``//<`` closing comments
- Add and remove brackets ``>``, ``<>`` to the begining of existing comments.
- Ensure empty lines are depth 0
- Edit the indentation of lines for SC formatting.

What will scfmt not do?
- Turn one line into many
- Turn many lines into one
- Ensure a ``max_width``
- Add or remove empty lines

## Noteworthy bugs

- Per [issue #1](https://github.com/sloganking/Structured-commenting-formatter/issues/1), ``scfmt`` will mistake lines in multi-line strings or multi-line comments, as strucutred comments if those lines start with your language's comment starter. This means multi-line strings and comments such as the one below would get formatted, even though they should not be.

```rust
/*
//>
comment line that will get indented after formatting
//<
*/

let multi_line_str = "
//>
string line that will get indented after formatting
//<
"

```
