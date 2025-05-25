use std::process::Command;
use which::which;

fn main() {
    // Check if rustup is available
    match which("rustup") {
        Ok(_) => install_via_rustup(),
        Err(_) => panic!("rustup is needed when using rustfmt and clippy"),
    }

    // Use rustup-init to initialize the toolchains (not always needed)
    if which("rustup-init").is_ok() {
        Command::new("rustup-init")
            .args(["-y", "--quiet"])
            .status()
            .expect("Failed to run rustup-init");
    }
}

fn install_via_rustup() {
    let rustup_output = Command::new("rustup").args(["show"]).output();

    match rustup_output {
        Ok(output) => {
            let std_output = String::from_utf8_lossy(&output.stdout);
            let parse_result = parse_installed_toolchains(&std_output);

            match parse_result {
                ParseResult::OnlyDefault => {
                    Command::new("rustup")
                        .args(["component", "add", "rustfmt"])
                        .status()
                        .expect("Failed to add rustfmt component");

                    Command::new("rustup")
                        .args(["component", "add", "clippy"])
                        .status()
                        .expect("Failed to add clippy component");

                    println!("Added rustfmt and clippy components for default toolchain");
                }
                ParseResult::Multiple(toolchains) => {
                    for toolchain in toolchains {
                        Command::new("rustup")
                            .args(["component", "add", "rustfmt", "--toolchain", &toolchain])
                            .status()
                            .expect("Failed to add rustfmt component");

                        Command::new("rustup")
                            .args(["component", "add", "clippy", "--toolchain", &toolchain])
                            .status()
                            .expect("Failed to add clippy component");

                        println!(
                            "Added rustfmt and clippy components for toolchain: {}",
                            toolchain
                        );
                    }
                }
            }
        }

        Err(e) => panic!("Failed to run rustup show: {}", e),
    }
}

enum ParseResult {
    OnlyDefault,
    Multiple(Vec<String>),
}

fn parse_installed_toolchains(std_output: &str) -> ParseResult {
    let mut toolchains = Vec::new();
    let mut in_toolchains_section = false;

    for line in std_output.lines() {
        if line.contains("installed toolchains") {
            in_toolchains_section = true;
            continue;
        }

        if in_toolchains_section {
            if line.trim().is_empty() || line.trim().contains("-----") {
                continue;
            }

            if line.contains("active toolchain") {
                break;
            }

            let toolchain = line.split_whitespace().next().unwrap();
            toolchains.push(toolchain.to_string());
        }
    }

    if toolchains.is_empty() {
        ParseResult::OnlyDefault
    } else {
        ParseResult::Multiple(toolchains)
    }
}
