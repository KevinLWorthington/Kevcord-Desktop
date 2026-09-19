// Kevcord desktop shell: a small native window around the Kevcord web app.
// Closing the window hides it to the tray so voice and notifications keep
// running in the background.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    menu::{CheckMenuItem, Menu, MenuItem},
    tray::{TrayIconBuilder, MouseButton, MouseButtonState, TrayIconEvent},
    Manager,
};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};

/// Origin of the bundled connect page (differs per platform in Tauri).
fn home_url() -> &'static str {
    if cfg!(windows) {
        "http://tauri.localhost/index.html?reset=1"
    } else {
        "tauri://localhost/index.html?reset=1"
    }
}

fn show_main(app: &tauri::AppHandle) {
    if let Some(win) = app.get_webview_window("main") {
        let _ = win.show();
        let _ = win.unminimize();
        let _ = win.set_focus();
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            // Launching a second copy just surfaces the running one.
            show_main(app);
        }))
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        // External links from the web app open in the system browser.
        .plugin(tauri_plugin_opener::init())
        // Native notifications. Its script also swaps in a window.Notification
        // that shows real Windows toasts, which the webview can't do on its own.
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let open = MenuItem::with_id(app, "open", "Open Kevcord", true, None::<&str>)?;
            let autostart_on = app.autolaunch().is_enabled().unwrap_or(false);
            let autostart = CheckMenuItem::with_id(
                app,
                "autostart",
                "Start with system",
                true,
                autostart_on,
                None::<&str>,
            )?;
            let change = MenuItem::with_id(app, "change", "Change server", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit Kevcord", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&open, &autostart, &change, &quit])?;

            let autostart_item = autostart.clone();
            TrayIconBuilder::with_id("kevcord")
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("Kevcord")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_tray_icon_event(|tray, event| {
                    // Left-click the tray icon to bring the window forward.
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        show_main(tray.app_handle());
                    }
                })
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "open" => show_main(app),
                    "autostart" => {
                        let launcher = app.autolaunch();
                        let enabled = launcher.is_enabled().unwrap_or(false);
                        let _ = if enabled {
                            launcher.disable()
                        } else {
                            launcher.enable()
                        };
                        let _ = autostart_item
                            .set_checked(launcher.is_enabled().unwrap_or(false));
                    }
                    "change" => {
                        if let Some(win) = app.get_webview_window("main") {
                            let _ = win.eval(&format!("location.replace('{}')", home_url()));
                        }
                        show_main(app);
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .build(app)?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                // Hide to tray instead of quitting.
                let _ = window.hide();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("failed to run Kevcord desktop");
}
