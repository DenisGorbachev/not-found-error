//! # Overview
//!
//! ```
//! # use not_found_error::{NotFoundError, Require};
//! assert_eq!(Some(10).require(), Ok(10));
//!
//! assert_eq!(None.require(), Err(NotFoundError::<i32>::new()));
//! ```
//!
//! This crate provides a generic `NotFoundError<T>` type and associated
//! utilities for handling "not found" scenarios in a type-safe and ergonomic manner.
//!
//! You can convert `Option<T>` to `Result<T, NotFoundError<T>` using [`require`](require) function or [`Require`](Require) extension trait.
//!
//! You can convert `Option<T>` to `Result<T, NotFoundError<AnotherType>` using [`not_found`](not_found) function or [`OkOrNotFound`](OkOrNotFound) extension trait.
//!
//! ## Features
//!
//! * [x] Generic `NotFoundError<T>` type
//! * [x] Conversion functions and traits to transform `Option<T>` into `Result<T, NotFoundError<T>>`
//! * [x] Conversion functions and traits to transform `Option<T>` into `Result<T, NotFoundError<AnotherType>>`
//!
//! ## Examples
//!
//! ```
//! use not_found_error::{NotFoundError, require, Require};
//!
//! let item = require([1, 2, 3].into_iter().next());
//! assert_eq!(item, Ok(1));
//!
//! let item = require([].into_iter().next());
//! assert_eq!(item, Err(NotFoundError::<i32>::new()));
//!
//! // Using the `require` extension method
//! let item = [1, 2, 3].into_iter().next().require();
//! assert_eq!(item, Ok(1));
//!
//! let item = [].into_iter().next().require();
//! assert_eq!(item, Err(NotFoundError::<i32>::new()));
//! ```

use std::any::type_name;
use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

/// Represents an error indicating that a value was not found.
///
/// This struct is generic over the type `T` that was not found.
///
/// # Examples
///
/// ```
/// use not_found_error::NotFoundError;
///
/// let error: NotFoundError<i32> = NotFoundError::new();
/// assert_eq!(error.to_string(), "i32 not found");
/// ```
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct NotFoundError<T>(pub PhantomData<T>);

impl<T> NotFoundError<T> {
    /// Creates a new `NotFoundError`.
    ///
    /// # Examples
    ///
    /// ```
    /// use not_found_error::NotFoundError;
    ///
    /// let error: NotFoundError<String> = NotFoundError::new();
    /// ```
    pub fn new() -> Self {
        Self(PhantomData)
    }

    /// Convenience method to automatically convert the error to a result.
    ///
    /// # Examples
    ///
    /// ```
    /// use not_found_error::NotFoundError;
    ///
    /// let result: Result<i32, NotFoundError<i32>> = NotFoundError::result();
    /// assert!(result.is_err());
    /// ```
    pub fn result<Err: From<Self>>() -> Result<T, Err> {
        Err(Self::new().into())
    }
}

impl<T> Default for NotFoundError<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> std::fmt::Display for NotFoundError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} not found", type_name::<T>())
    }
}

impl<T: Debug> Error for NotFoundError<T> {}

/// Converts `Option<T>` to `Result<T, NotFoundError<T>>`
///
/// # Examples
///
/// ```
/// # use not_found_error::require;
/// # let items = [0, 1, 2];
/// let item = require(items.first());
/// ```
///
/// # See also
///
/// - [`Require`]: Trait for converting `Option<T>` to `Result<T, NotFoundError<T>>`
/// - [`OkOrNotFound`]: Trait for converting `Option<T>` to `Result<T, NotFoundError<AnotherType>>`
#[inline(always)]
pub fn require<T>(option: Option<T>) -> Result<T, NotFoundError<T>> {
    option.ok_or(NotFoundError(PhantomData))
}

/// A shorter version of `NotFoundError::new()`.
///
/// Useful in places where you need to convert `Option<T>` into `Result<T, NotFoundError<AnotherType>>` (notice that `T != AnotherType`).
///
/// # Examples
///
/// ```
/// # use std::path::Path;
/// # use not_found_error::{not_found, NotFoundError};
/// # pub struct WorkspaceRoot;
/// pub fn get_root(path: &Path) -> Result<&Path, NotFoundError<WorkspaceRoot>> {
///      find_root(path).ok_or(not_found())
/// }
/// # pub fn find_root(path: &Path) -> Option<&Path> { todo!() }
/// ```
///
/// # See also
///
/// - [`require`]: Function to convert `Option<T>` to `Result<T, NotFoundError<T>>`
/// - [`OkOrNotFound`]: Trait for converting `Option<T>` to `Result<T, NotFoundError<AnotherType>>`
#[inline(always)]
pub fn not_found<AnotherType>() -> NotFoundError<AnotherType> {
    NotFoundError(PhantomData)
}

/// An extension trait for `Option<T>` to convert it to `Result<T, NotFoundError<T>>`
///
/// # Examples
///
/// ```
/// # use not_found_error::Require;
/// # let items = [0, 1, 2];
/// let item = items.first().require();
/// ```
///
/// # See also
///
/// - [`require`]: Function to convert `Option<T>` to `Result<T, NotFoundError<T>>`
/// - [`OkOrNotFound`]: Trait for converting `Option<T>` to `Result<T, NotFoundError<AnotherType>>`
pub trait Require {
    type T;

    fn require(self) -> Result<Self::T, NotFoundError<Self::T>>;
}

impl<T> Require for Option<T> {
    type T = T;

    #[inline(always)]
    fn require(self) -> Result<Self::T, NotFoundError<Self::T>> {
        self.ok_or(NotFoundError(PhantomData))
    }
}

/// An extension trait for `Option<T>` to convert it to `Result<T, NotFoundError<AnotherType>>`
///
/// Useful in places where you need `NotFoundError<AnotherType>` instead of `NotFoundError<T>`.
///
/// # Examples
///
/// ```
/// # use std::path::Path;
/// # use not_found_error::{NotFoundError, OkOrNotFound};
/// # pub struct WorkspaceRoot;
/// pub fn get_root(path: &Path) -> Result<&Path, NotFoundError<WorkspaceRoot>> {
///      find_root(path).ok_or_not_found()
/// }
/// # pub fn find_root(path: &Path) -> Option<&Path> { todo!() }
/// ```
///
/// # See also
///
/// - [`Require`]: Trait for converting `Option<T>` to `Result<T, NotFoundError<T>>`
/// - [`require`]: Function to convert `Option<T>` to `Result<T, NotFoundError<T>>`
pub trait OkOrNotFound {
    type T;

    fn ok_or_not_found<B>(self) -> Result<Self::T, NotFoundError<B>>;
}

impl<T> OkOrNotFound for Option<T> {
    type T = T;

    #[inline(always)]
    fn ok_or_not_found<B>(self) -> Result<Self::T, NotFoundError<B>> {
        self.ok_or(NotFoundError(PhantomData))
    }
}
