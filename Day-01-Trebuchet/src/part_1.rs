pub fn process(input: &str) -> String {
    let output: u32 = input.lines()
        .map(|line| {
            let digits: Vec<u32> = line.chars()
                .filter_map(|char| char.to_digit(10))
                .collect();

            let first = digits.first().unwrap();
            let last = digits.last().unwrap();

            first * 10 + last
        })
        .sum();

    output.to_string()
}

#[test]
fn test() {
    let input = include_str!("../input/test_1.txt");
    let result = process(input);

    assert_eq!(result, "142");
}
