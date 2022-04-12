use std::{fmt, str::FromStr};

use clap::{load_yaml, App};
use clipboard::{ClipboardContext, ClipboardProvider};
use uuid::{
    v1::{Context, Timestamp},
    Uuid,
};

enum UUIDFormat {
    V1,
    V3,
    V4,
    V5,
}

impl fmt::Display for UUIDFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            UUIDFormat::V1 => write!(f, "v1"),
            UUIDFormat::V5 => write!(f, "v5"),
            UUIDFormat::V3 => write!(f, "v3"),
            UUIDFormat::V4 => write!(f, "v4"),
        }
    }
}

impl FromStr for UUIDFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "v1" => Ok(UUIDFormat::V1),
            "v5" => Ok(UUIDFormat::V5),
            "v3" => Ok(UUIDFormat::V3),
            "v4" => Ok(UUIDFormat::V4),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Flags {
    copy: bool,
}

impl Flags {
    fn new() -> Self {
        Self { copy: false }
    }
    fn set_copy(&mut self, copy: bool) {
        self.copy = copy;
    }
}

// TODO: `namespace` and `name` should be cli args 
fn main() {
    let y = load_yaml!("spec/cli.yml");
    let matches = App::from(y).get_matches();

    let mut flags = Flags::new();
    if matches.is_present("copy") {
        flags.set_copy(true);
    }

    if let Some(format) = matches.value_of("format") {
        let f = UUIDFormat::from_str(format).unwrap();

        let result: Uuid;
        match f {
            UUIDFormat::V1 => {
                let context = Context::new(42);
                let ts = Timestamp::from_unix(&context, 1497624119, 1234);
                result = Uuid::new_v1(ts, &[1, 2, 3, 4, 5, 6]).expect("failed to generate UUID");
            }
            UUIDFormat::V5 => {
                let name = "test".as_bytes();
                let namespace = Uuid::NAMESPACE_DNS;
                result = Uuid::new_v5(&namespace, name)
            }
            UUIDFormat::V3 => {
                let name = "test".as_bytes();
                let namespace = Uuid::NAMESPACE_DNS;
                result = Uuid::new_v3(&namespace, name);
            }
            UUIDFormat::V4 => {
                result = Uuid::new_v4();
            }
        }

        println!("{}", result);

        if flags.copy {
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            let copied = ctx.set_contents(result.to_string());
            match copied {
                Ok(_) => println!("copy to clipboard!!"),
                Err(_) => println!("failed copy to clipboard"),
            }
        }
    } else {
        println!("<FORMAT> wasn't used...");
    }
}
