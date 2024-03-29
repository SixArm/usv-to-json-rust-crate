//! clap setup.
//!
//! clap is a crate for command line argument parsing.
//! See https://docs.rs/clap/
//!
//! We prefer clap using the `command!` macro, which runs at compile time.
//! We prefer clap using the builder pattern, which offers more capabilities.
//!
//! We favor our convention of doing clap setup in a file named `clap.rs`,
//! rather than in `main.rs`, because we favor the separation of concerns.

use clap::Arg;

pub fn clap() -> crate::app::args::Args {
    let matches = clap::command!()
    .name("usv-to-json")
    .version("1.2.0")
    .author("Joel Parker Henderson <joel@joelparkerhenderson.com>")
    .about("Convert Unicode Separated Values (USV) to JavaScript Object Notation (JSON)")
    .arg(Arg::new("test")
        .long("test")
        .help("Print test output for debugging, verifying, tracing, and the like.\nExample: --test")
        .action(clap::ArgAction::SetTrue))
    .arg(Arg::new("verbose")
        .short('v')
        .long("verbose")
        .help("Set the verbosity level: 0=none, 1=error, 2=warn, 3=info, 4=debug, 5=trace.\nExample: --verbose …")
        .action(clap::ArgAction::Count))
    .get_matches();

    crate::app::args::Args {
        test: matches.get_flag("test"),
        log_level: crate::app::log::u8_to_log_level(matches.get_count("verbose")),
    }
}
