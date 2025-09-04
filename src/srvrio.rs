use std::{
    fs::File,
    io::{self, BufRead, BufReader, Error, Read},
};

pub fn load_html(filename: &str) -> Result<String, io::Error> {
    let html_file = File::open(filename)?;
    let mut reader = BufReader::new(html_file);
    let mut html_data = String::new();
    _ = reader.read_to_string(&mut html_data).unwrap_or_else(|_| {
        todo!();
    });
    Ok(html_data)
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
pub fn read_binary_data(filename: &str) -> Result<Vec<u8>, io::Error> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut binary_vec: Vec<u8> = Vec::with_capacity(100);
    _ = reader.read_to_end(&mut binary_vec).unwrap();
    Ok(binary_vec)
}
