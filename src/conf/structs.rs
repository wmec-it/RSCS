// &'static str = Compacts &str to static length after definition
// &'a str      = Only defined for as long as what uses it is defined
// <'a>         = Defines the same as above, but for the entire Struct
#[allow(dead_code)]
pub struct Theme<'a> {
    pub primary: &'a str,
    pub success: &'a str,
    pub error: &'a str,
    pub info: &'a str,
    pub warning: &'a str,
}

#[allow(dead_code)]
pub struct Delimiters<'a> {
    pub layer1: &'a str,
    pub layer1info: &'a str,
    pub layer1error: &'a str,
    pub layer1success: &'a str,
    pub layer1add: &'a str,
    pub layer2: &'a str,
    pub layer2info: &'a str,
    pub layer2error: &'a str,
    pub layer2success: &'a str,
    pub layer2add: &'a str,
    pub layer3: &'a str,
    pub layer3info: &'a str,
    pub layer3error: &'a str,
    pub layer3success: &'a str,
    pub layer3add: &'a str,
    pub frown: &'a str,
}
