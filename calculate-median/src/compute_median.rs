pub fn compute_median(vec: Vec<i32>) -> Result<i32, &'static str> {
    let length: usize = vec.len();

    if length == 0 {return Err("Empty vector")}

    let mut sorted_vec : Vec<i32> = vec.to_vec();
    sorted_vec.sort();     

    let divisible_length = if length % 2 == 0 { length } else { length - 1 };

    let result_index = divisible_length / 2;

    return Ok(sorted_vec[result_index]);
}

#[cfg(test)]
mod tests {
    #[test]
    fn given_ordered_even_input_compute_median() {
        let input: Vec<i32> = vec![1, 4, 25, 46];
        let result: i32 = super::compute_median(input).unwrap();

        assert_eq!(result, 25);
    }

    #[test]
    fn given_ordered_odd_input_compute_median() {
        let input: Vec<i32> = vec![1, 4, 8, 25, 46];
        let result: i32 = super::compute_median(input).unwrap();

        assert_eq!(result, 8);
    }

    #[test]
    fn given_single_input_compute_median() {
        let input: Vec<i32> = vec![8];
        let result: i32 = super::compute_median(input).unwrap();

        assert_eq!(result, 8);
    }
    
    #[test]
    fn handle_empty_vec() {
        let input: Vec<i32> = vec![];
        let result: &str = super::compute_median(input).unwrap_err();

        assert_eq!(result, "Empty vector");
    }
#[test]
    fn given_unordered_even_input_compute_median() {
        let input: Vec<i32> = vec![46, 4, 1, 25];
        let result: i32 = super::compute_median(input).unwrap();

        assert_eq!(result, 25);
    }

#[test]
    fn given_unordered_odd_input_compute_median() {
        let input: Vec<i32> = vec![1, 4, 46, 25, 8];
        let result: i32 = super::compute_median(input).unwrap();

        assert_eq!(result, 8);
    }

    
}
