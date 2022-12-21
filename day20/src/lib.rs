use std::{collections::HashMap, cmp::Ordering, thread::current};

fn index<T>(list: &Vec<T>, i: i64) -> usize {
    let mut index = i;
    while index < 0 {
        index += list.len() as i64;
    }
    
    index as usize % list.len()
} 

pub fn part1(input: &str) -> String {
    let mut list = input
        .lines()
        .enumerate()
        .map(|(i, s)| (i, s.parse::<i32>().unwrap()))
        .collect::<Vec<_>>();

    // let mut order_indices: HashMap<_, _> = (0..list.len()).map(|i| (i, i)).collect();
    // let mut order_indices: Vec<_> = (0..list.len()).collect();

    // for (_,v) in list.iter() {
    //     print!("{v}, ");
    // }
    // println!();

    for init_index in 0..list.len() {
        // let current_index = *order_indices.get(&init_index).unwrap();
        // let current_index = order_indices.iter().position(|x| *x== init_index).unwrap();
        let current_index = list.iter().position(|(i,_)|*i==init_index).unwrap();
        let (_, value) = list[current_index];

        let (neighboard_offset, dir): (i64, i64) = match value.cmp(&0) {
            Ordering::Greater => (1, 1),
            Ordering::Less => (list.len() as i64 - 1, -1),
            Ordering::Equal => (0,0),
        };
        for i in 0..(value.abs() as i64) {
            let swap_index = current_index as i64 + (dir * i);
            let left_i = index(&list, swap_index as i64);
            let right_i = index(&list, swap_index + neighboard_offset);

            // let left_oi = *order_indices.get(&left_i).unwrap();
            // let right_oi = *order_indices.get(&right_i).unwrap();
            // order_indices.insert(left_oi, right_i);
            // order_indices.insert(right_oi, left_i);

            // order_indices.swap(left_i, right_i);

            list.swap(left_i, right_i);
        }

        // for (_,v) in list.iter() {
        //     print!("{v}, ");
        // }
        // println!();
    }


    let zero_index = list.iter().position(|(_,v)|*v==0).unwrap() as i64;
    let val0 = list[index(&list, zero_index + 1000)].1;
    let val1 = list[index(&list, zero_index + 2000)].1;
    let val2 = list[index(&list, zero_index + 3000)].1;

    let sum = val0 + val1 + val2;

    sum.to_string()
}

pub fn part2(input: &str) -> String {
    let mut list = input
    .lines()
    .enumerate()
    .map(|(i, s)| (i, 811589153 * s.parse::<i64>().unwrap()))
    .collect::<Vec<_>>();

    // for (_,v) in list.iter() {
    //     print!("{v}, ");
    // }
    // println!();

    for _ in 0..10 {
        for init_index in 0..list.len() {
            let current_index = list.iter().position(|(i,_)|*i==init_index).unwrap();
            let (_, value) = list[current_index];

            let modulo = value.rem_euclid((list.len() - 1) as i64);

            for i in 0..modulo {
                let swap_index = current_index as i64 + i;
                let left_i = index(&list, swap_index as i64);
                let right_i = index(&list, swap_index + 1);

                list.swap(left_i, right_i);
            }
        }
    }

    let zero_index = list.iter().position(|(_,v)|*v==0).unwrap() as i64;
    let val0 = list[index(&list, zero_index + 1000)].1;
    let val1 = list[index(&list, zero_index + 2000)].1;
    let val2 = list[index(&list, zero_index + 3000)].1;

    let sum = val0 + val1 + val2;

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);
        assert_eq!(result, "3");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);
        assert_eq!(result, "1623178306");
    }
}
