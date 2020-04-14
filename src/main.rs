//to import a different .rs file
//it is important to specify each mod as pub else we won't be able to access it
//nesting will be done as per the folder structure and similar was done while using it
mod primitives {
    pub mod arrays;
    pub mod slices;
    pub mod tuples;
}
mod custom_types {
    pub mod structs;
}
//to use specific pub function from above import -- @1
use custom_types::structs::access_structs;
use primitives::arrays::access_array;
use primitives::slices::access_slice;
use primitives::tuples::access_tuples;

fn main() {
    // if skip @1 then out of scope function can be consumed in below way
    // tuples::access_tuples();
    println!("\r\ntuples:");
    access_tuples();
    println!("\r\narrays:");
    access_array();
    println!("\r\nslices:");
    access_slice();
    println!("\r\nstructs:");
    access_structs();
}
