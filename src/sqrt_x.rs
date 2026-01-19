fn my_sqrt(num: i32) -> i32 {
    let num = num as i64;
    let mut l = 0;
    let mut r = num;

    while l < r {
        let m = l + (r - l + 1) / 2;
        if m * m == num {
            return m as i32;
        } else if m * m < num {
            l = m;
        } else {
            r = m - 1;
        }
    }

    l as i32
}
pub fn main() {
    let x = 5;
    println!("{}", my_sqrt(x));
}
