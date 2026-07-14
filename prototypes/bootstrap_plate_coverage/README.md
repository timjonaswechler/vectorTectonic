# PROTOTYP — Bootstrap zur vollständigen Plattenabdeckung

**Frage:** Fühlt sich der minimale Zustandsablauf vom Seed mit einem Superkontinent, zehn unteilbaren Kratonen und 75 % nicht aufgelöster alter Ozeankruste über Rift, atomaren Break-up, Rücken und konsumierenden Graben bis zu 100 % expliziter Plattenabdeckung stimmig an?

Dieser wegwerfbare Logik-Prototyp prüft nur die Reihenfolge und die sichtbaren Zustandsinvarianten. Er implementiert **keine** sphärische Geometrie, Kinematik, Kandidatenbewertung, Flussintegration oder Produktionsarchitektur. Die 5-%-Flächenschritte sind absichtlich normalisierte Ablaufmarker und keine wissenschaftliche Rate. Weil das Planungsrepository noch keinen Projektruntime oder Task-Runner besitzt, verwendet der Prototyp ausschließlich Python 3 aus der Standardbibliothek.

## Start

```bash
python3 prototypes/bootstrap_plate_coverage/run.py
```

Am günstigsten lässt sich der vorgesehene Pfad mit wiederholtem `n` durchspielen. Mit `x` wird sichtbar, dass ein Kratonschnitt abgelehnt wird. Der Bildschirm zeigt nach jeder Aktion den vollständigen relevanten Zustand und alle fünf geprüften Invarianten.

## Erwarteter Ablauf zur Diskussion

1. Am konvergenten Seed-Rand qualifiziert alter Ozean als unbewegte Unterseite für Subduktion.
2. Ein Rift folgt einer nichtkratonischen Schwäche und lässt alle zehn Kratone ganz.
3. Nach 35 aktiven Myr endet die Seed-Platte atomar; zwei Nachfolgerplatten und ein aktiver Rücken entstehen, aber noch keine Ozeanfläche im Break-up-Moment.
4. Erst mit aktivem Rücken **und** aktivem konsumierendem Graben wird Hintergrundfläche schrittweise durch aufgelöste ozeanische Kruste auf expliziten Platten ersetzt.
5. Bei Hintergrundfläche null wechselt die Phase einmalig zur Reife; der Prototyp stoppt dort bewusst.

## Wegwerfhinweis

Nach gemeinsamer Beantwortung der Frage wird nur die bestätigte Entscheidung in der Spezifikation bewahrt; dieser TUI-Prototyp soll anschließend gelöscht oder ersetzt werden.
