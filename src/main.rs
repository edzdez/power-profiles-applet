use std::process::Command;

use gtk::prelude::*;
use libappindicator::{AppIndicator, AppIndicatorStatus};

fn get_current_power_profile() -> String {
    let output = Command::new("powerprofilesctl")
        .arg("get")
        .output()
        .expect("Failed to call powerprofilesctl")
        .stdout;

    String::from_utf8(output).unwrap().trim().to_string()
}

fn get_all_power_profiles() -> Vec<String> {
    let output = Command::new("powerprofilesctl")
        .arg("list")
        .output()
        .expect("Failed to call powerprofilesctl")
        .stdout;

    let output = String::from_utf8(output).unwrap();
    output
        .trim()
        .split("\n\n")
        .map(|output| {
            let (a, _) = output.split_once('\n').unwrap();
            let a = a.replace('*', "").replace(' ', "");
            a[..a.len() - 1].trim().to_string()
        })
        .collect()
}

fn set_power_profile(label: &str) {
    Command::new("powerprofilesctl")
        .args(["set", label])
        .spawn()
        .expect("Failed to call powerprofilesctl");
}

fn create_menu() -> gtk::Menu {
    let menu = gtk::Menu::builder().halign(gtk::Align::Start).build();

    let current_profile_label = gtk::MenuItem::builder()
        .label("Power Profiles: ")
        .halign(gtk::Align::Start)
        .sensitive(false)
        .build();

    let divider = gtk::SeparatorMenuItem::new();

    menu.append(&current_profile_label);
    menu.append(&divider);

    let current_profile = get_current_power_profile();
    let profiles = get_all_power_profiles();

    let mut group = None;
    for profile in profiles {
        let profile_button = gtk::RadioMenuItem::builder().label(&profile).build();
        // when activated, set the power profile to this profile
        profile_button.connect_activate(|b| {
            // don't act if being deactivated
            if !b.is_active() {
                return;
            }

            let label = b.label().unwrap().to_string();

            println!("setting power profile to {}", &label);
            set_power_profile(&label);
        });

        // add this button to the group
        match group {
            Some(ref group) => profile_button.join_group(Some(group)),
            None => group = Some(profile_button.clone()),
        }

        // activate the button for the currently active power profile
        if profile == current_profile {
            profile_button.activate();
        }
        menu.append(&profile_button);
    }

    menu
}

fn main() {
    gtk::init().expect("Failed to init gtk");

    let mut indicator = AppIndicator::new("Power Profiles Indicator", "battery-good");
    indicator.set_status(AppIndicatorStatus::Active);

    let mut menu = create_menu();

    indicator.set_menu(&mut menu);
    menu.show_all();
    gtk::main();
}
