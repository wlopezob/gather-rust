# ADR-001: Movimiento por tiles enteros

- Estado: Aceptado
- Fecha: 2026-11-10

## Contexto

En gather-rust el `world-server` es la autoridad: decide dónde está cada jugador. Para que el movimiento se sienta inmediato, el navegador no espera la respuesta del servidor, que tarda entre 50 y 150 ms por la red. Mueve el avatar al instante (predicción) y envía solo la intención, por ejemplo "Right". El servidor aplica esa intención y luego confirma o corrige la posición.

Esto impone dos requisitos:

1. **Determinismo.** Con la misma entrada, el navegador y el servidor deben calcular exactamente lo mismo. Si los resultados difieren, el servidor corrige y el avatar salta hacia atrás en cada paso. Por eso los dos usan el mismo crate `world-core`, compilado de forma nativa para el servidor y a WebAssembly para el navegador.
2. **Validación en el servidor.** El cliente no es confiable: un navegador modificado puede enviar movimientos imposibles, como atravesar un muro o teletransportarse. El servidor nunca acepta una posición enviada por el cliente: recibe intenciones, las valida con `world-core` y rechaza las inválidas.

Compartir el código no basta: el modelo de posiciones también debe dar resultados idénticos en ambas plataformas y ser fácil de validar. Este ADR define ese modelo.

## Decisión

Las posiciones del mundo son casillas (tiles) enteras, y el movimiento avanza de casilla en casilla:

- Una posición es `TilePos { x: u16, y: u16 }`. El origen `(0, 0)` está arriba a la izquierda: `x` crece hacia la derecha e `y` hacia abajo.
- Hay 4 direcciones (`Up`, `Down`, `Left`, `Right`), sin diagonales.
- Un paso (`TilePos::step`) avanza exactamente una casilla. Usa `checked_add` y `checked_sub`: si el resultado se sale del rango de `u16`, devuelve `None` en lugar de desbordar.
- `TileMap` guarda la capa de colisión. `is_walkable` devuelve `false` para un muro y para cualquier posición fuera del mapa.
- `world-core` no usa números de punto flotante (`f32`, `f64`) en el estado ni en las reglas del mundo. Que el movimiento se vea suave es responsabilidad del cliente.

## Alternativas consideradas

1. **Posiciones continuas con `f32`**, con velocidad y `dt` por frame, como en la mayoría de los juegos.
   - Gana: movimiento libre y fluido, con diagonales y cualquier velocidad.
   - Se descarta sobre todo por la validación: el servidor ya no puede comprobar que el jugador avanzó exactamente una casilla. Tendría que aceptar "avanzó como mucho velocidad × tiempo, más un margen", y un cliente modificado puede moverse más rápido sin salirse de ese margen.
   - Además, el resultado depende de cuántos frames hubo. Sumar `1/fps` en `f32` durante 1 segundo da 0.999999702 a 60 fps, 0.999998271 a 144 fps y 1.000000119 a 20 ticks, así que el navegador y el servidor se desincronizan. Las colisiones necesitan rectángulos que ocupan varias casillas, y los tests necesitan tolerancias.

2. **Punto fijo**: enteros que representan fracciones de casilla (por ejemplo, 1/16), como guardar dinero en centavos dentro de un `long`.
   - Gana: no hay decimales, así que siempre da el mismo resultado en el servidor y en el navegador, y la validación es exacta. Permite movimiento suave dentro de la casilla y diagonales.
   - Se descarta porque el avatar vuelve a quedar entre casillas (por ejemplo, en 4.375). Las colisiones vuelven a necesitar rectángulos que ocupan hasta 4 casillas, y aparecen velocidades y diagonales con √2 aproximado. Con `u16` y fracciones de 1/16, el mapa máximo baja a 4 096 casillas; para evitarlo habría que usar `u32`, con 8 bytes por posición. Una oficina virtual no necesita precisión dentro de la casilla: la suavidad visual la da el cliente interpolando.

## Consecuencias

**Positivas**

- La validación en el servidor es exacta: un movimiento es válido si avanza una casilla hacia una casilla caminable. No hay márgenes que un tramposo pueda aprovechar.
- Una colisión es leer un `bool` en la capa del mapa.
- Una posición ocupa 4 bytes en cada mensaje, y se envían unos 20 mensajes por segundo por jugador.
- Los tests comparan valores exactos con `assert_eq!`, sin tolerancias.

**Negativas**

- La posición real avanza "a saltos", de una casilla por vez.
- No hay diagonales ni movimiento libre.
- Cambiar esta decisión más adelante obliga a tocar `world-core`, el protocolo, el servidor y el cliente. Si llegara a hacer falta, se escribe un ADR nuevo que reemplace a este.

**Mitigación**

- El cliente interpola: la lógica salta de casilla en casilla, pero Bevy desliza el sprite desde la casilla anterior hasta la nueva a lo largo de varios frames (lección 3.4). En el dibujo sí se usan floats, porque no afectan las reglas del mundo.

**Tamaño del mapa**

- Con `u16`, un mapa puede medir hasta 65 535 × 65 535 casillas: unos 2 millones de píxeles de ancho con casillas de 32 px. En la práctica no es un límite. Al contrario: un mapa así ocuparía unos 4 GB de memoria, así que la carga de mapas (lección 1.2) debe fijar un tamaño máximo razonable.
- El tamaño no está fijo en el código. Cada espacio tiene su propio mapa, hecho en Tiled y subido por la API (lección 4.4), y `TileMap::new` recibe el ancho y el alto como datos.
