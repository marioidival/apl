use super::primitive::Primitive::*;
use super::object::Object;
use super::error::Error;

use std::io::prelude::*;
use std::io::{stdin, stdout};


type Result<T> = ::std::result::Result<T, Error>;


pub(crate) fn print(data: Object) -> Result<Object> {
    match data {
        Object::Primitive(Integer(n)) => println!("{}", n),
        Object::Primitive(Str(s)) => println!("{}", s),
        Object::Primitive(Boolean(b)) => println!("{}", b),
        Object::Primitive(Float(f)) => println!("{}", f),
        _ => return Err(Error::OtherError("invalid argument".into()))
    }

    stdout().flush().expect("flush");
    Ok(Object::Unit)
}

pub(crate) fn input() -> Result<Object> {
    let mut buf = ::std::string::String::new();

    stdin()
        .read_line(&mut buf)
        .map_err(|e| {
            eprint!("cannot read stdin: {}", e);
            Error::OtherError("cannot read stdin".into())
        })
        .and_then(|_| {
            Ok(Object::Primitive(Str(buf.trim_right_matches("\r\n").into())))
        })
}