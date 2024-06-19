use rand::Rng;


mod ArtsFile;
mod consts;
mod Art;
mod system_info;

fn main() {
    let artfile = ArtsFile::arts_file::ArtsFile::new();
    let art_number:u8 = rand::thread_rng().gen_range(0..artfile.arts_amount-1);
    let art = Some(artfile.arts.get(art_number));
    let system_info = system_info::system_info::SystemInfo::new();

}
