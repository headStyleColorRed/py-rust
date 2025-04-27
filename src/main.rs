use tch::Tensor;

fn main() {
    // Create two tensors
    let t1 = Tensor::from_slice(&[1.0f32, 2.0, 3.0]);
    let t2 = Tensor::from_slice(&[4.0f32, 5.0, 6.0]);

    // Add them together
    let sum = &t1 + &t2;

    // Print the result
    println!("t1: {:?}", t1);
    println!("t2: {:?}", t2);
    println!("sum: {:?}", sum);
}
