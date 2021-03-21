use clap::{App, Arg};

pub fn app() -> App<'static, 'static> {
    App::new("Surface DTX Daemon")
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Use the specified config file")
            .takes_value(true))
}
