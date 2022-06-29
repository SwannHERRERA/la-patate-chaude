use recover_secret::challenge_resolve::solve_secret_sentence_challenge;
use recover_secret::models::RecoverSecretInput;

fn main() {
    //RecoverSecretInput
    let recover_secret_input: RecoverSecretInput = RecoverSecretInput {
        word_count: 2,
        letters: "C'echCt chut cou't htu'ehuest o".parse().unwrap(),
        tuple_sizes: vec![5, 6, 5, 4, 2, 4, 5],
    };
    //word_count: 2, letters: "C'echCt chut cou't htu'ehuest o", tuple_sizes: [5, 6, 5, 4, 2, 4, 5]

    let output = solve_secret_sentence_challenge(&recover_secret_input);
    println!("{:?}", output);
}
