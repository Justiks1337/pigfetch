pub mod arts_file {
    use std::fs::File;
    use std::io::{BufRead, Result};
    use crate::Art::arts::Art;

    pub struct ArtsFile {
        file: File,
        pub arts: Vec<Art>,
        pub arts_amount: u8,
    }

    impl ArtsFile {
        pub fn new() -> Self {
            let mut file = File::open("arts.txt")?;
            let mut arts: Vec<Art> = Vec::new();
            let mut arts_amount: u8 = 0;

            ArtsFile { file, arts, arts_amount}
        }

        fn read_file(&mut self) {
            let file_clone = self.file.try_clone().expect("Failed load arts.txt");
            let reader = BufRead::new(file_clone);
            self.read_art(reader);
        }

        fn read_art(&mut self, lines: Result<String>) {
            let mut s = lines.iter().next();
            let mut art = Art::new();
            let mut line: i8 = 0;
            while s.chars().nth(0) != Some('-') {
                art.add_line(s, line);
                line += line;
                s = lines.iter().next();
            }

            self.arts.push(art);
            lines.iter().next();
            self.read_art(lines);
        }
    }
}