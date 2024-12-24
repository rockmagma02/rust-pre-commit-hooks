use std::process::Command;

fn main() {
    let rustup_result = Command::new("rustup")
        .args(["show"])
        .output()
        .expect("Failed to run rustup show");

    if !rustup_result.status.success() {
        panic!("Failed to run rustup show");
    }

    let rustup_output = String::from_utf8_lossy(&rustup_result.stdout);
    let toolchains = parse_toolchains(&rustup_output);


    for toolchain in toolchains {
        Command::new("rustup")
            .args(["component", "add", "rustfmt", "--toolchain", &toolchain])
            .status()
            .expect("Failed to add rustfmt component");

        println!("Added rustfmt component for toolchain: {}", toolchain);

        Command::new("rustup")
            .args(["component", "add", "clippy", "--toolchain", &toolchain])
            .status()
            .expect("Failed to add clippy component");

        println!("Added clippy component for toolchain: {}", toolchain);
    }

    Command::new("rustup-init")
        .args(["-y", "--quiet"])
        .status()
        .expect("Failed to init rustup");
}


fn parse_toolchains(rustup_output: &str) -> Vec<String> {
    let mut toolchains = Vec::new();
    let mut in_toolchains_section = false;

    for line in rustup_output.lines() {
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

    toolchains
}
