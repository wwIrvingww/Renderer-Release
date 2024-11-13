
# Proyecto de Simulación de Planetas

Este proyecto es una simulación interactiva de un sistema planetario en 3D, desarrollado en Rust utilizando diversas bibliotecas para gráficos, ruido procedural, y reproducción de audio. El objetivo es ofrecer una experiencia visual de exploración espacial, con múltiples planetas que orbitan alrededor de un centro, efectos de shaders personalizados y música ambiental.

## Requisitos

- **Rust**: Asegúrate de tener instalado Rust. Puedes instalarlo desde [Rust Official Website](https://www.rust-lang.org/).
- **Dependencias**: Este proyecto utiliza varias dependencias, incluyendo:
  - `minifb` para la ventana gráfica.
  - `nalgebra_glm` para operaciones matemáticas y transformaciones.
  - `rodio` para la reproducción de audio.
  - `fastnoise-lite` para generar ruido procedural.

Para instalar todas las dependencias, simplemente corre:

```bash
cargo build
```

## Ejecución del proyecto

Para ejecutar el proyecto en modo release (recomendado para mejor rendimiento), utiliza el siguiente comando:

```bash
cargo run --release
```

## Controles del programa

### Navegación:

- **Teclas de Flechas (←, →, ↑, ↓)**: Orbitar la cámara alrededor del sistema planetario.
- **Teclas W/S**: Hacer zoom (acercar o alejar la cámara).
- **Tecla B**: Activar/desactivar la vista Bird Eye (vista desde arriba).
- **Tecla Escape**: Salir del programa.

### Selección de Planetas:

Puedes moverte rápidamente a la órbita de un planeta usando las teclas numéricas:

- **Tecla 1**: Saltar al planeta rocoso (`Rocky Planet`).
- **Tecla 2**: Saltar al planeta gaseoso (`Gaseous Planet`).
- **Tecla 3**: Saltar al planeta congelado (`Frozen Planet`).
- **Tecla 4**: Saltar al planeta tierra (`Earth Planet`).
- **Tecla 5**: Saltar al planeta oceánico (`Oceanic Planet`).
- **Tecla 6**: Saltar al OVNI (`UFO`).
- **Tecla 7**: Saltar al agujero negro (`Gargantua`).
- **Tecla 8**: Saltar al agujero de gusano (`Wormhole`).

### Audio:

La simulación incluye música de fondo que se reproduce en un bucle infinito. Puedes ajustar el volumen en el archivo `audio_player.rs` si es necesario.

## Demostración

Puedes ver un video de demostración del proyecto funcionando en el siguiente enlace:

[![Video de demostración](https://img.youtube.com/vi/EAsEjQPio24/0.jpg)](https://www.youtube.com/watch?v=6-lyr18Gy4Q)

## Créditos

Este proyecto fue desarrollado por **Irving Acosta** como parte de un proyecto universitario. Incluye implementación de shaders personalizados, manejo de colisiones y reproducción de audio utilizando Rust.

## Notas adicionales

- **Optimización**: El proyecto se ejecuta mejor en modo release (`cargo run --release`).
- **Licencia**: Este proyecto se distribuye bajo la licencia MIT.

## Contacto

Para cualquier duda o sugerencia, por favor, contacta a través del repositorio del proyecto.
