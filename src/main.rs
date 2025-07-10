use terminal_profiles::WindowsTerminalSettings;

fn main() {
    let Some(path) = std::env::args().nth(1) else {
        println!("UASGE: terminal-profiles settings.json");
        return;
    };

    let s =
        std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("failed to read file: {path}"));

    let json: WindowsTerminalSettings = serde_json::from_str(&s).expect("failed to serde");

    let s = serde_json::to_string_pretty(&json).unwrap();
    println!("{s}")
}
