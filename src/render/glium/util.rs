pub fn normalize<const N: usize>(vector: [f32; N]) -> [f32; N] {
    let mut len: f32 = 0_f32;

    for item in vector.iter() {
        len += item * item;
    }

    len = len.sqrt();

    let mut new_vector = [0_f32; N];

    for i in 0..N {
        new_vector[i] = vector[i] / len;
    }

    println!("{new_vector:?}");

    new_vector
}
