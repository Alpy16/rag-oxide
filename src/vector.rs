use std::io::{self, Error, ErrorKind};

pub fn vector_norm(vector: &[f32]) -> f32 {
    let sum_of_squares: f32 = vector.iter().map(|x| x * x).sum();
    sum_of_squares.sqrt()
}

pub fn normalize_vector(vector: &[f32]) -> Result<Vec<f32>, std::io::Error> {
    let norm = vector_norm(vector);
    if norm == 0.0 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Cannot normalize a zero vector",
        ));
    }
    Ok(vector.iter().map(|x| x / norm).collect())
}

pub fn dot_product(a: &[f32], b: &[f32]) -> Result<f32, std::io::Error> {
    if a.len() != b.len() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Vectors must have the same length",
        ));
    }
    Ok(a.iter().zip(b.iter()).map(|(x, y)| *x * *y).sum())
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

#[test]
fn test_normalize_vector() {
    let vector = vec![3.0, 4.0];
    let normalized = normalize_vector(&vector).unwrap();
    assert!((normalized[0] - 0.6).abs() < 1e-6);
    assert!((normalized[1] - 0.8).abs() < 1e-6);
}

#[test]
fn test_normalize_zero_vector() {
    let vector = vec![0.0, 0.0];
    let result = normalize_vector(&vector);
    assert!(result.is_err());
}

#[test]
fn dot_product_returns_correct() {
    let a_vector = vec![1.0, 2.0, 3.0];
    let b_vector = vec![4.0, 5.0, 6.0];
    let result = dot_product(&a_vector, &b_vector).unwrap();
    assert!((result - 32.0).abs() < 1e-6);
}

#[test]
fn dot_product_returns_error_for_different_lengths() {
    let a_vector = vec![1.0, 2.0, 3.0];
    let b_vector = vec![4.0, 5.0];
    let result = dot_product(&a_vector, &b_vector);
    assert!(result.is_err());
}

#[test]
fn dot_product_returns_error_for_empty_vectors() {
    let a_vector: Vec<f32> = vec![];
    let b_vector: Vec<f32> = vec![];
    let result = dot_product(&a_vector, &b_vector);
    assert_eq!(result.unwrap(), 0.0);
}
