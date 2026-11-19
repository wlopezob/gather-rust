# Progreso

Actualizado el 18 de noviembre de 2026. El plan está en [`PLAN.md`](PLAN.md); este archivo dice por dónde vamos.

## Ahora

- **1.3 cerrada el 18 de noviembre.** Falta abrir y mergear su PR, con la revisión del profesor ya en la bitácora.
- **Siguiente:** 1.4 Proximidad (grilla espacial, histéresis 5/7, `criterion`). Lectura previa: Book cap. 13 (closures e iteradores) y cap. 10 (genéricos y traits).
- **La 1.4 abre con** las dos preguntas de repaso de los temas 1 y 2.

## Lecciones

| Lección | Cierre | Horas anotadas | PR |
|---|---|---|---|
| 0.1 Diagnóstico y entorno | 8 nov 2026 | 1 | – |
| 0.2 Calidad desde el día 1 | 8 nov 2026 | 1 | #1, #2 |
| 1.1 Tipos del dominio | 10 nov 2026 | 3 | #3 |
| 1.2 Cargar mapas de Tiled | 14 nov 2026 | 4 | #4 |
| 1.3 Movimiento y colisiones | 18 nov 2026 | 2 (3 sesiones: 30, 40 y 50 min) | – |

Total anotado: 11 horas en 17 días. Desde la 1.3, una línea por sesión (fecha y minutos), porque el recálculo del Hito 1 depende de ese dato.

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

De aquí salen las 2 preguntas con las que abre cada lección, en el orden de la lista. Un tema sale de la lista cuando lo respondes bien.

1. **Desbordamiento al multiplicar medidas y `usize::from`.** Cuatro veces: defensa de la 1.1, código de la 1.2, R1 de la 1.2 (faltó `release`) y R1 de la 1.3 (invirtió `debug` y `release`, y no planteó la cuenta del mapa de 257×257).
2. **Límites antes de procesar input no confiable.** D2 de la 1.2, sin responder. R2 de la 1.3, parcial: faltó que serde construye todas las capas antes de revisar `width`, y propuso validar en el front, que no es un control de seguridad.
3. **Invariantes y deserialización: por qué un `derive` puede saltarse `new`.** Defensa D3 de la 1.2: faltó la razón principal.
4. **Préstamo de un campo frente a préstamo de todo `self` a través de un método.** E0502 de la 1.3: costó entenderlo; la nota final aún dice "otro objeto de esa clase" en lugar de "otro campo del mismo objeto".

Ya superados: el `?` con `Result` y `From` (D1 de la 1.2); `clone` frente a `&` (D1 y E0382 de la 1.3); qué une `'a` entre la entrada y la salida (D2 de la 1.3).

## Pendientes fuera de las lecciones

- Corregir los espacios al final de la línea 42 de `ci.yml`, en la 1.5.
- Revisar a mano las versiones de las GitHub Actions en cada punto de control de fase (reemplaza a Dependabot).
