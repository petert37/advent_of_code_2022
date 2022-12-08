use std::fs;

#[derive(Debug)]
struct File {
    name: String,
    size: i32,
}

#[derive(Debug)]
struct Directory {
    name: String,
    parent: Option<usize>,
    files: Vec<File>,
    directories: Vec<usize>,
}

impl Directory {
    fn get_sum_size(&self, directories: &Vec<Directory>) -> i32 {
        let mut sum_size = 0;
        for file in &self.files {
            sum_size += file.size;
        }
        for directory in &self.directories {
            if let Some(directory) = directories.get(*directory) {
                sum_size += directory.get_sum_size(directories);
            }
        }
        return sum_size;
    }
}

fn main() {
    let input = fs::read_to_string("src/day7/input.txt").unwrap();

    let root = Directory {
        name: "/".to_string(),
        parent: None,
        files: vec![],
        directories: vec![],
    };

    let mut directories = vec![];
    directories.push(root);
    let mut current_directory: usize = 0;

    input.lines().skip(1).for_each(|line| {
        if line.starts_with("$") {
            if line.starts_with("$ cd") {
                if let Some(dir_name) = line.strip_prefix("$ cd ") {
                    if dir_name == ".." {
                        if let Some(cd) = directories.get_mut(current_directory) {
                            if let Some(parent) = cd.parent {
                                current_directory = parent;
                            }
                        }
                    } else {
                        if let Some((i, _)) = directories.iter().enumerate().find(|(_, dir)| {
                            dir.name == dir_name && dir.parent == Some(current_directory)
                        }) {
                            current_directory = i;
                        }
                    }
                }
            }
        } else {
            if line.starts_with("dir") {
                if let Some(name) = line.strip_prefix("dir ") {
                    if let Some(d) = directories
                        .iter()
                        .find(|d| d.name == name && d.parent == Some(current_directory))
                    {
                        eprintln!("Directory already exists: {}", d.name);
                    } else {
                        directories.push(Directory {
                            name: name.to_string(),
                            parent: Some(current_directory),
                            files: vec![],
                            directories: vec![],
                        });
                        let new_directory_index = directories.len() - 1;

                        if let Some(cd) = directories.get_mut(current_directory) {
                            cd.directories.push(new_directory_index);
                        }
                    }
                }
            } else {
                let parts = line.split(" ").collect::<Vec<&str>>();
                let size = match parts.get(0) {
                    Some(s) => (**s).parse().unwrap(),
                    None => -1,
                };
                let name = match parts.get(1) {
                    Some(s) => s,
                    None => "UNKNOWN",
                };

                if let Some(cd) = directories.get_mut(current_directory) {
                    if let None = cd.files.iter().find(|f| f.name == name) {
                        cd.files.push(File {
                            name: name.to_string(),
                            size: size,
                        });
                    } else {
                        eprintln!("File already exists: {}/{}", cd.name, name);
                    }
                }
            }
        }
    });

    let sum = directories
        .iter()
        .map(|d| d.get_sum_size(&directories))
        .filter(|size| *size <= 100_000)
        .sum::<i32>();

    println!("Sum size of small directories: {}", sum);

    let used_space = directories
        .get(0)
        .and_then(|d| Some(d.get_sum_size(&directories)))
        .unwrap_or(0);
    let free_space = 70_000_000 - used_space;
    let space_to_free = 30_000_000 - free_space;

    if let Some((_, sum)) = directories
        .iter()
        .enumerate()
        .map(|(i, d)| (i, d.get_sum_size(&directories)))
        .filter(|(_, sum)| *sum >= space_to_free)
        .min_by(|(_, sum1), (_, sum2)| sum1.cmp(sum2))
    {
        println!("Size of directory to delete: {}", sum);
    }
}
