use csv::Writer;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let cur_dir = env::current_dir().expect("Could not open the current directory");
    let cur_dir_s = get_string_from_path(&cur_dir);
    let one_back = one_dir_back(&cur_dir_s);
    if let Ok(dir_iter) = fs::read_dir(cur_dir) {
        create_csv(dir_iter, &one_back);
    }
}

fn create_csv(directory: fs::ReadDir, path: &str) {
    let mut wrt = Writer::from_path(path).expect("Did not write csv");
    wrt.write_record(&["type", "number", "desc"])
        .expect("Did not write header");
    for file in directory {
        let cur_path = file.unwrap().path();
        let cur_entry = parse_file_names(&cur_path);
        wrt.write_record(&cur_entry).expect("Did not write line");
    }
}

fn get_string_from_path(entry: &PathBuf) -> String {
    entry.clone().into_os_string().into_string().unwrap()
}

fn parse_file_names(entry: &PathBuf) -> Vec<String> {
    let opened = get_string_from_path(entry);
    let s1 = opened.as_str().split("/").last().unwrap().split(" ");
    let mut s_vec: Vec<String> = Vec::new();
    let mut count: i8 = 0;
    let mut desc: String = String::new();
    for sp in s1 {
        if count < 2 {
            count += 1;
            s_vec.push(sp.to_string());
        } else {
            desc.push_str(sp);
        }
    }
    s_vec.push(desc);
    s_vec
}

fn one_dir_back(directory: &str) -> String {
    let dir_sp = directory.split("/");
    let mut one_back = String::new();
    if let Some(last_sp) = dir_sp.clone().last() {
        for st in dir_sp {
            if st != last_sp {
                one_back.push_str(st);
                one_back.push_str("/");
            }
        }
    }
    one_back.push_str("significant_snps.csv");
    one_back
}
