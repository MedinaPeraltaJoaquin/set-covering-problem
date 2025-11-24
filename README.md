# Proyecto 3: Problema de Cobertura de Conjuntos

---

## 📦 Requisitos

Antes de compilar y correr el proyecto, asegúrate de tener instalados los siguientes programas:

### 1. Rust y Cargo
Instala el *toolchain* oficial de Rust que incluye `cargo` (el gestor de paquetes y compilación):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Verifica la instalación:
```bash
rustc --version
cargo --version
```
---

## Construcción del proyecto.
Para esto, clona este repositorio y entra en la carpeta del proyecto:
```bash
git clone https://github.com/MedinaPeraltaJoaquin/set-covering-problem
cd set-covering-problem
```
Compila en modo debug:
```bash
cargo build --release
```

---

## 🚀 Ejecución del Proyecto

Para ejecutar el programa, puedes usar el comando `cargo run --` seguido de las opciones, o ejecutar directamente el binario compilado `target/release/set-covering-problem`:

```bash
cargo run -- <opciones>
./target/debug/set-covering-problem <opciones>
```

### 📋 Opciones de Línea de Comandos

Indica --help o -h para mostrar el menú completo:
```bash
Uso: programa [opciones]

Opciones:
  -h, --help         Muestra esta ayuda y termina
  -v                 Activa el modo verbose
  -svg               Activa el modo de salida SVG (Genera imágenes de convergencia y árbol final)
  -s <I> <F>         Genera semillas en el rango [I, F] (ej: -s 1 10)
  -s <n>             Inicializa con la semilla n (ej: -s 42)
  -rs <n>            Genera n semillas aleatorias
```
### Ejemplo de Ejecución:

### ⚙️ Archivo de Configuración (.env)
