use clap::{App, Arg, ArgMatches};

pub fn parse<'a, 'b>(app: String) -> ArgMatches<'a> {
    App::new(app)
        .about("Testing out some bunyan logging!")
        .version(crate_version!())
        .arg(
            Arg::with_name("level")
                .help("Log level")
                .short("l")
                .long("level")
                .takes_value(true)
                .required(false),
        )
        .get_matches()
}
