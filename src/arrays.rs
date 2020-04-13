// An array is a collection of objects of the same type T, stored in contiguous memory
// syntax [type: size]

pub fn access_array() {
    let a: [i32; 4] = [1, 2, 3, 4];
    //arrays are 0 index based
    println!("first index: {}, last index: {}", a[0], a[3]);

    //initializing all with same value
    let b: [i32; 500] = [0; 500];
    println!(
        "initializing all with same value: {}, {}, {}",
        b[0], b[250], b[499]
    );

    println!("size of array: {}", b.len());
}
