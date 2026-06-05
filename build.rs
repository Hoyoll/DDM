
fn main() {
    let mut proc = std::process::Command::new("cmd")
        .args(["/C", "npm run build --prefix=front"]).spawn().unwrap();

    let status = proc.wait().expect("failed to wait for npm build");

    if !status.success() {
        panic!("npm build failed");
    }
}


