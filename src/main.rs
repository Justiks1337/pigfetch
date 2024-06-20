use rand::Rng;


mod arts_file;
mod consts;
mod art;
mod system_info;

fn main() {
    let mut artfile = arts_file::arts_file::ArtsFile::new();
    &artfile.read_file();
    println!("{}", artfile.arts_amount);
    let art_number:usize = rand::thread_rng().gen_range(0..artfile.arts_amount);
    let art: [String; 8] = artfile.arts[art_number].art_lines.clone();
    let binding = system_info::system_info::SystemInfo::new();
    let mut system_info = binding.to_hashmap().into_iter();
    for line in art {
        let Some((attr, &ref value)) = system_info.next() else { todo!() };
        if value.clone().unwrap().is_empty() {
            continue
        }
        println!("{} {}: {}", line, attr, value.clone().unwrap().to_string());
    }

    for (attr, &ref val) in system_info {
        if val.clone().unwrap().is_empty() {
            continue
        }
        println!("{}: {}", attr, val.clone().unwrap().to_string());
    }
}
