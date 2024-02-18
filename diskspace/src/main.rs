use std::{fs, ptr};
use byte_unit::{Byte, Unit, UnitType};

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let paths = fs::read_dir("/Users/andrii.ladyhin/Exploits").unwrap();

    let mut res: Vec<(String, u64)> = vec![];
    read_dir("/Users/andrii.ladyhin/Exploits", &mut res);

    for row in res {
        println!("Name: {}: {}", row.0, Byte::from_u64(row.1).get_appropriate_unit(UnitType::Binary));
    }
}

fn read_dir(path: &str, res: &mut Vec<(String, u64)>) -> u64  {
    let paths = fs::read_dir(path).unwrap();
    let mut size_sum: u64 = 0;
    let min_size: u64 = 15_000_000;

    for path in paths {
        let path = path.unwrap().path();

        if path.is_dir() {
            let dir_max_size = read_dir(path.to_str().unwrap(), res);
            if dir_max_size >= min_size {
                res.push( (path.to_str().unwrap().to_string(), dir_max_size));
            }
        }

        let metadata = path.symlink_metadata().expect("Smth went wrong with: ");

        if metadata.len() >= min_size {
            res.push((path.to_str().unwrap().to_string(), metadata.len()));
        }
        size_sum += metadata.len();
        // res.insert(0, ((*path.to_str().unwrap()).clone(), metadata.len()));
        // println!("Name: {}: {}", path.display(), Byte::from_u64(metadata.len()).get_appropriate_unit(UnitType::Binary));
    }

    size_sum
}
