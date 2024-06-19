pub mod arts {
    pub struct Art {
        art_lines: [String; 8]
    }

    impl Art {
        pub fn new() -> Self {
            let mut art_lines: [String; 8] = Default::default();
            Art {art_lines}
        }

        pub fn add_line(&mut self, line: String, index: i8) {
            self.art_lines[index] = line;
        }
    }
}