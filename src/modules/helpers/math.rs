use std::ops::Add;
use rocket::http::ext::IntoCollection;

pub struct Math {}
impl Math {
    pub fn round_float_to_n_decimals(number: f64, decimals: i32) -> f64 {
        let multiplier = 10.0_f64.powi(decimals);
        (number * multiplier).round() / multiplier
    }

    pub fn mean(nums: &Vec<f64>) -> f64 {
        let sum: f64 = nums.iter().sum();
        let len = nums.len() as f64;
        sum / len
    }

    pub fn standard_deviation(nums: &Vec<f64>) -> f64 {
        let mean = Math::mean(nums);
        let mut sum = 0.0;
        for num in nums {
            sum += (num - mean).powi(2);
        }

        (sum / nums.len() as f64).sqrt()
    }

    pub fn median(nums: Vec<f64>) ->f64 {
        // sort the list
        let mut nums = nums;
        nums.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // get the middle element
        let middle = nums.len() / 2;
        if nums.len() % 2 == 0 {
            // if the list has an even number of elements, take the average of the two middle elements
            let a = nums[middle - 1];
            let b = nums[middle];
            (a + b) / 2.0
        } else {
            // if the list has an odd number of elements, take the middle element
            nums[middle]
        }
    }


}