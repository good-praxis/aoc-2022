use common::*;
use std::collections::HashMap;

fn main() {
    let mut size_map: HashMap<String, u32> = HashMap::new();
    size_map.insert("/".to_string(), 0);
    let mut pwd = Vec::new();
    for line in get_lines_from_file("day-07") {
        let line = line.unwrap();
        let command: Vec<&str> = line.split(' ').collect();
        match command[..] {
            ["$", "cd", ".."] => {
                pwd.pop();
            }
            ["$", "cd", dir] => pwd.push(dir.to_string()),
            ["$", _] => continue,
            ["dir", dir] => {
                size_map.insert(get_path(&pwd, Some(dir)), 0);
            }
            [size, _] => add_size_to_pwd(&mut size_map, size.parse().unwrap(), &pwd),
            _ => panic!("Unexpected pattern"),
        }
    }
    let small_fries: u32 = size_map.values().filter(|v| **v <= 100000).sum();
    let available_space = 70000000 - size_map.get("/").unwrap();
    let needed_space = 30000000 - available_space;

    let mut delete_this = size_map
        .values()
        .filter(|v| **v >= needed_space)
        .collect::<Vec<&u32>>();
    delete_this.sort();

    present_result(small_fries, Some(**delete_this.first().unwrap()));
}

fn add_size_to_pwd(map: &mut HashMap<String, u32>, size: u32, pwd: &Vec<String>) {
    for i in 0..pwd.len() {
        let path = get_path(&pwd[..=i], None);
        *map.get_mut(&path).unwrap() += size;
    }
}
fn get_path(pwd: &[String], dir: Option<&str>) -> String {
    let mut path = pwd.join("/");
    if let Some(dir) = dir {
        path.push('/');
        path.push_str(dir);
    }
    path
}
