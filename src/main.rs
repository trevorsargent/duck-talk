use std::io::{stdout, Write};

use curl::easy::Easy;

// fetch a random word
fn get_random_word() -> String {
    return String::from("lemniscate");
}

// fetch the word's definition
fn get_definition(_word: String) -> String {
    return String::from("name of symbol representing infinity");
}

// construct the pretty display
fn prettify(_word: String, _definition: String) -> String {
    return format!(
        "pretend the following is beautifully formatted\n{}: {}",
        _word, _definition
    );
}

fn parseResult(data: &[u8]) -> json::JsonValue {
    return json::parse(
        r#"data
        "#,
    )
    .unwrap();
}

fn main() {
    let mut easy = Easy::new();
    easy.url("https://api.wordnik.com/v4/words.json/randomWord?hasDictionaryDef=true&minLength=5&maxLength=-1&api_key=5vl8hqmmi8dw4lwe13jcd5zf0q3cenlh1zqci78gk2jp1wdbj").unwrap();

    easy.write_function().unwrap();
    easy.perform().unwrap();
    // let word = get_random_word();
    // let definition = get_definition(word.clone());
    // println!("{}", prettify(word, definition));
}
