# Changelog

# `v1` Release phase

## `v1.4` EitherOf/BothOf 

Public type and case naming consistency.

- Updated test cases in `test_either.rs` to use `EitherOf` instead of `Either`.
- Renamed the types `Both` to `BothOf` and `Either` to `EitherOf` in `src/lib.rs` and `src/both.rs`.
- Renamed the cases `BothOf` to `Both` and `EitherOf` to `Either` of the exported cases of the `AnyOf` type.
- Modified `README.md` to reflect the updated naming convention.
- Refactored associated documentation, examples, and comments across the codebase.

## `v1.3` 

## `v1.3.4` Shorthand variants

- Incorporated new shorthand variants, `Left`, `Right`, `Neither`, `EitherOf`, and `BothOf`, and where applicable for cleaner syntax.

## `v1.3.3` Tests

- Add missing tests for `AnyOf`.

## `v1.3.2` From

`From` and `Into` implementations and tests.

- Added the `conversions` module with type conversion implementations.
- Exported the `conversions` module in `src/lib.rs`.
- Updated `doc/types.plantuml` to reflect new struct member additions.
- Add dedicated test modules for `Either`, `Both`, and `AnyOf`.

## `v1.3.1` Any

`Any`, `from_any` and `from_either`.

- Set CI badge to reference the 'ci' branch.
- Rename `AnyOf` to `LeftOrRight` in README.
- Added `types.plantuml` and `any_of-type-diagram.png`.
- Updated `README.md` with references to the diagram.
- Added `Any<T, U>` type alias in `concepts.rs`.
- Introduced `from_any` and `from_either` methods to `AnyOf`.
- Updated `Cargo.toml` and `Cargo.lock` to version `1.4.0-dev`.
- Added `is_either` method in `src/lib.rs`.
- Deprecated `is_one` method with a deprecation notice.
- Updated `Swap::Output` type definition in `doc/types.plantuml`.
- Expanded `AnyOf4`, `AnyOf8`, and `AnyOf16` classes with structured accessor methods in `doc/types.plantuml`.

### `v1.3.0` Traits

`Swap`, `Map` and `Unwrap`.

- Fixed typo: "additional the types" -> "additional types".
- Adjusted line breaks for better readability.
- Unified formatting of code examples and lists.
- Updated project status to clarify termination and semantic versioning policy.
- Added `Swap` trait and implementations for `Both`, `Either` and `AnyOf`.
- Added `Map` trait and implementations for `Both`, `Either`, and `AnyOf`.
- Added `Unwrap` trait and implementations for `Both`, `Either`, and `AnyOf`.
- Updated documentation and examples for the newly added traits.
- Adjusted imports across: `src`, `tests.rs`, and `lib.rs`.
- Specify types in `Either` example definitions.

## `v1.2` Abstraction

`Not` and `LeftOrRight`.

- Imported `core::ops::Not` in both `both.rs` and `either.rs`.
- Added `Not` trait implementation for `Both<L, R>` in `both.rs`.
- Added `Not` trait implementation for `Either<L, R>` in `either.rs`.
- Add explicit type annotation in doc example.
- Added GitHub Actions CI badge to the README.
- Updated an example in `src/any_of_x.rs` to use `AnyOf4` instead of `AnyOf8`.
- Adjusted example to use `new_left` constructor and simplified method call.
- Fix incorrect assertions in map_right examples.
- Fixed test assertion in code documentation to check `is_both`.
- Removed unnecessary module qualifiers in `src/lib.rs` imports.
- - Added `LeftOrRight` trait definition in `src/concepts.rs`.
- Implemented `LeftOrRight` for `Either`, `Both`, and `AnyOf`.
- Removed redundant methods from `Either` and `AnyOf`.
- Adjusted imports and updated references in documentation and examples.
- Updated README.md to reflect the new `LeftOrRight` trait feature.

## `v1.1` Fix operators

`swap()` and operators `&`, `|` and `!`.

- Added the `swap` method in `Both` with documentation and examples.
- Simplified `map` implementation for `Either` in `AnyOf`.
- Simplified `swap` implementation for `Either` and `Both` in `AnyOf`.
- Improved import statement in documentation/example.
- Replaced `+`, `-`, and `!` operators with `&`, `|`, and `!` respectively.
- Updated related method names and documentation to match new operators.
- Adjusted `impl` traits (`Add`, `Sub`, `Neg`) to `BitAnd`, `BitOr`, and `Not`.
- Enhanced README to explain new operator semantics and improve clarity.
- Added `rust-patterns` category and `left_right`, `ADT` keywords in `Cargo.toml`.

## `v1.0` Release

- Added `Sub` operator (`-`) by implementing `filter` logic.
- Added `Neg` operator (`-`) for swapping `AnyOf` components.
- Simplified `from_either` and `into_either` using direct assignments.
- Reordered import statements.
- Improved code documentation for `filter`, `swap`, and operator use cases.
- Introduced `map` method in `Both` to transform paired values.
- Modified `AnyOf`'s `map` implementation to use `Both::map`.
- Updated comments and examples.
- Added `tests` module under conditional compilation in `lib.rs`.
- Created `src/tests.rs` to house the new unit tests.
- Defined a helper function `mul2` for testing purposes.
- Implemented and verified three test cases for `AnyOf`: both values, right value only, and neither value present.
- Enabled `crate::` imports for the `tests` module.

# `v0` Development phase

## `v0.11` RC

### `v0.11.0`

* Refactor the `any_of_x` module.
* Updated the description of `AnyOf` to "sum of product type" for precision in README.

## `v0.10` Beta

### `v0.10.2`

* Refine README for clarity and consistency

### `v0.10.1`

* Add detailed documentation for `Add` implementation in `AnyOf`.
* Completed documentation of `AnyOf`, `Both`, and `Either` types. Expanded examples, added missing functions.

### `v0.10.0`

* Correct the `Add` trait implementation to properly return `Self` instead of `()`.

## `v0.9` Alpha

### `v0.9.2`

* Removed optional `std` feature and enforce `no_std` globally.
* Removed the conditional compilation for the `Add` implementation on `AnyOf`, making it always available.
* Documentation for the `Both` struct and the `Either` enum.

### `v0.9.1`

* Move the `Both` struct representing paired values and move its implementation from `either.rs` to a dedicated `both.rs` module.
* Update `lib.rs` to include crate-level documentation, examples, and support for the `Both` variant in the `AnyOf` enum.

### `v0.9.0`
Introduces the `any_of` crate for handling flexible enums: `Neither`, `Either`, and `Both`, 
along with nested types such as `AnyOf4`, `AnyOf8`, and `AnyOf16`.

* Includes core functionality, type definitions, and utility methods, supporting `#![no_std]`.  
* Added `Cargo.toml`, `Cargo.lock`, and `.gitignore` for management and build configuration.
* Added a conditional `Add` implementation for the `AnyOf` enum when the `std` feature is enabled.
* Included a `README.md`, `LICENSE`, and clarified project details, marking it as actively developing towards 1.0.0.