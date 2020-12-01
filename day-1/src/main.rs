use helpers;

fn part_1(numbers: &Vec<i32>) {
    'outer: for num in numbers {
        for complement in 1..(numbers.len()) {
            if (num + numbers[complement]) == 2020 {
                println!("Part 1\n======");
                println!("Numbers are: {} and {}", num, numbers[complement]);
                println!("Answer is: {}", num * numbers[complement]);
                break 'outer;
            }
        }
    }
}

fn part_2(numbers: &Vec<i32>) {
    'outer: for (index, num) in numbers.iter().enumerate() {
        for complement in numbers.iter().skip(index) {
            let remainder = 2020 - num - complement;
            if remainder > 0 {
                if let Some(value) = numbers.iter().find(|&&x| x == remainder) {
                    println!("Part 2\n======");
                    println!("Value: {}", value);
                    println!("num: {}", num);
                    println!("numbers[complemenet]: {}", complement);
                    println!("Answer is: {}", value * num * complement);
                    break 'outer;
                }
            }
        }
    }
}

fn main() {
    match helpers::read_input(String::from("1")) {
        Ok(data) => {
            let numbers = data
                .trim()
                .split("\n")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            part_1(&numbers);
            part_2(&numbers);
        }
        Err(err) => eprintln!("Doesn't work"),
    }
}
