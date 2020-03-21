use std::{
    env, error::Error, fs, io::Write, path::Path,
};

const SOURCE_DIR: &str = "badapple";
const NUM_FILES: usize = 1974;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("bmps.rs");
    let source_path = env::current_dir().unwrap().join(SOURCE_DIR);
    let mut bmps = fs::File::create(&dest_path)?;

    writeln!(
        &mut bmps,
        r#"
            use lazy_static::lazy_static;

            lazy_static!{{
                static ref BMPS: Vec<Bmp<'static>>= {{
                    let mut inner = Vec::with_capacity({num_files});
        "#,
        num_files = NUM_FILES
    )?;

    for index in 1..NUM_FILES {
        let file_name = fs::canonicalize(source_path.join(format!("{}.bmp", index)));
        writeln!(
            &mut bmps,
            r#"inner.push(Bmp::from_slice(include_bytes!("{path}")).unwrap());"#,
            path = file_name.unwrap().display()
        )?;
    }

    writeln!(
        &mut bmps,
        r#"
                    inner
                }};
            }}
        "#
    )?;

    Ok(())
}
