pub type Position = (usize, usize);

pub fn transpose<T: Clone>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return Vec::new();
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut transposed = Vec::with_capacity(cols);
    for _ in 0..cols {
        transposed.push(Vec::with_capacity(rows));
    }

    for i in 0..rows {
        for j in 0..cols {
            transposed[j].push(matrix[i][j].clone());
        }
    }

    transposed
}

pub fn manhattan_distance(pos1: Position, pos2: Position) -> usize {
    (pos1.0 as i64 - pos2.0 as i64).abs() as usize + (pos1.1 as i64 - pos2.1 as i64).abs() as usize
}
