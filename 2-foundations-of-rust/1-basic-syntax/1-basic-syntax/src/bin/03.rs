fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];
    let mut larger = input[0];
    let mut smallest = input[0];
    for x in 0..=7 {
        if input[x] > larger {
            larger = input[x];
        } else {
            if input[x] < smallest {
                smallest = input[x];
            }
        }
    }
    // TODO

    println!("{} is largest and {} is smallest", larger, smallest);
}
