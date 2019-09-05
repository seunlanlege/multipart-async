// Copyright 2017 `multipart-async` Crate Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
use std::borrow::Cow;
use std::fmt;
use std::mem;
use std::str::Utf8Error;

pub use futures_core::*;
use std::task::Poll::{self, *};

pub use crate::helpers::*;

use super::{Error, Result};
use std::convert::Infallible;

pub fn ready_ok<R, T, E>(val: T) -> Poll<R>
where
    R: From<Result<T, E>>,
{
    Poll::Ready(Ok(val).into())
}

pub fn error<T, E, S: Into<String>>(e: S) -> Result<T, E> {
    Err(Error::Parsing(e.into()))
}

pub fn ready_err<T, E, S: Into<String>>(e: S) -> Poll<Result<T, E>> {
    Poll::Ready(Err(Error::Parsing(e.into())))
}

pub fn utf8_err<T, E>(e: Utf8Error) -> Result<T, E> {
    Err(Error::Parsing(e.to_string()))
}

