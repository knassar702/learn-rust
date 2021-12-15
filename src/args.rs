extern crate clap;
use clap::{App, Arg, ArgMatches,SubCommand};

pub fn args() -> ArgMatches<'static> {
     // Create the CLI app
    return App::new("trash-cli")
                          .version("1.0")
                          .author("Khaled Nassar <knassar702@gmail.com>")
                          .about("Minimal CLI recycle bin written in Rust 🦀")
                          .arg(Arg::with_name("list")
                               .long("list")
                               .help("List all FIles in the trash")
                               )
                          .arg(Arg::with_name("file")
                               .long("file")
                               .help("Name of your file ")
                               .takes_value(true)
                               .global(true)
                               )
                          .subcommands(
                              vec![
                                SubCommand::with_name("put")
                                    .about("Used for put this file to re-bin")
                                      ,
                                SubCommand::with_name("empty")
                                    .about("Used for empty all recycle_bin")
                                ,
                                SubCommand::with_name("remove")
                                    .about("remove this file from recycel_bin"),
                          ])
                          .get_matches();

}

