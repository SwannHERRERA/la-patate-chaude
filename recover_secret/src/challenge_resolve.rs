use crate::models::{RecoverSecret, RecoverSecretInput, RecoverSecretOutput};

pub fn solve_secret_sentence_challenge(input: &RecoverSecretInput) -> RecoverSecretOutput {
    let mut tuples = retrieve_tuples_from_letters(&input);
    let secret_sentence = retrieve_secret_sentence_from_tuples(&mut tuples);

    RecoverSecretOutput { secret_sentence }
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

fn retrieve_secret_sentence_from_tuples(tuples: &mut Vec<Vec<char>>) -> String {
    let mut propositions: Vec<String> = Vec::new();
    retrieve_possible_strings_from_tuples(tuples, &mut propositions);
    retrieve_secret_sentence_from_possible_strings(&propositions)
}

fn retrieve_secret_sentence_from_possible_strings(propositions: &Vec<String>) -> String {
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

    tuple.iter().enumerate().for_each(|(index, c)| {
        let mut new_propositions: Vec<String> = Vec::new();
        match index {
            0 => {
                if tuple_length > 1 {
                    // there is at least one element after
                    propositions.iter().for_each(|proposition| {
                        if is_present(proposition, c) {
                            // check if is present before next tuple char
                            if is_present(proposition, tuple.get(index + 1).unwrap()) {
                                // get string before, insert, concat...
                            } else {
                                let mut new_proposition = proposition.clone();
                                new_proposition.push(*c);
                                new_propositions.push(new_proposition);
                            }
                        } else {
                            // not present, insert before next char
                            let mut new_proposition = get_string_before(proposition, tuple.get(index + 1).unwrap());
                            new_proposition.push(*c);
                            new_proposition.push(*tuple.get(index + 1).unwrap());
                            new_proposition.push_str(&get_string_after(proposition, tuple.get(index + 1).unwrap()));

                            new_propositions.push(new_proposition);
                        }
                    });
                } else {
                    // single tuple, insert wherever we want if not present
                }
            }
            index_match if index_match == tuple_length - 1 => {
                // last element, there is at least one element before
            }
            _ => {
                // middle element, there is at least one element before and one element after
            }
        }

        propositions.clear();
        propositions.append(&mut new_propositions);
    });
    propositions
}

fn get_string_before(string: &String, character: &char) -> String {
    let mut new_string = String::new();
    for c in string.chars() {
        if c == *character {
            break;
        }
        new_string.push(c);
    }
    new_string
}

fn get_string_after(string: &String, character: &char) -> String {
    let mut new_string = String::new();
    let index = string.chars().position(|c| c == *character).unwrap();
    for c in string.chars().skip(index + 1) {
        new_string.push(c);
    }
    new_string
}

fn is_present(string: &String, character: &char) -> bool {
    string.chars().any(|c| c == *character)
}