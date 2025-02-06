# AnyOf : a versatile type for Rust

This project implements a flexible [Algebraic Data Type](https://en.wikipedia.org/wiki/Algebraic_data_type) : `AnyOf`.

`AnyOf` is an optional sum of product type of two types, which enables **clear and safe** data representations
in functional and type-driven programming.

## Overview

At the core of the project is the `AnyOf` enum, a general-purpose algebraic type,
alongside additional types such as `Either` and `Both`.

These abstractions allow to express dynamic states, optional values, and branching logic in a natural and explicit manner.

### Key Types

1. **`AnyOf<L, R>`**
    - A flexible type that represents four possible states:
        - `Neither`: No value is present.
        - `Either::Left`: Only the left value is present.
        - `Either::Right`: Only the right value is present.
        - `Both`: Both values are present.
    - Conceptually, it combines variants in the following way:
      ```
      AnyOf<L, R> = Neither | Either<L, R> | Both<L, R>
      ```

2. **`Either<L, R>`**
    - A simple sum type representing one of two values.
    - Variants:
        - `Left(L)`
        - `Right(R)`
    - Ideal for binary decision-making.
    - Conceptually, it is the type :
      ```
      Either<L, R> = Left(L) | Right(R)
      ```

3. **`Both<L, R>`**
    - A product type that pairs two values, `left` and `right`, of potentially different types.
    - Conceptually, it is the type :
      ```
      Both<L, R> = (L, R)
      ```

4. **Enhanced Type Composition**
    - Complex types like `AnyOf4`, `AnyOf8`, and `AnyOf16` are implemented for handling larger, 
    structured combinations via nested `AnyOf` structures.

### Features and Utilities

- Methods inspired by Rust's `Option` and `Result` types:
    - Creation utilities : `new`, `new_left`, `new_both`, etc.
    - State checks: `is_neither`, `is_left`, `is_both`, etc.
    - Transformations: `map_left`, `map_right`, `swap`, etc.
    - Unwrapping: `unwrap_left`, `unwrap_right`, `unwrap_both`.

- Flexible combinations :
    - Operators like `+` to merge, or `-` to filter or swap values in `AnyOf`.
    - Default value handling and state manipulation methods.

### Use Cases

`AnyOf` and its related types simplify dynamic state management and are well-suited for:

- Branching logic in functional programming.
- Handling optional or partial data.
- Implementing explicit and exhaustive handling of all potential states.
- Minimizing boilerplate for complex decision-making.

## Motivation

The project aims to enrich Rust's type system with expressive and flexible types 
for representing data combinations and states.

* Unlike the Rust's `Result` type, the types `Either` or `AnyOf` has no error semantic,
* `AnyOf<L, R>` can be also be viewed as two options : `(Option<L>, Option<R>)` (a product type of sum type),

## Status

The library is still under development, but it follows semantic versioning.

[Feedback is welcome](mailto:dev-any-of@trehinos.eu)!

## License

&copy; 2025 Sébastien Geldreich  
Distributed under the MIT License.