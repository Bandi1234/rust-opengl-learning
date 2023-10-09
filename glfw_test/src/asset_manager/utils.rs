pub const RES : &str = "res/";
pub const MODEL : &str = "res/models/";
pub const M_CACHE : &str = "res/cache/models/";
pub const SHADER : &str = "res/shaders/";
pub const IMG : &str = "res/images/";

pub fn init() {
    check_dir(RES);
    check_dir(MODEL);
    check_dir(M_CACHE);
    check_dir(SHADER);
    check_dir(IMG);
}

pub fn check_dir(path : &str) {
    if !std::path::Path::new(path).exists() {
        std::fs::DirBuilder::new().recursive(true).create(path).unwrap();
    }
}

pub fn f_exists(path : &str) -> bool {
    std::path::Path::new(path).exists()
}

pub fn f_modif_dur(path : &str) -> u128 {
    std::fs::File::open(path).unwrap().metadata().unwrap().modified().unwrap().elapsed().unwrap().as_millis()
}