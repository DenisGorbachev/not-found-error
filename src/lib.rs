use std::any::type_name;
use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct NotFoundError<T>(pub PhantomData<T>);

impl<T> NotFoundError<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }

    /// Convenience method to automatically convert the error to a result
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
/// See also: [`Require`](Require)
///
/// # Examples
///
/// ```
/// # use not_found_error::require;
/// # let items = [0, 1, 2];
/// let item = require(items.first());
/// ```
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
#[inline(always)]
pub fn not_found<T>() -> NotFoundError<T> {
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
