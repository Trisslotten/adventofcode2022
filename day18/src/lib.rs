use std::{collections::HashSet};

type Int = i16;
type Pos = (Int, Int, Int);

const OFFSETS: [Pos; 6] = [
    (1,0,0),
    (-1,0,0),
    (0,1,0),
    (0,-1,0),
    (0,0,1),
    (0,0,-1),
];

pub fn part1(input: &str) -> String {
    let mut voxels: HashSet<Pos> = HashSet::new();

    for line in input.lines() {
        let mut pos = line.split(',');
        let x = pos.next().unwrap().parse::<Int>().unwrap();
        let y = pos.next().unwrap().parse::<Int>().unwrap();
        let z = pos.next().unwrap().parse::<Int>().unwrap();

        voxels.insert((x,y,z));
    }

    let mut sum = 0;

    for p in voxels.iter() {
        let (x,y,z) = *p;

        for (ox, oy, oz) in OFFSETS {
            if !voxels.contains(&(x+ox,y+oy,z+oz)) {
                sum += 1;
            }
        }

    }

    sum.to_string()
}

struct OutsideChecker<'a> {
    outside_cells: HashSet<Pos>,
    inside_cells: HashSet<Pos>,
    voxels: &'a HashSet<Pos>,
    min: Pos,
    max: Pos,

    found: HashSet<Pos>,
}
impl OutsideChecker<'_> {
    fn is_outside_perimiter(&self, (x,y,z): Pos) -> bool {
        let (mnx, mny, mnz) = self.min;
        let (mxx, mxy, mxz) = self.max;
        
        x < mnx || x > mxx
        || y < mny || y > mxy
        || z < mnz || z > mxz
    }
    fn is_outside(&mut self, pos: Pos) -> bool {
        let mut stack: Vec<Pos> = Vec::new();
        stack.push(pos);

        self.found.clear();

        while !stack.is_empty() {
            let curr_pos = stack.pop().unwrap();
            if self.is_outside_perimiter(curr_pos) || self.outside_cells.contains(&curr_pos) {
                for p in self.found.iter() {
                    self.outside_cells.insert(*p);
                }
                return true;
            } else if self.inside_cells.contains(&curr_pos) {
                for p in self.found.iter() {
                    self.inside_cells.insert(*p);
                }
                return false;
            }
            if !self.found.contains(&curr_pos) {
                self.found.insert(curr_pos);
                let (x, y, z) = curr_pos;
                for (ox, oy, oz) in OFFSETS {
                    let neighbor = (x+ox,y+oy,z+oz);
                    if !self.voxels.contains(&neighbor) {
                        stack.push(neighbor);
                    }
                }
            }
        }

        for p in self.found.iter() {
            self.inside_cells.insert(*p);
        }
        false
    }
}

pub fn part2(input: &str) -> String {
    let mut voxels: HashSet<Pos> = HashSet::new();

    let mut min = (Int::MAX,Int::MAX,Int::MAX);
    let mut max = (Int::MIN,Int::MIN,Int::MIN);

    for line in input.lines() {
        let mut pos = line.split(',');
        let x = pos.next().unwrap().parse::<Int>().unwrap();
        let y = pos.next().unwrap().parse::<Int>().unwrap();
        let z = pos.next().unwrap().parse::<Int>().unwrap();

        min.0 = x.min(min.0);
        min.1 = y.min(min.1);
        min.2 = z.min(min.2);

        max.0 = x.max(max.0);
        max.1 = y.max(max.1);
        max.2 = z.max(max.2);

        voxels.insert((x,y,z));
    }

    let mut outside_checker = OutsideChecker {
        found: HashSet::new(),
        inside_cells: HashSet::new(),
        outside_cells: HashSet::new(),
        min,
        max,
        voxels: &voxels
    };

    let mut sum = 0;

    for p in voxels.iter() {
        let (x,y,z) = *p;

        for (ox, oy, oz) in OFFSETS {
            let pos = (x+ox,y+oy,z+oz);
            if !voxels.contains(&pos) && outside_checker.is_outside(pos) {
                sum += 1;
            }
        }

    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = 
"1,1,1
2,1,1";

    const INPUT2: &str = 
"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test_part1() {
        let result = part1(INPUT1);
        assert_eq!(result, "10");

        let result = part1(INPUT2);
        assert_eq!(result, "64");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT2);
        assert_eq!(result, "58");
    }
}
