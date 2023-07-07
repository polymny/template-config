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
    let env = env::vars().collect::<BTreeMap<_, _>>();
    let mut context = BTreeMap::new();
    context.insert("env", env);
    let context = Context::from_serialize(context)?;
    let output = Tera::one_off(&template_content, &context, false)?;

    println!("{}", output);

    Ok(())
}
