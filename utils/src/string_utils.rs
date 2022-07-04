use std::collections::HashMap;

use rand::Rng;

pub fn get_string_after_last_occurrence(string: &String, character: &char) -> String {
    let mut new_string = String::new();
    let index = rfind_utf8(string, *character);

    if index.is_some() {
        new_string.push_str(&string[(index.unwrap() + 1)..]);
    }
    // println!(
    //     "String after last occ '{}': '{}' -> '{}'",
    //     character, string, new_string
    // );
    new_string
}

// return string before last occurrence of character
pub fn get_string_before_last_occurrence(string: &String, character: &char) -> String {
    let mut new_string = String::new();
    let index = rfind_utf8(string, *character);

    if index.is_some() {
        new_string.push_str(&string[..index.unwrap()]);
    }
    // println!(
    //     "String before last occ '{}': '{}' -> '{}'",
    //     character, string, new_string
    // );
    new_string
}

pub fn get_string_before_first_occurrence(string: &String, character: &char) -> String {
    let mut new_string = String::new();
    let index = find_utf8(string, *character);

    if index.is_some() {
        new_string.push_str(&string[..index.unwrap()]);
    }

    // println!(
    //     "String before first occ '{}': '{}' -> '{}'",
    //     character, string, new_string
    // );
    new_string
}

pub fn get_string_after_first_occurrence(string: &String, character: &char) -> String {
    let mut new_string = String::new();
    let index = find_utf8(string, *character);

    if index.is_some() {
        new_string.push_str(&string[(index.unwrap() + 1)..]);
    }

    // println!(
    //     "String after first occ '{}': '{}' -> '{}'",
    //     character, string, new_string
    // );
    new_string
}

pub fn get_string_after_n_occurrence(
    string: &String,
    character: &char,
    occurrence: &usize,
) -> String {
    let mut new_string = String::new();
    let index = find_n_utf8(string, *character, occurrence);

    if index.is_some() {
        new_string.push_str(&string[(index.unwrap() + 1)..]);
    }

    // println!(
    //     "String after first occ '{}': '{}' -> '{}'",
    //     character, string, new_string
    // );
    new_string
}

pub fn get_string_before_sequence(string: &String, sequence: &String) -> String {
    let mut new_string = String::new();
    let index = rfind_sequence_utf8(string, sequence);

    if index.is_some() {
        new_string.push_str(&string[..index.unwrap()]);
    }

    // println!(
    //     "String before sequence '{}': '{}' -> '{}'",
    //     sequence, string, new_string
    // );
    new_string
}

pub fn get_string_after_sequence(string: &String, sequence: &String) -> String {
    let mut new_string = String::new();
    let index = find_sequence_utf8(string, sequence);

    if index.is_some() {
        new_string.push_str(&string[(index.unwrap() + sequence.len())..]);
    }

    // println!(
    //     "String before sequence '{}': '{}' -> '{}'",
    //     sequence, string, new_string
    // );
    new_string
}

pub fn get_string_after_vec_sequence(string: &String, sequence: &Vec<char>) -> String {
    let mut new_string = string.clone();

    for char_sequence in sequence {
        new_string = get_string_after_first_occurrence(&new_string, char_sequence);
    }

    new_string
}

pub fn get_string_before_vec_sequence(string: &String, sequence: &Vec<char>) -> String {
    let mut new_string = string.clone();

    for char_sequence in sequence.clone().iter().rev() {
        new_string = get_string_before_last_occurrence(&new_string, char_sequence);
    }

    new_string
}

pub fn get_string_before_vec_sequence_inclusive(string: &String, sequence: &Vec<char>) -> String {
    let after_string = get_string_after_vec_sequence(string, sequence);
    if after_string.len() == 0 {
        return string.clone();
    }
    get_string_before_sequence(string, &after_string)
}

pub fn get_string_after_vec_sequence_inclusive(string: &String, sequence: &Vec<char>) -> String {
    let before_string = get_string_before_vec_sequence(string, sequence);
    if before_string.len() == 0 {
        return string.clone();
    }
    get_string_after_sequence(string, &before_string)
}

pub fn get_string_before_n_occurrence(
    string: &String,
    character: &char,
    occurrence: &usize,
) -> String {
    let mut new_string = String::new();
    let index = find_n_utf8(string, *character, occurrence);

    if index.is_some() {
        new_string.push_str(&string[..index.unwrap()]);
    }

    // println!(
    //     "String after first occ '{}': '{}' -> '{}'",
    //     character, string, new_string
    // );
    new_string
}

pub fn add_char_at_index(string: &String, character: &char, index: &usize) -> String {
    if string.len() == 0 {
        return character.to_string();
    }
    let mut new_string = String::new();
    if *index >= string.len() {
        new_string.push_str(string);
        new_string.push(*character);
    } else {
        new_string.push_str(&string[..*index]);
        new_string.push(*character);
        new_string.push_str(&string[*index..]);
    }
    // println!(
    //     "String after adding char at index '{}': '{}' -> '{}'",
    //     character, string, new_string
    // );
    new_string
}

pub fn is_present(string: &String, character: &char) -> bool {
    string.contains(*character)
}

pub fn count_spaces_in_string(string: &String) -> usize {
    string.chars().filter(|&c| c == ' ').count()
}

pub fn word_count(string: &String) -> usize {
    string.split_whitespace().count()
}

pub fn generate_dictionary_hashmap(dictionary: &String) -> HashMap<char, Vec<String>> {
    let mut dictionary_hashmap: HashMap<char, Vec<String>> = HashMap::new();
    let mut current_char: char = ' ';
    let mut word_vec: Vec<String> = Vec::new();

    dictionary.split_whitespace().for_each(|word| {
        if word.chars().nth(0).unwrap() != current_char {
            dictionary_hashmap.insert(current_char, word_vec.clone());
            current_char = word.chars().nth(0).unwrap();
            word_vec = Vec::new();
            word_vec.push(word.to_string());
        } else {
            word_vec.push(word.to_string());
        }
    });

    return dictionary_hashmap;
}

pub fn is_word_in_dictionary(word: &String, dictionary: &HashMap<char, Vec<String>>) -> bool {
    let first_char = word.chars().next().unwrap();
    if dictionary.contains_key(&first_char) {
        let word_vec = dictionary.get(&first_char).unwrap();
        return word_vec.contains(word);
    }
    false
}

pub fn rfind_utf8(s: &str, chr: char) -> Option<usize> {
    if let Some(rev_pos) = s.chars().rev().position(|c| c == chr) {
        Some(s.chars().count() - rev_pos - 1)
    } else {
        None
    }
}

pub fn find_utf8(s: &str, chr: char) -> Option<usize> {
    if let Some(pos) = s.chars().position(|c| c == chr) {
        Some(pos)
    } else {
        None
    }
}

pub fn find_sequence_utf8(s: &str, sequence: &str) -> Option<usize> {
    if let Some(pos) = s.find(sequence) {
        Some(pos)
    } else {
        None
    }
}

pub fn rfind_sequence_utf8(s: &str, sequence: &str) -> Option<usize> {
    if let Some(rev_pos) = s.rfind(sequence) {
        Some(rev_pos)
    } else {
        None
    }
}

pub fn add_spaces_in_sequence(sequence: &str, nb_spaces: &usize) -> String {
    let mut new_sequence = String::new();
    let mut nb_spaces_left = *nb_spaces;
    for (index, current_char) in sequence[..sequence.len() - 1].chars().enumerate() {
        new_sequence.push(current_char);
        if current_char == ' ' || sequence.chars().nth(index + 1).unwrap() == ' ' {
            continue;
        }

        if nb_spaces_left > 0 {
            new_sequence.push(' ');
            nb_spaces_left -= 1;
        }
    }
    new_sequence.push(sequence.chars().last().unwrap());
    new_sequence
}

pub fn find_n_utf8(s: &str, chr: char, n: &usize) -> Option<usize> {
    let mut count = 0;
    let mut index = 0;
    for c in s.chars() {
        if c == chr {
            count += 1;
        }
        if count == *n {
            return Some(index);
        }
        index += 1;
    }
    None
}

pub fn generate_random_tuple(length: &usize) -> Vec<char> {
    let mut rng = rand::thread_rng();
    let mut random_string = Vec::new();
    for _ in 0..*length {
        // random char, only letters
        random_string.push(rng.sample(rand::distributions::Alphanumeric) as char);
    }
    random_string
}

pub fn is_sequence_valid(string: &String, sequence: &Vec<char>) -> bool {
    let mut string_check = string.clone();
    for char in sequence {
        if !is_present(&string_check, char) {
            return false;
        }
        string_check = get_string_after_first_occurrence(&string_check, char);
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::string_utils::{
        add_char_at_index, add_spaces_in_sequence, get_string_after_first_occurrence,
        get_string_after_last_occurrence, get_string_after_n_occurrence, get_string_after_sequence,
        get_string_after_vec_sequence, get_string_after_vec_sequence_inclusive,
        get_string_before_first_occurrence, get_string_before_last_occurrence,
        get_string_before_n_occurrence, get_string_before_sequence, get_string_before_vec_sequence,
        get_string_before_vec_sequence_inclusive, is_present,
    };

    #[test]
    fn test_is_present() {
        let string = "hello world".to_string();
        assert!(is_present(&string, &'o'));
        assert!(!is_present(&string, &'z'));
    }

    #[test]
    fn test_get_string_after_last_occurrence() {
        let string = "hello world".to_string();
        let new_string = get_string_after_last_occurrence(&string, &'o');
        assert_eq!(new_string, "rld".to_string());
        let not_found_string = get_string_after_last_occurrence(&string, &'z');
        assert_eq!(not_found_string, "".to_string());
        let empty_string = get_string_after_last_occurrence(&"".to_string(), &'z');
        assert_eq!(empty_string, "".to_string());
        let empty_string_2 = get_string_after_last_occurrence(&"z".to_string(), &'z');
        assert_eq!(empty_string_2, "".to_string());
    }

    #[test]
    fn test_get_string_before_last_occurrence() {
        let string = "hello world".to_string();
        let new_string = get_string_before_last_occurrence(&string, &'o');
        assert_eq!(new_string, "hello w".to_string());
        let not_found_string = get_string_before_last_occurrence(&string, &'z');
        assert_eq!(not_found_string, "".to_string());
        let empty_string = get_string_before_last_occurrence(&"z".to_string(), &'z');
        assert_eq!(empty_string, "".to_string());
    }

    #[test]
    fn test_get_string_before_first_occurrence() {
        let string = "hello world".to_string();
        let new_string = get_string_before_first_occurrence(&string, &'o');
        assert_eq!(new_string, "hell".to_string());
        let not_found_string = get_string_before_first_occurrence(&string, &'z');
        assert_eq!(not_found_string, "".to_string());
    }

    #[test]
    fn test_get_string_after_first_occurrence() {
        let string = "hello world".to_string();
        let new_string = get_string_after_first_occurrence(&string, &'o');
        assert_eq!(new_string, " world".to_string());
        let not_found_string = get_string_after_first_occurrence(&string, &'z');
        assert_eq!(not_found_string, "".to_string());
    }

    #[test]
    fn test_add_char_at_index() {
        let string = "hello world".to_string();
        let new_string = add_char_at_index(&string, &'z', &5);
        assert_eq!(new_string, "helloz world".to_string());
    }

    #[test]
    fn test_add_char_at_index_2() {
        let string = "hello world".to_string();
        let new_string = add_char_at_index(&string, &'z', &0);
        assert_eq!(new_string, "zhello world".to_string());
    }

    #[test]
    fn test_add_char_at_index_in_empty_string() {
        let string = "".to_string();
        let new_string = add_char_at_index(&string, &'z', &0);
        assert_eq!(new_string, "z".to_string());
    }
    #[test]
    fn test_add_char_at_index_in_end_of_string() {
        let string = "hello".to_string();
        let new_string = add_char_at_index(&string, &'z', &5);
        assert_eq!(new_string, "helloz".to_string());
    }

    #[test]
    fn text_get_string_after_n_occurrence() {
        let string = "hello world".to_string();
        let new_string = get_string_after_n_occurrence(&string, &'o', &2);
        assert_eq!(new_string, "rld".to_string());
        let not_found_string = get_string_after_n_occurrence(&string, &'z', &2);
        assert_eq!(not_found_string, "".to_string());
        let empty_string = get_string_after_n_occurrence(&"".to_string(), &'z', &2);
        assert_eq!(empty_string, "".to_string());
        let empty_string_2 = get_string_after_n_occurrence(&"z".to_string(), &'z', &2);
        assert_eq!(empty_string_2, "".to_string());
    }

    #[test]
    fn test_get_string_before_n_occurrence() {
        let string = "hello world".to_string();
        let new_string = get_string_before_n_occurrence(&string, &'o', &2);
        assert_eq!(new_string, "hello w".to_string());
        let not_found_string = get_string_before_n_occurrence(&"zzzzz".to_string(), &'z', &3);
        assert_eq!(not_found_string, "zz".to_string());
        let empty_string = get_string_before_n_occurrence(&"z".to_string(), &'z', &2);
        assert_eq!(empty_string, "".to_string());
    }

    #[test]
    fn test_get_string_before_sequence() {
        let string = "hello world".to_string();
        let new_string = get_string_before_sequence(&string, &"world".to_string());
        assert_eq!(new_string, "hello ".to_string());
        let not_found_string = get_string_before_sequence(&string, &"zzzzz".to_string());
        assert_eq!(not_found_string, "".to_string());
        let empty_string = get_string_before_sequence(&"z".to_string(), &"z".to_string());
        assert_eq!(empty_string, "".to_string());
    }

    #[test]
    fn test_get_string_after_sequence() {
        let string = "hello world".to_string();
        let new_string = get_string_after_sequence(&string, &"hello".to_string());
        assert_eq!(new_string, " world".to_string());
        let not_found_string = get_string_after_sequence(&string, &"zzzzz".to_string());
        assert_eq!(not_found_string, "".to_string());
        let empty_string = get_string_after_sequence(&"z".to_string(), &"z".to_string());
        assert_eq!(empty_string, "".to_string());
    }

    #[test]
    fn test_get_string_after_vec_sequence() {
        let new_string = get_string_after_vec_sequence(&"hello world".to_string(), &vec!['h', 'o']);
        assert_eq!(new_string, " world".to_string());
        let new_string = get_string_after_vec_sequence(&"hello world".to_string(), &vec!['h', ' ']);
        assert_eq!(new_string, "world".to_string());
        let new_string =
            get_string_after_vec_sequence(&"hello world".to_string(), &vec!['h', ' ', 'o']);
        assert_eq!(new_string, "rld".to_string());
        let new_string =
            get_string_after_vec_sequence(&"hello world".to_string(), &vec!['h', ' ', 'd']);
        assert_eq!(new_string, "".to_string());
    }

    #[test]
    fn test_get_string_before_vec_sequence() {
        let new_string =
            get_string_before_vec_sequence(&"hello world".to_string(), &vec!['o', 'o']);
        assert_eq!(new_string, "hell".to_string());
        let new_string =
            get_string_before_vec_sequence(&"hello world".to_string(), &vec![' ', 'o']);
        assert_eq!(new_string, "hello".to_string());
        let new_string = get_string_before_vec_sequence(&"hello world".to_string(), &vec!['o']);
        assert_eq!(new_string, "hello w".to_string());
        let new_string =
            get_string_before_vec_sequence(&"hello world".to_string(), &vec!['h', ' ', 'd']);
        assert_eq!(new_string, "".to_string());
    }

    #[test]
    fn test_get_string_before_vec_sequence_inclusive() {
        let new_string =
            get_string_before_vec_sequence_inclusive(&"hello world".to_string(), &vec!['o', 'o']);
        assert_eq!(new_string, "hello wo".to_string());
        let new_string =
            get_string_before_vec_sequence_inclusive(&"hello world".to_string(), &vec!['o', ' ']);
        assert_eq!(new_string, "hello ".to_string());
        let new_string =
            get_string_before_vec_sequence_inclusive(&"hello world".to_string(), &vec!['o']);
        assert_eq!(new_string, "hello".to_string());
        let new_string = get_string_before_vec_sequence_inclusive(
            &"hello world".to_string(),
            &vec!['h', ' ', 'd'],
        );
        assert_eq!(new_string, "hello world".to_string());
    }

    #[test]
    fn test_get_string_after_vec_sequence_inclusive() {
        let new_string =
            get_string_after_vec_sequence_inclusive(&"hello world".to_string(), &vec!['o', 'o']);
        assert_eq!(new_string, "o world".to_string());
        let new_string =
            get_string_after_vec_sequence_inclusive(&"hello world".to_string(), &vec![' ', 'o']);
        assert_eq!(new_string, " world".to_string());
        let new_string =
            get_string_after_vec_sequence_inclusive(&"hello world".to_string(), &vec!['o', ' ']);
        assert_eq!(new_string, "o world".to_string());
        let new_string = get_string_after_vec_sequence_inclusive(
            &"hello world".to_string(),
            &vec!['h', ' ', 'd'],
        );
        assert_eq!(new_string, "hello world".to_string());
    }

    #[test]
    fn test_add_spaces_in_sequence() {
        let new_string = add_spaces_in_sequence(&"hello world", &1);
        assert_eq!(new_string, "h ello world".to_string());
        let new_string = add_spaces_in_sequence(&"hello world", &0);
        assert_eq!(new_string, "hello world".to_string());
        let new_string = add_spaces_in_sequence(&"hello world", &6);
        assert_eq!(new_string, "h e l l o w o rld".to_string());
    }
}
