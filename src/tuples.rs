//A tuple is a collection of values of different types.
//Syntax (T1, T2, T3.... Tn), where T can be any type

//method should be named in snake_case
// function syntax fn fn_name(param_name: param_type ) -> return_type { }
fn reverse_tuple(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

//method should be named in snake_case
pub fn access_tuples() {
    let random_tuple: (i32, bool) = (10, false);

    println!("{}, {}", random_tuple.0, random_tuple.1);
    let _x: (bool, i32) = reverse_tuple(random_tuple);

    println!("{}, {}", _x.0, _x.1);
    // to print whole tuple, but long tuples cannot be printed
    println!("{:?}", _x);

    //tuples can be assigned to variables
    let (a, b) = _x;
    println!("{} , {}", a, b);
}
