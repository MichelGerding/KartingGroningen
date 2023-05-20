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
    NotFoundError {},
    #[snafu(display(""))]
    ConnectionError {},
    #[snafu(display(""))]
    PermissionDeniedError {},
    #[snafu(display(""))]
    DatabaseError {},
}

pub type CustomResult<T, E = Error> = Result<T, E>;
