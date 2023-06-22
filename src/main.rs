use std::collections::BTreeMap;
use std::error::Error;
use std::process::exit;
use std::{env, fs};

use tera::{Context, Tera};

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        if let Some(source) = e.source() {
            eprintln!("{}", source);
            exit(1);
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let template_file = env::args().skip(1).next().unwrap();
    let template_content = fs::read_to_string(&template_file)?;

    let context = Context::from_serialize(env::vars().collect::<BTreeMap<_, _>>())?;
    let output = Tera::one_off(&template_content, &context, false)?;

    println!("{}", output);

    Ok(())
}
