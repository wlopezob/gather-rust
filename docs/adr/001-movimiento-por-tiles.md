# ADR-001: Movimiento por tiles enteros

- Estado: Aceptado
- Fecha: 2026-11-10

## Contexto

¿Qué problema hay? Pistas: el servidor decide la posición real y el navegador
la predice para no esperar a la red. ¿Qué pasa si los dos calculan distinto?
¿Cómo detecta el servidor a un jugador que hace trampa?

## Decisión

¿Qué elegimos? Con los tipos que escribiste: TilePos con u16, 4 direcciones,
un paso de una casilla, aritmética con checked_*.

## Alternativas consideradas

1. Posiciones continuas con f32 y velocidad. ¿Por qué se descarta?
2. Punto fijo: enteros que representan fracciones de casilla (por ejemplo, 1/16).
   ¿Qué gana y qué complica?

## Consecuencias

Positivas: ¿qué se vuelve fácil? (validar en el servidor, colisiones, tamaño del mensaje)
Negativas: ¿qué perdemos? (diagonales, movimiento libre, se ve "a saltos")
¿Cómo mitigamos lo negativo? (el cliente interpola en la Fase 3)

Pide una "pista" si no sabes qué escribir en alguna sección.
