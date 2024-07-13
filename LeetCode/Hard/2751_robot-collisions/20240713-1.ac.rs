use std::collections::VecDeque;

#[derive(Debug)]
pub struct Robot {
    position: i32,
    health: i32,
    direction: char,
}

impl Solution {
    pub fn survived_robots_healths(
        positions: Vec<i32>,
        healths: Vec<i32>,
        directions: String,
    ) -> Vec<i32> {
        let mut robots = create_robots(positions, healths, directions);

        simulate(robots.iter_mut().map(|r| r).collect::<Vec<&mut Robot>>());

        robots.iter().map(|r| r.health).filter(|h| *h > 0).collect()
    }
}

fn simulate(mut robots: Vec<&mut Robot>) {
    robots.sort_by_key(|r| r.position);

    // > < > > > < <
    // < > > > >
    let mut queue: VecDeque<&mut Robot> = VecDeque::new();

    for robot in robots {
        if robot.direction == '>' {
            queue.push_back(robot);
            continue;
        }

        while let Some(front_robot) = queue.back_mut() {
            if front_robot.health < robot.health {
                robot.health -= 1;
                front_robot.health = 0;
                queue.pop_back().unwrap();
            } else if front_robot.health > robot.health {
                robot.health = 0;
                front_robot.health -= 1;
                break;
            } else {
                robot.health = 0;
                front_robot.health = 0;
                queue.pop_back().unwrap();
                break;
            }
        }
    }
}

fn create_robots(positions: Vec<i32>, healths: Vec<i32>, directions: String) -> Vec<Robot> {
    let mut robots: Vec<Robot> = vec![];
    let mut directions = directions.chars();
    for i in 0..positions.len() {
        robots.push(Robot {
            position: positions[i],
            health: healths[i],
            direction: if directions.next().unwrap() == 'L' {
                '<'
            } else {
                '>'
            },
        })
    }
    robots
}

pub struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
