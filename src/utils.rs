use git2::Repository;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::os::unix::fs::FileExt;
use std::path::PathBuf;

fn todo_file_locate<'a>() -> Option<PathBuf> {
    let repo = match Repository::discover(".") {
        Ok(repo) => repo,
        Err(_) => return None,
    };
    let root = match repo.path().parent() {
        Some(root) => root,
        None => return None,
    };
    Some(root.join(".todo"))
}

pub fn todo_exist() -> bool {
    if let Some(p) = todo_file_locate() {
        return p.exists();
    }
    false
}

pub fn todo_init() {
    if let Some(p) = todo_file_locate() {
        if let Ok(_) = File::create(p) {
            println!("Init .todo File For This Repo")
        }
    }
}

pub fn todo_add(task: &String) {
    if let Ok(_) = find_task(task) {
        println!("ERROR: Task \"{}\" already exists!", task.trim());
        return;
    }
    if let Some(p) = todo_file_locate() {
        let mut f = OpenOptions::new().append(true).open(p).unwrap();
        if let Ok(_) = f.write_all(task.as_bytes()) {
            println!("Add New Todo Task: \"{}\"", task.trim());
        }
    }
}

pub fn todo_list() {
    if let Some(p) = todo_file_locate() {
        std::fs::read_to_string(p)
            .unwrap()
            .split("\n")
            .map(|task| task.trim().to_string()) // trim and convert to string
            .filter(|task| !task.is_empty() && !task.starts_with("!")) // filter empty lines
            .for_each(|task| println!("{}", task));
    }
}

pub fn todo_done(task: &String) {
    let cursor = if let Ok(c) = find_task(task) {
        c
    } else {
        println!("ERROR: Task \"{}\" has not been added!", task.trim());
        return;
    };
    if let Some(p) = todo_file_locate() {
        let file = OpenOptions::new().write(true).open(&p).unwrap();
        if let Err(e) = file.write_at("!".as_bytes(), cursor) {
            println!("{}", e);
        } else {
            println!("Done your task {}! Great!", task.trim());
        }
    }
}

fn find_task(task: &String) -> Result<u64, bool> {
    let task = task.trim();
    let mut cursor = 0;
    let mut found = false;
    if let Some(p) = todo_file_locate() {
        let file = File::open(&p).unwrap();
        let mut reader = BufReader::new(&file);
        loop {
            let mut line = String::new();
            if let Ok(len) = reader.read_line(&mut line) {
                if len == 0 {
                    break;
                }
                line = line.trim().to_string();
                if line.starts_with("!") {
                    cursor += len;
                    continue;
                } else if line != task {
                    cursor += len;
                } else {
                    found = true;
                    break;
                }
            }
        }
    }
    if !found {
        Err(found)
    } else {
        Ok(cursor as u64)
    }
}

// #[cfg(test)]
// mod utils_test {
//     use super::todo_exist;
//
//     #[test]
//     fn test_todo_is_exist() {
//         assert_eq!(true, todo_exist())
//     }
// }
