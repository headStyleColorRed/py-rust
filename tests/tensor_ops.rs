use tch::Tensor;

#[test]
fn test_tensor_addition() {
    let t1 = Tensor::from_slice(&[1.0f32, 2.0, 3.0]);
    let t2 = Tensor::from_slice(&[4.0f32, 5.0, 6.0]);
    let sum = &t1 + &t2;

    // Compare tensor values directly
    assert!(sum.equal(&Tensor::from_slice(&[5.0f32, 7.0, 9.0])));
}

#[test]
fn test_tensor_shape() {
    let t1 = Tensor::from_slice(&[1.0f32, 2.0, 3.0]);
    assert_eq!(t1.size(), &[3]);
}

#[test]
fn test_tensor_multiplication() {
    let t1 = Tensor::from_slice(&[1.0f32, 2.0, 3.0]);
    let t2 = Tensor::from_slice(&[2.0f32, 2.0, 2.0]);
    let prod = &t1 * &t2;

    assert!(prod.equal(&Tensor::from_slice(&[2.0f32, 4.0, 6.0])));
}
