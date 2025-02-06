# AnyOf : a Versatile Algebraic Type for Rust

This project implements a flexible and expressive algebraic type, `AnyOf`, along with complementary structures,
to enhance Rust's type system for modeling data and state transitions.

It provides an advanced sum and product types, `AnyOf` which enables clear, safe,
and concise representations of data combinations in functional and type-driven programming.

## Overview

At the core of the project is the `AnyOf` enum, a general-purpose algebraic type, alongside additional types 
such as `Either` and `Both`. These abstractions allow developers to express dynamic states, optional values,
and branching logic in a natural and explicit manner.

### Key Types

1. **`AnyOf<L, R>`**
    - A flexible type that represents four possible states:
        - `Neither`: No value is present.
        - `Either::Left`: Only the left value is present.
        - `Either::Right`: Only the right value is present.
        - `Both`: Both values are present.
    - Conceptually, it combines variants in the following way:
      ```
      Both<L, R> = (L, R)
      Either<L, R> = Left(L) | Right(R)
      AnyOf<L, R> = Neither | Either<L, R> | Both<L, R>
      ```

2. **`Either<L, R>`**
    - A simple sum type representing one of two values.
    - Variants:
        - `Left(L)`
        - `Right(R)`
    - Ideal for binary decision-making.

3. **`Both<L, R>`**
    - A product type that pairs two values, `left` and `right`, of potentially different types.

4. **Enhanced Type Composition**
    - Complex types like `AnyOf4`, `AnyOf8`, and `AnyOf16` are introduced for handling larger, structured combinations via nested `AnyOf` structures.

### Features and Utilities

- Methods inspired by Rust's `Option` and `Result` types:
    - Creation utilities : `AnyOf::new`, `AnyOf::new_left`, `AnyOf::new_both`, etc.
    - State checks: `is_neither`, `is_left`, `is_both`, etc.
    - Transformations: `map_left`, `map_right`, `swap`, etc.
    - Unwrapping: `unwrap_left`, `unwrap_right`, `unwrap_both`.

- Flexible combinations :
    - Operators like `+` to merge and manipulate values in `AnyOf`.
    - Default value handling and state manipulation methods.

### Use Cases

`AnyOf` and its related types simplify dynamic state management and are well-suited for:

- Branching logic in functional programming.
- Handling optional or partial data.
- Implementing explicit and exhaustive handling of all potential states.
- Minimizing boilerplate for complex decision-making.

## Motivation

The project aims to enrich Rust's type system with expressive and flexible types for representing data combinations and states. 

With `AnyOf` and related abstractions, developers can achieve:
- **Clarity**: Models become easier to understand with precise combinations.
- **Safety**: Explicitly handle and exhaust all cases, avoiding runtime errors.
- **Reusability**: Utility functions reduce code repetition and foster cleaner APIs.

## Status

The library is under active development and is not yet versioned as `1.*.*`. Contributions and feedback are welcome!

## License

&copy; 2025 Sébastien Geldreich  
Distributed under the MIT License.