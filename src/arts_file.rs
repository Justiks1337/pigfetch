pub mod arts_file {
    use std::fs::File;
    use std::io::{BufRead, BufReader, Lines};
    use crate::art::arts::Art;

    pub struct ArtsFile {
        file: File,
        pub arts: Vec<Art>,
        pub arts_amount: usize,
    }

    impl ArtsFile {
        pub fn new() -> Self {
            let file = File::open("arts.txt");
            let arts: Vec<Art> = Vec::new();
            let arts_amount: usize = 0;

            ArtsFile { file: file.expect("REASON"), arts, arts_amount}
        }

        pub fn read_file(&mut self) {
            let file_clone = self.file.try_clone().expect("Failed load arts.txt");
            let reader = BufReader::new(file_clone);
            self.read_art(reader.lines());
        }

        fn read_art(&mut self, lines: Lines<BufReader<File>>) {
            let mut art = Art::new();
            let mut index: usize = 0;
            for line_result in lines {
                if let Ok(line) = line_result {
                    if line.starts_with('-') {
                        self.arts.push(art);
                        index = 0;
                        self.arts_amount += 1;
                        art = Art::new()
                    } else {
                        art.add_line(line, index);
                        index += 1;
                    }
                }
    }
        }
    }
}