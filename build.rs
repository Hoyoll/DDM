
fn main() {
    let mut proc = std::process::Command::new("cmd")
        .args(["/C", "npm run build --prefix=front"]).spawn().unwrap();
    let mut play_proc = std::process::Command::new("cmd")
        .args(["/C", "npm run build --prefix=play"]).spawn().unwrap();


    let status = proc.wait().expect("failed to wait for npm build");
    let play_status = play_proc.wait().expect("failed to wait for npm build");
     
    if !status.success() || !play_status.success() {
        panic!("npm build failed");
    }
}


