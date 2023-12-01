// https://adventofcode.com/2023/day/1

mod trebuchet {
    fn replace_number_letters(input: String) -> String {
        let mut replaced_input = input;

        const NUMBERS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
        const NUMBER_LETTERS: [&str; 10] = [
            "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        for (index, number_letter) in NUMBER_LETTERS.iter().enumerate() {
            replaced_input = replaced_input.replace(number_letter, NUMBERS[index]);
        }

        replaced_input
    }

    fn get_number_sum(input: &str) -> usize {
        let lines = input.split('\n');
        let mut two_digit_numbers: Vec<usize> = Vec::new();

        for line in lines {
            let line = replace_number_letters(line.to_string());
            let mut digits_in_line: Vec<usize> = Vec::new();
            for letter in line.chars() {
                if let Ok(digit) = letter.to_string().parse::<usize>() {
                    digits_in_line.push(digit);
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

            println!("{}, {}", first_digit, last_digit);

            two_digit_numbers.push(two_digit_string);
        }

        println!("{:#?}", two_digit_numbers);
        let mut sum: usize = 0;
        for number in two_digit_numbers {
            sum += number;
        }

        sum
    }

    #[cfg(test)]
    mod test {
        use super::get_number_sum;

        #[test]
        fn test_example_1() {
            let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
            let output = get_number_sum(input);
            let expected_answer: usize = 142;

            assert_eq!(output, expected_answer);
        }

        #[test]
        fn test_example_2() {
            let input = r#"eight2sevenkl
mrjstg5onetwoeightgcczx8vgrgl
9246
ninetwo2crrqk2grsctqxqbcrmrdsqbrz9eight
five6dlhx1
29qhsdqqtgrk4
mjgtrjnlttxjlsixsix5
bgnnvfsnbpx29vsjrlmgmsqthreeqxvclkhlv
sevennine3gsmxncqlqvfktxrtcone"#;
            let output = get_number_sum(input);
            let expected_answer: usize = 573;

            assert_eq!(output, expected_answer);
        }

        #[test]
        fn test_example_3() {
            let input = r#"ntqtg1
hsqkbfnxcbsixmvfhxrvxrvnineseven4
bkztqhhbdzr1sevenonenine1
v79zpdqfive
tqtwonesixlhhnvf77one1
2twofour
eight89seven
7six6fmzlrttxstdrg
seventhreecbldkqd89threesevenpplfv
7hmhxpg
threepmvqfc5rktgjl4xvdzmhrninef
7qnlqhtrrk1sevenqfgpkdfc271seven
4gxrqspvg1
zdgfscbzbdg8rvqseven"#;
            let output = get_number_sum(input);
            let expected_answer: usize = 757;

            assert_eq!(output, expected_answer);
        }
    }
}
