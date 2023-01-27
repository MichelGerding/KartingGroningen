use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display(""))]
    AlreadyExistsError {},
    #[snafu(display(""))]
    InvalidNameError {},
    #[snafu(display(""))]
    FileDoesNotExistError {},
    #[snafu(display(""))]
    HeatNotFoundError {},
    #[snafu(display(""))]
    ConnectionError {},
    #[snafu(display(""))]
    PermissionDeniedError {}
}

pub type CustomResult<T, E = Error> = std::result::Result<T, E>;