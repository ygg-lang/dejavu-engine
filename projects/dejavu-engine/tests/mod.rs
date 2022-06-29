use std::{
    env::{current_dir, set_current_dir},
    process::Command,
};

use dejavu_core::{DejavuWorkspace, QResult};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
#[ignore]
fn bootstrap() -> QResult {
    set_current_dir("../dejavu-bootstrap")?;
    let bootstrap = current_dir()?;
    let mut vm = DejavuWorkspace::new(&bootstrap)?;
    let config_path = vm.default_config_path();
    if config_path.exists() {
        vm.reload_config(&config_path)?;
    }
    let err = vm.compile_all();
    vm.print_errors(&err)?;
    vm.format_rs()?;
    Ok(())
}
