# Infraestructura local — Panadería Control

Este directorio contiene la configuración de Docker Compose para levantar **PostgreSQL + PostGIS** y el motor de ruteo **OSRM** en local.

## Requisitos previos

- [Docker Desktop](https://www.docker.com/products/docker-desktop/) (con Docker Compose v2)
- Archivo `.env` en la raíz del repositorio (copiado desde `.env.example`)

---

## 1. Levantar PostgreSQL + PostGIS

El servicio `osrm` utiliza el perfil `osrm` (`profiles: ["osrm"]`), por lo que puedes levantar **únicamente PostgreSQL/PostGIS** sin necesidad de haber descargado ni preprocesado mapas todavía.

Desde la raíz del proyecto:

```bash
docker compose -f infra/docker-compose.yml up -d
```

O desde `infra/` indicando el archivo `.env` de la raíz:

```bash
docker compose --env-file ../.env up -d
```

Para comprobar que la base de datos está lista (`healthy`):

```bash
docker compose -f infra/docker-compose.yml ps
```

---

## 2. Preparar datos de mapas para OSRM

El servicio `osrm` espera encontrar el grafo preprocesado `/data/region.osrm*` dentro de `infra/osrm-data/` y utiliza el algoritmo **MLD** (`--algorithm mld`).

### Paso 1: Crear el directorio de datos

Desde la raíz del proyecto:

```powershell
# PowerShell (Windows)
New-Item -ItemType Directory -Force -Path infra/osrm-data
```

```bash
# Bash (Linux / macOS)
mkdir -p infra/osrm-data
```

### Paso 2: Descargar el extracto `.osm.pbf` de tu región

Descarga el extracto correspondiente desde [Geofabrik](https://download.geofabrik.de/) y guárdalo como `infra/osrm-data/region.osm.pbf`.

Por ejemplo, para España o una comunidad/región específica:

```powershell
# PowerShell (Windows) — ejemplo con Madrid (ajusta la URL a tu región)
Invoke-WebRequest -Uri "https://download.geofabrik.de/europe/spain/madrid-latest.osm.pbf" -OutFile "infra/osrm-data/region.osm.pbf"
```

```bash
# Bash (Linux / macOS) — ejemplo con Madrid (ajusta la URL a tu región)
curl -L -o infra/osrm-data/region.osm.pbf https://download.geofabrik.de/europe/spain/madrid-latest.osm.pbf
```

> **Consejo:** Empieza con un extracto regional pequeño (ciudad, provincia o país pequeño) para agilizar el preprocesamiento y reducir el consumo de RAM.

### Paso 3: Preprocesar con `osrm-extract`, `osrm-partition` y `osrm-customize`

Ejecuta los tres pasos del pipeline MLD usando la misma imagen de OSRM (`ghcr.io/project-osrm/osrm-backend:latest`) con el perfil de vehículo `/opt/car.lua`:

**En PowerShell (Windows) desde la raíz del proyecto:**

```powershell
$osrmData = (Resolve-Path "infra/osrm-data").Path

docker run --rm -t -v "${osrmData}:/data" ghcr.io/project-osrm/osrm-backend:latest `
  osrm-extract -p /opt/car.lua /data/region.osm.pbf

docker run --rm -t -v "${osrmData}:/data" ghcr.io/project-osrm/osrm-backend:latest `
  osrm-partition /data/region.osrm

docker run --rm -t -v "${osrmData}:/data" ghcr.io/project-osrm/osrm-backend:latest `
  osrm-customize /data/region.osrm
```

**En Bash (Linux / macOS) desde la raíz del proyecto:**

```bash
docker run --rm -t -v "$(pwd)/infra/osrm-data:/data" ghcr.io/project-osrm/osrm-backend:latest \
  osrm-extract -p /opt/car.lua /data/region.osm.pbf

docker run --rm -t -v "$(pwd)/infra/osrm-data:/data" ghcr.io/project-osrm/osrm-backend:latest \
  osrm-partition /data/region.osrm

docker run --rm -t -v "$(pwd)/infra/osrm-data:/data" ghcr.io/project-osrm/osrm-backend:latest \
  osrm-customize /data/region.osrm
```

Al finalizar, en `infra/osrm-data/` existirán múltiples archivos `region.osrm.*` necesarios para `osrm-routed`.

---

## 3. Levantar PostgreSQL + OSRM juntos

Una vez generados los archivos `infra/osrm-data/region.osrm*`, arranca ambos servicios activando el perfil `osrm`:

```bash
docker compose -f infra/docker-compose.yml --profile osrm up -d
```

### Verificar que OSRM responde

Sustituye las coordenadas por dos puntos válidos dentro de tu extracto (formato `longitud,latitud`):

```bash
curl "http://localhost:5000/route/v1/driving/-3.7038,40.4168;-3.6883,40.4153?overview=false"
```

Si la respuesta contiene `"code":"Ok"`, el servidor OSRM está operativo en `http://localhost:5000`.

---

## 4. Detener los servicios

```bash
# Detener contenedores conservando el volumen de PostgreSQL
docker compose -f infra/docker-compose.yml --profile osrm down

# Detener y eliminar también el volumen de datos de PostgreSQL (¡borra la BD local!)
docker compose -f infra/docker-compose.yml --profile osrm down -v
```
