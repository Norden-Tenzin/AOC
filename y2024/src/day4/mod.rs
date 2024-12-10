use std::fs;

pub fn problem1(path: &str) -> usize {
    fn rows(matrix: &Vec<Vec<&str>>) -> usize {
        let mut res: usize = 0;
        for row in matrix {
            res += row.join("").match_indices("XMAS").count();
            res += row.join("").match_indices("SAMX").count();
        }
        return res;
    }

    fn diagonal_forward(matrix: &Vec<Vec<&str>>) -> usize {
        let mut res: usize = 0;
        let rows = matrix.len();
        let cols = matrix[0].len();

        for k in 0..=(cols + rows - 2) {
            let mut line: String = String::new();
            for j in 0..=k {
                let i = k - j;
                if i < rows && j < cols {
                    line += matrix[i][j];
                }
            }
            res += line.match_indices("XMAS").count();
            res += line.match_indices("SAMX").count();
        }
        return res;
    }

    fn diagonal_backward(matrix: &Vec<Vec<&str>>) -> usize {
        let mut res: usize = 0;
        let rows = matrix.len();
        let cols = matrix[0].len();

        for k in 0..=(cols + rows - 2) {
            let mut line: String = String::new();
            for j in 0..=k {
                let i = k - j;

                // Ensure indices are within bounds
                if i < rows && j < cols && (cols - 1 >= j) {
                    line += matrix[i][cols - 1 - j];
                }
            }
            res += line.match_indices("XMAS").count();
            res += line.match_indices("SAMX").count();
        }
        return res;
    }

    fn vertical(matrix: &Vec<Vec<&str>>) -> usize {
        let mut res: usize = 0;
        let rows = matrix.len();
        let cols = matrix[0].len();

        for i in 0..cols {
            let mut line: String = String::new();
            for j in 0..rows {
                line += matrix[j][i];
            }
            res += line.match_indices("XMAS").count();
            res += line.match_indices("SAMX").count();
        }
        return res;
    }

    let file = fs::read_to_string(path).expect("unable to read file.");
    let mut grid: Vec<Vec<&str>> = Vec::new();
    let mut res: usize = 0;

    for line in file.lines() {
        grid.push(line.split("").collect());
    }

    res += rows(&grid);
    res += diagonal_forward(&grid);
    res += diagonal_backward(&grid);
    res += vertical(&grid);

    return res;
}

pub fn problem2(path: &str) -> usize {
    let file = fs::read_to_string(path).expect("unable to read file.");
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut res: usize = 0;

    for line in file.lines() {
        grid.push(line.chars().collect::<Vec<char>>());
    }

    let rows = grid.len();
    let cols = grid[0].len();
    for row in 0..rows - 2 {
        for col in 0..cols - 2 {
            let square = extract_square_section(&grid, row, col, 3);
            if check_if_valid(&square) {
                res += 1;
            }
        }
    }

    fn extract_square_section<T: Clone>(
        matrix: &Vec<Vec<T>>,
        start_row: usize,
        start_col: usize,
        size: usize,
    ) -> Vec<Vec<T>> {
        let mut square_section = Vec::new();

        for i in 0..size {
            if let Some(row) = matrix.get(start_row + i) {
                let slice = row
                    .iter()
                    .skip(start_col)
                    .take(size)
                    .cloned()
                    .collect::<Vec<T>>();
                square_section.push(slice);
            }
        }
        return square_section;
    }

    fn check_if_valid(matrix: &Vec<Vec<char>>) -> bool {
        return diagonal_forward(matrix) && diagonal_backward(matrix);
    }

    fn diagonal_forward(matrix: &Vec<Vec<char>>) -> bool {
        let mut res: usize = 0;
        let rows = matrix.len();
        let cols = matrix[0].len();

        for k in 0..=(cols + rows - 2) {
            let mut line: String = String::new();
            for j in 0..=k {
                let i = k - j;
                if i < rows && j < cols {
                    line.push(matrix[i][j]);
                }
            }
            res += line.match_indices("MAS").count();
            res += line.match_indices("SAM").count();
        }
        if res > 0 {
            return true;
        }
        return false;
    }

    fn diagonal_backward(matrix: &Vec<Vec<char>>) -> bool {
        let mut res: usize = 0;
        let rows = matrix.len();
        let cols = matrix[0].len();

        for k in 0..=(cols + rows - 2) {
            let mut line: String = String::new();
            for j in 0..=k {
                let i = k - j;

                // Ensure indices are within bounds
                if i < rows && j < cols && (cols - 1 >= j) {
                    line.push(matrix[i][cols - 1 - j]);
                }
            }
            res += line.match_indices("MAS").count();
            res += line.match_indices("SAM").count();
        }
        if res > 0 {
            return true;
        }
        return false;
    }

    //println!("{:?}", extract_square_section(&simple_grid, 0, 0, 3));

    return res;
}
