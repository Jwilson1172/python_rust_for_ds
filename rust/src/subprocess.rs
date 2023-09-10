
use std::io::Command;

fn os_commands_example_01() {
    let echo_cmd = if cfg!(target_os = "windows") {
        Command::new("cmd")
                                    .args(["/C", "echo hey from windows"])
                                    .output()
                                    .expect("failed to execute command.")
    } else {
        Command::new("sh")
                                    .args(["-c", "echo hey from windows"])
                                    .output()
                                    .expect("failed to execute command.")
    };
    println!("\n\n");
    let cmd_output = String::from_utf8(echo_cmd.stdout).expect("could not parse bytes");

    println!("{}", cmd_output);
}
