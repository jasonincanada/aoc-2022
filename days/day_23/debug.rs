
print_grove(&grove, "Initial State");

print_grove(&grove, &format!("End of Round {round}"));


fn print_grove(grove: &Grove, title: &str) {
    let (left, right, top, bottom) = rectangle_around(grove);

    println!("== {} ==", title);

    for row in top..=bottom {
        for col in left..=right {
            print!("{}", grove[row][col]);
        }
        println!("")
    }
    println!("")
}