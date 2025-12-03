enum DialDirection {
    Left,
    Right,
}

struct RotaryDial {
    current_index: i16,
    raw_position: i16,
    times_landed_on_zero: i16,
    times_passes_zero: i16,
}

impl RotaryDial {
    fn new() -> Self {
        Self {
            current_index: 50,
            raw_position: 50,
            times_landed_on_zero: 0,
            times_passes_zero: 0,
        }
    }
    // div_euclid will give us the count of "wrap arounds"
    // rem_euclid will give us the current current_index
    fn turn_dial(&mut self, direction: DialDirection, distance: i16) {
        match direction {
            DialDirection::Left => {
                self.raw_position = self.current_index - distance;
                self.times_passes_zero = (self.raw_position - 1).div_euclid(100);
                self.current_index -= distance;
            }
            DialDirection::Right => {
                let initial_position: i16 = self.raw_position;
                self.raw_position = self.current_index + distance;
                if initial_position == 0 {
                    self.times_passes_zero = (self.raw_position - 1).div_euclid(100);
                } else if distance > initial_position {
                    self.times_passes_zero = (self.raw_position - 1).div_euclid(100);
                } else {
                    self.times_passes_zero += 0
                }
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

pub fn part2(input: &[String]) -> String {
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
    rotatary_dial.times_passes_zero.to_string()
}
