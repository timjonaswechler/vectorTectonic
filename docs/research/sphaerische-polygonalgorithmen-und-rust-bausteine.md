# Research: Sphärische Polygonalgorithmen und Rust-Bausteine bewerten

**Issue:** GitHub #4
**Stand:** 2026-07-14
**Gelesener Projektkontext:** `CONTEXT.md`, `README.md`, `docs/research/platten-und-superkontinentzyklus.md`, `docs/research/tektonische-kernfeatures.md`

## Zusammenfassung

Für den Simulationskern ist die **S2-Algorithmusfamilie** die belastbarste Grundlage: geodätische Kanten auf der Kugel, robuste Prädikate mit Hochpräzisions-/Exact-Fallback, Boolesche Operationen, gemeinsames Snap-Rounding und explizite semi-offene Randsemantik sind direkt einschlägig. Eine globale sphärische Flächenaufteilung als orientierter Half-Edge-/Face-Graph ist für Issue #6 eine stark begründete Repräsentationskandidatin; dieses Research-Ticket legt das Zustandsmodell nicht vorweg, zeigt aber, warum unabhängig gespeicherte und nacheinander verschnittene Plattenpolygone gemeinsame Grenzen, Nachbarschaft sowie Lückenfreiheit über viele Änderungen nicht strukturell garantieren. [S2-Prädikate](https://github.com/google/s2geometry/blob/4b624b27714e462cceca71a1670816c7f0059fa9/src/s2/s2predicates.h), [S2BooleanOperation](https://github.com/google/s2geometry/blob/4b624b27714e462cceca71a1670816c7f0059fa9/src/s2/s2boolean_operation.h), [S2Builder](https://github.com/google/s2geometry/blob/4b624b27714e462cceca71a1670816c7f0059fa9/src/s2/s2builder.h)

Die junge Pure-Rust-Portierung **`s2rst` 0.4.0** deckt auf API- und Quellcodeebene bereits den passenden Funktionsumfang ab, bezeichnet sich im Repository am Release-Commit aber selbst als Beta und ausdrücklich als nicht produktionsreif. Sie ist daher ein guter Spike-/Vergleichskandidat, aber noch keine verantwortbare alleinige Produktionsabhängigkeit. Empfohlen wird ein begrenzter Bake-off `s2rst` gegen einen schmalen, opaken `cxx`-Wrapper um S2 C++; CGAL `Nef_S2` dient optional als exaktes GPL-/kommerzielles Testorakel, nicht als Standard-Runtime. [`s2rst` README am Tag v0.4.0](https://github.com/torgebo/s2rst/blob/9d3df96d358b6beb0f879601a15e9daa19f25d7d/README.md), [`s2rst` API 0.4.0](https://docs.rs/s2rst/0.4.0/s2rst/s2/index.html), [S2 C++](https://github.com/google/s2geometry/tree/4b624b27714e462cceca71a1670816c7f0059fa9), [CGAL Nef_S2](https://doc.cgal.org/latest/Nef_S2/index.html)

## Abgrenzung und Entscheidungsbegriffe

- **Kugelmodell:** Einheitskugel plus separater physischer Radius. Kanten sind kürzere Großkreis-/Geodätenbögen. Das ist nicht dasselbe wie ellipsoidische WGS84-Geodäten.
- **Geometrische Robustheit:** Prädikate liefern auch nahe degenerierter Lagen konsistente Vorzeichen; Konstruktionen müssen ihre unvermeidliche Rundung topologisch kontrollieren.
- **Topologische Robustheit:** Nach einer Operation sind Randentscheidungen konsistent; Ausgaben haben keine ungesplitteten Kreuzungen oder unbeabsichtigten T-Knoten und können als lückenlose Partition validiert werden.
- **Nachbarschaft:** erfordert eine nachweisbar gemeinsame Kante; „Abstand kleiner epsilon“ und bloße Punktberührung reichen nicht.
- **Teilung:** bedeutet algorithmisch, eine Linie in eine Kantenanordnung einzufügen, Kreuzungen zu splitten und die entstehenden Flächen zu bestimmen; `polygon.difference(buffer(line))` allein genügt nicht.

## Findings

### 1. Wiederholte Änderungen erfordern gemeinsame Kantenverarbeitung

CGAL `Nef_S2` belegt, dass sphärische Arrangements mit `svertex`, `shalfedge` und `sface` unter Komplement, Schnitt, Vereinigung, Differenz und topologischen Operationen abgeschlossen werden können. S2Builder kann Polygonnetze aus unsortierten Kanten aufbauen, Gegenkanten verlangen und mehrere Layer gemeinsam snap-rounden, sodass keine Kreuzungen oder T-Vertices zwischen gemeinsam verarbeiteten Grenzen verbleiben. [CGAL Nef_S2 Manual](https://doc.cgal.org/latest/Nef_S2/index.html), [S2Builder-Quellcode](https://github.com/google/s2geometry/blob/4b624b27714e462cceca71a1670816c7f0059fa9/src/s2/s2builder.h)

Daraus folgt für die **Algorithmenauswahl**: Betroffene Nachbarflächen müssen in einer gemeinsamen Overlay-/Builder-Operation verarbeitet werden; unabhängige Reparatur einzelner Polygone ist ungeeignet, weil fast gleiche, aber nicht identische Grenzen entstehen können. Ob das spätere Zustandsmodell dauerhaft einen Half-Edge-Graph oder eine andere Repräsentation speichert, bleibt ausdrücklich offen.

### 2. Robustheit benötigt gefilterte exakte Prädikate plus kontrolliertes Snap-Rounding

S2s Orientierungs- und Distanzprädikate berechnen zunächst konservative Fehlergrenzen und fallen bei Unsicherheit auf Mehrfachpräzision beziehungsweise exakte Arithmetik und symbolische Perturbation zurück. Dadurch bleiben Orientierungs-, Kreuzungs- und Ordnungsentscheidungen selbstkonsistent. [S2-Prädikate](https://github.com/google/s2geometry/blob/4b624b27714e462cceca71a1670816c7f0059fa9/src/s2/s2predicates.h)

Eine neu konstruierte Schnittkoordinate kann bei `f64` dennoch nicht allgemein exakt dargestellt werden. S2BooleanOperation wertet bei `Precision::EXACT` die Eingabe exakt aus und snap-roundet anschließend die konstruktive Ausgabe; der Standard bewahrt Eingabevertices, erhöht bei Kreuzungen den Radius jedoch mindestens um den garantierten Schnittpunktfehler. [S2BooleanOperation](https://github.com/google/s2geometry/blob/4b624b27714e462cceca71a1670816c7f0059fa9/src/s2/s2boolean_operation.h) S2Builder garantiert Schranken für Vertex-/Kantenbewegung, Mindestabstände, topologische Erhaltung bis auf Degenerierungen und Idempotenz, sofern keine erneute Vereinfachung verlangt wird. Vereinfachung selbst ist ausdrücklich nicht idempotent. [S2Builder](https://github.com/google/s2geometry/blob/4b624b27714e462cceca71a1670816c7f0059fa9/src/s2/s2builder.h)

**Anforderungen an den späteren Geometriepfad:**

1. Keine selbstgebauten `abs(det) < epsilon`-Prädikate für topologische Entscheidungen.
2. Eine dokumentierte `SnapPolicy` pro Welt/Run; alle von einer Operation betroffenen Layer gemeinsam snap-rounden.
3. Keine wiederholte Kettenvereinfachung im autoritativen Operationspfad; Generalisierung gehört in eine abgeleitete Darstellung.
4. Jede Snap-Bewegung gegen eine noch festzulegende Mindestfeaturebreite prüfen; kollabierte Details müssen erkannt und als Diagnose oder fachlich definierter Übergang ausgegeben werden.

S2 kann Punktloops und Gegenkantenpaare als positive oder negative Degenerierungen darstellen; `S2LaxPolygonShape` ist dafür vorgesehen. Das macht durch Snapping oder Vereinfachung entstandene Degenerierungen repräsentierbar, garantiert aber nicht, dass kleine Löcher erhalten bleiben. [S2 Degenerate Geometry](https://s2geometry.io/devguide/degenerate_geometry.html)

### 3. Randsemantik ist eine Modellentscheidung; für eine Partition ist `SEMI_OPEN` richtig

S2BooleanOperation definiert Polygon- und Polylinienränder explizit als offen, semi-offen oder geschlossen. Im semi-offenen Polygonmodell gehört bei einer vollständigen Kachelung jeder Punkt und jede gerichtete Kante genau zu einem Polygon; damit entsteht weder doppelte Eigentümerschaft noch eine Randlücke. [S2BooleanOperation](https://github.com/google/s2geometry/blob/4b624b27714e462cceca71a1670816c7f0059fa9/src/s2/s2boolean_operation.h), [S2 Degenerate Geometry](https://s2geometry.io/devguide/degenerate_geometry.html)

**Empfehlung für die spätere Query-Semantik:**

- Für eindeutige Punktzuordnung in einer vollständigen Plattenpartition: `PolygonModel::SEMI_OPEN`.
- Für die Frage „berühren sich die Abschlüsse?“ inklusive Rand: gezielt `CLOSED`.
- Für reine Innenraumtests ohne Rand: gezielt `OPEN`.

Die Semantik darf nicht implizit je API-Aufruf wechseln. Sie gehört in benannte Query-Funktionen und Tests. Unterschiedliche Dimensionen benötigen ebenfalls bewusste Semantik: Bei S2 gewinnt in der Vereinigung die höhere, im Schnitt die niedrigere Dimension; das Subtrahieren einer Linie von einem Polygon ändert das Polygon nicht. Eine Riftlinie teilt daher nicht allein als Mengendifferenz eine Fläche, sondern muss als Kante in die Anordnung eingefügt werden. [S2BooleanOperation](https://github.com/google/s2geometry/blob/4b624b27714e462cceca71a1670816c7f0059fa9/src/s2/s2boolean_operation.h)

### 4. Schnitt, Teilung, Vereinigung und Differenz brauchen einen gemeinsamen Overlay-Ablauf

**Von der Engine abzudeckender Ablauf:**

1. Betroffene Grenzkanten und neue Ereigniskurven gemeinsam als Eingabe sammeln.
2. Identische/antipodale Nachbarpunkte ablehnen; Bogenstücke nötigenfalls so unterteilen, dass kein Stück 180° lang ist. S2Polyline verbietet Kanten der Länge 0° und 180°. [S2 Basic Types](http://s2geometry.io/devguide/basic_types.html)
3. Echte Kreuzungen mit robusten Prädikaten finden und an gemeinsamen Vertices splitten.
4. Alle betroffenen Layer mit derselben SnapPolicy gemeinsam durch den Builder führen. S2Builder dokumentiert für gemeinsam begrenzte Polygone, dass gemeinsames Bauen Kreuzungen und T-Vertices vermeidet. [S2Builder](https://github.com/google/s2geometry/blob/4b624b27714e462cceca71a1670816c7f0059fa9/src/s2/s2builder.h)
5. Ausgaben validieren und erst dann an den Aufrufer zurückgeben; Zuordnung, Provenienz, Commit/Rollback und dauerhafte Zustandsform sind Aufgaben späterer Tickets.

S2BooleanOperation liefert Union, Intersection, Difference und Symmetric Difference für Regionen aus Punkten, Polylinien und Polygonen mit geodätischen Grenzen und kann Ausgabe über Builder-Layer in gewünschte Repräsentationen leiten. Eingabepolygon-Innenräume innerhalb eines Operanden müssen disjunkt sein; andernfalls ist vorher zu normalisieren. [S2BooleanOperation](https://github.com/google/s2geometry/blob/4b624b27714e462cceca71a1670816c7f0059fa9/src/s2/s2boolean_operation.h)

**Operationszuordnung:**

- **Schnitt:** robustes Bogen-Crossing plus materialisierter gemeinsamer Vertex; Flächenschnitt über Boolean Overlay.
- **Teilung:** Ereigniskurve in eine Anordnung einfügen und die entstehenden Flächen ausgeben.
- **Vereinigung/Differenz:** S2-Boolean-Overlay mit explizitem Randmodell.
- **Nachbarschaft:** gemeinsame Kante positiver Länge aus der gemeinsam gebauten Ausgabe ableiten, nicht über Abstandstoleranz.

### 5. Lückenlose Abdeckung verlangt stärkere Prüfungen als eine Flächensumme

**Engine-unabhängige Prüfkriterien:**

1. Jeder Vertex ist endlich und auf die Einheitskugel normalisierbar.
2. Keine Kante hat gleiche oder antipodale Endpunkte; keine ungesplitteten echten Kantenkreuzungen.
3. Alle Ringe schließen und sind nach den Regeln des gewählten Engines gültig.
4. Semi-offene Punktlokalisierung ordnet Stichproben einer vollständigen Kachelung genau einer Ausgabe zu.
5. Die Summe orientierter Flächen ist innerhalb einer aus den verwendeten Routinen abgeleiteten Fehlerschranke `4π` Steradiant; die Flächensumme ist nur Zusatzprüfung.
6. Jede gemeldete Kantennachbarschaft besitzt eine gemeinsame Kante positiver Länge und umgekehrt.
7. Kanonisierter Ergebnis-Hash ist bei identischer Eingabe, Engine-Version, SnapPolicy und Operationsreihenfolge identisch.

Repräsentationsspezifische Invarianten wie `twin(twin(e)) = e` oder `V - E + F = 2` werden erst festgelegt, wenn das Zustandsmodell gewählt ist. S2s semi-offene Semantik garantiert bei einer Kugelkachelung eindeutige Punktzuordnung; S2Builder kann Gegenkanten im Mesh verlangen. [S2 Degenerate Geometry](https://s2geometry.io/devguide/degenerate_geometry.html), [S2Builder](https://github.com/google/s2geometry/blob/4b624b27714e462cceca71a1670816c7f0059fa9/src/s2/s2builder.h)

### 6. Bewertung aktuell verfügbarer Rust-Bausteine

| Baustein | Nachgewiesene Fähigkeiten | Lücken/Risiken | Entscheidung |
|---|---|---|---|
| **`s2rst` 0.4.0** | Pure-Rust-Port von S2; dokumentiert `Point`, `Loop`, `Polygon`, `Polyline`, `ShapeIndex`, robuste Prädikate mit Exact-Fallback, Boolean Ops, Winding Ops, Buffer, Builder/Snapping, Validation und Cell-Index. Repository enthält Implementierungen, Beispiele, Benchmarks und Fuzz-Workflow. Apache-2.0. [`s2rst` API 0.4.0](https://docs.rs/s2rst/0.4.0/s2rst/s2/index.html), [Release-Commit](https://github.com/torgebo/s2rst/tree/9d3df96d358b6beb0f879601a15e9daa19f25d7d), [crates.io](https://crates.io/crates/s2rst/0.4.0) | Sehr jung (0.4.0, Juni 2026), geringe Nutzung; das Repository-README am Release-Commit sagt „beta“ und „should not be used in production“. Unabhängiger Port, keine Google-Affiliation. API-/Semantikparität ist durch eigene Differentialtests zu belegen. | **Spike bevorzugt**, aber Produktionsfreigabe erst nach dem unten definierten Gate. |
| **`s2` / `rust-s2` 0.1.0** | S2-Zellen, CellUnion, LatLng, Point, Region und Teile der Kantenprädikate. [Repository-Status am untersuchten Commit](https://github.com/yjh0502/rust-s2/tree/76486fba58355158046a3bdbf00408400826813f) | Eigenes README listet Loop, Polygon, Polyline und ShapeIndex als „pending“, Prädikate nur „in progress“; damit fehlen gerade die Kernoperationen dieses Issues. | **Ablehnen** für Polygonkern; allenfalls Cell-ID-Helfer, wofür `s2rst` bereits kohärenter ist. |
| **`spherical_geometry` 0.4.2** | Punkte, Großkreise/-bögen, Bogenkreuzung, Polygonkonstruktion, Point-in-Polygon und Schnittabfrage Polygon–Bogen. [README/Quellbaum](https://github.com/stellarguesser/spherical-geometry/tree/3dddc491061a75bcd70f9208b2591cee72b585b1), [crates.io](https://crates.io/crates/spherical_geometry/0.4.2) | README nennt aktive Entwicklung und keine API-Garantie; kein dokumentiertes Polygon-Overlay (Union/Difference), kein globaler Builder, kein exakter Prädikat-Fallback oder Partitionserhalt. Beispiele verwenden `f32`. | **Nicht als Topologiekern**; höchstens Lern-/Testhilfe. |
| **`geo` 0.33.1** | Reife Rust-Geometrietypen, planare Boolean Ops und einige ausdrücklich sphärische Distanz-/Längenalgorithmen. [`geo` Boolean-Ops](https://docs.rs/geo/0.33.1/geo/algorithm/bool_ops/index.html) | Die Crate-Dokumentation bezeichnet Geometrien und Boolean Ops als planar. Längen mit Haversine machen das Polygon-Overlay nicht sphärisch; Pol-/Antimeridianfälle und Großkreisbögen werden so nicht korrekt modelliert. | **Nur DTO/Interop/Anzeige**, nie für autoritative sphärische Union, Difference, Split oder Nachbarschaft. |
| **`geos` 11.1.1** | Rust-Bindings zur etablierten GEOS-C-API, umfangreiche planare Topologie. [`geos` docs](https://docs.rs/geos/11.1.1/geos/) | GEOS/Simple-Features arbeitet im kartesischen 2D-Koordinatenraum; Projektion in Teilkarten erzeugt Naht-, Pol- und Großflächenprobleme. | **Ablehnen** für Kern; optional für lokale projizierte Exportwerkzeuge. |
| **`geographiclib-rs` 0.2.7** | Direkte/inverse ellipsoidische Geodäten und Polygonumfang/-fläche; MIT. [`geographiclib-rs` docs](https://docs.rs/geographiclib-rs/0.2.7/geographiclib_rs/) | Dokumentiert nur eine Teilmenge von GeographicLib; keine Polygon-Union/-Difference, kein Arrangement oder Nachbarschaftsgraph. Das ellipsoidische Modell widerspricht zudem dem hier gesetzten Kugelmodell, wenn beides unkontrolliert gemischt wird. | **Nicht für Topologie**; nur falls später bewusst ellipsoidische Messwerte verlangt werden. |
| **`h3o` 0.10.0** | Pure Rust H3; Zellenüberdeckung, mehrere Containment-Modi und Auflösen von Zellen zu Polygonumrissen. [`h3o::geom`](https://docs.rs/h3o/0.10.0/h3o/geom/index.html) | Diskrete, auflösungsabhängige Approximation. `ContainsCentroid` kann Zellen eindeutig zuweisen, bildet aber nicht die exakte Vektorgrenze ab. Der Projektkontext verlangt vektorbasierte Polygone statt Raster-/Zellzustand. | **Nur Diagnose/Spatial-Binning**, nicht autoritativer Zustand. |
| **`subsphere` 0.7.1** | Implizite feste Kugeltessellationen mit Face/HalfEdge/Twin-Nachbarschaft. [`subsphere` docs](https://docs.rs/subsphere/0.7.1/subsphere/) | Topologie ist an generierte Tessellationsfamilien gebunden; keine frei veränderlichen Polygon-Overlays belegt. | **Konzeptreferenz/Visualisierung**, nicht dynamischer Plattenkern. |

### 7. Bewertung etablierter Engines über FFI

#### Google S2 C++ — Referenzbaseline und Fallback-Kandidat

**Fähigkeiten:** Apache-2.0; echte Kugelgeometrie; robuste Prädikate; Punkte, Linien und Polygone; Union, Schnitt, Differenz und XOR; offene/semi-offene/geschlossene Ränder; Degenerierungen; räumliche Indizes; gemeinsames Snap-Rounding und Mesh-Aufbau. [S2 Repository am untersuchten Commit](https://github.com/google/s2geometry/tree/4b624b27714e462cceca71a1670816c7f0059fa9), [S2BooleanOperation](https://github.com/google/s2geometry/blob/4b624b27714e462cceca71a1670816c7f0059fa9/src/s2/s2boolean_operation.h), [S2Builder](https://github.com/google/s2geometry/blob/4b624b27714e462cceca71a1670816c7f0059fa9/src/s2/s2builder.h)

**FFI-Empfehlung für den Bake-off:** Einen kleinen C++-Adapter über `cxx` 1.0.197 anbinden, nicht S2-Typen breit mit `bindgen` spiegeln. `cxx` prüft gemeinsam deklarierte Rust-/C++-Signaturen statisch und integriert C++-Builds über `cxx-build`; C++-Fehler werden im Adapter in `Result` übersetzt. [CXX-Dokumentation 1.0.197](https://docs.rs/cxx/1.0.197/cxx/), [Cargo-Integration](https://cxx.rs/build/cargo.html), [Fehlerabbildung](https://cxx.rs/binding/result.html)

Für den Vergleich genügt eine opake Oberfläche aus Punkt-/Ringdaten, Operation, BoundaryModel und SnapPolicy hinein sowie Geometrie und Diagnosecode hinaus. Die dauerhafte Modulschnittstelle wird hier nicht festgelegt.

**Risiken:** S2 veröffentlicht weiterhin 0.x und garantiert daher keine API/ABI-Stabilität; der Build zieht CMake- und Abseil-Abhängigkeiten nach sich. Das erhöht CI-, Packaging- und Cross-Compile-Aufwand. [S2 README am untersuchten Commit](https://github.com/google/s2geometry/blob/4b624b27714e462cceca71a1670816c7f0059fa9/README.md) Für einen Einsatz müssen Versionen gepinnt und die ABI vollständig gekapselt werden.

#### CGAL `Nef_S2` — exaktes Orakel, nicht Default-Runtime

CGAL `Nef_S2` ist unter Booleschen und topologischen Operationen abgeschlossen, bietet Punktlokalisierung und eine explizite sphärische Inzidenzstruktur. Die geodätischen Arrangement-Traits verlangen exakte Prädikate, meist exakte Konstruktionen, und führen die Rechnungen rational aus. [Nef_S2 Manual](https://doc.cgal.org/latest/Nef_S2/index.html), [Geodesic Arrangement Traits](https://doc.cgal.org/latest/Arrangement_on_surface_2/classCGAL_1_1Arr__geodesic__arc__on__sphere__traits__2.html)

**Lücken/Risiken:** C++-FFI und voraussichtlich höhere Kosten exakter rationaler Konstruktionen. Das Paket `Nef_S2` ist laut offizieller Referenz **GPL**; alternativ ist eine kommerzielle CGAL-Lizenz nötig. [CGAL Paketlizenz](https://doc.cgal.org/latest/Nef_S2/group__PkgNefS2Ref.html), [CGAL Lizenzmodell](https://www.cgal.org/license.html) Für das MIT-Projekt ist es deshalb ohne geklärte Lizenz kein Runtime-Default. Als separates Entwicklungs-/Differentialtest-Werkzeug ist es technisch wertvoll; rechtliche Prüfung bleibt erforderlich.

#### GeographicLib C++

Die C++-Engine berechnet robuste ellipsoidische Geodäten, Segmentkreuzungen sowie Umfang/Fläche geodätischer Polygone; `PolygonArea` ist jedoch kein Boolean-Overlay. [GeographicLib PolygonArea](https://geographiclib.sourceforge.io/C++/doc/classGeographicLib_1_1PolygonAreaT.html), [GeographicLib Intersect](https://geographiclib.sourceforge.io/C++/doc/classGeographicLib_1_1Intersect.html) Sie löst dieses Issue daher nicht als Topologie-Engine.

### 8. Abgegrenzter Bake-off der beiden S2-Implementierungen

Der Vergleich prüft ausschließlich die für dieses Ticket relevanten Operationen: `union`, `intersection`, `difference`, `symmetric_difference`, Point-in-Polygon mit drei Randmodellen, Kantenkreuzung/-split, gemeinsames Snapping, Validierung, Flächenmessung und Ableitung gemeinsamer Kanten. Persistenz, fachliche Eigentümerschaft, dauerhafte Meshrepräsentation und Modulgrenzen sind nicht Teil des Bake-offs.

Beide Kandidaten erhalten dieselben normalisierten 3D-Punkte, dieselbe Operationsreihenfolge und dieselbe SnapPolicy. Ausgaben werden durch Ringorientierung, Startvertex und stabile Ringsortierung kanonisiert. Zwei Ausgaben gelten geometrisch als äquivalent, wenn ihre symmetrische Differenz unter derselben semi-offenen Semantik leer ist und ihre gemeinsame-Kanten-Inzidenzen übereinstimmen; bloß ähnliche Vertexkoordinaten reichen nicht.

### 9. Produktions-Gate und Pflichttests

**Korpus:** handkonstruierte Fälle an Pol und Antimeridian, fast kollineare/fast berührende Bögen, identische und umgekehrte Kanten, T-Knoten, sehr kleine Faces/Löcher, Full-/Empty-Sphere, mehr als Hemisphäre große Polygone, Mehrfachschnitt an einem Vertex sowie lange deterministische Rift-/Subduktionssequenzen.

**Property-/Metamorphictests:**

- `A ∪ A = A`, `A ∩ A = A`, `A \ A = ∅`, `A ∪ complement(A) = full` unter derselben Randsemantik.
- Union und Schnitt kommutativ; Differenz nicht künstlich symmetrisieren.
- Split plus sofortige Vereinigung rekonstruiert die ursprüngliche Partition bis zur festgelegten Snap-Äquivalenz.
- Rotation aller Eingaben mit derselben Quaternion ändert Topologie und Flächen nicht.
- Vertauschen von Eingabereihenfolge oder stabilem Iterationslayout ändert kanonisierten Ergebnis-Hash nicht.
- Nach jedem von mindestens 10.000 zufällig, aber reproduzierbar erzeugten Events gelten alle Invarianten aus Finding 5.
- Differentialtest `s2rst` gegen S2 C++; bei kleinen rational darstellbaren Fällen zusätzlich CGAL-Orakel, sofern lizenz-/buildseitig isoliert möglich.
- Fuzzing von Parser, Builder, Boolean Ops und Serialisierung; kein Panic/UB/Timeout außerhalb definierter Ressourcenlimits.

**Freigabekriterium für `s2rst`:** Der Spike ist auf höchstens fünf Arbeitstage begrenzt und läuft mindestens auf macOS/ARM64 sowie Linux/x86_64 im Release-Profil. Pro Kandidat und Target läuft Fuzzing mindestens 30 Minuten. `s2rst` wird bevorzugt, wenn (a) im festen Korpus, in 100 Seeds mit je 10.000 Operationen und im Fuzzing keine topologische Abweichung nach der oben definierten Äquivalenz und kein Panic auftritt, (b) wiederholte Läufe pro Target denselben kanonischen Hash liefern und (c) p95-Laufzeit sowie Peak-RSS auf dem festen Korpus jeweils höchstens das Doppelte des S2-C++-Adapters betragen. Andernfalls bleibt S2 C++ hinter `cxx` der Fallback; absolute Produktbudgets werden erst mit realen Weltgrößen festgelegt.

## Fähigkeiten, Lücken und Risiken im Überblick

### Abgedeckt

- Sphärische Punkt-/Bogenprädikate, Kreuzungen und Point-in-Polygon: S2 C++, in `s2rst` dokumentiert portiert.
- Polygon-Union, -Schnitt, -Differenz, XOR: S2 C++ und `s2rst` API.
- Konsistentes Snap-Rounding mehrerer gemeinsam begrenzter Layer: S2Builder; in `s2rst` Builder-API vorhanden.
- Eindeutige Eigentümerschaft einer vollständigen Kachelung: semi-offene S2-Semantik.
- Gemeinsame Kantenverarbeitung und eindeutige Randzuordnung: durch S2Builder plus semi-offene Semantik; die lückenlose Abdeckung des dauerhaften Zustands muss die spätere Repräsentationsentscheidung zusätzlich garantieren.

### Nicht automatisch abgedeckt

- Fachliche Entscheidung, wem eine bei Bewegung entstehende Konflikt-Face gehört.
- Erhaltung stabiler Feature-/Platten-IDs nach einem Overlay; Provenienzregeln sind projektspezifisch.
- Garantierte Bitidentität zwischen verschiedenen CPU-/Compiler-/Engine-Versionen. Determinismus ist nur mit gepinntem Backend, kanonischer Sortierung und Plattformtests belastbar.
- Beliebige nicht-geodätische Kurven. Klein-/Kleinkreise oder Euler-Trajektorien müssen als eigener Kurventyp implementiert oder mit dokumentierter Fehlergrenze in Großkreisbögen tesselliert werden.
- Unbegrenzte Detailerhaltung: Features unterhalb Snapradius können degenerieren.

### Risiken nach Schwere

- **Hoch:** `s2rst` ist laut eigener Primärquelle Beta/nicht produktionsreif; eine vorschnelle alleinige Abhängigkeit gefährdet die Kerninvarianten.
- **Hoch:** Unabhängige Polygonreparatur je Platte kann trotz robuster Einzel-Booleans Lücken, Überlappungen und auseinanderlaufende gemeinsame Grenzen erzeugen.
- **Hoch:** Randmodell, Snapradius oder Konfliktpriorität implizit zu lassen macht Ergebnisse reihenfolge- und toleranzabhängig.
- **Mittel:** S2-C++-FFI bringt Build-/ABI-Komplexität und gepinnte Abhängigkeiten.
- **Mittel:** CGAL `Nef_S2` ist GPL/kommerziell; unbeabsichtigtes Linken wäre mit der gewünschten MIT-Distribution abzuklären.
- **Mittel:** Wiederholte Vereinfachung ist nicht idempotent und kann Drift erzeugen; deshalb nicht im autoritativen Operationspfad.
- **Mittel:** Großkreisbögen erlauben keine eindeutige 180°-Kante; Eingaben brauchen deterministische Unterteilung.
- **Niedrig:** H3O/`subsphere` können als Diagnosehilfen mit dem Vektorzustand verwechselt werden; klare Modulgrenzen verhindern dies.

## Entscheidungsreife Empfehlung

1. **Algorithmusfamilie festlegen:** robuste S2-Prädikate, geodätische Bögen, semi-offene Eigentumssemantik, gemeinsames Snap-Rounding und atomare Overlay-Transaktionen; die konkrete Zustandsrepräsentation bleibt Issue #6 vorbehalten.
2. **Nicht festlegen:** `s2rst` als alleinige Produktionsengine. Stattdessen den begrenzten Bake-off mit derselben Testfassade und demselben adversarial Korpus durchführen.
3. **Referenzbaseline:** S2 C++ hinter einem schmalen `cxx`-Adapter, sofern der Bake-off `s2rst` nicht nachweislich gleichwertig freigibt.
4. **Rust-Spike:** `s2rst` wegen passendem Funktionsumfang und Apache-2.0 priorisieren; `s2`, `spherical_geometry`, `geo` und GEOS nicht als Topologiekern weiterverfolgen.
5. **Validierungsorakel:** CGAL `Nef_S2` nur isoliert für kleine exakte Differentialfälle und erst nach Lizenzprüfung; nicht in die MIT-Runtime linken.
6. **Abnahmekriterium:** Nicht „Boolean Call liefert Erfolg“, sondern alle zur gewählten Repräsentation gehörenden Eigentums-, Flächen-, Nachbarschafts- und Determinismusinvarianten nach langen wiederholten Eventfolgen.

## Sources

### Kept

- [Google S2: `s2predicates.h`](https://github.com/google/s2geometry/blob/4b624b27714e462cceca71a1670816c7f0059fa9/src/s2/s2predicates.h) — Primärquellcode zu Fehlergrenzen, Exact-Fallback und symbolischer Perturbation.
- [Google S2: `s2boolean_operation.h`](https://github.com/google/s2geometry/blob/4b624b27714e462cceca71a1670816c7f0059fa9/src/s2/s2boolean_operation.h) — Primärquellcode zu Operationen, Eingabeanforderungen, Randmodellen, Präzision und Degenerierungen.
- [Google S2: `s2builder.h`](https://github.com/google/s2geometry/blob/4b624b27714e462cceca71a1670816c7f0059fa9/src/s2/s2builder.h) — Primärquellcode zu sphärischem Snap-Rounding, Topologiegarantien, gemeinsamen Layern und Mesh-Gegenkanten.
- [S2 Degenerate Geometry](https://s2geometry.io/devguide/degenerate_geometry.html) — offizielle Semantik für Degenerierungen und lücken-/überlappungsfreie semi-offene Kachelung.
- [S2 Basic Types](http://s2geometry.io/devguide/basic_types.html) — offizielle Repräsentation und Einschränkungen sphärischer Kanten/Polygone.
- [`s2rst` Release-Commit](https://github.com/torgebo/s2rst/tree/9d3df96d358b6beb0f879601a15e9daa19f25d7d) und [Rustdoc 0.4.0](https://docs.rs/s2rst/0.4.0/s2rst/s2/index.html) — Primärquellen zu Umfang, Implementierung und explizitem Beta-Status.
- [`rust-s2` 0.1.0 am untersuchten Commit](https://github.com/yjh0502/rust-s2/tree/76486fba58355158046a3bdbf00408400826813f) — Primärquelle zum ausdrücklich unvollständigen Polygonstatus.
- [CGAL Nef_S2](https://doc.cgal.org/latest/Nef_S2/index.html), [Arrangement Traits](https://doc.cgal.org/latest/Arrangement_on_surface_2/classCGAL_1_1Arr__geodesic__arc__on__sphere__traits__2.html), [Paketlizenz](https://doc.cgal.org/latest/Nef_S2/group__PkgNefS2Ref.html) — offizielle Fähigkeiten, Arithmetik und Lizenz.
- [`geo` 0.33.1 Boolean-Ops](https://docs.rs/geo/0.33.1/geo/algorithm/bool_ops/index.html) und [`geos` 11.1.1](https://docs.rs/geos/11.1.1/geos/) — offizielle Abgrenzung der planaren Algorithmen.
- [`geographiclib-rs` 0.2.7](https://docs.rs/geographiclib-rs/0.2.7/geographiclib_rs/) und [GeographicLib PolygonArea](https://geographiclib.sourceforge.io/C++/doc/classGeographicLib_1_1PolygonAreaT.html) — offizieller Funktionsumfang, kein behauptetes Overlay.
- [`h3o::geom` 0.10.0](https://docs.rs/h3o/0.10.0/h3o/geom/index.html), [`subsphere` 0.7.1](https://docs.rs/subsphere/0.7.1/subsphere/) — offizielle diskrete/Tessellationsfähigkeiten und deren Scope.
- [`cxx` 1.0.197](https://docs.rs/cxx/1.0.197/cxx/) — offizielle Rust/C++-Interop-Eigenschaften.

### Dropped

- SEO-Vergleichsseiten, Blogposts und Forenantworten — keine Primärbelege.
- Planare Clipping-Crates wie `i_overlay`, `geo-booleanop2`, `polygon_clipping` und Clipper2 — ihr dokumentierter 2D-Polygonoverlay beantwortet die direkte Kugeloperation nicht.
- Behauptungen aus Such-Snippets, `geo`-Boolean-Ops seien sphärisch — widersprechen der offiziellen `geo`-Dokumentation („planar“).
- H3/H3O als exakte Partition — diskrete Zellüberdeckung ist eine Approximation und verletzt den kanonischen Vektor-Scope.

## Gaps

- Es wurde kein projektspezifischer Prototyp oder Benchmark ausgeführt; die Leistungs- und Paritätsentscheidung zwischen `s2rst` und S2 C++ bleibt deshalb bewusst offen. Als Plausibilitätscheck lief `cargo test -p s2rst --release` am unveränderten Tag `v0.4.0` lokal mit 3.472 bestandenen Tests und ohne Fehlschlag (15 ignoriert); das ersetzt weder den projektspezifischen adversarial Korpus noch die Produktionsfreigabe.
- `s2rst` ist extrem jung. Nur Differential-, Property- und Langlauftests können zeigen, ob der dokumentierte API-Umfang für die konkreten Degenerierungs- und Meshfälle vollständig korrekt portiert ist.
- Die fachliche Konfliktregel für Überlappung/Lücke nach Eulerbewegung ist nicht Gegenstand einer Geometriebibliothek und muss in der späteren Simulationsspezifikation entschieden werden.
- Eine rechtliche Bewertung von GPL-/kommerzieller CGAL-Nutzung ist keine Rechtsberatung und muss vor Integration erfolgen.
- Falls Grenzen künftig Klein- statt Großkreisbögen sein müssen, ist eine eigene Kurven-/Tessellierungsentscheidung mit globaler Fehlergrenze nötig; die hier bewerteten S2-Boolean-Kanten sind geodätisch.
