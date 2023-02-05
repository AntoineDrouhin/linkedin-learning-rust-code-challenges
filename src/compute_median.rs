



pub fn compute_median(vec:Vec<i32>) -> i32 {

    let length: usize = vec.len();

    let divisible_length: usize = if length % 2 == 0 { length} else {length + 1 };

    let result_index: usize = divisible_length / 2;

    return vec[result_index];

    
}




#[cfg(test)]
mod tests {
    #[test]
    fn given_ordered_odd_input_compute_medi() {
        let input: Vec<i32> = vec![1,4,25,46];
        let result:i32 = super::compute_median(input);
        
        assert_eq!(result, 25);
    }
}
