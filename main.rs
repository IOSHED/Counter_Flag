fn main() {
    let counter = [false; 3];

    let location_points: [[String; 2]; 3] = [[String::from("100\n"), String::from("200\n")],
                                             [String::from("123\n"), String::from("876\n")],
                                             [String::from("567\n"), String::from("342\n")]];
    counter_flag(counter, location_points);
}

fn counter_flag(mut counter: [bool; 3], location_points: [[String; 2]; 3]) {
    loop {
        let mut x = String::new();
        println!("Position X:   ");
        std::io::stdin().read_line(&mut x).expect("Sorry, panic");

        let mut y = String::new();
        println!("Position Y:   ");
        std::io::stdin().read_line(&mut y).expect("Sorry, panic");

        for i in 0..location_points.len() {
            if x == location_points[i][0] && y == location_points[i][1] {
                counter[i] = true;
                println!("================Point: {:?}================", counter);
                if counter[0] && counter[1] && counter[2] {
                    println!("________________-------|You win!.|-------________________");
                    break
                }
            }
        }
    }
}
