use recover_secret::challenge_resolve::{
    solve_secret_sentence_challenge, solve_secret_string_challenge,
};
use recover_secret::file_utils::read_file;
use recover_secret::models::RecoverSecretInput;
use recover_secret::string_utils::generate_dictionary_hashmap;

fn main() {
    println!("Reading dictionary file...");
    let dictionary = read_file("data/liste-de-ses-morts.dic");
    println!("Generating hashmap...");
    let dictionary_hashmap = generate_dictionary_hashmap(&dictionary);
    println!("Done !");

    //RecoverSecretInput
    let recover_secret_input1: RecoverSecretInput = RecoverSecretInput {
        word_count: 2,
        letters: "C'echCt chut cou't htu'ehuest o".parse().unwrap(),
        tuple_sizes: vec![5, 6, 5, 4, 2, 4, 5],
    };

    // let recover_secret_input2: RecoverSecretInput = RecoverSecretInput {
    //     word_count: 3,
    //     letters: "iffiiilfatroridatol ft f".parse().unwrap(),
    //     tuple_sizes: vec![3, 3, 3, 3, 3, 3, 3, 3],
    // };
    let recover_secret_input2: RecoverSecretInput = RecoverSecretInput {
        word_count: 3,
        letters: "iffiii".parse().unwrap(),
        tuple_sizes: vec![3, 3],
    };

    println!("Solving challenge 1...");
    let output = solve_secret_sentence_challenge(&recover_secret_input1, &dictionary_hashmap);
    println!("{:?}", output);
    println!("Solving challenge 2...");
    let output = solve_secret_string_challenge(&recover_secret_input2);
    println!("{:?}", output);
}
