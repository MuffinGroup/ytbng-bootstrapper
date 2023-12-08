use std::env;
use std::process::Command;
use std::path::Path;

fn main() {
    // Specify the path to the Python installer executable
    let python_installer_path = Path::new("python-3.12.1-amd64.exe");
    let fullpath = format!("{}/{}", env::current_dir().expect("").display(), python_installer_path.display());

    // Run the Python installer silently
    let output = Command::new(fullpath)
        .args(&["/quiet", "InstallAllUsers=1", "PrependPath=1"])
        .output()
        .expect("Failed to execute Python installer");

    // Check if the installer was successful
    if output.status.success() {
        println!("Python has been successfully installed!");
    } else {
        // Print the error message if the installer failed
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}