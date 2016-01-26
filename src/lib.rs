extern crate bufstream;
extern crate byteorder;

mod traits;
#[macro_use]
mod macros;
mod v5;
mod v6;
mod v7;
mod v8;
mod v9;
mod generic;
mod netflow;

#[test]
fn it_works() {
}
