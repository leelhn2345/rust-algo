fn convert_number_to_words(number: usize) -> String {
    let ones = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let teens = [
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];
    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    let convert_less_than_thousand = |number: usize| {
        if number > 1000 {
            return convert_number_to_words(number);
        }
        let hundred = number / 100;
        let remainder = number % 100;

        let hundred_word: String = ones[hundred].into();
        let remainder_word: String = match remainder {
            0..=9 => ones[remainder].into(),
            10..=19 => {
                let teens_word = remainder % 10;
                teens[teens_word].into()
            }
            20..=99 => {
                let ten_word = remainder / 10;
                let one_word = remainder % 10;

                if one_word != 0 {
                    tens[ten_word].to_owned() + " " + ones[one_word]
                } else {
                    tens[ten_word].to_owned()
                }
            }
            _ => panic!("this should never happen"),
        };

        if hundred != 0 && remainder == 0 {
            format!("{} hundred", hundred_word)
        } else if hundred != 0 && remainder != 0 {
            format!("{} hundred and {}", hundred_word, remainder_word)
        } else {
            remainder_word
        }
    };

    if number > 1_000_000 {
        let millions = number / 1_000_000;
        let remainder = number % 1_000_000;
        format!(
            "{} million, {}",
            convert_less_than_thousand(millions),
            convert_less_than_thousand(remainder)
        )
    } else if number > 1_000 {
        let thousands = number / 1_000;
        let remainder = number % 1_000;

        format!(
            "{} thousand {}",
            convert_less_than_thousand(thousands),
            convert_less_than_thousand(remainder)
        )
    } else {
        convert_less_than_thousand(number)
    }
}
fn main() {
    // println!("Hello, world!");
    let number: usize = 100_010_000;
    assert!(number < 999_999_999, "number should be below 1 billion");
    let words = convert_number_to_words(number);
    println!("{words} only");
}
