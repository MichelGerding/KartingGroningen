pub struct Math {}
impl Math {
    /// # round float to n decimals
    ///
    /// ## Arguments
    /// * `number` - The number to round
    /// * `n` - The number of decimals to round to
    ///
    /// ## Returns
    /// * 'f64' - The rounded number
    pub fn round_float_to_n_decimals(number: f64, decimals: i32) -> f64 {
        let multiplier = 10.0_f64.powi(decimals);
        (number * multiplier).round() / multiplier
    }

    /// # get the mean of a vector of f64
    ///
    /// ## Arguments
    /// * `nums` - The vector of numbers to get the mean of
    ///
    /// ## Returns
    /// * 'f64' - The mean of the vector
    pub fn mean(nums: &Vec<f64>) -> f64 {
        let sum: f64 = nums.iter().sum();
        let len = nums.len() as f64;
        sum / len
    }

    /// # get the standard deviation of a vector of f64
    ///
    /// ## Arguments
    /// * `nums` - The vector of numbers to get the standard deviation of
    ///
    /// ## Returns
    /// * 'f64' - The standard deviation of the vector
    pub fn standard_deviation(nums: &Vec<f64>) -> f64 {
        let mean = Math::mean(nums);
        let mut sum = 0.0;
        for num in nums {
            sum += (num - mean).powi(2);
        }

        (sum / nums.len() as f64).sqrt()
    }

    /// # get the median of a vector of f64
    ///
    /// ## Arguments
    /// * `nums` - The vector of numbers to get the median of
    ///
    /// ## Returns
    /// * 'f64' - The median of the vector
    pub fn median(nums: Vec<f64>) -> f64 {
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
