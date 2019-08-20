use std::cmp::Ordering;
use std::convert::TryFrom;
use std::io;

pub fn run() {
    println!("Give a list of numbers, separated by `,` (comma).");
    println!("I will give you back the mean, median, and mode.");

    loop {
        println!("Enter your list:");

        let input = get_input();

        if input.trim().is_empty() {
            println!("Nice try, but you must enter a list.");

            continue;
        }

        let numbers: Vec<i32> = input
            .split(',')
            .filter_map(|s| s.trim().parse::<i32>().ok())
            .collect();

        let mean = get_mean(&numbers);
        let median = get_median(&numbers);
        let mode = match get_mode(&numbers) {
            Some(num) => num.to_string(),
            None => String::from("None"),
        };

        println!("The mean is: {}", mean);
        println!("The median is: {}", median);
        println!("The mode is: {}", mode);

        println!("Would you like to try another list? (y/n)");

        let input = get_input();

        match input.chars().nth(0) {
            Some('y') => continue,
            Some(_) | None => break,
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();

    if let Err(error) = io::stdin().read_line(&mut input) {
        println!("An error occurred while reading the input: {}", error);
    }

    input
}

fn get_mean(numbers: &Vec<i32>) -> i32 {
    let sum: i32 = numbers.iter().sum();
    let len = match i32::try_from(numbers.len()) {
        Ok(num) => num,
        Err(_) => 1,
    };

    sum / len
}

fn get_median(numbers: &Vec<i32>) -> i32 {
    if numbers.len() == 1 {
        return numbers[0];
    }

    let mut sorted = numbers.clone();
    let middle = ((numbers.len() + 1) / 2) - 1;

    sorted.sort();

    match middle % 2 {
        0 => sorted[middle],
        _ => get_mean(&sorted[middle..=middle + 1].to_vec()),
    }
}

fn get_mode(numbers: &Vec<i32>) -> Option<i32> {
    let mode = numbers
        .iter()
        .fold(Vec::new(), |mut vec: Vec<(i32, i32)>, num| {
            if let Some(index) = vec.iter().position(|(n, _)| n == num) {
                let (_, count) = vec[index];

                vec[index] = (*num, count + 1);
            } else {
                vec.push((*num, 1));
            }

            vec
        })
        .iter()
        .fold((0, 0), |mode, item| match item {
            (num, count) if mode.1.cmp(count) == Ordering::Less => (*num, *count),
            _ => mode,
        });

    match mode {
        (_, 0) | (_, 1) => None,
        (num, _) => Some(num),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mean_calculates_correctly() {
        let nums: Vec<i32> = (0..=10).collect();

        assert_eq!(get_mean(&nums), 5);
    }

    #[test]
    fn median_with_one_item() {
        let nums = vec![1];

        assert_eq!(get_median(&nums), 1);
    }

    #[test]
    fn median_with_even_number_of_items() {
        let nums = vec![4, 2, 5, 1];

        assert_eq!(get_median(&nums), 3);
    }

    #[test]
    fn median_with_odd_number_of_items() {
        let nums = vec![2, 5, 1, 4, 3];

        assert_eq!(get_median(&nums), 3);
    }

    #[test]
    fn mode_when_no_duplicates_in_list() {
        let nums: Vec<i32> = (0..=10).collect();

        assert_eq!(get_mode(&nums), None)
    }

    #[test]
    fn mode_when_duplicates_in_list() {
        let nums = vec![1, 2, 2, 3, 3, 4];

        assert_eq!(get_mode(&nums), Some(2));
    }

    #[test]
    fn mode_when_empty_list() {
        let nums = vec![];

        assert_eq!(get_mode(&nums), None);
    }
}
