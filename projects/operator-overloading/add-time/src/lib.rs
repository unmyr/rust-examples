use std::ops::Add;

#[derive(Debug, PartialEq)]
#[allow(unused)]
struct Time {
    sec: i32,
    min: i32,
    hour: i32,
}

impl Add for Time {
    type Output = Time;

    fn add(self, other: Time) -> Time {
        let sum_sec = self.sec + other.sec;
        let sum_min = self.min + other.min + sum_sec / 60;
        let sum_hour = self.hour + other.hour;
        Time {
            sec: sum_sec % 60,
            min: sum_min % 60,
            hour: sum_hour + sum_min / 60,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        use crate::Time;
        assert_eq!(
            Time {
                hour: 0,
                min: 59,
                sec: 59
            } + Time {
                hour: 0,
                min: 0,
                sec: 2
            },
            Time {
                hour: 1,
                min: 0,
                sec: 1
            }
        );
    }
}
