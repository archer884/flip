use clipboard::{ClipboardContext, ClipboardProvider};
use std::error::Error;

mod text {
    pub fn angry() -> &'static str {
        "(ノಠ益ಠ)ノ彡┻━┻"
    }

    pub fn look() -> &'static str {
        "(┛ಠ_ಠ)┛彡┻━┻"
    }
}

enum Mode {
    Angry,
    Look,
}

impl Mode {
    fn from_args() -> Mode {
        use std::env;

        let command = env::args().nth(1).map(|s| s.to_lowercase());
        match command.as_ref().map(AsRef::as_ref) {
            Some("angry") => Mode::Angry,
            _ => Mode::Look,
        }
    }
}

fn main() -> Result<(), Box<Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    match Mode::from_args() {
        Mode::Angry => write(&mut ctx, text::angry()),
        Mode::Look => write(&mut ctx, text::look()),
    }
}

fn write(ctx: &mut ClipboardContext, text: &str) -> Result<(), Box<Error>> {
    ctx.set_contents(text.into())
}
