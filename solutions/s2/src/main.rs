fn main() {
    const LIMIT : u32 = 4000000;

    let mut sum : u32 = 0;
    let mut a = 0;
    let mut b = 1;

    while a < LIMIT {
        if a % 2 == 0 {
            sum += a;
        }

        let c = a + b;
        a = b;
        b = c;
    }

    println!("{}", sum);
}
