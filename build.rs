#[cfg(target_os = "windows")]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_manifest(include_str!("app.manifest"));
    if let Err(error) = res.compile() {
        eprint!("{error}");
        std::process::exit(1);
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {}
