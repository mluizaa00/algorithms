use crate::util::matrix;

/*
 * The fibonacci clock utilizes the number 1 to 8 to represent the hours
 * and minutes. Each representation varies by number and by color.
 *
 * In a mathematical form, the clock can be represented by a matrix
 * of 3 rows and 3 columns.
 *
 * Each column has a 'weight' of 1, 1, 3, 5 and 8, respectively.
 *
 * We can obtain the values by the following rules:
 * - Hours: For every value of the red line and the blue line, multiply
 * each one by the column weight. Sum all after.
 * - Minutes: For every value of the green line and the blue line, multiply
 * each one by the column weight. Sum all after and multiply by 5.
 */
const ROWS: usize = 3;
const COLUMNS: usize = 5;

pub fn clock(
    red_rows_as_present: Option<Vec<usize>>,
    blue_rows_as_present: Option<Vec<usize>>,
    green_rows_as_present: Option<Vec<usize>>,
) -> Clock {
    let column_weight = vec![1, 1, 3, 5, 8];

    let mut matrix = vec![vec![0; COLUMNS]; ROWS];

    matrix.fill(0, red_rows_as_present.unwrap_or_else(Vec::new));
    matrix.fill(1, blue_rows_as_present.unwrap_or_else(Vec::new));
    matrix.fill(2, green_rows_as_present.unwrap_or_else(Vec::new));

    println!("The matrix in operation is the following one:");
    matrix::print(&matrix);
    
    let hour = matrix.calculate(COLUMNS, &column_weight, 0, 1, 1);
    let minutes = matrix.calculate(COLUMNS, &column_weight, 1, 2, 5);

    println!(
        "\nThe fibonacci clock is at {} hours and {} minutes.",
        hour, minutes
    );

    Clock { hour, minutes }
}

pub struct Clock {
    hour: i32,
    minutes: i32,
}

trait FibonacciClockMatrix {
    fn fill(&mut self, row: usize, map: Vec<usize>);
    fn calculate(
        &mut self,
        columns: usize,
        weights: &Vec<i32>,
        row_one: usize,
        row_two: usize,
        multiplier: i32,
    ) -> i32;
}

impl FibonacciClockMatrix for Vec<Vec<i32>> {
    fn fill(&mut self, row: usize, indices: Vec<usize>) {
        if indices.is_empty() || self.is_empty() {
            return;
        }

        for index_as_true in indices {
            self[row][index_as_true] = 1;
        }
    }

    fn calculate(
        &mut self,
        columns: usize,
        weights: &Vec<i32>,
        row_one: usize,
        row_two: usize,
        multiplier: i32,
    ) -> i32 {
        (0..columns).fold(0, |acc, index| {
            let weight = weights[index];
            let red = self[row_one][index] * weight;
            let blue = self[row_two][index] * weight;
            acc + (red + blue) * multiplier
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn red_and_green_calculation() {
        let clock = clock(
            Some(vec!(3, 4)),
            None,
            Some(vec!(3)),
        );
        assert_eq!(clock.hour, 13);
        assert_eq!(clock.minutes, 25)
    }

    #[test]
    fn red_and_blue_calculation() {
        let clock = clock(
            Some(vec!(3, 4)),
            Some(vec!(3)),
            None,
        );
        assert_eq!(clock.hour, 18);
        assert_eq!(clock.minutes, 25)
    }

    #[test]
    fn red_and_blue_and_green_calculation() {
        let clock = clock(
            Some(vec!(3, 4)),
            Some(vec!(3)),
            Some(vec!(0)),
        );
        assert_eq!(clock.hour, 18);
        assert_eq!(clock.minutes, 30)
    }
}
