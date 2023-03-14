pub mod utils;
pub mod year_2022;

use year_2022::day_1_calorie_counting::CalorieCounter;

fn main() {
    let max_calories = CalorieCounter::new().get_total_calories_three_max();
    println!("{}", max_calories)
}
