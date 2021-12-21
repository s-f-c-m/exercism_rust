/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    // unimplemented!()
    Vec::new()
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    // unimplemented!("create a zeroized buffer of {} bytes", count)
    // let mut vec: Vec<u8> = Vec::new();
    // for _i in 0..count {
    //     vec.push(0)
    // }
    let vec = vec![0; count];
    vec
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    // unimplemented!()
    let mut vec: Vec<u8> = Vec::new();
    vec.push(1);
    vec.push(1);
    for i in 2..5 {
        vec.push(&vec[i - 1] + &vec[i - 2]);
    }
    vec
}
