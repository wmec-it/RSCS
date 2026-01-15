use crate::conf;

pub mod bcuninstaller;
pub mod utils;

pub fn start() {
    bcuninstaller::setup();
    for program_id in conf::vars::REMOVE_PROGRAMS_STOREHELPER {
        utils::remove::storehelper(program_id);
    }
}
