/*
Termframe is a terminal frame maker for applications in the terminal.
The project is managed at:
https://github.com/G0rocks/termframe
*/


struct Frame {
    character: String,
    color: String,
    shape: String,
    size: [i32; 2]  //[x,y]
}

impl Default for Frame {
    fn default() -> Frame {
        Frame {
            character: String::from("#"),
            color: String::from("#"),
            shape: String::from("#"),
            size: [25, 5]
        }
    }
}

impl New for Frame {
    fn new() -> Frame {
        Frame {
            character: String::from("#"),
            color: String::from("#"),
            shape: String::from("#"),
            size: [25, 5]
        }
    }
}

fn main() {
    let frame = Frame { ..Default::default()};

    frame.print();
}

