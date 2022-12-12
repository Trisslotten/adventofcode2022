// from https://en.wikipedia.org/wiki/A*_search_algorithm

fn idx((x, y): (i32, i32), w: usize) -> usize {
    x as usize + y as usize * w
}

fn hueristic((x, y): (i32, i32), (ex, ey): (i32, i32)) -> i32 {
    (x.abs_diff(ex) + y.abs_diff(ey)) as i32
}

pub fn part1(input: &str) -> String {
    let mut heightmap: Vec<u8> = Vec::new();

    let mut width: usize = 0;
    let mut height: usize = 0;
    for line in input.lines() {
        heightmap.extend_from_slice(line.as_bytes());
        width = line.as_bytes().len();
        height += 1;
    }

    let mut start = (0, 0);
    let mut end = (0, 0);

    for y in 0..height {
        for x in 0..width {
            let c = &mut heightmap[idx((x as i32, y as i32), width)];
            if *c == b'S' {
                start = (x as i32, y as i32);
                *c = b'a';
            } else if *c == b'E' {
                end = (x as i32, y as i32);
                *c = b'z';
            }
        }
    }

    let mut open_set: Vec<(i32, i32)> = Vec::new();
    open_set.push(start);

    let mut g_score = Vec::new();
    g_score.resize(heightmap.len(), i32::MAX);
    g_score[idx(start, width)] = 0;

    let mut f_score = Vec::new();
    f_score.resize(heightmap.len(), i32::MAX);
    f_score[idx(start, width)] = hueristic(start, end);

    let mut shortest_distance = 0;

    while !open_set.is_empty() {
        let mut current_idx = 0;
        let mut min_cost = i32::MAX;
        for (i, x) in open_set.iter().enumerate() {
            let curr_score = f_score[idx(*x, width)];
            if curr_score < min_cost {
                min_cost = curr_score;
                current_idx = i;
            }
        }

        let (x, y) = open_set.swap_remove(current_idx);

        if (x, y) == end {
            shortest_distance = g_score[idx(end, width)];
            break;
        }

        let (mut xo, mut yo) = (1, 0);
        let curr_g_score = g_score[idx((x, y), width)];
        let curr_height = heightmap[idx((x, y), width)];

        for _ in 0..4 {
            let nx = x + xo;
            let ny = y + yo;

            if nx >= 0
                && nx < width as i32
                && ny >= 0
                && ny < height as i32
                && heightmap[idx((nx, ny), width)] as i32 - curr_height as i32 <= 1
            {
                let tentative_g_score = curr_g_score + 1;
                if tentative_g_score < g_score[idx((nx, ny), width)] {
                    g_score[idx((nx, ny), width)] = tentative_g_score;
                    f_score[idx((nx, ny), width)] = tentative_g_score + hueristic((nx, ny), end);
                    if !open_set.contains(&(nx, ny)) {
                        open_set.push((nx, ny))
                    }
                }
            }

            (xo, yo) = (-yo, xo);
        }
    }
    
    shortest_distance.to_string()
}

pub fn part2(input: &str) -> String {
    let mut heightmap: Vec<u8> = Vec::new();

    let mut width: usize = 0;
    let mut height: usize = 0;
    for line in input.lines() {
        heightmap.extend_from_slice(line.as_bytes());
        width = line.as_bytes().len();
        height += 1;
    }

    let mut start = (0, 0);
    let mut end = (0, 0);

    for y in 0..height {
        for x in 0..width {
            let c = &mut heightmap[idx((x as i32, y as i32), width)];
            if *c == b'S' {
                start = (x as i32, y as i32);
                *c = b'a';
            } else if *c == b'E' {
                end = (x as i32, y as i32);
                *c = b'z';
            }
        }
    }
    (start, end) = (end, start);

    let mut open_set: Vec<(i32, i32)> = Vec::new();
    open_set.push(start);

    let mut g_score = Vec::new();
    g_score.resize(heightmap.len(), i32::MAX);
    g_score[idx(start, width)] = 0;

    let mut shortest_distance = 0;

    while !open_set.is_empty() {
        let mut current_idx = 0;
        let mut min_cost = i32::MAX;
        for (i, x) in open_set.iter().enumerate() {
            let curr_score = g_score[idx(*x, width)];
            if curr_score < min_cost {
                min_cost = curr_score;
                current_idx = i;
            }
        }

        let (x, y) = open_set.swap_remove(current_idx);
        let curr_height = heightmap[idx((x, y), width)];
        let curr_g_score = g_score[idx((x, y), width)];

        if curr_height == b'a' {
            shortest_distance = g_score[idx((x, y), width)];
            break;
        }
        
        let (mut xo, mut yo) = (1, 0);
        for _ in 0..4 {
            let nx = x + xo;
            let ny = y + yo;

            if nx >= 0
                && nx < width as i32
                && ny >= 0
                && ny < height as i32
                && heightmap[idx((nx, ny), width)] as i32 - curr_height as i32 >= -1
            {
                let tentative_g_score = curr_g_score + 1;
                if tentative_g_score < g_score[idx((nx, ny), width)] {
                    g_score[idx((nx, ny), width)] = tentative_g_score;
                    if !open_set.contains(&(nx, ny)) {
                        open_set.push((nx, ny))
                    }
                }
            }

            (xo, yo) = (-yo, xo);
        }
    }

    shortest_distance.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = 
"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);
        assert_eq!(result, "31");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);
        assert_eq!(result, "29");
    }
}
