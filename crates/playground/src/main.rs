use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo playground <playground-name>");
        return;
    }

    let playground = &args[1];
    match playground.as_str() {
        "relm4_cairo_visualizer" => run_playground("relm4_cairo_visualizer"),
        // Add other playgrounds here
        _ => println!("Unknown playground: {}", playground),
    }
}

fn run_playground(crate_name: &str) {
    let status = Command::new("cargo")
        .args(&["run", "-p", crate_name])
        .status()
        .expect("Failed to execute playground");

    if !status.success() {
        eprintln!("Playground execution failed");
    }
}
