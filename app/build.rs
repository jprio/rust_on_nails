use std::env;
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    cornucopia()?;

    Ok(())
}

fn cornucopia() -> Result<(), std::io::Error> {
    let queries_path = "queries";
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let file_path = Path::new(&out_dir).join("cornucopia.rs");
    let db_url = env::var_os("DATABASE_URL").unwrap();

    // Rerun this build script if the queries or migrations change.
    println!("cargo:rerun-if-changed={queries_path}");

    let output = std::process::Command::new("cornucopia")
        .arg("-q")
        .arg(queries_path)
        //.arg("--derive-ser")
        .arg("-d")
        .arg(&file_path)
        .arg("live")
        .arg(db_url)
        .output()?;

    // If Cornucopia couldn't run properly, try to display the error.
    if !output.status.success() {
        panic!("{}", &std::str::from_utf8(&output.stderr).unwrap());
    } else {
        println!("Cornucopia : Ok");
    }

    Ok(())
}
