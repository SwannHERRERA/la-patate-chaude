use crate::models::{RecoverSecretInput, RecoverSecretOutput};

pub fn solve_secret_sentence_challenge(input: &RecoverSecretInput) -> RecoverSecretOutput {
    let mut tuples = retrieve_tuples_from_letters(&input);
    let secret_sequence = retrieve_secret_sequence_from_tuples(&mut tuples);
    let secret_sentence = retrieve_secret_sentence_from_secret_sequence(&secret_sequence, &input.word_count);
    RecoverSecretOutput { secret_sentence }
}

fn retrieve_secret_sentence_from_secret_sequence(sequence: &String, word_count: &usize) -> String {
    let mut sentence = "".to_string();
    let mut sequence_index: usize = 0;
    let mut word_count_left = word_count - 1;
    while word_count_left > 0 {
        sentence.push(sequence.chars().nth(sequence_index).unwrap());
        sentence.push(' ');
        word_count_left -= 1;
        sequence_index += 1;
    }
    for index in sequence_index..sequence.len() {
        sentence.push(sequence.chars().nth(index).unwrap());
    }
    sentence
}

fn retrieve_tuples_from_letters(input: &RecoverSecretInput) -> Vec<Vec<char>> {
    let mut tuples: Vec<Vec<char>> = Vec::new();
    let mut current_index = 0;
    input.tuple_sizes.iter().for_each(|size| {
        let mut tuple: Vec<char> = Vec::new();
        for i in current_index..(current_index + *size) {
            tuple.push(input.letters.chars().nth(i).unwrap());
        }
        tuples.push(tuple);
        current_index += *size;
    });
    tuples
}

fn retrieve_secret_sequence_from_tuples(tuples: &mut Vec<Vec<char>>) -> String {
    let mut propositions: Vec<String> = Vec::new();
    retrieve_possible_strings_from_tuples(tuples, &mut propositions);
    retrieve_secret_sequence_from_possible_strings(&propositions)
}

fn retrieve_secret_sequence_from_possible_strings(propositions: &Vec<String>) -> String {
    if propositions.len() > 0 {
        propositions[0].clone()
    } else {
        panic!("No solution found.");
    }
}

fn retrieve_possible_strings_from_tuples(
    tuples: &mut Vec<Vec<char>>,
    propositions: &mut Vec<String>,
) {
    if tuples.is_empty() {
        return;
    }

    let tuple = tuples.remove(0);
    if propositions.is_empty() {
        let mut string = String::new();
        tuple.iter().for_each(|c| {
            string.push(*c);
        });
        propositions.push(string);
    } else {
        let mut other_propositions: Vec<String> =
            retrieve_possible_strings_from_tuple(tuple, propositions);
        propositions.clear();
        propositions.append(&mut other_propositions);
    }

    retrieve_possible_strings_from_tuples(tuples, propositions);
}

fn retrieve_possible_strings_from_tuple(
    tuple: Vec<char>,
    propositions: &Vec<String>,
) -> Vec<String> {
    let mut other_propositions: Vec<String> = Vec::new();

    propositions.iter().for_each(|proposition| {
        let new_propositions = retrieve_possible_strings_from_string(&tuple, proposition);
        new_propositions.iter().for_each(|new_proposition| {
            if !other_propositions.contains(new_proposition) {
                other_propositions.push(new_proposition.clone());
            }
        });
    });
    other_propositions
}

fn retrieve_possible_strings_from_string(tuple: &Vec<char>, old_proposition: &String) -> Vec<String> {
    let mut propositions: Vec<String> = Vec::new();
    propositions.push(old_proposition.clone());
    let tuple_length = tuple.len();

    tuple.iter().enumerate().for_each(|(index, current_char)| {
        let mut new_propositions: Vec<String> = Vec::new();
        match index {
            0 => {
                if tuple_length > 1 {
                    // there is at least one element after
                    let next_char = tuple.get(index + 1).unwrap();

                    propositions.iter().for_each(|proposition| {
                        let mut new_proposition: String;
                        if !is_present(proposition, next_char) {
                            new_proposition = proposition.clone();
                        } else {
                            new_proposition = get_string_before_last_occurrence(proposition, next_char);
                        }

                        if !is_present(&new_proposition, current_char) {
                            new_proposition.push(*current_char);
                        }

                        if is_present(proposition, next_char) {
                            new_proposition.push(*next_char);
                            new_proposition.push_str(&get_string_after_last_occurrence(proposition, next_char));
                        }
                        // println!("'{}' -> '{}'", proposition, new_proposition);
                        new_propositions.push(new_proposition);
                    });
                } else {
                    // single tuple, insert wherever we want if not present
                    propositions.iter().for_each(|proposition| {
                        let mut new_proposition = proposition.clone();
                        if !is_present(&new_proposition, current_char) {
                            new_proposition.push(*current_char);
                        }
                        // println!("'{}' -> '{}'", proposition, new_proposition);
                        new_propositions.push(new_proposition);
                    });
                }
            }
            index_match if index_match == tuple_length - 1 => {
                // last element, there is at least one element before
                let previous_char = tuple.get(index - 1).unwrap();

                propositions.iter().for_each(|proposition| {
                    let mut new_proposition: String;

                    if is_present(proposition, previous_char) {
                        new_proposition = get_string_after_first_occurrence(proposition, previous_char);
                    } else {
                        new_proposition = proposition.clone();
                    }

                    if !is_present(&new_proposition, current_char) {
                        new_proposition.push(*current_char);
                    }

                    if is_present(proposition, previous_char) {
                        let mut tmp = get_string_before_first_occurrence(proposition, previous_char);
                        tmp.push(*previous_char);
                        tmp.push_str(&new_proposition);
                        new_proposition = tmp;
                    }
                    // println!("'{}' -> '{}'", proposition, new_proposition);
                    new_propositions.push(new_proposition);
                });
            }
            _ => {
                // middle element, there is at least one element before and one element after
                let previous_char = tuple.get(index - 1).unwrap();
                let next_char = tuple.get(index + 1).unwrap();

                propositions.iter().for_each(|proposition| {
                    let mut new_proposition: String;

                    if is_present(proposition, previous_char) {
                        new_proposition = get_string_after_first_occurrence(proposition, previous_char);
                    } else {
                        new_proposition = proposition.clone();
                    }
                    if is_present(&new_proposition, next_char) {
                        new_proposition = get_string_before_last_occurrence(&new_proposition, next_char);
                    }

                    if !is_present(&new_proposition, current_char) {
                        new_proposition.push(*current_char);
                    }

                    if is_present(proposition, previous_char) {
                        let mut tmp = get_string_before_first_occurrence(proposition, previous_char);
                        tmp.push(*previous_char);
                        tmp.push_str(&new_proposition);
                        new_proposition = tmp;
                    }
                    if is_present(proposition, next_char) {
                        let tmp = get_string_after_last_occurrence(proposition, next_char);
                        new_proposition.push(*next_char);
                        new_proposition.push_str(&tmp);
                    }
                    // println!("'{}' -> '{}'", proposition, new_proposition);
                    new_propositions.push(new_proposition);
                })
            }
        }

        propositions.clear();
        propositions.append(&mut new_propositions);
    });
    propositions
}

fn get_string_after_last_occurrence(string: &String, character: &char) -> String {
    let mut new_string = String::new();
    let mut index: i32 = (string.len() as i32) - 1;

    while index >= 0 && string.chars().nth(index as usize).unwrap() != *character {
        index -= 1;
    }

    if index >= 0 {
        for i in index + 1..(string.len() as i32) {
            new_string.push(string.chars().nth(i as usize).unwrap())
        }
    }
    // println!("String after last occ '{}': '{}' -> '{}'", character, string, new_string);

    new_string
}

// return string before last occurrence of character
fn get_string_before_last_occurrence(string: &String, character: &char) -> String {
    let mut new_string = String::new();
    let mut found_index: i32 = (string.len() - 1) as i32;

    while found_index >= 0 && string.chars().nth(found_index as usize).unwrap() != *character {
        found_index -= 1;
    };

    if found_index >= 0 {
        for i in 0..found_index {
            new_string.push(string.chars().nth(i as usize).unwrap())
        }
    }
    // println!("String before last occ '{}': '{}' -> '{}'", character, string, new_string);
    new_string
}

fn get_string_before_first_occurrence(string: &String, character: &char) -> String {
    let mut new_string = String::new();

    for i in 0..string.len() {
        if string.chars().nth(i).unwrap() == *character {
            break;
        }
        new_string.push(string.chars().nth(i).unwrap());
    }

    // println!("String before first occ '{}': '{}' -> '{}'", character, string, new_string);
    new_string
}

fn get_string_after_first_occurrence(string: &String, character: &char) -> String {
    let mut new_string = String::new();
    let option_index = string.chars().position(|c| c == *character);
    if option_index.is_some() {
        let index = option_index.unwrap();
        for c in string.chars().skip(index + 1) {
            new_string.push(c);
        }
    }
    // println!("String after first occ '{}': '{}' -> '{}'", character, string, new_string);
    new_string
}

fn is_present(string: &String, character: &char) -> bool {
    string.chars().any(|c| c == *character)
}