use sigmaos_theme::{Config, ThemeComponent, Component};
fn main() {
    env_logger::init();
    let mut c = ThemeComponent::new(Config::default());
    if let Err(e) = c.initialize() { eprintln!("Failed: {}", e); std::process::exit(1); }
    println!("SigmaOS theme running");
    let _ = c.shutdown();
}
