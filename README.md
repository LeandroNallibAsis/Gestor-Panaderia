# Panadería Control

Sistema integral para la gestión operativa de panadería, pedidos, producción y logística de reparto con optimización y cálculo de rutas.

## Stack tecnológico

- **Cliente multiplataforma (Desktop / Android):** [Tauri 2](https://v2.tauri.app/) + [SvelteKit](https://svelte.dev/)
- **Backend API:** [Rust](https://www.rust-lang.org/) + [Axum](https://github.com/tokio-rs/axum)
- **Base de datos geoespacial:** PostgreSQL 16 + PostGIS 3.4
- **Motor de ruteo:** [OSRM](https://project-osrm.org/) (`osrm-backend` con algoritmo MLD)

## Estructura del monorepo

```text
.
├── apps/
│   └── client/         # Aplicación SvelteKit + Tauri 2 (escritorio y Android)
├── crates/
│   ├── core/           # Dominio, tipos compartidos y lógica de negocio en Rust
│   └── server/         # Servidor API REST/WebSocket con Axum y acceso a PostGIS/OSRM
├── infra/
│   ├── docker-compose.yml  # Servicios locales: PostgreSQL/PostGIS y OSRM
│   ├── osrm-data/          # Extractos .osm.pbf y grafos .osrm (ignorado en Git)
│   └── README.md           # Guía detallada para descargar y preprocesar mapas OSRM
├── .env.example        # Plantilla de variables de entorno
└── README.md
```

---

## Arranque en local

### 1. Configurar variables de entorno

Copia `.env.example` a `.env` en la raíz del proyecto y ajusta los valores si es necesario:

```powershell
# PowerShell (Windows)
Copy-Item .env.example .env
```

```bash
# Bash (Linux / macOS)
cp .env.example .env
```

### 2. Levantar infraestructura (PostgreSQL/PostGIS y OSRM)

Para arrancar únicamente **PostgreSQL + PostGIS** (no requiere mapas descargados):

```bash
docker compose -f infra/docker-compose.yml up -d
```

Si ya has preparado los datos de mapa en `infra/osrm-data/` siguiendo [infra/README.md](infra/README.md), levanta **PostgreSQL + OSRM**:

```bash
docker compose -f infra/docker-compose.yml --profile osrm up -d
```

### 3. Ejecutar el backend (Axum)

Desde la raíz del workspace de Rust:

```bash
cargo run -p server
```

El servidor leerá la configuración del entorno (`DATABASE_URL`, `JWT_SECRET`, `OSRM_URL`, `SERVER_HOST`, `SERVER_PORT`) y escuchará por defecto en `http://0.0.0.0:3000`.

### 4. Ejecutar el cliente (SvelteKit + Tauri)

Desde `apps/client/`:

```bash
npm install
# Modo web (SvelteKit)
npm run dev

# Modo escritorio (Tauri 2)
npm run tauri dev
```

---

## Requisitos para compilar en Android (fases posteriores)

Para empaquetar `apps/client` como app Android con Tauri 2, prepara el entorno nativo de Android siguiendo estos pasos:

### 1. Instalar Android Studio, SDK y NDK

1. Descarga e instala [Android Studio](https://developer.android.com/studio).
2. Abre **Android Studio → Settings / Preferences → Languages & Frameworks → Android SDK**:
   - En la pestaña **SDK Platforms**, instala el SDK objetivo (recomendado Android 14 / API 34 o superior).
   - En la pestaña **SDK Tools**, marca e instala:
     - **Android SDK Build-Tools**
     - **Android SDK Command-line Tools (latest)**
     - **Android SDK Platform-Tools**
     - **NDK (Side by side)**
3. Anota la ruta del SDK (por defecto en Windows: `%LOCALAPPDATA%\Android\Sdk`; en macOS: `~/Library/Android/sdk`; en Linux: `~/Android/Sdk`) y la versión exacta del NDK instalado dentro de `<SDK>/ndk/<version>`.

### 2. Configurar variables de entorno

Define las variables `JAVA_HOME`, `ANDROID_HOME` y `NDK_HOME`, y añade `platform-tools` y `cmdline-tools` al `PATH`:

**En Windows (PowerShell):**

```powershell
[Environment]::SetEnvironmentVariable("JAVA_HOME", "C:\Program Files\Android\Android Studio\jbr", "User")
[Environment]::SetEnvironmentVariable("ANDROID_HOME", "$env:LOCALAPPDATA\Android\Sdk", "User")
[Environment]::SetEnvironmentVariable("NDK_HOME", "$env:LOCALAPPDATA\Android\Sdk\ndk\<VERSION_NDK>", "User")
```

*(Reinicia la terminal después de definir las variables y sustituye `<VERSION_NDK>` por la carpeta instalada, por ejemplo `27.0.12077973`.)*

**En Linux / macOS (`~/.bashrc` o `~/.zshrc`):**

```bash
export JAVA_HOME="/Applications/Android Studio.app/Contents/jbr/Contents/Home" # o ruta de JDK 17+ en Linux
export ANDROID_HOME="$HOME/Android/Sdk" # en macOS: "$HOME/Library/Android/sdk"
export NDK_HOME="$ANDROID_HOME/ndk/<VERSION_NDK>"
export PATH="$PATH:$ANDROID_HOME/platform-tools:$ANDROID_HOME/cmdline-tools/latest/bin"
```

### 3. Instalar los targets de Rust para Android

```bash
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
```

### 4. Inicializar y ejecutar Tauri Android

Cuando el proyecto `apps/client` esté creado, inicializa la plantilla Android (genera `src-tauri/gen/android`, ignorado en Git) y ejecuta en emulador o dispositivo conectado por USB (`adb devices`):

```bash
cd apps/client
npm run tauri android init
npm run tauri android dev
```

Para generar el APK/AAB de distribución:

```bash
npm run tauri android build
```
