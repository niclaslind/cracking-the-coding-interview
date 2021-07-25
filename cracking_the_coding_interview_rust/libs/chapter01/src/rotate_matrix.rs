type Matrix = Vec<Vec<i32>>;

// Because we're rotating teh matrix by 90x90 degrees, the easiest way to do this is to implement
// the rotation in layers. We perform a circular rotation on each layer, moving the top edge to the right edge,
// the right edge to the bottom edge, the bottom edge to the left edge, and the left edge to the top edge.
fn rotate_matrix(matrix: &Matrix) -> Matrix {
    let mut matrix = matrix.clone();

    (0..matrix.len() / 2).for_each(|layer| {
        let first = layer;
        let last = matrix.len() - 1 - layer;

        (first..last).for_each(|i| {
            let offset = i - first;

            // save top
            let top = matrix[first][i];

            // left -> top
            matrix[first][i] = matrix[last - offset][first];

            // bottom -> left
            matrix[last - offset][first] = matrix[last][last - offset];

            // right -> bottom
            matrix[last][last - offset] = matrix[i][last];

            // top -> right
            matrix[i][last] = top; // right <- saved top
        });
    });

    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_matrix_90_degrees() {
        let matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];

        let rotated_matrix = rotate_matrix(&matrix);

        let expected_rotated_matrix = vec![
            vec![13, 9, 5, 1],
            vec![14, 10, 6, 2],
            vec![15, 11, 7, 3],
            vec![16, 12, 8, 4],
        ];

        assert_eq!(expected_rotated_matrix, rotated_matrix);
    }
}
