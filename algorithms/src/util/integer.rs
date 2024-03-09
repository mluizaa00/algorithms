pub trait Integer {
    fn to_algarisms(self) -> Vec<i32>;
}

impl Integer for i32 {
    fn to_algarisms(self) -> Vec<i32> {
        self.to_string()
            .chars()
            .map(|char| char.to_digit(10).unwrap_or(0) as i32)
            .collect()
    }
}

pub trait IntegerVec {
    fn sort_by_descending(&mut self);
    fn char_to_int(&mut self) -> i32;
}

impl IntegerVec for Vec<i32> {
    fn sort_by_descending(&mut self) {
        self.sort_by(|a, b| b.cmp(a));
    }

    fn char_to_int(&mut self) -> i32 {
        self.iter().fold(0, |acc, &digit| acc * 10 + digit)
    }
}