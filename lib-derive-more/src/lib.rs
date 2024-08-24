#![no_std]

#[derive(Debug, Eq, PartialEq, derive_more::Display, derive_more::Error)]
pub enum Error {
    #[display("Unable to write result")]
    WriteResult,
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub fn foo(a: i32, b: i32) -> Result<i32> {
    a.checked_add(b).ok_or_else(|| Error::WriteResult)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = foo(2, 2);
        assert_eq!(result, Ok(4));
    }
}
