fn is_perfect_square(num: i32) -> bool {
    let num = num as i64;
    let mut l = 0;
    let mut r = num;

    while l < r {
        let m = l + (r - l) / 2;
        if m * m == num {
            return true;
        } else if m * m < num {
            l = m + 1;
        } else {
            r = m;
        }
    }

    (l * l) == num
}

pub fn main() {
    let num = 1;
    println!("{}", is_perfect_square(num));
}
