enum DialDirection {
    Left,
    Right,
}

struct RotaryDial {
    current_index: i16,
    times_landed_on_zero: i16,
}

impl RotaryDial {
    fn new() -> Self {
        Self {
            current_index: 50,
            times_landed_on_zero: 0,
        }
    }

    fn turn_dial(&mut self, direction: DialDirection, distance: i16) {
        match direction {
            DialDirection::Left => {
                self.current_index -= distance;
            }
            DialDirection::Right => {
                self.current_index += distance;
            }
        }
        self.current_index = self.current_index.rem_euclid(100);
        if self.current_index == 0 {
            self.times_landed_on_zero += 1;
        }
    }
}

pub fn part1(input: &[String]) -> String {
    let mut rotatary_dial = RotaryDial::new();
    for line in input {
        let direction_char = line.chars().next().unwrap();
        let distance: i16 = line[1..].parse().unwrap();

        let direction = match direction_char {
            'L' => DialDirection::Left,
            'R' => DialDirection::Right,
            _ => panic!(
                "Invalid direction, should be L or R. Received: {}",
                direction_char
            ),
        };
        rotatary_dial.turn_dial(direction, distance);
    }
    rotatary_dial.times_landed_on_zero.to_string()
}

pub fn part2(_input: &[String]) -> String {
    "Not implemented".to_string()
}
