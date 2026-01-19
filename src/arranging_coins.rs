fn arrange_coins(n: i32) -> i32 {
    let n = n as i64;
    let mut l = 0;
    let mut r = n;
    while l < r {
        let m = l + (r - l + 1) / 2;
        let calc = m * (m + 1) / 2;
        if calc == n {
            return m as i32;
        } else if calc < n {
            l = m;
        } else {
            r = m - 1;
        }
    }

    l as i32
}

pub fn main() {
    let n = 8;
    println!("{}", arrange_coins(n));
}
