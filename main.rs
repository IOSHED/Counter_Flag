fn main() {
    let win = false;
    let counter = [false; 3];
    let location_points: [[String; 2]; 3] = [[String::from("100\n"), String::from("200\n")],
                                             [String::from("123\n"), String::from("123\n")],
                                             [String::from("567\n"), String::from("342\n")]];
    counter_flag(win, counter, location_points);
}

fn counter_flag(mut win: bool, mut counter: [bool; 3], location_points: [[String; 2]; 3]) {
    while win == false {
        let x= creature_input(String::from("X"));
        let y = creature_input(String::from("Y"));

        for i in 0..location_points.len() {
            if x == location_points[i][0] && y == location_points[i][1] {
                counter[i] = true;
                println!("================Point: {:?}================", counter);
                if counter[0] && counter[1] && counter[2] {
                    println!("________________-------|You win!.|-------________________");
                    win = true;
                }
            }
        }
    }
}

fn creature_input(name: String) -> String {
        let mut arg = String::new();
        println!("Position {}:   ", name);
        std::io::stdin().read_line(&mut arg).expect("Sorry, panic");
        return arg
}
