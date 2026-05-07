#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{TrayIconBuilder, TrayIconEvent},
    Emitter, Manager,
};
use tauri_plugin_global_shortcut::ShortcutState;
use serde::{Deserialize, Serialize};

struct CloseAction(Mutex<String>);

#[derive(Serialize, Deserialize)]
struct ChapterData {
    title: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct BookFile {
    chapters: Vec<ChapterData>,
}

fn books_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("books");
    if !dir.exists() {
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    }
    Ok(dir)
}

#[tauri::command]
fn save_book(app: tauri::AppHandle, book_id: String, chapters: Vec<ChapterData>) -> Result<(), String> {
    let path = books_dir(&app)?.join(format!("{}.json", book_id));
    let data = BookFile { chapters };
    let json = serde_json::to_string(&data).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn load_chapter(app: tauri::AppHandle, book_id: String, chapter_idx: usize) -> Result<String, String> {
    let path = books_dir(&app)?.join(format!("{}.json", book_id));
    let json = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let data: BookFile = serde_json::from_str(&json).map_err(|e| e.to_string())?;
    data.chapters
        .get(chapter_idx)
        .map(|ch| ch.content.clone())
        .ok_or_else(|| format!("Chapter index {} out of range", chapter_idx))
}

#[tauri::command]
fn delete_book(app: tauri::AppHandle, book_id: String) -> Result<(), String> {
    let path = books_dir(&app)?.join(format!("{}.json", book_id));
    if path.exists() {
        fs::remove_file(&path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn win_minimize(window: tauri::Window) {
    let _ = window.minimize();
}

#[tauri::command]
fn win_maximize(window: tauri::Window) {
    if window.is_maximized().unwrap_or(false) {
        let _ = window.unmaximize();
    } else {
        let _ = window.maximize();
    }
}

#[tauri::command]
fn win_hide_to_tray(window: tauri::Window) {
    let _ = window.hide();
}

#[tauri::command]
fn win_quit(app: tauri::AppHandle) {
    app.exit(0);
}

#[tauri::command]
fn set_close_action(state: tauri::State<CloseAction>, action: String) {
    if let Ok(mut a) = state.0.lock() {
        *a = action;
    }
}

fn restore_window(app: &tauri::AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.show();
        let _ = w.unminimize();
        let _ = w.set_focus();
        let _ = w.emit("restored-from-tray", ());
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            restore_window(app);
        }))
        .plugin({
            let builder = tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, _shortcut, event| {
                    if event.state == ShortcutState::Pressed {
                        restore_window(app);
                    }
                });
            match builder.with_shortcuts(["ctrl+shift+s"]) {
                Ok(b) => b.build(),
                Err(e) => {
                    eprintln!("Warning: Could not register Ctrl+Shift+S: {}. Tray click still works.", e);
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(|app, _shortcut, event| {
                            if event.state == ShortcutState::Pressed {
                                restore_window(app);
                            }
                        })
                        .build()
                }
            }
        })
        .manage(CloseAction(Mutex::new("ask".to_string())))
        .invoke_handler(tauri::generate_handler![
            win_minimize,
            win_maximize,
            win_hide_to_tray,
            win_quit,
            set_close_action,
            save_book,
            load_chapter,
            delete_book,
        ])
        .setup(|app| {
            let _ = books_dir(app.handle());

            let show_item = MenuItemBuilder::with_id("show", "显示窗口").build(app)?;
            let quit_item = MenuItemBuilder::with_id("quit", "退出").build(app)?;
            let menu = MenuBuilder::new(app)
                .item(&show_item)
                .separator()
                .item(&quit_item)
                .build()?;

            let icon = app.default_window_icon().cloned()
                .expect("Failed to load app icon for tray");

            let _tray = TrayIconBuilder::new()
                .icon(icon)
                .tooltip("ChatNovel")
                .menu(&menu)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "show" => restore_window(app),
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click { button, .. } = event {
                        if button == tauri::tray::MouseButton::Left {
                            restore_window(tray.app_handle());
                        }
                    }
                })
                .build(app)?;

            let app_handle = app.handle().clone();
            if let Some(window) = app.get_webview_window("main") {
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        let action = app_handle
                            .state::<CloseAction>()
                            .0
                            .lock()
                            .map(|a| a.clone())
                            .unwrap_or_else(|_| "ask".to_string());
                        if let Some(w) = app_handle.get_webview_window("main") {
                            let _ = w.emit("win-close-requested", action);
                        }
                    }
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running ChatNovel");
}
