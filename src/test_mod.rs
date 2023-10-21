fn sum_num(a: i32, b: i32) -> i32 {
    return a + b;
}

#[test]
fn sum_test() {
    let a = 10;
    let b = 10;

    let res = sum_num(a, b);

    assert_eq!(res, a + b);
}

#[test]
fn can_hold_test() {
    use super::struct_mod;

    let box1 = struct_mod::Rect {
        name: String::from("Front Box"),
        dimension: struct_mod::Dimension(10, 15),
        color: struct_mod::Color(255, 255, 255),
    };

    let box2 = struct_mod::Rect {
        name: String::from("Smaller Box"),
        dimension: struct_mod::Dimension(5, 5),
        color: struct_mod::Color(0, 0, 0),
    };

    let can_hold = box1.can_hold(box2);

    assert!(!can_hold, "Should hold the second Box");
}
