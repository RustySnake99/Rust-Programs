use std::io::{self, Write};

fn read_matrix(rows: usize, cols: usize, name: &str) -> Vec<Vec<i32>> {
    let mut matrix = vec![vec![0; cols]; rows];
    let mut input = String::new();

    println!("Enter elements for matrix {} ({}x{}):", name, rows, cols);
    for i in 0..rows {
        for j in 0..cols {
            print!("{}[{}][{}]: ", name, i + 1, j + 1);
            io::stdout().flush().unwrap();
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            matrix[i][j] = input.trim().parse().expect("Enter a valid integer!");
        }
    }
    matrix
}

fn multiply_matrix(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let rows_a = a.len();
    let cols_a = a[0].len();
    let cols_b = b[0].len();

    let mut result = vec![vec![0; cols_b]; rows_a];

    for i in 0..rows_a {
        for j in 0..cols_b {
            for k in 0..cols_a {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    
    result
}

fn display_matrix(matrix: &Vec<Vec<i32>>) {
    for i in matrix {
        for j in i {
            print!("{:5}", j);
        }
        println!();
    }
}

fn main() {
    let mut input = String::new();

    print!("Enter the number of rows for matrix 'A': ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    let r: usize = input.trim().parse().expect("Please enter a valid positive integer!");

    input.clear();
    print!("Enter the number of columns of matrix 'A': ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let c: usize = input.trim().parse().expect("Please enter a valid positive integer!");

    println!("Rows of matrix 'B' has been set to {} to make matrix multiplication possible", r);
    input.clear();
    print!("Enter the number of columns of matrix 'B': ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let c2: usize = input.trim().parse().expect("Please enter a valid positive integer!");
    let a = read_matrix(r, c, "a");
    let b = read_matrix(c, c2, "b");

    let result = multiply_matrix(&a, &b);
    println!("Product of 'A' and 'B' is: ");
    display_matrix(&result);


}