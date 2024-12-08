fn main() {
    let numbers = [1, 2, 123456, 123123123, 5, 6, 1231902831290381290381902, 5555];

    let mut higher: i128 = numbers[0];

    for &number in &numbers {
        if number > higher {
            higher = number;
        }
    }

    println!("{}", higher);

}
