/*  https://adventofcode.com/2022/day/18  */

fn main() {
    let input = Input::from("input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

struct Input { cubes : Vec<Cube> }

#[derive(PartialEq)]
struct Cube {
    x: i32,
    y: i32,
    z: i32
}

// count the number of exposed cube sides, ie no other cube is adjacent to that side
fn part1(input: &Input) -> usize {
    let mut exposed = 0;

    for cube in &input.cubes {
        let neighbours = cube.get_neighbors();

        exposed += 6 - input.cubes.iter()
                                  .filter(|c| *c != cube)
                                  .filter(|c| neighbours.contains(c))
                                  .count();
    }

    exposed
}

// only consider the external surfaces that can be reached (flood fill and count cube sides)
fn part2(input: &Input) -> usize {

    // shift everything up diagonally so we can start exploring at the origin and know
    // there isn't already cube there, and so the flood fill can get around the outside edges
    let cubes: Vec<Cube> = input.cubes.iter()
                                      .map(|c| Cube::new(c.x+1, c.y+1, c.z+1))
                                      .collect();

    let x_bound = input.cubes.iter().map(|cube| cube.x).max().unwrap() + 2;
    let y_bound = input.cubes.iter().map(|cube| cube.y).max().unwrap() + 2;
    let z_bound = input.cubes.iter().map(|cube| cube.z).max().unwrap() + 2;

    // contour the outside of the lava drop
    count_faces(&cubes,
                x_bound,
                y_bound,
                z_bound)
}

// flood fill from the cube at the origin and count how many times we run into a cube face
fn count_faces(cubes: &[Cube], x: i32, y: i32, z: i32) -> usize {
    let mut queue:   Vec<Cube> = vec![Cube::new(0,0,0)];
    let mut visited: Vec<Cube> = vec![];
    let mut faces = 0;

    while let Some(cursor) = queue.pop()
    {
        // get the cells around the cursor
        let around = cursor.get_neighbors()
                           .into_iter()
                           .filter(|cube|    cube.x >= 0 && cube.y >= 0 && cube.z >= 0
                                          && cube.x <= x && cube.y <= y && cube.z <= z)
                           .collect::<Vec<Cube>>();

        // count how many neighbours are actually cubes, these are faces we can count
        faces += around.iter()
                       .filter(|c| cubes.contains(c))
                       .count();

        // queue up unvisited neighbours
        let mut next = around.into_iter()
                             .filter(|c| !cubes.contains(c))
                             .filter(|c| !visited.contains(c))
                             .filter(|c| !queue.contains(c))   // don't forget the queue or we'll
                             .collect();                       // visit locations more than once

        queue.append(&mut next);
        visited.push(cursor);
    }
    
    faces
}

impl Cube {
    fn get_neighbors(&self) -> Vec<Cube> {
        vec![
            Cube::new(self.x+1, self.y  , self.z  ),
            Cube::new(self.x-1, self.y  , self.z  ),
            Cube::new(self.x  , self.y+1, self.z  ),
            Cube::new(self.x  , self.y-1, self.z  ),
            Cube::new(self.x  , self.y  , self.z+1),
            Cube::new(self.x  , self.y  , self.z-1),
        ]
    }
}


/* Parsing */

impl Input {
    fn from(file: &str) -> Self {
        let contents = std::fs::read_to_string(file).expect("Couldn't read input");
        Input::from_string(contents.trim())
    }

    fn from_string(s: &str) -> Self {
        Input {
            cubes: s.lines().map(Cube::from_string).collect()
        }
    }
}

impl Cube {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Cube { x, y, z }
    }

    fn from_string(s: &str) -> Self {
        let mut line = s.split(',').into_iter();

        Cube::new(line.next().unwrap().parse().unwrap(),
                  line.next().unwrap().parse().unwrap(),
                  line.next().unwrap().parse().unwrap())
    }
}


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&get_example()), 64);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_example()), 58);
    }

    fn get_example() -> Input {
        Input::from_string(
            "2,2,2\n\
             1,2,2\n\
             3,2,2\n\
             2,1,2\n\
             2,3,2\n\
             2,2,1\n\
             2,2,3\n\
             2,2,4\n\
             2,2,6\n\
             1,2,5\n\
             3,2,5\n\
             2,1,5\n\
             2,3,5"
        )
    }
}

/*  $ time target/release/day_18.exe
    Part 1: 4400
    Part 2: 2522

    real    0m0.290s
*/
