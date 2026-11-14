# Progreso

Actualizado el 14 de noviembre de 2026. El plan está en [`PLAN.md`](PLAN.md); este archivo dice por dónde vamos.

## Ahora

- **Lección en curso: 1.2 Cargar mapas de Tiled.** El código quedó aprobado el 13 de noviembre: 22 tests, clippy y deny limpios.
- **Falta para cerrarla:** tus notas en `docs/bitacora/1.2-mapas-tiled.md`, el PR con el CI en verde y las preguntas de defensa.
- **Después:** crear el `justfile` con la receta `check` y empezar la 1.3.

## Lecciones

| Lección | Cierre | Horas anotadas | PR |
|---|---|---|---|
| 0.1 Diagnóstico y entorno | 8 nov 2026 | 1 | – |
| 0.2 Calidad desde el día 1 | 8 nov 2026 | 1 | #1, #2 |
| 1.1 Tipos del dominio | 10 nov 2026 | 3 | #3 |
| 1.2 Cargar mapas de Tiled | En curso | – | – |

Las 5 horas anotadas parecen menos que las reales. Desde la 1.2 se anota una línea por sesión (fecha y minutos), porque el recálculo del Hito 1 depende de ese dato.

## Puntos de control

| Al cerrar | Fecha del plan | Fecha real |
|---|---|---|
| Fase 0 | 9 nov 2026 | 8 nov 2026 |
| Fase 1 | 14 dic 2026 | – |
| Fase 2 | 1 feb 2027 | – |
| Fase 3 | 15 mar 2027 | – |
| Fase 4 | 19 abr 2027 | – |
| Fase 5 | 17 may 2027 | – |
| Fase 6 | 14 jun 2027 | – |

A este ritmo, la Fase 1 cierra a inicios de diciembre, antes de la fecha del plan.

## Publicaciones en LinkedIn

Una por hito, como máximo una semana después de cerrarlo. El repo es público, así que cada post enlaza al README o al ADR que respalda lo que cuenta.

| Hito | Tema | Estado | Fecha | Enlace |
|---|---|---|---|---|
| 1 | El mismo código de simulación en servidor y navegador | Pendiente | – | – |
| 2 | Un actor por espacio con Tokio | Pendiente | – | – |
| 3 | Predicción de cliente con Rust y WASM | Pendiente | – | – |
| 4 | De Spring a Axum: la misma API, en Rust | Pendiente | – | – |
| 5 | Video por proximidad: el servidor decide quién ve a quién | Pendiente | – | – |
| 6 | Un producto Rust en producción: los números reales | Pendiente | – | – |

Estados: Pendiente → Borrador (en revisión) → Publicado.

## Temas pendientes de repaso

De aquí salen las 2 preguntas con las que abre cada lección. Un tema sale de la lista cuando lo respondes bien.

- Préstamo (`&self`) frente a copia, `clone` y move.
- Desbordamiento y conversiones con `usize::from`. Falló en la defensa de la 1.1 y volvió a aparecer en el código de la 1.2.
- El `?` con `Option` y con `Result`.

## Pendientes fuera de las lecciones

- Corregir los espacios al final de la línea 42 de `ci.yml`, en la 1.5.
- Revisar a mano las versiones de las GitHub Actions en cada punto de control de fase (reemplaza a Dependabot).
