use crate::utils::reader::Reader;

pub struct CalorieCounter {}

impl CalorieCounter {
    // Constructor of CalorieCounter
    pub fn new() -> Self {
        Self {}
    }

    // Get total food calories of the elf that is carrying the most calories
    pub fn get_max_calories(&self) -> u32 {
        let mut max_calories: u32 = 0;
        let mut calories_counter = 0;
        loop {
            let content = self.read_line();
            if self.is_empty_or_dot(&content) {
                if max_calories < calories_counter {
                    max_calories = calories_counter;
                }
                calories_counter = 0;
                if self.is_dot(&content) {
                    break;
                }
            }
            let calories: u32 = content.parse().unwrap_or_default();
            calories_counter += calories;
        }
        max_calories
    }

    // Get the sum of the three elves that are carryin the most calories
    pub fn get_total_calories_three_max(&self) -> u32 {
        let mut vector: Vec<u32> = Vec::new();
        let mut calories_counter: u32 = 0;
        loop {
            let content = self.read_line();
            if self.is_empty_or_dot(&content) {
                vector.push(calories_counter);
                calories_counter = 0;
                if self.is_dot(&content) {
                    break;
                }
            }
            let calories: u32 = content.parse().unwrap_or_default();
            calories_counter += calories;
        }
        self.get_sum_three_bigger(&mut vector)
    }

    // Read a line from the stdio
    fn read_line(&self) -> String {
        match Reader::read_line() {
            Ok(line) => line,
            Err(_) => panic!("Error when reading a line from stdin"),
        }
    }

    // Check if the text is empty or is a dot
    fn is_empty_or_dot(&self, text: &str) -> bool {
        text.is_empty() || self.is_dot(text)
    }

    // Check if the text is a dot
    fn is_dot(&self, text: &str) -> bool {
        text.eq(".")
    }

    // Get the sum of the 3 bigger numbers of a vector
    fn get_sum_three_bigger(&self, vector: &mut Vec<u32>) -> u32 {
        let mut sum = 0;
        vector.sort_by(|a, b| b.cmp(a));
        for i in 0..3 {
            sum += vector.get(i).unwrap_or(&0);
        }
        sum
    }
}
