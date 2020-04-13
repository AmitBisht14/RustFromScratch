// slices are similar to array, could be considered as a part of array for which size is not required to be specified
// to create a slice it should always be borrowed from the array else it will throw compile time error
pub fn access_slice() {
    let x: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
    analyze_slice(&x[2..5]);

    println!("local slice");
    let local_slice: &[i32] = &x[2..3];
    println!("size of slice: {}", local_slice.len());
    println!(
        "items of slice- start: {}, end: {}",
        local_slice[0],
        local_slice[local_slice.len() - 1]
    );
}

fn analyze_slice(slice: &[i32]) {
    println!("size of slice: {}", slice.len());
    println!(
        "items of slice- start: {}, end: {}",
        slice[0],
        slice[slice.len() - 1]
    );
}
