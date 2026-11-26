#[tauri::command]
fn obtener_info_cliente() -> &'static str {
    "Panadería Control v0.1.0"
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![obtener_info_cliente])
        .run(tauri::generate_context!())
        .expect("error al iniciar la aplicación Tauri de Panadería Control");
}
