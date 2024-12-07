use std::collections::HashMap;

#[derive(Debug)]
struct Directory {
    dirs: HashMap<String, Directory>,
    files: HashMap<String, usize>,
}

impl Directory {
    fn new() -> Self {
        Self {
            dirs: HashMap::new(),
            files: HashMap::new(),
        }
    }
}

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> std::io::Result<()> {
    let mut buf = String::new();

    std::io::stdin().read_line(&mut buf)?;
    assert_eq!(buf.trim(), "$ cd /");
    buf.clear();

    let mut root_dir = Directory::new();

    build_directory_structure(&mut root_dir)?;

    let mut ans = 0;
    calculate_size(&root_dir, &mut ans);
    println!("{}", ans);

    Ok(())
}

fn calculate_size(work_dir: &Directory, ans: &mut usize) -> usize {
    let total_size = work_dir
        .dirs
        .values()
        .map(|d| calculate_size(d, ans))
        .sum::<usize>()
        + work_dir.files.values().sum::<usize>();

    if total_size <= 100_000 {
        *ans += total_size;
    }

    total_size
}

fn build_directory_structure(work_dir: &mut Directory) -> std::io::Result<bool> {
    let mut buf = String::new();

    while std::io::stdin().read_line(&mut buf)? != 0 {
        if buf.trim() == "$ cd .." {
            return Ok(true);
        } else if buf.starts_with("$ cd ") {
            let dir_name = buf.trim().split_once("$ cd ").unwrap().1;
            if !build_directory_structure(work_dir.dirs.get_mut(dir_name).unwrap())? {
                break;
            }
        } else if buf.starts_with("$ ls") {
        } else if buf.starts_with("dir") {
            let dir_name = buf.trim().split_once("dir ").unwrap().1;
            work_dir.dirs.insert(dir_name.to_string(), Directory::new());
        } else {
            let (size, name) = buf.trim().split_once(' ').unwrap();
            let size = size.parse::<usize>().unwrap();
            work_dir.files.insert(name.to_string(), size);
        }

        buf.clear();
    }

    Ok(false)
}
