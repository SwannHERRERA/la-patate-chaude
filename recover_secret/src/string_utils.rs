use std::collections::HashMap;

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
    string.chars().any(|c| c == *character)
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
        return word_vec.iter().any(|w| w == word);
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

#[cfg(test)]
mod tests {
    use crate::string_utils::{
        add_char_at_index, get_string_after_first_occurrence, get_string_after_last_occurrence,
        get_string_before_first_occurrence, get_string_before_last_occurrence, is_present,
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
}
