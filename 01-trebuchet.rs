// https://adventofcode.com/2023/day/1

mod test;
mod trebuchet {
    #[allow(dead_code)]
    fn get_number_sum(input: &str) -> usize {
        let lines = input.split('\n');
        let mut two_digit_numbers: Vec<usize> = Vec::new();

        for line in lines {
            let mut digits_in_line: Vec<char> = Vec::new();
            for letter in line.chars() {
                if letter.is_numeric() {
                    digits_in_line.push(letter);
                };
            }

            let first_digit = digits_in_line
                .first()
                .expect("Failed to read the first digit.");
            let last_digit = digits_in_line
                .last()
                .expect("Failed to read the second digit.");
            let two_digit_string = format!("{}{}", first_digit, last_digit)
                .parse::<usize>()
                .expect("Failed to read two digit number.");

            two_digit_numbers.push(two_digit_string);
        }

        let mut sum: usize = 0;
        for number in two_digit_numbers {
            sum += number;
        }

        sum
    }

    mod test {
        #[cfg(test)]
        mod get_number_sum {
            use super::super::get_number_sum;

            #[test]
            fn test_example() {
                let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
                let output = get_number_sum(input);
                let expected_answer: usize = 142;

                assert_eq!(output, expected_answer);
            }
        }
    }
}
