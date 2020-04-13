//to import a different .rs file
mod tuples;
//to use specific pub function from above import -- @1
use tuples::access_tuples;

mod arrays;
mod slices;
use arrays::access_array;
use slices::access_slice;
fn main() {
    // if skip @1 then out of scope function can be consumed in below way
    // tuples::access_tuples();
    println!("\r\ntuples:");
    access_tuples();
    println!("\r\narrays:");
    access_array();
    println!("\r\nslices:");
    access_slice();
}
