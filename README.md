# Rust 3D Graphics Engine

Este proyecto es un motor de gráficos 3D escrito en Rust. Es capaz de cargar y renderizar modelos 3D en formato `.obj`, utilizando transformaciones de rotación, traslación y escala. Los usuarios pueden interactuar con el modelo a través de entradas de teclado para realizar rotaciones, ajustes de zoom y mover la cámara en un modo de órbita.

### Teclas de Control

- **A / D**: Mover la cámara hacia la izquierda/derecha en el espacio 3D.
- **W / S**: Acercar/alejar la cámara del modelo (Zoom).
- **Flechas de Dirección**: Rotar la cámara alrededor del modelo (órbita).
- **Escape**: Cerrar la aplicación.

# Controles y configuración de Planet Shader

Este proyecto incluye varios shaders para simular distintos tipos de planetas y fenómenos. A continuación encontrará una guía para controlar cada sombreador y cómo cambiar entre ellos en tiempo real.


## Controles de Shader

Las siguientes teclas se utilizan para alternar entre shaders:

- **1**: Selecciona el **Shader de Planeta Rocoso**

    - **Modelo**: Esfera

    - **Descripción**: Un shader diseñado para imitar una superficie planetaria seca y rocosa con terreno accidentado.

- **2**: Selecciona el **Shader de Planeta Gaseoso**

    - **Modelo**: Esfera

    - **Descripción**: Un shader que representa un gigante gaseoso con capas de nubes animadas y sutiles cambios de color.

- **3**: Selecciona el **Shader de Planeta Congelado**

    - **Modelo**: Esfera

    - **Descripción**: Simula un planeta helado con texturas granuladas y variaciones de color que emulan hielo y nieve.

- **4**: Selecciona el **Shader de Planeta Terrestre**

    - **Modelo**: Esfera

    - **Descripción**: Un shader que se asemeja a la Tierra, con texturas de océano y tierra, movimiento de nubes y un efecto atmosférico.

- **5**: Selecciona el **Shader de Planeta Oceánico**

    - **Modelo**: Esfera

    - **Descripción**: Crea un mundo cubierto de océano con patrones de olas dinámicas y reflejos de luz.

- **6**: Selecciona el **Shader de OVNI**

    - **Modelo**: OVNI

    - **Descripción**: Un shader metálico y reflectante para modelos de OVNI, con luces pulsantes y un aura.

- **7**: Selecciona el **Shader de Gargantúa**

    - **Modelo**: Ojo

    - **Descripción**: Un shader de agujero negro inspirado en Gargantúa de *Interstellar*, con un disco de acreción y emisión intensa.

- **8**: Selecciona el **Shader de Agujero de Gusano**

    - **Modelo**: Esfera

    - **Descripción**: Simula un agujero de gusano con un borde brillante y un patrón en cruz en el centro.

## Uso

Cada shader puede seleccionarse usando la tecla asignada (1-8). Al seleccionar un shader, el modelo y la cámara se restablecerán automáticamente a una configuración adecuada.
 
### Descripción del Proyecto

El motor utiliza la librería `minifb` para manejar la ventana y el buffer gráfico, y `nalgebra_glm` para los cálculos de matrices y vectores. Los modelos en formato `.obj` se cargan a través de un módulo que procesa sus vértices y los transforma de acuerdo a las entradas de usuario.

Las transformaciones del modelo incluyen:
- **Rotación**: Alrededor de los ejes X, Y y Z.
- **Escalado**: Control de zoom para acercar o alejar el modelo.
- **Traslación**: Movimiento del modelo en el espacio de la ventana.

### Ejecución

Para ejecutar este proyecto en modo de lanzamiento, sigue estos pasos:

1. Asegúrate de tener Rust instalado.
2. Coloca el archivo `.obj` en la carpeta `src/assets/` con el nombre `spaceship.obj`.
3. Corre el siguiente comando para compilar y ejecutar el proyecto:
```cargo run --release```




### Dependencias

- `nalgebra_glm`: Para cálculos de matrices y vectores.
- `minifb`: Para la gestión de ventanas y gráficos.
- Otros módulos internos para manejar el framebuffer, shader, y procesamiento de vértices.

### Video:

[Shaders 3D Model](https://www.youtube.com/watch?v=PFDPWAGDjos)



