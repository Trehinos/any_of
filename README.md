# Any of

The project revolves around defining and implementing a versatile enum type, `AnyOf`, along with its utilities, which
provide a robust general-purpose sum type. Sum types are essential tools in functional programming as they allow for
explicit representation of values that could be one of several distinct cases.

## The `AnyOf` Enum

The `AnyOf` enum represents four possible cases:

- **Neither:** Neither of the types is present.
- **Either::Left:** The left type (`T`) is present.
- **Either::Right:** The right type (`U`) is present.
- **Both:** Both types (`T` and `U`) are present.

Conceptually, this can be represented as:

```
AnyOf<T, U> = Neither | Left(T) | Right(U) | Both(T, U)
```

This provides a significant degree of flexibility in modeling data, particularly for scenarios where you need to express
combinations of two types.

### Methods and Utilities

The methods provided for the `AnyOf` enum are designed to resemble those of the Rust standard library’s `Option` and
`Result` types. These include methods such as `unwrap`, which simplify working with the possible variants.

### The `Either` Struct

The project also includes the `Either` struct, a simpler sum type that represents two possibilities:

- **Left(T):** The left value.
- **Right(U):** The right value.

Unlike `AnyOf`, `Either` cannot represent the `Neither` or `Both` cases.

### Other types

* Provides the types `Couple<T, U> = (T, U)` and `Pair<T> = Couple<T, T>`,
* Provides the type `Both` internally used by `AnyOf`.

## Purpose of the Project

The main goal of this project is to enrich the Rust type system with precise and expressive types like `AnyOf` and
`Either`. These types allow to avoid ambiguity and write code that is safer and more intentional.

By providing clear and concise sum types, this project aims to empower developers to handle complex logical branches and
multiple states of data with ease, while leveraging the expressive power of Rust’s type system.

### Benefits

- Clear representation of combinations of values.
- Safer code through explicit handling of all possible states.
- Reusable utilities and methods that reduce boilerplate.

This project is ideal for users looking to handle advanced sum types effectively, offering flexibility and clarity for a
wide range of use cases in functional and type-driven Rust programming.

## Current status

Still in active development before 1.0.0.

## License

&copy; 2025 Sébastien Geldreich  
MIT License
