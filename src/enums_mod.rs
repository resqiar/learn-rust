#[derive(Debug)]
struct Language {
    kind: LanguageKind,
    value: Phrase,
}

impl Language {
    fn print(&self) {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
enum LanguageKind {
    Indonesian,
    Javanese,
    English,
}

impl LanguageKind {
    fn print(&self) {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
enum Phrase {
    MorningSalute(String),
    NightSalute(String),
}

pub fn main() {
    let user_a_lang = LanguageKind::Indonesian;
    let user_b_lang = LanguageKind::Javanese;
    let user_c_lang = LanguageKind::English;

    user_a_lang.print();
    user_b_lang.print();
    user_c_lang.print();

    let user_a = Language {
        kind: user_a_lang,
        value: Phrase::MorningSalute(String::from("Hai, Selamat Pagi!")),
    };

    let user_b = Language {
        kind: user_b_lang,
        value: Phrase::NightSalute(String::from("Sugeng Dalu!")),
    };

    let user_c = Language {
        kind: user_c_lang,
        value: Phrase::MorningSalute(String::from("Hi! Good Morning!")),
    };

    user_a.print();
    user_b.print();
    user_c.print();
}
