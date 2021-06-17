use crate::set::Set;


/// usage
/// 
/// header
/// 
/// section1
/// cmd1
/// cmd2
/// 
/// section2
/// cmd3
/// cmd4
/// 
/// footer
pub trait HelpGenerator {
    fn new_section(&mut self, section: &str, help: &str);

    fn add_cmd(&mut self, section: &str, cmd: &str);

    fn rem_cmd(&mut self, section: &str, cmd: &str);

    fn set_usage(&mut self, help: &str);

    fn set_footer(&mut self, help: &str);

    fn set_header(&mut self, help: &str);

    fn generate_cmd_help(&self, cmd: &str) -> &str;

    fn generate(&self) -> &str;
}

pub struct SetHelpGenerator<'a> {
    set: &'a dyn Set,
}