mod args;
mod trash;
use crate::{
    trash::*,
    args::args
};

fn main() {
    let arg = args();
    recycel_check();
    if arg.is_present("list") {
        recycle_bin_list().for_each(|x|{
            println!("{:?}",x.unwrap().path());
        });
        return;
    } 
    if arg.subcommand_matches("empty").is_some() {
        recycle_bin_empty();
        return; 
    }
    if !arg.is_present("file"){
        println!("No file specified");
        return;
    }
    let trash = Trash{path:String::from(arg.value_of("file").unwrap())};

    if arg.subcommand_matches("put").is_some() {
        if !trash.check(true) {
            println!("file doesn't exist");
            return;
        }
        trash.file_move();
        return;
    }

    if !trash.check(false) {
        println!("file doesn't exist");
        return;
    }
    if arg.subcommand_matches("remove").is_some() {
        trash.recycle_bin_delete();
    }


}
