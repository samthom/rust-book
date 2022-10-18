fn main() {
    let mut words: Vec<String> = Vec::new();

    words.push(String::from("Hello"));
    words.push(String::from("First"));
    words.push(String::from("Ok"));

    let vowels = vec!["a", "A", "e", "E", "i", "I", "o", "O", "u", "U"];
    let vowel_prefix = "hay";
    let consonant_prefix = "ay";

    for word in words {
        let mut vowel = false;
        for char in &vowels {
            vowel = word.starts_with(char);
            if vowel {
                println!("{}-{}", word, vowel_prefix);
                break;
            }
        }
        if !vowel {
            let ch = word.chars().nth(0).unwrap();
            println!("{}-{}{}", &word[1..], ch, consonant_prefix);
        }
    }
}
