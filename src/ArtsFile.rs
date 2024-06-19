mod arts_file {
    use std::fs::File;
    use std::io::{BufRead, Result};
    use crate::Art::arts::Art;

    struct ArtsFile {
        file: File,
        arts: Vec<Art>,
        arts_amount: i32,
    }

    impl ArtsFile {
        fn new() -> Self {
            let mut file = File::open("arts.txt")?;
            let mut arts: Vec<Art> = Vec::new();
            let mut arts_amount: i32 = 0;

            ArtsFile { file, arts, arts_amount }
        }

        fn read_file(&self) {
            let file_clone = self.file.try_clone().expect("Failed load arts.txt");
            let mut reader = BufRead::new(file_clone);
            for line in reader.lines() {
                let s = String::from(line);

                if s.chars().nth(0) == "-" {}
            }
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