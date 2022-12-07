/*  https://adventofcode.com/2022/day/7  */

fn main() {
    let input = Input::from("input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

struct Input { system: Directory }

struct Directory {
    name : String,
    dirs : Vec<Directory>,
    files: Vec<File>,
    total_size: usize
}

type File = usize;

// sum the sizes of all directories up to 100k in size
fn part1(input: &Input) -> usize {    
    let totals = get_all_totals_from(&input.system);

    totals.iter()
          .filter(|&size| size <= &100000)
          .sum()
}

// find the smallest directory we'd have to delete to free enough space
fn part2(input: &Input) -> usize {    
    let mut totals = get_all_totals_from(&input.system);

    let free_at_least = 30_000_000 - (70_000_000 - input.system.total_size);

    totals.sort();
    totals.into_iter()
          .skip_while(|&size| size < free_at_least)
          .next()
          .expect("assumed at least one size >= free_at_least")
}

fn get_all_totals_from(system: &Directory) -> Vec<usize> {
    let mut sizes = vec![];
    get_total_sizes(system, &mut sizes);
    sizes
}

fn get_total_sizes(system: &Directory, mut vec: &mut Vec<usize>) {
    system.dirs.iter()
               .for_each(|dir| get_total_sizes(&dir, &mut vec));

    vec.push(system.total_size)
}


/* Parsing */

impl Input {
    fn from(file: &str) -> Self {
        let contents = std::fs::read_to_string(file).expect("Couldn't read input");
        Input::from_string(contents.trim())
    }

    fn from_string(s: &str) -> Self {

        // the top node in our tree. everything will be added somewhere under root
        let mut root = Directory::new("".to_string());

        // push and pop off the stack of directory names as we encounter cd commands
        let mut path: Vec<String> = vec![];

        for line in s.lines().skip(1) {

            // changing directory
            if line.starts_with("$ cd ") {                
                match &line[5..] {
                    ".." => { path.pop().expect("found a 'cd ..' but already at root path"); },
                    dir  => { path.push(dir.to_string()); }
                };
            }

            // nothing to do for ls
            else if line.starts_with("$ ls") {
            }

            // a directory
            else if line.starts_with("dir ") {
                at_path_do(&mut root,
                           &path,
                           |node| node.dirs.push(Directory::new(line[4..].to_string())));

            }

            // a file with a size/name (the name isn't used anywhere so we don't collect it)
            else {
                at_path_do(&mut root,
                           &path,
                           |node| node.files.push(file_size_from(line)));
            }
        }

        // not really a parsing task, it's more a task for the domain to handle, but
        // since the system is mutable for the moment let's just add the sizes in now
        tally_sizes(&mut root);

        Input {
            system: root
        }
    }
}

// recursively sum the size of directories based on their files and sub-dirs sizes
fn tally_sizes(node: &mut Directory) -> usize {

    node.total_size =
        node.dirs.iter_mut()        // this needs to be iter_mut() because within map's call to
                 .map(tally_sizes)  // tally_sizes the node.total_size field will be updated
                 .sum::<usize>()

      + node.files.iter()           // add the total file sizes from this directory
                  .sum::<usize>();

    node.total_size
}

// recursively descend through the filesystem to a certain path,
// then do some operation on that directory
fn at_path_do<F>(node: &mut Directory,
                 path: &[String],
                 operation: F)
    where F: Fn(&mut Directory) -> ()
{
    // we're at the target path
    if path.is_empty() {
        // this will either add a file or a directory depending on how
        // it was defined in the calling code
        operation(node);
    } else {
        // recurse into the next directory in the path
        at_path_do(node.dirs.iter_mut()
                            .find(|dir| dir.name == path[0])
                            .expect("tried to 'cd' into a non-existent directory"),
                   &path[1..],
                   operation);
    }
}

impl Directory {
    fn new(name: String) -> Self {
        Directory {
            name,
            files: vec![],
            dirs: vec![],
            total_size: 0
        }
    }
}

// get the first number from a string
fn file_size_from(line: &str) -> usize {
    line.split(' ')
        .next().unwrap()
        .parse().unwrap()
}


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tree() {
        let input = get_example();
        assert_eq!(input.system.files.len(), 2);
        assert_eq!(input.system.dirs.len(), 2);
    }

    #[test]
    fn test_tally_sizes() {
        let input = get_example();        
        assert_eq!(input.system.total_size, 48381165);
    }

    #[test]
    fn test_part1() {
        let input = get_example();
        assert_eq!(part1(&input), 95437);
    }

    #[test]
    fn test_part2() {
        let input = get_example();
        assert_eq!(part2(&input), 24933642);
    }

    fn get_example() -> Input {
        let example = "$ cd /\n\
        $ ls\n\
        dir a\n\
        14848514 b.txt\n\
        8504156 c.dat\n\
        dir d\n\
        $ cd a\n\
        $ ls\n\
        dir e\n\
        29116 f\n\
        2557 g\n\
        62596 h.lst\n\
        $ cd e\n\
        $ ls\n\
        584 i\n\
        $ cd ..\n\
        $ cd ..\n\
        $ cd d\n\
        $ ls\n\
        4060174 j\n\
        8033020 d.log\n\
        5626152 d.ext\n\
        7214296 k";

        Input::from_string(&example)
    }
}
