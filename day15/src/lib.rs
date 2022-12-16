type IPos = i64;

struct Sensor {
    pos: (IPos, IPos),
    beacon: (IPos, IPos),
    distance: u64,
}

impl Sensor {
    fn in_range(&self, (ox, oy): (IPos, IPos)) -> bool {
        let (x, y) = self.pos;
        x.abs_diff(ox) + y.abs_diff(oy) <= self.distance
    }
}

pub fn part1(input: &str, y: IPos) -> String {
    let mut sensors = Vec::new();

    let mut min_x = IPos::MAX;
    let mut max_x = IPos::MIN;

    for line in input.lines() {
        let (sensor_str, beacon_str) = line.split_once(": closest beacon is at ").unwrap();

        let (sensor_x, sensor_y) = {
            let sensor_str = sensor_str.get(10..).unwrap();
            let (x_str, y_str) = sensor_str.split_once(", ").unwrap();
            let x = x_str.get(2..).unwrap().parse::<IPos>().unwrap();
            let y = y_str.get(2..).unwrap().parse::<IPos>().unwrap();
            (x, y)
        };

        let (beacon_x, beacon_y) = {
            let (x_str, y_str) = beacon_str.split_once(", ").unwrap();
            let x = x_str.get(2..).unwrap().parse::<IPos>().unwrap();
            let y = y_str.get(2..).unwrap().parse::<IPos>().unwrap();
            (x, y)
        };

        let distance = sensor_x.abs_diff(beacon_x) + sensor_y.abs_diff(beacon_y);

        min_x = min_x.min(sensor_x - distance as IPos);
        max_x = max_x.max(sensor_x + distance as IPos);

        sensors.push(Sensor {
            pos: (sensor_x, sensor_y),
            beacon: (beacon_x, beacon_y),
            distance,
        });
    }

    let mut count = 0;

    for x in min_x..=max_x {
        if sensors
            .iter()
            .any(|i| i.in_range((x, y)) && i.beacon != (x, y))
        {
            count += 1;
        }
    }

    count.to_string()
}

pub fn part2(input: &str, max: IPos) -> String {
    let mut sensors = Vec::new();

    let mut min_x = IPos::MAX;
    let mut max_x = IPos::MIN;

    for line in input.lines() {
        let (sensor_str, beacon_str) = line.split_once(": closest beacon is at ").unwrap();

        let (sensor_x, sensor_y) = {
            let sensor_str = sensor_str.get(10..).unwrap();
            let (x_str, y_str) = sensor_str.split_once(", ").unwrap();
            let x = x_str.get(2..).unwrap().parse::<IPos>().unwrap();
            let y = y_str.get(2..).unwrap().parse::<IPos>().unwrap();
            (x, y)
        };

        let (beacon_x, beacon_y) = {
            let (x_str, y_str) = beacon_str.split_once(", ").unwrap();
            let x = x_str.get(2..).unwrap().parse::<IPos>().unwrap();
            let y = y_str.get(2..).unwrap().parse::<IPos>().unwrap();
            (x, y)
        };

        let distance = sensor_x.abs_diff(beacon_x) + sensor_y.abs_diff(beacon_y);

        min_x = min_x.min(sensor_x - distance as IPos);
        max_x = max_x.max(sensor_x + distance as IPos);

        sensors.push(Sensor {
            pos: (sensor_x, sensor_y),
            beacon: (beacon_x, beacon_y),
            distance,
        });
    }

    let range = 0..max;

    for sensor in sensors.iter() {
        let (sx, sy) = sensor.pos;
    
        let mut start_x = 0;
        let mut start_y = -1-(sensor.distance as IPos);

        let mut dx = 1;
        let mut dy = 1;
        for _ in 0..4 {
            let mut x = sx + start_x;
            let mut y = sy + start_y;
            for _step in 0..=sensor.distance {
                if range.contains(&x)
                    && range.contains(&y)
                    && sensors.iter().all(|s| !s.in_range((x, y)))
                {
                    return (x * 4000000 + y).to_string();
                }

                x += dx;
                y += dy;
            }

            (dx, dy) = (dy, -dx);
            (start_x, start_y) = (start_y, -start_x);
        }
    }

    "nothing found".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_part1() {
        let result = part1(INPUT, 10);
        assert_eq!(result, "26");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT, 20);
        assert_eq!(result, "56000011");
    }
}
