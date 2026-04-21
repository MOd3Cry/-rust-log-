fn main() {
    let array = [1, 5, 8, 10, 24, 67, 124, 2345];

    let mut left_wall = 0;
    let mut right_wall = array.len() - 1;

    let finding = 67;

    while left_wall < right_wall {
        let mid_wall = (left_wall + right_wall) / 2;
        if array[mid_wall] < finding {
            left_wall = mid_wall;
        } else if array[mid_wall] > finding {
            right_wall = mid_wall;
        } else {
            let result = mid_wall;
            println!("{}", array[result]);
            break;
        }
    }
}
