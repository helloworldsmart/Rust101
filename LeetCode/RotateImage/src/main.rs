fn main() {
    // TODO:
    let mut matrix: Vec<Vec<i32>> = [[5,1,9,11],[2,4,8,10],[13,3,6,7],[15,14,12,16]];
    rotate(matrix: matrix)
}

pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let height = matrix.len();
    let width = matrix[0].len();
    
    let mut tmp;
    
    for i in 0..(height/2) {
        for j in 0..width {
            tmp = matrix[i][j];
            matrix[i][j] = matrix[height-1-i][j];
            matrix[height-1-i][j] = tmp;
        }
    }
    
    for i in 0..height {
        for j in i..width {
            tmp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = tmp;
        }
    }
}