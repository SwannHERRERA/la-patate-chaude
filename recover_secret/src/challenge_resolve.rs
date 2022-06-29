use crate::models::{RecoverSecretInput, RecoverSecretOutput};
use crate::string_utils::{get_string_after_first_occurrence, get_string_after_last_occurrence, get_string_before_first_occurrence, get_string_before_last_occurrence, is_present};

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


#[cfg(test)]
mod tests {
    use crate::challenge_resolve::solve_secret_sentence_challenge;
    use crate::models::RecoverSecretInput;

    #[test]
    fn test_solve_secret_sentence_challenge() {
        let recover_secret_input: RecoverSecretInput = RecoverSecretInput {
            word_count: 1,
            letters: "iffiiilfatroridato".parse().unwrap(),
            tuple_sizes: vec![3, 3, 3, 3, 3, 3],
        };

        let answer = solve_secret_sentence_challenge(&recover_secret_input);
        assert_eq!(answer.secret_sentence, "iiriflfatrod".to_string());
    }

    #[test]
    fn another_test_solve_secret_sentence_challenge() {
        let recover_secret_input: RecoverSecretInput = RecoverSecretInput {
            word_count: 1,
            letters: "rtlthotzo".parse().unwrap(),
            tuple_sizes: vec![3, 3, 3],
        };

        let answer = solve_secret_sentence_challenge(&recover_secret_input);
        assert_eq!(answer.secret_sentence, "rtlhzo".to_string());
    }

    #[test]
    fn test_solve_secret_sentence_challenge_multiple_words() {
        let recover_secret_input: RecoverSecretInput = RecoverSecretInput {
            word_count: 6,
            letters: "iffiiilfatroridato".parse().unwrap(),
            tuple_sizes: vec![3, 3, 3, 3, 3, 3],
        };

        let answer = solve_secret_sentence_challenge(&recover_secret_input);
        assert_eq!(answer.secret_sentence, "i i r i f lfatrod".to_string());
    }
}