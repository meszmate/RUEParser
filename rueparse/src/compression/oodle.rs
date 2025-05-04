use libloading;
use oodle_safe;

pub struct Oodle {
    lib: libloading::Library,
}

impl Oodle {
    pub fn new(path: &str) -> Result<Self, libloading::Error> {
        let lib = unsafe { libloading::Library::new(path)? };
        Ok(Self { lib })
    }
}
