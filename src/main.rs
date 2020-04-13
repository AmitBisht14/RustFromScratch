//to import a different .rs file
mod tuples;
//to use specific pub function from above import -- @1
use tuples::access_tuples;
fn main() {
    // if skip @1 then out of scope function can be consumed in below way
    // tuples::access_tuples();
    access_tuples();
}
