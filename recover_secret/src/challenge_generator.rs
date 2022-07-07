use rand::Rng;

use utils::string_utils::{generate_random_tuple, is_sequence_valid, word_count};

use crate::challenge_resolve::retrieve_tuples_from_letters;
use crate::models::{RecoverSecretInput, RecoverSecretOutput};

pub fn generate_challenge() -> RecoverSecretInput {
    let mut rng = rand::thread_rng();
    let word_count = rng.gen_range(1..=5);
    let mut tuple_sizes: Vec<usize> = Vec::new();
    let mut letters: String = String::new();

    for _ in 0..rng.gen_range(5..10) {
        let tuple_size = rng.gen_range(1..=8);
        tuple_sizes.push(tuple_size);
        let tuple = generate_random_tuple(&tuple_size);
        letters.push_str(tuple.iter().collect::<String>().as_str());
    }

    RecoverSecretInput {
        word_count,
        letters,
        tuple_sizes,
    }
}

pub fn validate_challenge(
    challenge_input: &RecoverSecretInput,
    challenge_output: &RecoverSecretOutput,
) -> bool {
    if word_count(&challenge_output.secret_sentence) != challenge_input.word_count {
        return false;
    }

    for tuple in retrieve_tuples_from_letters(&challenge_input) {
        if !is_sequence_valid(&challenge_output.secret_sentence, &tuple) {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use crate::challenge_generator::{generate_challenge, validate_challenge};
    use crate::models::{RecoverSecretInput, RecoverSecretOutput};

    #[test]
    fn test_generate_challenge() {
        let challenge_input = generate_challenge();
        assert_eq!(
            challenge_input.letters.len(),
            challenge_input.tuple_sizes.iter().sum()
        );
        assert!(challenge_input.word_count <= challenge_input.letters.len());
    }

    #[test]
    fn test_validate_challenge_valid() {
        let recover_secret_input = RecoverSecretInput {
            word_count: 1,
            letters: "iffiiilfatroridato".parse().unwrap(),
            tuple_sizes: vec![3, 3, 3, 3, 3, 3],
        };

        let recover_secret_output = RecoverSecretOutput {
            secret_sentence: "iiriflfatrod".to_string(),
        };

        assert!(validate_challenge(
            &recover_secret_input,
            &recover_secret_output
        ));
    }

    #[test]
    fn test_validate_challenge_valid_2() {
        let recover_secret_input = RecoverSecretInput {
            word_count: 3,
            letters: " it fridft Ilfrlafdl tfidatrodliidIl fridIlft od".to_string(),
            tuple_sizes: vec![8, 3, 4, 4, 6, 5, 4, 7, 7],
        };

        let recover_secret_output = RecoverSecretOutput {
            secret_sentence: "Il fait froid".to_string(),
        };

        assert!(validate_challenge(
            &recover_secret_input,
            &recover_secret_output
        ));
    }

    #[test]
    fn test_validate_challenge_invalid() {
        let recover_secret_input = RecoverSecretInput {
            word_count: 1,
            letters: "iffiiilfatroridato".parse().unwrap(),
            tuple_sizes: vec![3, 3, 3, 3, 3, 3],
        };

        let recover_secret_output = RecoverSecretOutput {
            secret_sentence: "il fait froid".to_string(),
        };

        assert!(!validate_challenge(
            &recover_secret_input,
            &recover_secret_output
        ));
    }

    #[test]
    fn test_validate_challenge_invalid_2() {
        let recover_secret_input = RecoverSecretInput {
            word_count: 3,
            letters: " it fridft Ilfrlafdl tfidatrodliidIl fridIlft od".to_string(),
            tuple_sizes: vec![8, 3, 4, 4, 6, 5, 4, 7, 7],
        };

        let recover_secret_output = RecoverSecretOutput {
            secret_sentence: "lI fiat froid".to_string(),
        };

        assert!(!validate_challenge(
            &recover_secret_input,
            &recover_secret_output
        ));
    }
}
