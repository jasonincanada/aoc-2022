
fn print_rope(rope: &[Coord]) {
    let left   = 0; //rope.iter().map(|knot| knot.col).min().unwrap();
    let right  = 5; //rope.iter().map(|knot| knot.col).max().unwrap();
    let bottom = 0; //rope.iter().map(|knot| knot.row).min().unwrap();
    let top    = 4; //rope.iter().map(|knot| knot.row).max().unwrap();

    let indexed: Vec<(usize, &Coord)> = rope.iter().enumerate().collect();

    for row in (bottom..=top).rev() {
        for col in left..=right {
            let here = Coord {row, col};
            let knots_here = indexed.iter()
                                    .filter(|(_, coord)| **coord == here)
                                    .map(|(i, _)| *i)
                                    .collect::<Vec<usize>>();

            if knots_here.is_empty() {
                print!(".");
            } else {
                let lowest = knots_here.iter().min().unwrap();
                print!("{}", lowest);
            }
        }

        println!();
    }

    println!();
}

fn wait_for_enter() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
}
