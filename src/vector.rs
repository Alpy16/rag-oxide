pub fn vector_norm(vector: &[f32]) -> f32 {
    let sum_of_squares: f32 = vector.iter().map(|x| x * x).sum();
    sum_of_squares.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
}

#[test]
fn test_vector_norm() {
    let vector = vec![3.0, 4.0];
    let norm = vector_norm(&vector);
    assert!((norm - 5.0).abs() < 1e-6);
}
