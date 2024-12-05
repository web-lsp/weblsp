use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

use proc_macro2::TokenStream;

const HEADER_COMMENT: &str = r#"// This file is auto-generated and any manual changes to it will be overwritten.
//
// Run `cargo run -p codegen` from the project root to regenerate it.
"#;

pub struct CodegenFile {
    file: File,
}

impl CodegenFile {
    /// Create a new generated file.
    pub fn create<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();
        let mut file = File::create(path)?;

        writeln!(file, "{HEADER_COMMENT}")?;

        Ok(Self { file })
    }

    /// Formats and appends the tokens to the output file.
    pub fn append(&mut self, tokens: TokenStream) -> io::Result<()> {
        // rustfmt unfortunately bails out on our generated file because the lines are super long and it's generally wonky-looking.
        // prettyplease isn't perfect, but it's much better
        let parsed = syn::parse_file(&tokens.to_string()).unwrap();
        let formatted = prettyplease::unparse(&parsed);

        writeln!(self.file, "{}", formatted)?;

        Ok(())
    }
}
