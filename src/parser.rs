use std::ascii::OwnedAsciiExt;

pub enum Mode {
    Help,
    Delete,
    DeleteList,
    ShowList,
    ShowAll,
    Default
}

pub struct RrOptions<'r> {
    pub mode: Mode,
    pub list: Option<&'r str>,
    pub key: Option<&'r str>,
    pub value: Option<&'r str>
}

impl<'r> RrOptions<'r> {
    fn new() -> RrOptions<'r> {
        RrOptions{
            mode: Default,
            list: None,
            key: None,
            value: None
        }
    }
}

pub fn parse<'r> (args: Vec<String>) -> RrOptions<'r> {
    let mut rr_options = RrOptions::new();

    // - 1 becaust the args include the binary name, they are always >= 1
    match args.len() - 1 {

        1 => {
            match args[1].clone().into_ascii_lower().as_slice() {
                "?" => rr_options.mode = Help,
                "showall" => rr_options.mode = ShowAll,
                _ => rr_options.mode = Default
            }
        },

        2 => {
            match args[1].clone().into_ascii_lower().as_slice() {
                "delete" => rr_options.mode = Delete,
                "deletelist" => rr_options.mode = DeleteList,
                "showlist" => rr_options.mode = ShowList,
                _ => rr_options.mode = Default
            }
        },

        3 => {
            match args[1].clone().into_ascii_lower().as_slice() {
                "delete" => rr_options.mode = Delete,
                _ => rr_options.mode = Default
            }
        },
        _ => rr_options.mode = Help
    }

    rr_options
}