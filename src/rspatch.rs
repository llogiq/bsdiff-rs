extern crate bsdiff;

use bsdiff::bspatch::patch;
use std::fs::File;
use std::io::{Read, Write, stdin};

static USAGE: &'static str = "Usage: rspatch <oldfile> <newfile> [<patch>]";

fn main() {
    let mut args = std::env::args_os();
    let _ = args.next(); // ignore executable
    let oldpath = args.next().expect(USAGE);
    let curpath = args.next().expect(USAGE);
    let patchpath = args.next();

    let mut oldfile = File::open(oldpath).unwrap();
    let oldlen = oldfile.metadata().unwrap().len() as usize;
    let mut old = vec![0; oldlen];
    if oldfile.read_to_end(&mut old).unwrap() != oldlen {
        panic!("could not read old file");
    }
    let cur = match patchpath {
        Some(ref path) => {
            let mut patchfile = File::open(path).unwrap();
            patch(&old, &mut patchfile).unwrap()
        },
        None => {
            let stdin = stdin();
            let mut stdin_lock = stdin.lock();
            patch(&old, &mut stdin_lock).unwrap()
        },
    };
    let mut curfile = File::create(curpath).unwrap();
    curfile.write_all(&cur).unwrap();
    curfile.flush().unwrap();
}
