use std::{io::{self, BufReader, BufRead}, fs::File, collections::HashMap};

pub mod test_positions;

pub fn load_config() -> HashMap<String, String> {
    let mut map = HashMap::new();

    let lines = read_lines("./tests/config.cfg");

    for line in lines {
        let line = line.unwrap();
        let mut ite = line.split('=');
        let name = ite.next().expect("Invalid format in cfg").trim().to_owned();
        let value = ite.next().expect("Invalid format in cfg").trim().to_owned();
        map.insert(name, value);
    };

    map
}

fn read_lines(filename: &str) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap(); 
    return io::BufReader::new(file).lines(); 
}