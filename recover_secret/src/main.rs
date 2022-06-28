use recover_secret::challenge_resolve::solve_secret_sentence_challenge;
use recover_secret::models::RecoverSecretInput;

fn main() {
    //RecoverSecretInput
    let recover_secret_input: RecoverSecretInput = RecoverSecretInput {
        word_count: 1,
        letters: "iffiiilfatroridato".parse().unwrap(),
        tuple_sizes: vec![3, 3, 3, 3, 3, 3],
    };

    let output = solve_secret_sentence_challenge(&recover_secret_input);
    println!("{:?}", output);
}