use common::*;

fn main() {
    let circuit = Circuit::new(get_unwraped_lines_from_file("day-10"));
    let signal_strength = circuit.skip(19).step_by(40).sum::<i16>();

    present_result(signal_strength, None);
}

struct Circuit<I: Iterator<Item = String>> {
    inner: I,
    cycle: i16,
    x: i16,
    wait: bool,
    next_addx: Option<i16>,
}
impl<I: Iterator<Item = String>> Circuit<I> {
    fn new(inner: I) -> Self {
        Self {
            inner,
            cycle: 1,
            x: 1,
            wait: false,
            next_addx: None,
        }
    }
    fn draw(&self) -> char {
        if (self.x - 1..=self.x + 1).contains(&((self.cycle - 1) % 40)) {
            return '#';
        }
        '.'
    }
}
impl<I: Iterator<Item = String>> Iterator for Circuit<I> {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.wait {
            self.wait = false;
        } else {
            if let Some(addx) = self.next_addx {
                self.x += addx;
                self.next_addx = None;
            }
            if let Some(instruction) = self.inner.next() {
                if let Some(addx_str) = instruction.split(' ').nth(1) {
                    self.wait = true;
                    self.next_addx = Some(addx_str.parse().unwrap())
                }
            } else {
                return None;
            }
        }

        if (self.cycle - 1) % 40 == 0 {
            println!();
        }
        print!("{}", self.draw());

        let res = self.cycle * self.x;
        self.cycle += 1;
        Some(res)
    }
}
