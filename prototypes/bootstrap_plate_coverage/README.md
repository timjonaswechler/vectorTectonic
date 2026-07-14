# PROTOTYP — visueller Bootstrap zur vollständigen Plattenabdeckung

**Frage:** Wirkt der räumliche Ablauf vom driftenden Superkontinent mit zehn unteilbaren Kratonen über Rift und atomaren Break-up bis zur vollständigen expliziten Plattenabdeckung nachvollziehbar?

Dieser wegwerfbare Rust-/Bevy-Prototyp zeigt bewusst ein **deterministisch geskriptetes Anschauungsszenario**. Er prüft Formen, Drift, sichtbare Phasenübergänge und Flächenbilanz – nicht die spätere sphärische Geometrie-Engine, Ereignisauswahl oder physikalische Kalibrierung. Die Polygonformen und Geschwindigkeiten sind illustrative Platzhalter.

## Start

Rust 1.89 oder neuer wird benötigt.

```bash
cd prototypes/bootstrap_plate_coverage && cargo run --release
```

Der erste Build lädt und kompiliert Bevy. Danach startet der Ablauf automatisch.

## Bedienung

- `Leertaste`: Pause/Fortsetzen
- `←` / `→`: in 5-Myr-Schritten prüfen
- `R`: Neustart
- linke Maustaste ziehen: Globus drehen

## Sichtbarer Ablauf

1. **0–5 Myr:** Ein Superkontinent mit zehn hell markierten, unteilbaren Kratonen driftet gegen den unbewegten alten Ozeanhintergrund; ein Graben ist rot markiert.
2. **5–17,5 Myr:** Ein gelbes Rift folgt nichtkratonischer Kruste.
3. **17,5–35 Myr:** Das Rift lokalisiert sich, aber beide Kontinenthälften bewegen sich weiterhin als eine starre Seed-Platte.
4. **Bei 35 Myr:** Atomarer Break-up; erst jetzt entstehen zwei Platten und ein cyanfarbener Rücken. Im Break-up-Moment entsteht noch keine Ozeanfläche.
5. **35–110 Myr:** Die Platten driften auseinander. Polygonale Ozeanflächen wechseln vom alten grauen Hintergrund zu bekannten blauen Plattengebieten.
6. **Bei 110 Myr:** Hintergrundfläche ist null; die Initialisierung endet. Die Reifephase wird bewusst nicht prototypisiert.

## Nicht enthalten

Keine frei veränderbaren Parameter, keine Produktionsdatenstrukturen, keine robuste sphärische Überlagerung und keine vollständige Ereignislogik. Solche Interaktion gehört nur dann in einen weiteren Prototyp, wenn nach dieser Sichtprüfung noch eine konkrete Designfrage offen bleibt; andernfalls in die spätere MVP-Implementierung.

## Wegwerfhinweis

Nach gemeinsamer Beantwortung der Frage wird nur die bestätigte Entscheidung in der Spezifikation bewahrt. Dieser Prototyp soll anschließend gelöscht oder ersetzt werden.
