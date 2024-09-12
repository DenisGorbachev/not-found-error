<!-- DO NOT EDIT -->
<!-- This file is automatically generated by README.ts. -->
<!-- Edit README.ts if you want to make changes. -->

# Convert Option to Result

[![Build](https://github.com/DenisGorbachev/not-found-error/actions/workflows/ci.yml/badge.svg)](https://github.com/DenisGorbachev/not-found-error)
[![Documentation](https://docs.rs/not-found-error/badge.svg)](https://docs.rs/not-found-error)

## Overview

```rust
// Convert Option<i32> to Result<i32, NotFoundError<i32>>

assert_eq!(Some(10).require(), Ok(10));

assert_eq!(None.require(), Err(NotFoundError::<i32>::new()));
```

This crate provides a generic `NotFoundError<T>` type and associated
utilities for handling “not found” scenarios in a type-safe and ergonomic manner.

You can convert `Option<T>` to `Result<T, NotFoundError<T>` using [`require`][__link0] function or [`Require`][__link1] extension trait.

You can convert `Option<T>` to `Result<T, NotFoundError<AnotherType>` using [`not_found`][__link2] function or [`OkOrNotFound`][__link3] extension trait.

## Features

* [x] Generic `NotFoundError<T>` type
* [x] Conversion functions and traits to transform `Option<T>` into `Result<T, NotFoundError<T>>`
* [x] Conversion functions and traits to transform `Option<T>` into `Result<T, NotFoundError<AnotherType>>`

## Examples

```rust
use not_found_error::{NotFoundError, Require, locate, require};

// Using the `require` function
let item = require([1, 2, 3].into_iter().next());
assert_eq!(item, Ok(1));

// Using the `require` function
let item = require([].into_iter().next());
assert_eq!(item, Err(NotFoundError::<i32>::new()));

// Using the `require` extension method
let item = [1, 2, 3].into_iter().next().require();
assert_eq!(item, Ok(1));

// Using the `require` extension method
let item = [].into_iter().next().require();
assert_eq!(item, Err(NotFoundError::<i32>::new()));

// Try to find a number greater than 10 (which doesn't exist in our list)
let numbers = &[1, 2, 3];
let result = locate(numbers, |&&n| n == 0);
assert_eq!(result, Err(NotFoundError::new()));
```

   [__cargo_doc2readme_dependencies_info]: ggGkYW0BYXSEGyMws-dKI-LpG9swkVXG-rikGwSuJGhB0NVbG974QPrPJF6XYXKEGwRzzb7naNYKG2gdwYR7DdCLG0pVq38AfRxlG0D7v_C8zAyOYWSBg29ub3QtZm91bmQtZXJyb3JlMC4yLjJvbm90X2ZvdW5kX2Vycm9y
 [__link0]: https://docs.rs/not-found-error/latest/not_found_error/?search=require
 [__link1]: https://docs.rs/not-found-error/latest/not_found_error/trait.Require.html
 [__link2]: https://docs.rs/not-found-error/latest/not_found_error/?search=not_found
 [__link3]: https://docs.rs/not-found-error/latest/not_found_error/trait.OkOrNotFound.html


## Installation

```shell
cargo add not-found-error
```

## Gratitude

Like the project? [⭐ Star this repo](https://github.com/DenisGorbachev/not-found-error) on GitHub!

## License

[Apache License 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
