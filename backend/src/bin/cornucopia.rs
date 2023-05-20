use cornucopia::{CodegenSettings, Error};

fn main() -> Result<(), Error> {
    let queries_path = "database/queries";
    let schema_file = "database/schema.sql".to_string();
    let destination = "src/cornucopia.rs";
    let settings = CodegenSettings {
        is_async: true,
        derive_ser: false,
    };

    println!("cargo:rerun-if-changed={queries_path}");
    println!("cargo:rerun-if-changed={schema_file}");
    cornucopia::generate_managed(
        queries_path,
        vec![schema_file],
        Some(destination),
        false,
        settings,
    )?;

    Ok(())
}
