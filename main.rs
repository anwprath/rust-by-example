use std::fmt;

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    let mut arr: [i32; 5] = [0; 5];
    let arr2 = [-1; 5];

    println!("address: {:?} ; {:?}", arr.as_ptr(), arr);
    println!("address: {:?} ; {:?}", arr2.as_ptr(), arr2);

    arr = arr2;
    println!("address: {:?} ; {:?}", arr.as_ptr(), arr);
    println!("address: {:?} ; {:?}", arr2.as_ptr(), arr2);

    analyze_slice(&arr);

    for i in 0..(arr.len()+1) {
        match arr.get(i) {
            Some(val) => println!("{}", val),
            None => println!("Slow down! {} is too far!", i),
        }
    }
}
