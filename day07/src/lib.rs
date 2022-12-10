
struct File {
    size: usize,
}

struct Directory {
    name: String,
    sub_dirs: Vec<usize>,
    files: Vec<File>,
    my_index: usize,
    parent_index: usize,

    total_size: usize, 
}
impl Directory {
    fn new(name: String, my_index: usize, parent_index: usize) -> Directory {
        Directory{
            name,
            sub_dirs: Vec::new(),
            files: Vec::new(),
            my_index,
            parent_index,
            total_size: 0,
        }
    }
}

struct FileSystem {
    directories: Vec<Directory>,
    current_dir: usize,
}

impl FileSystem {
    fn new() -> FileSystem {
        let mut new_filesystem = FileSystem{
            directories: Vec::new(),
            current_dir: 0,
        };

        new_filesystem.directories.push(Directory::new("/".to_string(), 0, 0));

        new_filesystem
    }

    fn move_up(&mut self) {
        let parent = self.directories[self.current_dir].parent_index;
        self.current_dir = parent;
    }

    fn move_down(&mut self, dir: &str) {
        let sub_dirs = &self.directories[self.current_dir].sub_dirs;
        for sub_dir in sub_dirs {
            if self.directories[*sub_dir].name == dir {
                self.current_dir = *sub_dir;
                break;
            }
        }
    }

    fn add_file(&mut self, size: usize) {
        self.directories[self.current_dir].files.push(File{size: size});
    }

    fn add_dir(&mut self, name: &str) {
        let new_index = self.directories.len();
        self.directories.push(Directory::new(name.to_string(), new_index, self.current_dir));
        self.directories[self.current_dir].sub_dirs.push(new_index);
    }

    fn process_commands(&mut self, input: &str) {
        for line in input.lines() {
            let mut tokens = line.split(" ");
            let first = tokens.next().unwrap();
    
            match first {
                "$" => {
                    let command = tokens.next().unwrap();
                    if command == "cd" {
                        let dir = tokens.next().unwrap();
                        match dir {
                            "/" => {
                                self.current_dir = 0;
                            }
                            ".." => {
                                self.move_up();
                            }
                            _ => {
                                self.move_down(dir);
                            }
                        }
                    }
                }
                "dir" => {
                    let dir_name = tokens.next().unwrap();
                    self.add_dir(dir_name);
                }
                _ => {
                    let size = first.parse::<usize>().unwrap();
                    // let file_name = tokens.next().unwrap();
                    self.add_file(size);
                }
            }
        }
    }

    fn calculate_directory_sizes(&mut self) {
        let mut traversal: Vec<usize> = Vec::new();
        let mut path: Vec<usize> = Vec::new();
    
        traversal.push(0);
    
        while !traversal.is_empty() {
            let curr_index = *traversal.last().unwrap();
            if !path.is_empty() && *path.last().unwrap() == curr_index {
                // post order visit
                // add children sizes to local size
                let mut total_children_size = 0;
                for dir in self.directories[curr_index].sub_dirs.iter() {
                    total_children_size += self.directories[*dir].total_size;
                }
                self.directories[curr_index].total_size += total_children_size;
    
                traversal.pop();
                path.pop();
            } else {
                // in order visit
                // calculate local file cost
                let curr_dir = &mut self.directories[curr_index];
                for f in curr_dir.files.iter() {
                    curr_dir.total_size += f.size;
                }
    
                path.push(curr_index);
                for dir in self.directories[curr_index].sub_dirs.iter().rev() {
                    traversal.push(*dir);
                }
            }
        }
    }
}

pub fn part1(input: &str) -> String {
    let mut fs = FileSystem::new();

    fs.process_commands(input);
    fs.calculate_directory_sizes();

    let mut sum = 0;

    for dir in &fs.directories {
        if dir.total_size <= 100000 {
            sum += dir.total_size;
        }
    }

    sum.to_string()
}

pub fn part2(input: &str) -> String {
    let mut fs = FileSystem::new();

    fs.process_commands(input);
    fs.calculate_directory_sizes();

    let total_usage = fs.directories[0].total_size;
    let free_space = 70000000 - total_usage;
    let smallest_free = 30000000-free_space;

    let mut current_smallest = total_usage;

    for dir in &fs.directories {
        if dir.total_size >= smallest_free && dir.total_size < current_smallest {
            current_smallest = dir.total_size;
        }
    }

    current_smallest.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT0: (&str, &str, &str) = (
"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k",
"95437",
"24933642"
);

    const INPUTS: [(&str, &str, &str); 1] = [
        INPUT0,
    ];

    #[test]
    fn test_part1() {
        for (input, output, _) in INPUTS {
            let result = part1(input);
            assert_eq!(result, output);
        }
    }

    #[test]
    fn test_part2() {
        for (input, _, output) in INPUTS {
            let result = part2(input);
            assert_eq!(result, output);
        }
    }
}
