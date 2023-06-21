pub struct Keyboard {
    state: [bool; 16],
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard { state: [false; 16] }
    }

    pub fn pressed(&mut self, btn: usize) {
        self.state[btn] = true;
    }

    pub fn released(&mut self, btn: usize) {
        self.state[btn] = false;
    }

    pub fn is_pressed(&self, btn: usize) -> bool {
        self.state[btn]
    }

    pub fn any_pressed(&self) -> Option<usize> {
        self.state.iter().position(|&x| x)
    }
}
