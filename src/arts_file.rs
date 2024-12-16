use crate::art::arts::Art;

pub struct ArtsFile<'a> {
    file: &'a str,
    pub arts: Vec<Art>,
    pub arts_amount: usize,
}

impl ArtsFile<'_> {
    pub fn new() -> Self {
        let file = include_str!("../arts.txt");
        let arts: Vec<Art> = Vec::new();
        let arts_amount: usize = 0;

        ArtsFile {
            file,
            arts,
            arts_amount,
        }
    }

    pub fn read_file(&mut self) {
        let file = self.file;
        self.read_art(file.lines());
    }

    fn read_art(&mut self, lines: core::str::Lines) {
        let mut art = Art::new();
        let mut index: usize = 0;
        for line in lines {
            if line.starts_with('-') {
                self.arts.push(art);
                index = 0;
                self.arts_amount += 1;
                art = Art::new()
            } else {
                art.add_line(line.to_string(), index);
                index += 1;
            }
        }
    }
}
