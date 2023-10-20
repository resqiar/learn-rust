#[derive(Debug)]
struct Game<T, U> {
    total_game: T,
    final_score: U,
}

// fn largest<T>(list: &[T]) -> &T {
//     let mut result = &list[0];
//
//     for v in list {
//         if v > result {
//             result = v;
//         }
//     }
//
//     return result;
// }

pub fn main() {
    let new_game = Game {
        total_game: 20,
        final_score: 20.5,
    };

    println!("{:?}", new_game);
}
