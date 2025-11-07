use std::{fs, path::Path};

use typify::{TypeSpace, TypeSpaceSettings};

fn main() {
    let content = fs::read_to_string("schemas/container.json").unwrap();
    let schema = serde_json::from_str(&content).unwrap();

    let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(false));
    type_space.add_root_schema(schema).unwrap();

    let contents =
        prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream()).unwrap());

    let mut out_file = Path::new("src/parser/container").to_path_buf();
    out_file.push("model.rs");
    fs::write(out_file, contents).unwrap();
}
