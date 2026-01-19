use std::cmp::Ordering;

const N: i32 = 10;
const PICK: i32 = 6;

fn guess(num: i32) -> i32 {
    match PICK.cmp(&num) {
        Ordering::Less => -1,
        Ordering::Greater => 1,
        Ordering::Equal => 0,
    }

}

fn guess_number(n: i32) -> i32 {
    let mut l = 0;
    let mut r = n;

    while l < r {
        let m = l + (r - l) / 2;
        let res = guess(m);
        if res == 0 {
            return m;
        } else if res == 1 {
            l = m + 1;
        } else {
            r = m;
        }
    }

    l
}

pub fn main() {
    println!("{}", guess_number(N));
}
