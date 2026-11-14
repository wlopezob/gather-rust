# Plan: de Java senior a Rust senior construyendo Rust-first Virtual Workspace

Arquitectura de referencia: [`docs/propuesta-arquitectura.md`](docs/propuesta-arquitectura.md).
Dedicación: 1,5 horas al día, unas 10 horas por semana. Las fechas de los puntos de control son aproximadas y se recalculan al cerrar la Fase 1.

La arquitectura de referencia describe el destino con varios nodos (Kubernetes, leases en Redis). Este plan llega primero a producción con un solo nodo y deja lo demás para la Fase 7.

Por dónde vamos, las fechas reales y los temas pendientes de repaso están en [`PROGRESO.md`](PROGRESO.md).

## Cómo trabajamos

**Roles.** Tú escribes todo el código y tomas las decisiones finales. Yo (Claude) soy el profesor: explico, especifico, reviso y pregunto. No edito archivos de código.

**Por qué este orden.** Primero el núcleo más difícil y más "Rust" (simulación y tiempo real), sin interfaz, porque ahí está el aprendizaje que te hace senior y el riesgo técnico más alto. La API va después porque ahí tu experiencia Java ya pesa. La calidad (CI) existe desde el día 1 y el primer despliegue llega en la fase 2, no al final.

### Ciclo de cada lección

1. **Repaso (los dos).** Antes de empezar, 2 preguntas tomadas de los temas pendientes de `PROGRESO.md`. La que falles vuelve en la lección siguiente, hasta que salga bien.
2. **Lectura (tú).** 20–30 minutos del capítulo que indica la lección, antes de implementar.
3. **Contexto (yo).** Qué construimos y para qué: el objetivo funcional, el no funcional (rendimiento, seguridad, operación) y los conceptos de Rust nuevos, con su equivalente en Java cuando ayude. Una lectura corta.
4. **Especificación (yo).** Firmas públicas, criterios de aceptación y casos de prueba que debes escribir. Desde la 1.3, una parte de cada lección va **sin guía**: solo el requisito y los tests que debe pasar.
5. **Implementación (tú).** Intentas primero. Si te trabas, pides una pista (ver la escalera).
6. **Code review (yo).** Leo tu `git diff`, corro clippy y los tests, y comento como un lead: bloqueante, sugerencia o detalle, siempre con el porqué. Tú corriges.
7. **Defensa (tú).** Te hago 3–5 preguntas de entrevista sobre lo que construiste. Las respondes con tus palabras.
8. **Cierre (los dos).** Tú: commit, ADR si hubo una decisión y tus notas en `docs/bitacora/X.Y-nombre.md`. Yo: agrego mi revisión en ese archivo y marco la lección en este plan.

### Escalera de pistas

- **Pista 1:** el concepto o la parte de la documentación que necesitas.
- **Pista 2:** la estructura: firmas, tipos, el esqueleto del algoritmo.
- **Pista 3:** un fragmento de código de ejemplo que escribes y adaptas tú.

Solo subimos de nivel cuando lo pidas o después de un intento real. Copiar no enseña; equivocarse y corregir con review sí.

### Autonomía progresiva

| Fases | Qué te doy yo | Qué haces tú |
|---|---|---|
| 0–1 | Explicación, firmas y ejemplos para cada concepto nuevo. Desde la 1.3, una parte sin guía por lección | Implementar y escribir los tests |
| 2–3 | Firmas y criterios de aceptación | Implementar, diseñar los tests |
| 4–7 | Solo requisitos y restricciones | Diseñar firmas, módulos y tests; defender el diseño |

### Definition of Done de una lección

- `just check` pasa: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo nextest run` y `cargo deny check`.
- CI en verde.
- Commit con mensaje convencional (`feat:`, `fix:`, `test:`, `docs:`).
- ADR en `docs/adr/` si hubo una decisión de diseño.
- Archivo `docs/bitacora/X.Y-nombre.md` con tus notas (una línea por sesión con fecha y minutos, qué aprendiste, qué te costó, qué harías distinto) y mi revisión (code review, preguntas de defensa con la respuesta completa, temas para repasar).
- Preguntas de repaso y de defensa respondidas.

### Cómo retomar

- "Sigamos con el plan": reviso el repo y continúo la lección en curso.
- "Lección X.Y": vamos a una lección concreta.
- "Pista": subo un nivel en la escalera.
- "Revisa": hago code review de tus cambios. Antes de escribirlo, corre `just check` y confirma que aplicaste todos los comentarios del review anterior.

### Ritmo: una sesión de 90 minutos al día

- **10 min:** repaso de dónde quedamos y las 2 preguntas de repaso.
- **60 min:** implementas.
- **20 min:** review, correcciones y commit.

Una lección toma entre 2 y 4 sesiones. Cada sesión termina con un commit, aunque sea pequeño. Si un día solo tienes 30 minutos, úsalos: la constancia diaria rinde más que una sesión larga a la semana.

### Puntos de control

Fechas aproximadas, empezando el 2 de noviembre de 2026 a unas 10 horas por semana.

| Al cerrar | Fecha aproximada | Lo que ya sabes hacer |
|---|---|---|
| Fase 0 | 9 nov 2026 | Workspace, toolchain y CI |
| Fase 1 | 14 dic 2026 | Rust fluido: ownership, traits, errores, tests y benchmarks |
| Fase 2 | 1 feb 2027 | Async, Tokio, concurrencia y carga: nivel de entrevista backend senior |
| Fase 3 | 15 mar 2027 | WASM y cliente |
| Fase 4 | 19 abr 2027 | API, datos y login |
| Fase 5 | 17 may 2027 | Audio y video por proximidad |
| Fase 6 | 14 jun 2027 | Producto en producción con usuarios |

Las fiestas de fin de año probablemente muevan las fechas desde la Fase 2. Las Fases 4 y 6 pueden ir más rápido por tu experiencia en Java y DevOps.

---

## Fase 0 · Preparación (≈1 semana)

- [x] **0.1 Diagnóstico y entorno**
  - Funcional: ninguno todavía; dejamos lista la base del repositorio.
  - No funcional: toolchain fijado y builds reproducibles.
  - Rust: rustup y cargo; workspace virtual frente a un proyecto Maven multi-módulo; edición 2024; `Cargo.lock`; perfiles.
  - Entregable: `git init`, workspace con un crate `world-core` vacío, `rust-toolchain.toml`, `.gitignore`, `docs/bitacora/`, primer commit. Diagnóstico: 6 preguntas para calibrar tu punto de partida.
- [x] **0.2 Calidad desde el día 1**
  - No funcional: gates de calidad automáticos en cada push.
  - Rust: rustfmt, clippy y `[workspace.lints]`, cargo-nextest, cargo-deny (licencias y vulnerabilidades).
  - Entregable: GitHub Actions con fmt, clippy, tests y deny; README con el badge de CI.

## Fase 1 · world-core: la simulación (≈4–5 semanas)

- [x] **1.1 Tipos del dominio**
  - Funcional: posiciones en tiles, direcciones, jugadores y un mapa con capa de colisión.
  - No funcional: determinismo; la misma entrada da el mismo resultado en servidor y navegador. ADR-001: movimiento por tiles enteros.
  - Rust: structs y enums, newtype (`PlayerId`), `derive`, `Copy` vs `Clone` vs move, bloques `impl`, módulos y visibilidad, `Option`.
  - Entregable: tipos del dominio con tests unitarios.
- [x] **1.2 Cargar mapas de Tiled**
  - Funcional: usar mapas hechos en el editor Tiled (`.tmj`).
  - No funcional: el mapa es input no confiable; límites de tamaño, errores claros y ningún `panic`.
  - Rust: serde, `Result` y `?`, `thiserror`, `From`, lifetimes básicos.
  - Lectura (repaso al cerrar): Book cap. 9 (manejo de errores).
  - Entregable: loader de mapas con fixtures válidos e inválidos.
- [ ] **Antes de la 1.3: `just check`**
  - Instalar `just` y crear un `justfile` con la receta `check`: fmt, clippy, nextest y deny. Es el comando que corres antes de cada "revisa".
- [ ] **1.3 Movimiento y colisiones**
  - Funcional: mover un avatar; las paredes y los bordes bloquean.
  - No funcional: invariantes verificadas con property testing.
  - Rust: `&self` vs `&mut self`, el borrow checker en la práctica, `HashMap`, `match` exhaustivo, `proptest`.
  - Lectura: Book cap. 4 (ownership y préstamos) y cap. 8 (colecciones, la parte de `HashMap`).
  - Sin guía: la propiedad de `proptest` la diseñas tú.
  - Entregable: `World::apply` que acepta o rechaza un input, y la propiedad "nadie termina dentro de una pared".
- [ ] **1.4 Proximidad**
  - Funcional: saber quién está cerca de quién, con histéresis (entrar a 5 tiles, salir a 7).
  - No funcional: grilla espacial O(n) en lugar de comparar todos contra todos; medirlo. ADR-002.
  - Rust: iteradores y closures, genéricos con trait bounds, conciencia de allocations, `criterion`.
  - Lectura: Book cap. 13 (closures e iteradores) y cap. 10 (genéricos y traits).
  - Entregable: grilla espacial y benchmark con 1 000 jugadores, con resultados en la bitácora.
- [ ] **1.5 Portabilidad a WASM**
  - No funcional: `world-core` compila a `wasm32-unknown-unknown` y CI lo verifica.
  - Rust: targets, `cfg`, features de Cargo, higiene de dependencias.
  - Lectura: Cargo Book, sección "Features".
  - Spike de una sesión: compilar un Bevy 2D mínimo a WASM y anotar el tamaño (con `wasm-opt` y brotli) y el tiempo de carga. Es el mayor riesgo del cliente. Si el resultado no es aceptable, el renderer se decide ahora con un ADR, no en la Fase 3.
  - Entregable: job de CI para `wasm32` y los números del spike en la bitácora.
- [ ] **1.6 El mundo en la terminal**
  - Funcional: cargar un `.tmj`, dibujarlo en ASCII y caminar con WASD. Es lo primero que se ve del proyecto.
  - No funcional: el ejemplo usa solo la API pública de `world-core`; si algo es incómodo de usar, se corrige la API.
  - Rust: carpeta `examples/`, `std::io`, `Display`, `anyhow` en binarios frente a `thiserror` en librerías.
  - Lectura: Book cap. 12 (un programa de línea de comandos).
  - Entregable: `cargo run --example caminar`. Se puede adelantar apenas cierre la 1.3.
- [ ] **Hito 1:** núcleo probado y medido. Post: "El mismo código de simulación en servidor y navegador".
- [ ] **Recalcular el plan** con tus horas reales por lección y ajustar las fechas de los puntos de control.

## Fase 2 · protocol + world-server: tiempo real (≈6–7 semanas)

- [ ] **2.1 Protocolo**
  - Funcional: mensajes cliente ↔ servidor (entrar, input, delta, chat, error).
  - No funcional: compatibilidad entre versiones; bytes por mensaje medidos. Lo que llega por la red es input no confiable: fuzzing con `cargo fuzz` sobre el decodificador del protocolo y sobre `TileMap::from_tmj` (necesita el toolchain nightly solo para ese comando).
  - Rust: enums con datos, serde + postcard, tests de ida y vuelta, `#[non_exhaustive]`; leer con `cargo expand` lo que genera un `derive`.
  - Lectura: Book cap. 6 (enums y `match`) y cap. 19 (patrones).
  - Entregable: crate `protocol` con versión explícita y dos fuzz targets.
- [ ] **2.2 Tokio y el actor del espacio**
  - Funcional: un espacio vivo que avanza 20 veces por segundo.
  - No funcional: presupuesto de tiempo por tick y qué hacer si un tick se atrasa.
  - Rust: async/await, tasks, `Send + 'static`, `mpsc`, `oneshot`, `broadcast`, `select!`. Por qué un actor y no `Arc<Mutex<_>>`. Antes de usar Tokio, escribir un `Future` a mano para entender `poll`, `Pin` y `Waker`.
  - Lectura: Book cap. 16 (concurrencia) y cap. 17 (async); tutorial oficial de Tokio.
  - Entregable: actor con loop de tick, probado con `tokio::time::pause`.
- [ ] **2.3 WebSocket con Axum**
  - Funcional: los clientes se conectan a un espacio y se mueven.
  - No funcional: backpressure (cola acotada por cliente y política para clientes lentos), heartbeats, tamaño máximo de mensaje, timeouts.
  - Rust: `Arc`, registro compartido de espacios, socket dividido en lector y escritor, cancel-safety, errores en código async.
  - Lectura: Book cap. 15 (smart pointers); *Rust Atomics and Locks* cap. 1.
  - Entregable: `world-server` aceptando conexiones reales.
- [ ] **2.4 Interés y deltas**
  - Funcional: cada cliente recibe solo lo que tiene cerca, con el último input confirmado.
  - No funcional: ancho de banda por cliente medido.
  - Rust: estructuras que evitan clones, `Bytes`, reutilizar buffers. Un caso de `unsafe` acotado (por ejemplo, un buffer de capacidad fija con `MaybeUninit`), verificado con Miri y comparado en un benchmark con la versión segura; se conserva solo si gana.
  - Lectura: Book cap. 20 (la parte de `unsafe`); *Rust for Rustaceans* cap. 9.
  - Entregable: deltas por vecindad, medición de bytes por segundo y un ADR sobre el caso de `unsafe`.
- [ ] **2.5 Bots y prueba de carga**
  - Funcional: crate `bots` con N clientes simulados que caminan al azar.
  - No funcional: capacidad medida (p50 y p99 del tick, mensajes por segundo, CPU), publicada en el README. La lógica del cliente (estado de la conexión, aplicar deltas, cola de inputs) vive en un crate `client-core` sin red, sin Bevy y sin Leptos: recibe mensajes y devuelve mensajes. Los bots son su primer usuario; después lo usan el cliente web y cualquier app de escritorio.
  - Rust: diseño "sans-IO", `tokio-tungstenite`, `JoinSet`, cancelación, spans de `tracing`.
  - Lectura: *Rust for Rustaceans* cap. 8 (async).
  - Entregable: crate `client-core` y reporte de carga reproducible.
- [ ] **2.6 Operabilidad**
  - No funcional: configuración por entorno, logs estructurados, métricas Prometheus, apagado ordenado.
  - Rust: `tracing-subscriber`, crate `metrics`, señales y `CancellationToken`.
  - Lectura: *Zero To Production in Rust*, capítulo de telemetría.
  - Entregable: `world-server` listo para operar.
- [ ] **2.7 Primer despliegue**
  - Funcional: `world-server` accesible en internet.
  - No funcional: imagen Docker pequeña (cargo-chef, musl o distroless), TLS, latencia real medida con bots desde tu máquina.
  - Entregable: VM con Docker y proxy TLS; latencias reales en la bitácora.
- [ ] **Hito 2:** 500 bots en un espacio con números publicados. Post: "Un actor por espacio con Tokio".
- [ ] **Primer PR externo:** elegir un issue en un crate que ya uses (Tokio, Axum, postcard, proptest) y abrir el PR. Leer código ajeno también es parte del nivel senior.

Las lecturas de las Fases 3 a 7 se asignan al abrir cada fase.

## Fase 3 · Cliente WASM (≈5–6 semanas)

- [ ] **3.1 Leptos en el navegador**
  - Funcional: una página que se conecta al `world-server` y muestra quién está en el espacio.
  - No funcional: build reproducible con Trunk.
  - Rust: componentes y señales, wasm-bindgen, `web-sys` y `gloo`, closures que cruzan a JS y por qué piden `'static`.
  - Entregable: cliente Leptos mínimo conectado.
- [ ] **3.2 Bevy nativo**
  - Funcional: el mapa de Tiled dibujado y un avatar que camina usando `world-core`.
  - No funcional: separar simulación (core) de presentación (render).
  - Rust: ECS (componentes, sistemas, recursos, queries), plugins.
  - Entregable: `client-world` corriendo con `cargo run`.
- [ ] **3.3 Bevy en el navegador y el bridge**
  - Funcional: el mundo dentro de la página de Leptos.
  - No funcional: límite de tamaño del `.wasm` en CI.
  - Rust: crate `client-bridge` con canales, `Resource`, features de Bevy.
  - Entregable: un solo módulo WASM con UI y mundo.
- [ ] **3.4 Predicción, reconciliación e interpolación**
  - Funcional: moverse se siente instantáneo y los demás se mueven suave.
  - No funcional: latencia percibida; pruebas con latencia artificial. La predicción y la reconciliación van en `client-core` y se prueban en nativo, sin navegador; Bevy solo interpola y dibuja.
  - Rust: buffers circulares, diseño de estado del cliente.
  - Entregable: caminar con otros en el navegador contra el servidor desplegado.
- [ ] **Hito 3:** demo pública caminando con otras personas. Post: "Predicción de cliente con Rust y WASM".

## Fase 4 · API y datos (≈4–5 semanas)

Aquí tu experiencia Java pesa más. El foco es hacerlo idiomático en Rust y que tú diseñes.

- [ ] **4.1 Estructura de la API**
  - Funcional: endpoints de salud y de espacios.
  - No funcional: errores consistentes (problem+json), timeouts, request-id, CORS.
  - Rust: `Router`, `State`, extractores, `IntoResponse`, capas `tower`. Equivalente Java: controllers, `@ControllerAdvice` y filtros.
- [ ] **4.2 Postgres con sqlx**
  - Funcional: usuarios y espacios persistidos. `world-server` guarda un snapshot de cada espacio cada 5 segundos y lo carga al arrancar.
  - No funcional: queries verificadas al compilar (modo offline en CI), migraciones, transacciones.
  - Rust: traits de repositorio, genéricos frente a `dyn Trait`, tests de integración con testcontainers.
- [ ] **4.3 Login con OIDC**
  - Funcional: login real con Keycloak y permisos por espacio.
  - No funcional: validación de JWT con JWKS, rotación de llaves, tolerancia de reloj.
  - Rust: extractor `AuthUser`, caché de llaves con `Arc` y `RwLock`.
- [ ] **4.4 Espacios, mapas y directorio**
  - Funcional: crear espacios, subir mapas, invitar; la API devuelve a qué nodo conectarse. Hoy hay un solo nodo; la interfaz queda lista para varios.
  - No funcional: subida a S3 (MinIO local) con URL firmada, validación con el loader de `world-core`. Los leases en Redis llegan con el segundo nodo (7.7).
  - Rust: reutilizar crates del workspace entre binarios.
- [ ] **4.5 Contrato de la API**
  - No funcional: OpenAPI generado con `utoipa`, versionado de la API.
- [ ] **Hito 4:** crear un espacio, subir un mapa, invitar y entrar con login real. Post: "De Spring a Axum: la misma API, en Rust".

## Fase 5 · Audio y video con LiveKit (≈3–4 semanas)

- [ ] **5.1 LiveKit local y tokens**
  - Funcional: entrar a una llamada desde el navegador.
  - No funcional: tokens de vida corta firmados por la `api`.
  - Rust: `livekit-api`, bloques `extern` de wasm-bindgen para el shim TS, Promises ↔ Futures.
- [ ] **5.2 Suscripciones por proximidad**
  - Funcional: ves y oyes solo a quien está cerca.
  - No funcional: histéresis y debounce, rate limit hacia LiveKit, webhooks con firma verificada, idempotencia.
  - Rust: integrar un cliente externo dentro del actor sin bloquear el tick.
- [ ] **5.3 Zonas privadas y audio espacial**
  - Funcional: salas privadas y volumen por distancia.
  - No funcional: prueba de seguridad con un cliente modificado; verificar `canSubscribe: false`.
- [ ] **Hito 5:** conversación por proximidad entre personas reales. Post: "Video por proximidad: el servidor decide quién ve a quién".

## Fase 6 · Producción (≈4 semanas)

- [ ] **6.1 Imágenes y entorno local completo** con docker-compose de todo el stack.
- [ ] **6.2 Despliegue del piloto:** una VM con docker compose (`api`, `world-server`, Postgres, LiveKit con sus puertos UDP y proxy TLS), backups de Postgres y despliegue automático desde CI. Kubernetes pasa a la 7.7: ya lo dominas y no enseña Rust.
- [ ] **6.3 Observabilidad:** OpenTelemetry de punta a punta, dashboards en Grafana, SLOs (p99 del tick, éxito al entrar).
- [ ] **6.4 Resiliencia:** reiniciar `world-server` y verificar que el espacio vuelve desde el snapshot; pruebas de caos (red lenta, Postgres caído).
- [ ] **6.5 Seguridad y cadena de suministro:** cargo-deny y cargo-audit, SBOM, rate limiting, manejo de secretos.
- [ ] **Hito 6:** en producción con dominio propio y 3–5 usuarios piloto. Post: "Un producto Rust en producción: los números reales".

## Fase 7 · Diferenciadores (orden según lo que pidan tus usuarios)

- [ ] **7.1 Plugins WASM en el servidor** con wasmtime y Component Model (WIT), en sandbox con límites de CPU y memoria.
- [ ] **7.2 Supresión de ruido** con nnnoiseless en un AudioWorklet.
- [ ] **7.3 WebTransport** con wtransport y fallback a WebSocket.
- [ ] **7.4 App de escritorio:** decidir entre Tauri (envuelve el cliente web) y GPUI (UI nativa en Rust sobre `client-core`), con un spike de 2 sesiones y un ADR.
- [ ] **7.5 Perfilado y rendimiento:** flamegraphs con samply y reducción de allocations.
- [ ] **7.6 Cobros** con Stripe.
- [ ] **7.7 Varios nodos:** Kubernetes con Helm, OpenTofu, leases "espacio → nodo" en Redis y recuperación cuando cae un nodo. Solo cuando un nodo no alcance o un cliente lo pida.

---

## Temas de Rust senior y dónde se aprenden

| Tema | Lecciones |
|---|---|
| Ownership, borrowing y moves | 1.1, 1.3 |
| Lifetimes | 1.2, 2.4 |
| Traits, genéricos y `dyn Trait` | 1.4, 4.2 |
| Manejo de errores | 1.2, 4.1 |
| Iteradores y closures | 1.4 |
| `Box`, `Arc`, `Mutex`, `RwLock`, interior mutability | 2.3, 4.3 |
| Concurrencia: `Send` y `Sync` | 2.2, 2.3 |
| Async: Tokio, cancelación, backpressure | 2.2–2.5 |
| `Future`, `Pin` y `Waker` por dentro | 2.2 |
| Macros (derives por dentro) | 2.1 |
| WASM e interop con JS | 1.5, 3.1, 3.3, 5.1, 7.1 |
| `unsafe` y Miri | 2.4 |
| Testing: unitario, property, integración, fuzzing | 1.3, 2.1, 2.5, 4.2 |
| Rendimiento: benchmarks y profiling | 1.4, 2.5, 7.5 |
| Diseño de crates y APIs (semver, features) | 1.5, 2.1, 4.5 |
| Operación: observabilidad y despliegue | 2.6, 2.7, 6.x |

## Evidencia para tu perfil

- README con números medidos y cómo reproducirlos.
- ADRs en `docs/adr/` y la bitácora de aprendizaje.
- Un post en LinkedIn por hito (6 en total). Su estado y su enlace se registran en `PROGRESO.md`.

### Cómo se cierra un hito

1. **Demostración.** Lo que promete el hito funciona y se puede reproducir desde el README.
2. **README.** Se actualiza con los números medidos y cómo reproducirlos. Es lo primero que abre quien llega desde el post.
3. **Post.** Lo escribes tú, con tu voz. Yo lo reviso como un code review: que cada afirmación técnica sea correcta y que cada número salga de una medición del repo, no de una estimación.
   - Estructura: el problema, la decisión y su alternativa descartada, un número medido, qué aprendiste y el enlace al README o al ADR.
   - Con una imagen o un GIF de lo que se ve (desde el Hito 1, el ejemplo de la 1.6 en la terminal).
4. **Registro.** Fecha y enlace del post en `PROGRESO.md`.
- Al menos un PR aceptado en un crate que uses (el primero se abre al cerrar el Hito 2).
- El producto en producción con usuarios reales.

## Lecturas

Cada lección de las Fases 1 y 2 indica su lectura. Los capítulos del Book siguen la numeración de la edición en línea (doc.rust-lang.org/book).

- *The Rust Programming Language* (el Book): la base de la Fase 1.
- *Rust for Rustaceans*, Jon Gjengset: el nivel senior.
- *Zero To Production in Rust*, Luca Palmieri: backend en producción.
- *Rust Atomics and Locks*, Mara Bos (gratis en línea): concurrencia.
- Tutorial oficial de Tokio (tokio.rs).
