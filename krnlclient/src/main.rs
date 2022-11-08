#[cfg(windows)]
use dll_syringe::{Syringe, process::OwnedProcess};

#[cfg(windows)]
fn main() {
    println!("Injecting Krnl dll...");
    let target_process = OwnedProcess::find_first_by_name("RobloxPlayerBeta.exe").unwrap();
    // create a new syringe for the target process
    let syringe = Syringe::for_process(target_process);
    // inject the payload into the target process
    let injected_payload = syringe.inject("krnl.dll").unwrap();
}

#[cfg(not(windows))]
fn main() {
    println!("krnlclient can only be used in Windows environment");
}
