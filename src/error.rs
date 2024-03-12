//! This module contains the error types used by the library.

pub(crate) enum ErrorType<CE, PE> {
    Communication(CE),
    Pin(PE),
}
