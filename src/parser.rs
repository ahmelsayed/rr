use std::ascii::OwnedAsciiExt;

pub enum Mode {
    Help,
    Get,
    Set,
    Delete,
    ShowAll
}

pub struct RrOptions {
    pub mode: Mode,
    pub key: Box<Option<String>>,
    pub value: Box<Option<String>>
}

impl RrOptions {
    fn new() -> RrOptions {
        RrOptions{
            mode: Help,
            key: box None,
            value: box None
        }
    }
}

pub fn parse (args: Vec<String>) -> RrOptions {
    let mut rr_options = RrOptions::new();

    // - 1 becaust the args include the binary name, they are always >= 1
    match args.len() - 1 {

        1 => {
            match args[1].clone().into_ascii_lower().as_slice() {
                "?" | "help" => rr_options.mode = Help,
                "all" => rr_options.mode = ShowAll,
                _ => {
                    rr_options.mode = Get;
                    rr_options.key = box Some(args[1].clone().into_ascii_lower());
                }
            }
        },

        2 => {
            match args[1].clone().into_ascii_lower().as_slice() {
                "delete" => {
                    rr_options.mode = Delete;
                    rr_options.key = box Some(args[2].clone().into_ascii_lower());
                },
                _ => {
                    rr_options.mode = Set;
                    rr_options.key = box Some(args[1].clone().into_ascii_lower());
                    rr_options.value = box Some(args[2].clone().into_ascii_lower());
                }
            }
        },

        _ => rr_options.mode = Help
    }

    rr_options
}