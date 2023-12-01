// https://adventofcode.com/2023/day/1

pub mod trebuchet {
    const NUMBERS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    const NUMBER_LETTERS: [&str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    pub fn get_number_sum(input: &str) -> usize {
        // TODO: Rewrite Solution

        0
    }

    #[cfg(test)]
    mod test {
        mod replace_number_letters {
            use crate::trebuchet::get_number_sum;

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

            #[test]
            fn test_basic_numbers() {
                assert_eq!(get_number_sum("zero"), 0);
                assert_eq!(get_number_sum("one"), 1);
                assert_eq!(get_number_sum("two"), 2);
                assert_eq!(get_number_sum("three"), 3);
                assert_eq!(get_number_sum("four"), 4);
                assert_eq!(get_number_sum("five"), 5);
                assert_eq!(get_number_sum("six"), 6);
                assert_eq!(get_number_sum("seven"), 7);
                assert_eq!(get_number_sum("eight"), 8);
                assert_eq!(get_number_sum("nine"), 9);
            }

            #[test]
            fn test_overlapping_numbers() {
                assert_eq!(get_number_sum("twone"), 21);
                assert_eq!(get_number_sum("eightwo"), 82);
                assert_eq!(get_number_sum("nineight"), 98);
                assert_eq!(get_number_sum("eighthreee"), 83);
                assert_eq!(get_number_sum("nineeight"), 98);
            }

            #[test]
            fn test_other_edge_cases() {
                assert_eq!(get_number_sum("eeeight"), 8);
                assert_eq!(get_number_sum("oooneone"), 11);
            }
        }
    }
}
