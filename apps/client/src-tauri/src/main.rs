// Evita abrir una ventana de consola adicional en Windows en builds release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    panaderia_control_client_lib::run()
}
