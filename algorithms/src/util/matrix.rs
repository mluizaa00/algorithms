use num_traits::Num;
use std::{fmt::Debug, ops::{AddAssign, Mul}};

pub fn create<T: Default + Copy + From<u8>>(columns: usize, rows: usize) -> Vec<Vec<T>> {
    vec![vec![T::default(); columns]; rows]
}

pub fn print<T: Debug>(matrix: &Vec<Vec<T>>) {
    for row in matrix {
        for element in row {
            print!("{:4?} ", element);
        }
        println!()
    }
}

pub fn multiply<T>(first: Vec<Vec<T>>, second: Vec<Vec<T>>) -> Vec<Vec<T>>
    where T: Default + Mul<Output = T> + Num + AddAssign + Copy + From<u8> 
{
    let column_count = first[0].len();
    let row_count = second.len();

    let mut result: Vec<Vec<T>> = create(column_count, row_count);

    for row in 0..row_count {
        for column in 0..column_count {
            for k in 0..second[0].len() {
                result[row][column] += second[row][k] * first[k][column];
            }
        }
    }

    result
}