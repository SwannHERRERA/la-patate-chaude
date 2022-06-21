use std::collections::VecDeque;

use crate::message::{RecoverSecretInput, RecoverSecretOutput};

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
        for i in current_index..(current_index + *size - 1) {
            tuple.push(input.letters.chars().nth(i).unwrap());
            current_index += 1;
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
    todo!()
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

fn retrieve_possible_strings_from_tuple(tuple: Vec<char>, propositions: &mut Vec<String>) -> Vec<String> {
    Vec::new()
}
