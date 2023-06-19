use std::collections::BTreeMap;
use std::{env, fs};

use tera::{Context, Tera};

fn main() {
    let template_file = env::args().skip(1).next().unwrap();
    let template_content = fs::read_to_string(&template_file).unwrap();

    let context = Context::from_serialize(env::vars().collect::<BTreeMap<_, _>>()).unwrap();
    let output = Tera::one_off(&template_content, &context, false).unwrap();

    println!("{}", output);
}
