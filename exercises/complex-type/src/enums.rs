// Exercise 1
// Fill in the blank and fix the errors
// Make it compile
enum MessageOne {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

use std::fmt;

impl fmt::Display for MessageOne {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MessageOne::Quit => write!(f, "Quit"),
            MessageOne::Move { x, y } => write!(f, "Move({}, {})", x, y),
            MessageOne::Write(msg) => write!(f, "Write {}", msg),
            MessageOne::ChangeColor(r, g, b) => write!(f, "ChangeColor({}, {}, {})", r, g, b),
        }
    }
}

fn show_message(msg: MessageOne) {
    println!("{}", msg);
}

fn exercise1() {
    let msgs: [MessageOne; 3] = [
        MessageOne::Quit,
        MessageOne::Move { x: 1, y: 3 },
        MessageOne::ChangeColor(255, 255, 0),
    ];

    for msg in msgs {
        show_message(msg)
    }
}

// Exercise 2
// Fill in the blank and fix the errors
// Make it compile
// Run tests
enum Message {
    // TODO: implement the message variant types based on their usage below
    ChangeColor(i32, i32, i32),
    Echo(String),
    Move(Point),
    Quit,
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&self, s: String) {
        println!("{}", s);
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        // TODO: create a match expression to process the different message variants
        // Remember: When passing a tuple as a function argument, you'll need extra parentheses: fn function((t, u, p, l, e))
        match message {
            Message::ChangeColor(r, g, b) => self.change_color((r as u8, g as u8, b as u8)),
            Message::Echo(msg) => self.echo(msg),
            Message::Move(point) => self.move_position(point),
            Message::Quit => self.quit(),
        }
    }
}

// Exercise 3
// Fix the errors
// Run tests
#[derive(Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
            Direction::North => Direction::South,
        }
    }
}

// Exercise 4
// Implement logic :
// Run tests
// Enum representing arithmetic operations
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

// Perform arithmetic operations
fn perform_operation(operation: Operation, num1: f64, num2: f64) -> f64 {
    match operation {
        Operation::Add => num1 + num2,
        Operation::Subtract => num1 - num2,
        Operation::Multiply => num1 * num2,
        Operation::Divide => {
            if num2 == 0.0 {
                0.0
            } else {
                num1 / num2
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test for exercise 2

    #[test]
    fn exercise2_should_work() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
        };
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Echo(String::from("hello world")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
    }

    // Test for exercise 3

    #[test]
    fn exercise3_should_work() {
        assert_eq!(Direction::North.opposite(), Direction::South);
        assert_eq!(Direction::East.opposite(), Direction::West);
        assert_eq!(Direction::South.opposite(), Direction::North);
        assert_eq!(Direction::West.opposite(), Direction::East);
    }

    // Test for exercise 4
    #[test]
    fn test_perform_operation() {
        let add_result = perform_operation(Operation::Add, 5.0, 3.0);
        assert_eq!(add_result, 8.0);

        let subtract_result = perform_operation(Operation::Subtract, 10.0, 4.0);
        assert_eq!(subtract_result, 6.0);

        let multiply_result = perform_operation(Operation::Multiply, 2.5, 4.0);
        assert_eq!(multiply_result, 10.0);

        let divide_result = perform_operation(Operation::Divide, 12.0, 3.0);
        assert_eq!(divide_result, 4.0);
    }
}
