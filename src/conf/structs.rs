// &'static str = Compacts &str to static length after definition
// &'a str      = Only defined for as long as what uses it is defined
// <'a>         = Defines the same as above, but for the entire Struct
pub struct Theme<'a> {
    pub primary: &'a str,
    pub success: &'a str,
    pub error: &'a str,
    pub info: &'a str,
    pub warning: &'a str,
}
