#![allow(non_snake_case)]

mod assets;
mod tray;
mod windowItem;

use assets::LocalAssets;
use css_color::Srgb;
use dioxus::{document::document, html::option::selected, prelude::*};
use dioxus_desktop::{tao::event::KeyEvent, use_window, use_wry_event_handler, window, WindowBuilder};
use dioxus_logger::tracing::Level;
use std::{env, fmt::Display, fs, process::Command, str::FromStr, sync::mpsc::channel, time::Duration};
use tray::TrayIcon;
use windowItem::WindowItem;

fn main() {
    LocalAssets::extract_assets();
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args[1] == "--open" {
        open_window();
    } else {
        let (tx, rx) = channel();
        let current_color: Option<[u8; 4]> = {
            let path = LocalAssets::get_path("./assets/main.css".to_string());
            let file = fs::read_to_string(path).unwrap();
            let color = file
                .split_once("--global-color:")
                .unwrap()
                .1
                .split_once(";")
                .unwrap()
                .0
                .replace(" ", "");
            let srgb_color = Srgb::from_str(&color)
                .expect(&format!("Error parsing color: '{color}', if the color looks wrong make sure that main.css has something like this `--global-color: YOUR_COLOR;`"));

            Some([
                (srgb_color.alpha * (255 as f32)) as u8,
                (srgb_color.red * (255 as f32)) as u8,
                (srgb_color.green * (255 as f32)) as u8,
                (srgb_color.blue * (255 as f32)) as u8,
            ])
        };

        TrayIcon::spawn(
            LocalAssets::get_path("./assets/icon.png".to_string()),
            move || {
                tx.send(0).unwrap();
            },
            current_color,
        );
        loop {
            let _ = rx.recv();
            open_process();
            while rx.try_recv().is_ok() {
                //flush all recieved events that are piled up, while main window was open
            }
        }
    }
}

fn open_process() {
    let mut window = Command::new(
        env::current_exe()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap(),
    );
    window.arg("--open");
    let _ = window.status();
}

fn open_window() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    let window = WindowBuilder::new().with_decorations(false);
    let config = dioxus_desktop::Config::new()
        .with_disable_context_menu(true)
        .with_window(window);
    dioxus::LaunchBuilder::new().with_cfg(config).launch(App);
}

pub struct HoverProps {
    inside: bool,
    action: Action,
}
#[derive(Clone, PartialEq)]
pub enum Action {
    Reboot,
    Shutdown,
    Logout
}


impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self  {
            Action::Reboot => "Reboot",
            Action::Shutdown => "Shutdown",
            Action::Logout => "Logout",
        };
        write!(f, "{}", str)
    }
}

impl Action {
    fn get_image(&self) -> String {
        match self {
            Action::Reboot => "./assets/restart.svg",
            Action::Shutdown => "./assets/shutdown.svg",
            Action::Logout => "./assets/logout.svg",
        }.to_string()
    }
}


#[component]
fn App() -> Element {
    let mut selected_action: Signal<Option<Action>> = use_signal(|| None);
    let mut confirmed = use_signal(|| false);
    let onhover = move |event: HoverProps| {
        if confirmed() {
            return;
        }
        if event.inside {
            *selected_action.write() = Some(event.action);
        } else {
            *selected_action.write() = None;
        }
    };
    
    let onClick = move |action: Action| {
        if confirmed() {
            return;
        }
        *confirmed.write() = true;
        spawn(async move {
            tokio::time::sleep(Duration::from_secs(1)).await;
            match action {
                Action::Reboot => {
                    Command::new("reboot")
                        .spawn()
                        .expect("reboot")
                        .wait()
                        .unwrap();
                }
                Action::Shutdown => {
                    Command::new("shutdown")
                        .arg("now")
                        .spawn()
                        .expect("shutdown now")
                        .wait()
                        .unwrap();
                }
                Action::Logout => {
                    Command::new("loginctl")
                        .arg("terminate-session")
                        .arg(env::var_os("XDG_SESSION_ID").unwrap())
                        .spawn()
                        .expect("loginctl terminate-session")
                        .wait()
                        .unwrap();
                }
            }
        });
    };

    use_effect(move || {
        use_window().set_fullscreen(true);
    });

    // close window on escape button
    use_wry_event_handler(move |ev, hand| {
        match ev {
            dioxus_desktop::tao::event::Event::WindowEvent { window_id, event , .. } => {
                match event {
                    dioxus_desktop::WindowEvent::KeyboardInput { device_id, event, is_synthetic , .. } => {
                        match event.physical_key {
                            dioxus_desktop::tao::keyboard::KeyCode::Escape => {
                                use_window().close();
                            },
                            _ => {},
                        }
                    },
                    _ => {},
                }
            },
            _ => {},
        }
    });

    rsx! {

        link { rel: "stylesheet", href: LocalAssets::get_path("./assets/main.css"), }
        div {
            class: "vbox", height: "100%", width: "95%",
            div { class: "hbox", width: "100%", margin: "10%",
                WindowItem {action: Action::Reboot,    position: 1,  onhover: onhover, onClick: onClick, selected: if confirmed() {selected_action.read().clone()} else {None}}
                WindowItem {action: Action::Shutdown,  position: 0,  onhover: onhover, onClick: onClick, selected: if confirmed() {selected_action.read().clone()} else {None}}
                WindowItem {action: Action::Logout,    position: -1, onhover: onhover, onClick: onClick, selected: if confirmed() {selected_action.read().clone()} else {None}}
            }

            div {
                class: "text-div",
                width: if selected_action.read().is_some() {"100%"} else {"0%"},
                color: if selected_action.read().is_some() {"rgba(0, 0, 0, 1)"} else {"rgba(0, 0, 0, 0)"},
                if let Some(action) = &*selected_action.read() {
                    "{action}"
                }
            }
        }
        div {
            class: "bottom-corner",
            transform: if selected_action.read().is_some() {"translateY(100vh)"} else {""},
            button { onclick: move |_| window().close(),
                "Cancel"
            }
        }
    }
}
