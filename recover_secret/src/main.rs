use recover_secret::challenge_resolve::{
    solve_secret_sentence_challenge, solve_secret_string_challenge,
};
use recover_secret::file_utils::read_file;
use recover_secret::models::RecoverSecretInput;
use recover_secret::string_utils::generate_dictionary_hashmap;
use std::time::Instant;

fn main() {
    println!("Reading dictionary file...");
    let dictionary = read_file("data/liste-de-ses-morts.dic");
    println!("Generating hashmap...");
    let dictionary_hashmap = generate_dictionary_hashmap(&dictionary);
    println!("Done !");

    //RecoverSecretInput
    let recover_secret_input: RecoverSecretInput = RecoverSecretInput {
        word_count: 2,
        letters: "C'echCt chut cou't htu'ehuest o".parse().unwrap(),
        tuple_sizes: vec![5, 6, 5, 4, 2, 4, 5],
    };

    println!("Solving challenge 1 (true sentence)...\n{:?}", recover_secret_input);
    let i = Instant::now();
    let output = solve_secret_sentence_challenge(&recover_secret_input, &dictionary_hashmap);
    println!("{:?}", output);
    println!("{:?}", i.elapsed());

    let recover_secret_input: RecoverSecretInput = RecoverSecretInput {
        word_count: 1,
        letters: "iffiiilfatroridato".parse().unwrap(),
        tuple_sizes: vec![3, 3, 3, 3, 3, 3],
    };

    println!("\nSolving challenge 2 (random string)...\n{:?}", recover_secret_input);
        let i = Instant::now();
    let output = solve_secret_string_challenge(&recover_secret_input);
    println!("{:?}", output);
    println!("{:?}", i.elapsed());

    let recover_secret_input: RecoverSecretInput = RecoverSecretInput {
        word_count: 1,
        letters: "rtlthotzo".parse().unwrap(),
        tuple_sizes: vec![3, 3, 3],
    };
    println!("\nSolving challenge 3 (random string)...\n{:?}", recover_secret_input);
        let i = Instant::now();
    let output = solve_secret_string_challenge(&recover_secret_input);
    println!("{:?}", output);
    println!("{:?}", i.elapsed());

    let recover_secret_input: RecoverSecretInput = RecoverSecretInput {
        word_count: 6,
        letters: "iffiiilfatroridato".parse().unwrap(),
        tuple_sizes: vec![3, 3, 3, 3, 3, 3],
    };
    println!("\nSolving challenge 4 (random string)...\n{:?}", recover_secret_input);
        let i = Instant::now();
    let output = solve_secret_string_challenge(&recover_secret_input);
    println!("{:?}", output);
    println!("{:?}", i.elapsed());

    let recover_secret_input: RecoverSecretInput = RecoverSecretInput {
        word_count: 3,
        letters: "ififrdlfatoil ft f".parse().unwrap(),
        tuple_sizes: vec![6, 6, 6],
    };

    println!("\nSolving challenge 5 (true sentence)...\n{:?}", recover_secret_input);
        let i = Instant::now();
    let output = solve_secret_sentence_challenge(&recover_secret_input, &dictionary_hashmap);
    println!("{:?}", output);
    println!("{:?}", i.elapsed());

}
