use std::io::{self, Write};
use std::fs::{self, DirEntry};
use std::path::Path;

fn visit_dirs(mut state: &'static str, dir: &Path, cb: &dyn Fn(&'static str, &DirEntry) -> &'static str) -> io::Result<&'static str> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            state = if path.is_dir() {
                visit_dirs(state, &path, cb)?
            } else {
                cb(state, &entry)
            };
        }
    }
    Ok(state)
}
use std::env;

fn yes_or_no(state: &str, prompt: &String) -> Option<&'static str> {
    if state == "!" {
        return Some("y");
    }
    loop {
        let mut ans = String::new();

        io::stdout()
            .write(prompt.as_bytes())
            .expect("Failed to write prompt");
        io::stdout().write(b" [yYnNqQ!] ").expect("wtf?");
        io::stdout().flush().expect("Faied to flush");

        io::stdin()
            .read_line(&mut ans)
            .expect("Failed to read answer");
        
        if ans.trim() == "y" || ans.trim() == "Y" {
            return Some("y");
        }
        if ans.trim() == "!" {
            return Some("!");
        }
        if ans.trim() == "n" || ans.trim() == "N" {
            return None;
        }
        if ans.trim() == "q" || ans.trim() == "Q" {
            exit(0);
        }
    }
}

fn file_cb(mut state: &'static str, ent: &DirEntry) -> &'static str {
    let fname = ent.file_name();
    if let Err(utf) = utf_chk(&fname) {
        if let Some(s) = yes_or_no(state, &format!("rename: {:?} => {}? ", ent.path(), utf)) {
            if s == "!" {
                state = "!";    // state change: always yes
            }
            let path = ent.path();
            let dir = path.parent().unwrap();
            if let Err(e) = env::set_current_dir(&dir) {
                panic!("{:?}\ncannot set current dir to {:?}", e, &dir);
            }
            // utf += ".renamed";
            let nu = Path::new(&utf);
            if !Path::exists(&nu) {
                println!("{:?} does not exist.", nu);
                if let Err(e) = fs::OpenOptions::new()
                    .read(true).write(true).create(true).open(nu) {
                        panic!("{:?}\ncannot create {:?}", e, nu);
                    }
            } else {
                println!("{:?} does exist.", nu);
            }
            if let Err(e) = fs::rename(ent.file_name(), nu) {
                panic!("{:?}\ncannot rename {:?} to {}", e, ent, utf);
            } else {
                println!("renamed to {}", utf);
            }
        }
    } else {
        println!("{}", ent.path().display());
    }
    state
}
extern crate unicode_normalization;
use std::ffi::*;
use std::process::exit;
use unicode_normalization::UnicodeNormalization;

fn utf_chk(ent: &OsString) -> Result<(), String> {    
    let p0 = ent.as_os_str();
    let p1 = p0.to_str().unwrap();
    let c = p1.nfc().collect::<String>();
    if c != p1 {
        Err(c)
    } else {
        Ok(())
    }
}

use clap::{Arg, Command};

fn main() -> io::Result<()> {
    let matches = Command::new("fs")
        .version("0.1.0")
        .author("Damon Anton Permezel")
        .about("filesystem exploration")
        .arg(Arg::new("directory")
             .short('d')
             .long("dir")
             .takes_value(true)
             .help("Starting directory"))
        .arg(Arg::new("funny")
             .long("funny")
             .help("Generate funny files in target directory"))
        .get_matches();

    let dir = matches.value_of("directory").unwrap_or(".");
    let dir = Path::new(dir);
    if !dir.is_dir() {
        panic!("{:?} -- invalid directory", dir.display());
    }
    let dir = fs::canonicalize(dir)?;
    let dir = Path::new(dir.as_os_str());
    if matches.is_present("funny") {
        funny(&dir)?;
    }
    visit_dirs("", dir, &file_cb)?;
    Ok(())
}

// Generate some funny files to demonstrate the UTF-8 encoding issues.
//
fn funny(dir: &Path) -> io::Result<()> {
    env::set_current_dir(&dir)?;
    let fun = vec![
        "がぎぐげご",
        "ガギグゲゴ",
        "ばびぶべぼ",
        "バビブベボ",
        "ぱぴぷぺぽ",
        "パピプペポ",
        "ざじずぜぞ",
        "ザジズゼゾ",
        "ぎゅぴゅじゅにゅ"
    ];
    for fname in fun {
        let p = Path::new(&fname);
        if !Path::exists(&p) {
            let mut f = fs::OpenOptions::new()
                .read(true).write(true).create(true).open(p)?;
            f.write(fname.as_bytes())?;
            f.write(b"\n")?;
        }
    }
    Ok(())
}

