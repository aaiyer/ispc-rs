#[macro_use]
extern crate ispc;

// Import the generated bindings to our ISPC functions in
// as the module simple, note that all of the functions will be
// unsafe
ispc_module!(simple);

fn main() {
    let a = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let b = vec![5.0, 4.0, 3.0, 2.0, 1.0];
    let mut result = vec![0.0; a.len()];
    println!("a = {:?}\nb = {:?}", a, b);
    unsafe {
        // We use the generated bindings in the simple module to call
        // our ISPC function add_lists
        simple::add_lists(a.as_ptr(), b.as_ptr(),
                        result.as_mut_ptr(), a.len() as i32);
    }
    println!("a + b = {:?}", result);
}

