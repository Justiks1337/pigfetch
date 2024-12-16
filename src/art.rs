pub mod arts {
    pub struct Art {
        pub art_lines: [String; 8],
    }

    impl Art {
        pub fn new() -> Self {
            let art_lines: [String; 8] = Default::default();
            Art { art_lines }
        }

        pub fn add_line(&mut self, line: String, index: usize) {
            self.art_lines[index] = line;
        }
    }
}
