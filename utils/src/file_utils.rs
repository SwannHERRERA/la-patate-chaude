use std::fs::File;
use std::io::Read;

pub fn read_file(file_path: &str) -> String {
    let mut file = File::open(file_path).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    contents
}

pub fn read_dic_file_macro() -> String {
    let str = include_str!("../../data/liste-de-ses-morts.dic");
    str.to_string()
}

pub fn read_mazes_file_macro() -> String {
    let str = include_str!("../../data/mazes.txt");
    str.to_string()
}
