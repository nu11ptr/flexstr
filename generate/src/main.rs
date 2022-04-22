mod aliases;
mod impls;

pub(crate) use crate::aliases::*;
pub(crate) use crate::impls::*;

use flexgen::config::Config;
use flexgen::{register_fragments, CodeGenerator, Error};

fn main() -> Result<(), Error> {
    let fragments = register_fragments!(FlexStruct, FlexImpls, TypeAliases);
    let config = Config::from_default_toml_file()?;

    let gen = CodeGenerator::new(fragments, config)?;
    gen.generate_files()
}
