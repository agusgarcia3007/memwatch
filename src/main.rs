mod hotkey;
mod ipc;
mod killer;
mod metrics;
mod settings;
mod ui;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "toggle" => {
                if let Err(e) = ipc::send_toggle_command() {
                    eprintln!("Failed to send toggle command: {}. Is memwatch running?", e);
                    std::process::exit(1);
                }
                return;
            }
            "--help" | "-h" => {
                println!("memwatch - macOS process monitor");
                println!();
                println!("USAGE:");
                println!("  memwatch          Launch the GUI");
                println!("  memwatch toggle   Toggle window visibility");
                println!();
                println!("NOTE:");
                println!("  Global hotkey disabled in this build.");
                println!("  Use 'memwatch toggle' from terminal instead.");
                return;
            }
            _ => {
                eprintln!("Unknown command: {}", args[1]);
                eprintln!("Use 'memwatch --help' for usage information");
                std::process::exit(1);
            }
        }
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_min_inner_size([600.0, 400.0])
            .with_title("memwatch"),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "memwatch",
        options,
        Box::new(|cc| Ok(Box::new(ui::MemwatchApp::new(cc)))),
    );
}
