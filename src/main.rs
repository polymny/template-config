use std::collections::BTreeMap;
use std::error::Error;
use std::process::exit;
use std::{env, fmt, fs};

use tera::{Context, Tera};

#[derive(Debug)]
struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for CustomError {}

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
    let args = env::args().collect::<Vec<_>>();

    let template_file = match args.get(1) {
        Some(f) => f,
        _ => {
            return Err(Box::new(CustomError(
                "this program expects a template file as argument".into(),
            )))
        }
    };

    let template_content = fs::read_to_string(&template_file)?;

    if let Some(env_file) = args.get(2) {
        dotenvy::from_filename(env_file)?;
    }

    let env = env::vars().collect::<BTreeMap<_, _>>();
    let mut context = BTreeMap::new();
    context.insert("env", env);
    let context = Context::from_serialize(context)?;
    let output = Tera::one_off(&template_content, &context, false)?;

    println!("{}", output);

    Ok(())
}
