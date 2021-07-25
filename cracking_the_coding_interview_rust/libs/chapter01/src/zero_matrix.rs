type Matrix = Vec<Vec<i32>>;

fn zero_matrix(matrix: &Matrix) -> Matrix {
    let mut result = matrix.clone();

    (0..matrix.len()).for_each(|layer| {
        (0..matrix[layer].len()).for_each(|col| {
            if matrix[layer][col] == 0 {
                for i in 0..matrix.len() {
                    result[i][col] = 0;
                }

                for i in 0..matrix[layer].len() {
                    result[layer][i] = 0;
                }
            }
        });
    });

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_zero_matrix() {
        assert_eq!(
            vec![vec![0, 0], vec![0, 1]],
            zero_matrix(&vec![vec![0, 1], vec![1, 1]])
        );
        assert_eq!(
            vec![
                vec![0, 1, 0, 1],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 1, 0, 1]
            ],
            zero_matrix(&vec![
                vec![1, 1, 1, 1],
                vec![0, 1, 1, 1],
                vec![1, 1, 0, 1],
                vec![1, 1, 1, 1]
            ])
        );
    }
}
