use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

pub fn load_html(filename: &str) -> String {
    let html_file = File::open(filename).unwrap_or_else(|_| {
        todo!();
    });
    let mut reader = BufReader::new(html_file);
    let mut html_data = String::new();
    _ = reader.read_to_string(&mut html_data).unwrap_or_else(|_| {
        todo!();
    });
    html_data
}
pub fn load_config() -> Vec<String> {
    //this will change eventually
    let file = File::open("config.yaml").expect("No config.yaml found!");
    let reader = BufReader::new(file);
    let config_vec: Vec<String> = reader
        .lines()
        .map(|lines| lines.unwrap())
        .filter(|str| !str.starts_with('#'))
        .collect();
    let mut parsed_config_vec: Vec<String> = Vec::new();
    for config_element in config_vec {
        let mut config_element_split: Vec<_> = config_element.split(":").collect();
        let mut config_element_parsed = config_element_split[1].to_string();
        config_element_parsed.retain(|ch| !ch.is_whitespace());
        parsed_config_vec.push(config_element_parsed);
    }
    parsed_config_vec
}
