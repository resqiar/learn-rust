mod enums_mod;
mod generic_mod;
mod hash_mod;
mod match_mod;
mod string_mod;
mod struct_mod;
mod test_mod;
mod vector_mod;

fn main() {
    let x = "is x uppercase?";

    // This is called shadowing
    let x = x.to_uppercase();
    println!("Hello from neither me nor you, {x}");

    let numeric: i32 = "69".parse().expect("Not possible to parse to number");
    println!("{numeric}");

    let mut s = String::from("Hello World Is What ?");
    let s_len = get_len(&s);

    let t = &s;
    let x = &s;
    println!("{}, {}", t, x);

    let z = &mut s;
    println!("{}", z);

    println!("{}, {}", s, s_len);

    let b = split_word(&s, 1);
    println!("SPLIT WORD:{}", b);

    print_divider();

    // struct
    struct_mod::main();

    print_divider();

    // enums
    enums_mod::main();

    print_divider();

    // options
    match_mod::main();

    print_divider();

    // vectors
    vector_mod::main();

    print_divider();

    // string
    string_mod::main();

    print_divider();

    // hash map
    hash_mod::main();

    print_divider();

    // generics
    generic_mod::main();
}

fn get_len(input: &String) -> usize {
    input.len()
}

fn split_word(input: &str, num: usize) -> &str {
    let words: Vec<&str> = input.split_whitespace().collect();

    if words.len() < num {
        return input;
    }

    return words[num - 1];
}

fn print_divider() {
    println!("================================================");
    println!("");
}
