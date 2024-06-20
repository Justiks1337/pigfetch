use rand::Rng;


mod arts_file;
mod consts;
mod art;
mod system_info;

fn main() {
    let mut artfile = arts_file::arts_file::ArtsFile::new();
    &artfile.read_file();
    let art_number:usize = rand::thread_rng().gen_range(0..artfile.arts_amount);
    let art: [String; 8] = artfile.arts[art_number].art_lines.clone();
    let binding = system_info::system_info::SystemInfo::new();
    let mut system_info = binding.to_hashmap().into_iter();
    println!("\n");
    for line in art {
        let (attr, &ref value_option) = loop {
            if let Some((attr, value)) = system_info.next() {
                if value.is_some() {
                    break (attr, value);
                }
            } else {
                todo!();
            }
        };

        let value = value_option.clone().unwrap();
        println!("    \x1B[35m\x1B[1m{} \x1B[0m \x1B[35m\x1B[1m{}:\x1B[0m {}", line, attr, value.clone().to_string());
    }

    for (attr, &ref val) in system_info {
        if val.clone().is_none() {
            continue
        }
        println!("    \x1B[35m\x1B[1m{}:\x1B[0m {}", attr, val.clone().unwrap().to_string());
    }
    println!("\n");
}
