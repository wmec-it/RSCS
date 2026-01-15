use crate::conf;

pub mod bcuninstaller;

pub fn start() {
    bcuninstaller::setup();
    for program_id in conf::vars::REMOVE_PROGRAMS_STOREHELPER {
        bcuninstaller::remove::storehelper(program_id);
    }
    // TODO: Add the other removal/debloat methods
}
