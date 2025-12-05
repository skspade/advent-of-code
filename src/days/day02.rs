pub fn part1(input: &[String]) -> String {
    let input_line = input.first().unwrap();
    //Parse the comma delimited list - this will give us elements that look like 1234-5678
    let input_array = input_line.split(',');
    let mut has_repeated_twice = false;
    let mut invalid_numbers_total: i64 = 0;
    for range in input_array {
        let parse_range: Vec<_> = range
            .split('-')
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        let start_of_range = parse_range[0];
        let end_of_range = parse_range[1];

        for current_number in start_of_range..=end_of_range {
            let stringify_current_number = current_number.to_string();

            let current_number_length = stringify_current_number.len();
            let half = current_number_length / 2;

            if current_number_length % 2 != 0 {
                has_repeated_twice = false;
                continue;
            }
            let first_half: String = stringify_current_number.chars().take(half).collect();
            let secon_half: String = stringify_current_number.chars().skip(half).collect();
            if first_half == secon_half {
                has_repeated_twice = true;
                invalid_numbers_total += current_number;
            }
        }
    }

    invalid_numbers_total.to_string()
}

pub fn part2(_input: &[String]) -> String {
    "Not implemented".to_string()
}
