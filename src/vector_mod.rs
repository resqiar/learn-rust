pub fn main() {
    let result = fib(91);
    println!("{:?}", result);

    // let does_not_exist = &result[90];
    let does_not_exist = &result.get(90);
    match does_not_exist {
        Some(v) => println!("VALUE OF 90 in FIB!!! = {}", v),
        None => println!("Input is not reaching 90"),
    }
}

fn fib(target: usize) -> Vec<u64> {
    let mut fib_num: Vec<u64> = vec![0, 1];

    // if the target is less than 2
    if target < 2 {
        return fib_num[..target].to_vec();
    }

    for i in 2..target {
        let prev = &fib_num[i - 1];
        let before_prev = &fib_num[i - 2];
        let value = prev + before_prev;
        fib_num.push(value);
    }

    return fib_num;
}
