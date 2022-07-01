use std::collections::HashMap;

use crate::models::{RecoverSecretInput, RecoverSecretOutput};
use crate::string_utils::{
    add_char_at_index, add_spaces_in_sequence, get_string_after_first_occurrence,
    get_string_after_last_occurrence, get_string_after_n_occurrence,
    get_string_before_first_occurrence, get_string_before_last_occurrence,
    get_string_before_n_occurrence, is_present, is_word_in_dictionary, word_count,
};

pub fn solve_secret_sentence_challenge(
    input: &RecoverSecretInput,
    dictionary: &HashMap<char, Vec<String>>,
) -> RecoverSecretOutput {
    let mut tuples = retrieve_tuples_from_letters(&input);
    let secret_sentence =
        retrieve_secret_sentence_from_tuples(&mut tuples, &input.word_count, dictionary, &true);
    RecoverSecretOutput { secret_sentence }
}

pub fn solve_secret_string_challenge(input: &RecoverSecretInput) -> RecoverSecretOutput {
    let mut tuples = retrieve_tuples_from_letters(&input);
    let secret_sentence = retrieve_secret_sentence_from_tuples(
        &mut tuples,
        &input.word_count,
        &HashMap::new(),
        &false,
    );
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

fn retrieve_secret_sentence_from_tuples(
    tuples: &mut Vec<Vec<char>>,
    nb_words: &usize,
    dictionary: &HashMap<char, Vec<String>>,
    is_sentence_valid: &bool,
) -> String {
    let propositions =
        retrieve_possible_strings_from_tuples(tuples, &mut Vec::new(), nb_words, is_sentence_valid);

    // display_possibilities(&propositions);

    if propositions.len() > 0 {
        if *is_sentence_valid {
            return find_sentence(&propositions, dictionary);
        }
        return find_sequence(&propositions, nb_words);
    } else {
        panic!("No solution found.");
    }
}

fn find_sequence(propositions: &Vec<String>, nb_words: &usize) -> String {
    let sequence = propositions[0].clone();
    let current_word_count = word_count(&sequence);
    if current_word_count == *nb_words {
        return sequence;
    }

    return add_spaces_in_sequence(&sequence, &(*nb_words - current_word_count));
}

fn display_possibilities(propositions: &Vec<String>) {
    if propositions.len() > 0 {
        println!("===================================================");
        for i in 0..propositions.len() {
            println!("{:?}", propositions[i])
        }
        println!("===================================================");
        println!("{} possibilities ...", propositions.len(),);
    } else {
        println!("No solution found.");
    }
}

fn retrieve_possible_strings_from_tuples(
    tuples: &mut Vec<Vec<char>>,
    propositions: &mut Vec<String>,
    nb_words: &usize,
    is_sentence_valid: &bool,
) -> Vec<String> {
    if tuples.is_empty() {
        println!("{} propositions found.", propositions.len());
        return propositions.clone();
    } else {
        println!("{} propositions found.", propositions.len());
    }

    let tuple = tuples.remove(0);
    return if propositions.is_empty() {
        let string = tuple.iter().collect();
        propositions.push(string);
        retrieve_possible_strings_from_tuples(tuples, propositions, nb_words, is_sentence_valid)
    } else {
        let mut other_propositions: Vec<String> =
            retrieve_possible_strings_from_tuple(tuple, propositions, nb_words, is_sentence_valid);
        retrieve_possible_strings_from_tuples(
            tuples,
            &mut other_propositions,
            nb_words,
            is_sentence_valid,
        )
    };
}

fn retrieve_possible_strings_from_tuple(
    tuple: Vec<char>,
    propositions: &Vec<String>,
    nb_words: &usize,
    is_sentence_valid: &bool,
) -> Vec<String> {
    let mut other_propositions: Vec<String> = Vec::new();

    propositions.iter().for_each(|proposition| {
        let mut new_propositions =
            retrieve_possible_strings_from_string(&tuple, proposition, nb_words, is_sentence_valid);
        other_propositions.append(&mut new_propositions);
        // new_propositions.iter().for_each(|new_proposition| {
        //     if !other_propositions.contains(new_proposition) {
        //         other_propositions.push(new_proposition.clone());
        //     }
        // });
    });
    other_propositions
}

fn retrieve_possible_strings_from_string(
    tuple: &Vec<char>,
    old_proposition: &String,
    nb_words: &usize,
    is_sentence_valid: &bool,
) -> Vec<String> {
    let mut propositions: Vec<String> = Vec::new();
    propositions.push(old_proposition.clone());
    let tuple_length = tuple.len();

    tuple.iter().enumerate().for_each(|(index, current_char)| {
        let mut new_propositions: Vec<String> = Vec::new();
        match index {
            0 => process_first_element_of_tuple(
                tuple,
                &nb_words,
                &propositions,
                tuple_length,
                index,
                &current_char,
                &mut new_propositions,
                is_sentence_valid,
            ),
            index_match if index_match == tuple_length - 1 => {
                // last element, there is at least one element before
                process_last_element_of_tuple(
                    tuple,
                    &nb_words,
                    &propositions,
                    index,
                    &current_char,
                    &mut new_propositions,
                    is_sentence_valid,
                );
            }
            _ => {
                // middle element, there is at least one element before and one element after

                process_middle_element_of_tuple(
                    tuple,
                    &nb_words,
                    &propositions,
                    index,
                    &current_char,
                    &mut new_propositions,
                    is_sentence_valid,
                )
            }
        }

        propositions = new_propositions;
    });
    propositions
}

fn process_middle_element_of_tuple(
    tuple: &Vec<char>,
    nb_words: &usize,
    propositions: &Vec<String>,
    index: usize,
    current_char: &char,
    mut new_propositions: &mut Vec<String>,
    compute_all_possibilities: &bool,
) {
    let previous_char = tuple.get(index - 1).unwrap();
    let next_char = tuple.get(index + 1).unwrap();

    propositions.iter().for_each(|proposition| {
        let mut new_proposition: String;
        let is_current_char_present;
        let is_previous_char_present = is_present(proposition, previous_char);
        let is_next_char_present;

        if is_previous_char_present {
            new_proposition = get_string_after_first_occurrence(proposition, previous_char);
        } else {
            new_proposition = proposition.clone();
        }

        is_next_char_present = is_present(&new_proposition, next_char);

        if is_next_char_present {
            new_proposition = get_string_before_last_occurrence(&new_proposition, next_char);
        }

        is_current_char_present = is_present(&new_proposition, current_char);

        if !is_current_char_present {
            if *compute_all_possibilities {
                for i in 0..new_proposition.len() + 1 {
                    let final_proposition = add_char_at_index(&new_proposition, &current_char, &i);
                    push_proposition_with_string_between_chars(
                        &mut new_propositions,
                        previous_char,
                        next_char,
                        &is_previous_char_present,
                        &is_next_char_present,
                        proposition,
                        &final_proposition,
                        &nb_words,
                    );
                }
            } else {
                new_proposition.push(*current_char);
                push_proposition_with_string_between_chars(
                    &mut new_propositions,
                    previous_char,
                    next_char,
                    &is_previous_char_present,
                    &is_next_char_present,
                    proposition,
                    &new_proposition,
                    &nb_words,
                );
            }
        } else {
            push_proposition_with_string_between_chars(
                &mut new_propositions,
                previous_char,
                next_char,
                &is_previous_char_present,
                &is_next_char_present,
                proposition,
                &new_proposition,
                &nb_words,
            );
        }
    })
}

fn process_last_element_of_tuple(
    tuple: &Vec<char>,
    nb_words: &usize,
    propositions: &Vec<String>,
    index: usize,
    current_char: &char,
    mut new_propositions: &mut Vec<String>,
    compute_all_possibilities: &bool,
) {
    let previous_char = tuple.get(index - 1).unwrap();

    propositions.iter().for_each(|proposition| {
        let mut new_proposition: String;
        let is_previous_char_present = is_present(proposition, previous_char);
        let is_current_char_present;
        let mut previous_char_occurrence = tuple.iter().filter(|&c| c == previous_char).count();
        if previous_char == current_char {
            previous_char_occurrence -= 1;
        }

        // if is_previous_char_present {
        new_proposition =
            get_string_after_n_occurrence(proposition, previous_char, &previous_char_occurrence);
        // } else {
        //     new_proposition = proposition.clone();
        // }

        is_current_char_present = is_present(&new_proposition, current_char);

        if !is_current_char_present {
            if *compute_all_possibilities {
                for i in 0..new_proposition.len() + 1 {
                    let final_proposition = add_char_at_index(&new_proposition, &current_char, &i);
                    push_proposition_with_string_before_char(
                        &mut new_propositions,
                        previous_char,
                        &is_previous_char_present,
                        &previous_char_occurrence,
                        proposition,
                        &final_proposition,
                        &nb_words,
                    );
                }
            } else {
                new_proposition.push(*current_char);
                push_proposition_with_string_before_char(
                    &mut new_propositions,
                    previous_char,
                    &is_previous_char_present,
                    &previous_char_occurrence,
                    proposition,
                    &new_proposition,
                    &nb_words,
                );
            }
            // new_proposition.push(*current_char);
        } else {
            push_proposition_with_string_before_char(
                &mut new_propositions,
                previous_char,
                &is_previous_char_present,
                &previous_char_occurrence,
                proposition,
                &new_proposition,
                &nb_words,
            );
        }
    });
}

fn process_first_element_of_tuple(
    tuple: &Vec<char>,
    nb_words: &usize,
    propositions: &Vec<String>,
    tuple_length: usize,
    index: usize,
    current_char: &char,
    mut new_propositions: &mut Vec<String>,
    compute_all_possibilities: &bool,
) {
    if tuple_length > 1 {
        // there is at least one element after
        let next_char = tuple.get(index + 1).unwrap();

        propositions.iter().for_each(|proposition| {
            let mut new_proposition: String;
            let is_next_char_present = is_present(proposition, next_char);
            let is_current_char_present;

            if !is_next_char_present {
                new_proposition = proposition.clone();
            } else {
                new_proposition = get_string_before_last_occurrence(proposition, next_char);
            }

            is_current_char_present = is_present(&new_proposition, current_char);

            if !is_current_char_present {
                if *compute_all_possibilities {
                    for i in 0..new_proposition.len() + 1 {
                        let final_proposition =
                            add_char_at_index(&new_proposition, &current_char, &i);
                        push_proposition_with_string_after_char(
                            &mut new_propositions,
                            &next_char,
                            &is_next_char_present,
                            &proposition,
                            &final_proposition,
                            &nb_words,
                        );
                    }
                } else {
                    new_proposition.push(*current_char);
                    push_proposition_with_string_after_char(
                        &mut new_propositions,
                        &next_char,
                        &is_next_char_present,
                        &proposition,
                        &new_proposition,
                        &nb_words,
                    );
                }
            } else {
                push_proposition_with_string_after_char(
                    &mut new_propositions,
                    &next_char,
                    &is_next_char_present,
                    &proposition,
                    &new_proposition,
                    &nb_words,
                );
            }
        });
    } else {
        // single tuple, insert wherever we want if not present
        propositions.iter().for_each(|proposition| {
            let mut new_proposition = proposition.clone();
            if new_proposition.len() == 0 {
                new_proposition.push(*current_char);
                new_propositions.push(new_proposition);
            } else {
                if !is_present(&new_proposition, current_char) {
                    if *compute_all_possibilities {
                        for i in 0..new_proposition.len() + 1 {
                            new_propositions.push(add_char_at_index(
                                &new_proposition,
                                current_char,
                                &i,
                            ))
                        }
                    } else {
                        new_proposition.push(*current_char);
                        new_propositions.push(new_proposition);
                    }
                } else {
                    // println!("'{}' -> '{}'", proposition, new_proposition);
                    new_propositions.push(new_proposition);
                }
            }
        });
    }
}

fn push_proposition_with_string_between_chars(
    new_propositions: &mut Vec<String>,
    previous_char: &char,
    next_char: &char,
    is_previous_char_present: &bool,
    is_next_char_present: &bool,
    proposition: &String,
    new_proposition: &String,
    nb_words: &usize,
) {
    let mut final_proposition = new_proposition.clone();
    if *is_previous_char_present {
        let mut tmp = get_string_before_first_occurrence(proposition, previous_char);
        tmp.push(*previous_char);
        tmp.push_str(&final_proposition);
        final_proposition = tmp;
    }
    if *is_next_char_present {
        let tmp = get_string_after_last_occurrence(proposition, next_char);
        final_proposition.push(*next_char);
        final_proposition.push_str(&tmp);
    }
    // println!("'{}' -> '{}'", proposition, final_proposition);

    if word_count(&final_proposition) <= *nb_words {
        new_propositions.push(final_proposition);
    }
}

fn push_proposition_with_string_before_char(
    new_propositions: &mut Vec<String>,
    previous_char: &char,
    is_previous_char_present: &bool,
    previous_char_occurrence: &usize,
    proposition: &String,
    new_proposition: &String,
    nb_words: &usize,
) {
    let final_proposition;

    if *is_previous_char_present {
        let mut tmp =
            get_string_before_n_occurrence(proposition, previous_char, previous_char_occurrence);
        tmp.push(*previous_char);
        tmp.push_str(&new_proposition);
        final_proposition = tmp;
    } else {
        final_proposition = new_proposition.clone();
    }
    // println!("'{}' -> '{}'", proposition, final_proposition);
    if word_count(&final_proposition) <= *nb_words {
        new_propositions.push(final_proposition);
    }
}

fn push_proposition_with_string_after_char(
    new_propositions: &mut Vec<String>,
    next_char: &char,
    is_next_char_present: &bool,
    proposition: &String,
    new_proposition: &String,
    nb_words: &usize,
) {
    let mut final_proposition = new_proposition.clone();

    if *is_next_char_present {
        final_proposition.push(*next_char);
        final_proposition.push_str(&get_string_after_last_occurrence(proposition, next_char));
    }
    // println!("'{}' -> '{}'", proposition, final_proposition);
    if word_count(&final_proposition) <= *nb_words {
        new_propositions.push(final_proposition);
    }
}

fn find_sentence(possibilities: &Vec<String>, dictionary: &HashMap<char, Vec<String>>) -> String {
    for possibility in possibilities {
        let mut founded = true;
        let words: Vec<String> = possibility
            .to_ascii_lowercase()
            .split(|c: char| c == ' ' || c == '-')
            .map(|s| s.to_string())
            .collect();

        for word in words {
            if !is_word_in_dictionary(&word, dictionary) {
                founded = false;
                break;
            }
        }
        if founded {
            return possibility.clone();
        }
    }
    panic!("No sentence found");
}

#[cfg(test)]
mod tests {
    use crate::challenge_resolve::{
        solve_secret_sentence_challenge, solve_secret_string_challenge,
    };
    use crate::file_utils::read_file;
    use crate::models::RecoverSecretInput;
    use crate::string_utils::generate_dictionary_hashmap;

    #[test]
    fn test_solve_secret_sentence_challenge() {
        let dictionary = read_file("data-test/liste-de-ses-morts.dic");
        let dictionary_hashmap = generate_dictionary_hashmap(&dictionary);

        let recover_secret_input: RecoverSecretInput = RecoverSecretInput {
            word_count: 2,
            letters: "C'echCt chut cou't htu'ehuest o".parse().unwrap(),
            tuple_sizes: vec![5, 6, 5, 4, 2, 4, 5],
        };

        let answer = solve_secret_sentence_challenge(&recover_secret_input, &dictionary_hashmap);
        assert_eq!(answer.secret_sentence, "C'est chou".to_string());
    }

    #[test]
    fn test_solve_secret_string_challenge() {
        let recover_secret_input: RecoverSecretInput = RecoverSecretInput {
            word_count: 1,
            letters: "iffiiilfatroridato".parse().unwrap(),
            tuple_sizes: vec![3, 3, 3, 3, 3, 3],
        };

        let answer = solve_secret_string_challenge(&recover_secret_input);
        assert_eq!(answer.secret_sentence, "iiriflfatrod".to_string());
    }

    #[test]
    fn another_test_solve_secret_string_challenge() {
        let recover_secret_input: RecoverSecretInput = RecoverSecretInput {
            word_count: 1,
            letters: "rtlthotzo".parse().unwrap(),
            tuple_sizes: vec![3, 3, 3],
        };

        let answer = solve_secret_string_challenge(&recover_secret_input);
        assert_eq!(answer.secret_sentence, "rtlhzo".to_string());
    }

    #[test]
    fn test_solve_secret_sentence_challenge_multiple_words() {
        let recover_secret_input: RecoverSecretInput = RecoverSecretInput {
            word_count: 6,
            letters: "iffiiilfatroridato".parse().unwrap(),
            tuple_sizes: vec![3, 3, 3, 3, 3, 3],
        };

        let answer = solve_secret_string_challenge(&recover_secret_input);
        assert_eq!(answer.secret_sentence, "i i r i f lfatrod".to_string());
    }
}
