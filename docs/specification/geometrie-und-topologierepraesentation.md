# Geometrie- und Topologierepräsentation der Kugeloberfläche

Status: gemeinsam bestätigt für das MVP

Wayfinder-Ticket: [#6](https://github.com/timjonaswechler/vectorTectonic/issues/6)

Grundlagen: [fachliches Zustandsmodell](fachliches-zustandsmodell.md) und [Recherche zu sphärischen Polygonalgorithmen](../research/sphaerische-polygonalgorithmen-und-rust-bausteine.md)

## Zweck und Abgrenzung

Dieses Dokument legt die autoritative geometrische und topologische Repräsentation der Kugeloberfläche, ihre Invarianten, Rand- und Snapsemantik sowie die Schnittstelle für atomare Änderungen fest. Es konkretisiert, wie die vollständige Krusten- und Plattengebietsbelegung strukturell ohne unbeabsichtigte Lücken oder Überlappungen erhalten bleibt.

Nicht festgelegt werden Plattenbewegungsformeln (#7), Feature-Lebenszyklen (#8), Ereignisprioritäten und fachliche Konfliktregeln (#9), die dauerhafte Persistenzform (#11) oder konkrete Standardwerte und Qualitätsbudgets des Validierungsstandards (#13).

## 1. Kugel- und Kurvenmodell

Die autoritative Geometrie liegt auf einer Einheitskugel. Der physische Planetenradius ist ein separater positiver Laufparameter und verändert die gespeicherte Geometrie nicht.

Ein Vertex ist ein endlicher, normalisierter `f64`-Vektor in drei Dimensionen. NaN, Unendlichkeit und nicht normalisierbare Vektoren sind ungültig. Geographische Breite/Länge oder Bevy-Koordinaten sind ausschließlich Ein- und Ausgabeformate und werden am Rand des Moduls übersetzt.

Eine Kante ist ein gerichteter kürzerer Großkreis- beziehungsweise Geodätenbogen zwischen zwei Vertices. Identische und antipodale Endpunkte sind verboten, weil sie keinen eindeutigen solchen Bogen bestimmen. Erforderliche Kurven von mindestens 180 Grad werden deterministisch durch Zwischenvertices unterteilt. Andere Kurventypen, insbesondere Klein- oder Eulerbahnen, gehen nur als kontrolliert tessellierte Folge dieser Bögen in die autoritative Geometrie ein.

Längen werden autoritativ im Winkelmaß, Flächen in Steradiant berechnet. Für den Radius `R` gilt abgeleitet:

- physische Bogenlänge = Winkel · `R`,
- physische Fläche = Steradiant · `R²`.

Messwerte werden aus der kanonischen Geometrie abgeleitet und nicht redundant als autoritative Attribute gespeichert.

## 2. Eine gemeinsame sphärische Flächenanordnung

Die autoritative Oberflächenbelegung ist eine einzige gemeinsame sphärische Half-Edge-/Face-Anordnung. Sie ist die gemeinsame Verfeinerung der Krusten- und Plattengebietsbelegung; zwei unabhängig gepflegte Polygonbestände sind ausdrücklich ausgeschlossen.

### 2.1 Topologieelemente

- Ein **Vertex** hält eine kanonische Position auf der Einheitskugel und eine zyklische Inzidenz zu ausgehenden Half-Edges.
- Eine **Half-Edge** besitzt Ursprung, Gegenkante (`twin`), Nachfolger im Randzyklus (`next`) und inzidente Face. Ihre Geometrie ist der gerichtete kürzere Großkreisbogen zum Ursprung ihres Nachfolgers.
- Eine **Face** ist ein zusammenhängender offener zweidimensionaler Bereich mit einem oder mehreren Randzyklen. Mehrere Zyklen erlauben Löcher ohne künstliche Verbindungskanten.

Die Anordnung bildet eine geschlossene orientierte 2-Mannigfaltigkeit:

1. Jede ungerichtete Kante besteht aus genau zwei entgegengesetzten Half-Edges.
2. Jede Half-Edge besitzt genau eine inzidente Face, einen eindeutigen `twin` und einen eindeutigen `next`.
3. `twin(twin(e)) = e`; beide Half-Edges haben vertauschte Endpunkte.
4. Jeder Randzyklus ist geschlossen und so orientiert, dass seine Face lokal links der Half-Edges liegt.
5. Es gibt keine freien oder nur einseitig belegten Kanten, ungesplitteten echten Kreuzungen, unbeabsichtigten T-Knoten oder Nullkanten.
6. Bloße Punktberührung erzeugt keine Kantennachbarschaft.

Eine einzige randlose Full-Sphere-Face ist ein ausdrücklich erlaubter Sonderfall. Der leere Oberflächenzustand ist ungültig; Empty-Sphere-Geometrien dürfen lediglich als temporäre Operanden oder Ergebnisse einer Engine-Operation auftreten.

### 2.2 Gemeinsame Verfeinerung und Face-Zuordnungen

Jede atomare Face referenziert genau

- ein aktuelles Plattengebiet, also eine explizite Platte oder den nicht aufgelösten Hintergrund, und
- eine aktuelle Krustenregion.

Eine Kante darf im kanonischen Zustand nur bestehen, wenn sich mindestens eine dieser Referenzen zwischen ihren beiden Faces unterscheidet. Die Anordnung enthält somit keine Topologie nur zur Tessellierung oder Darstellung.

Ein Plattengebiet oder eine Krustenregion ergibt sich als Vereinigung aller Faces mit derselben fachlichen Referenz. Explizite Platten und Krustenregionen müssen jeweils über Kanten positiver Länge zusammenhängen. Gleiche Referenzen, die sich nur an Vertices berühren oder räumlich getrennt sind, verletzen die Zustandsinvarianten und erfordern getrennte Nachfolgeridentitäten. Nur der nicht aufgelöste Hintergrund darf mehrere getrennte Komponenten mit derselben besonderen Referenz besitzen.

Plattengebiets- und Krustenregionengrenzen werden durch Vergleich der Face-Referenzen abgeleitet. Eine Kante kann nur eine Krustenregionsgrenze, nur eine Plattengebietsgrenze oder beides sein. Sie erhält dadurch keine dauerhafte fachliche Identität.

### 2.3 Identität der Topologieelemente

Vertices, Half-Edges und atomare Faces besitzen ausschließlich snapshot-lokale technische Handles. Splitting, Snapping und Kanonisierung dürfen diese Handles ersetzen, ohne ein fachliches Herkunftsereignis auszulösen.

Dauerhafte fachliche Identitäten bleiben Platten, Krustenregionen und tektonischen Features vorbehalten. Eine Transaktion kann für ihren unmittelbaren Commit technische Alt-zu-Neu-Korrespondenzen liefern; diese werden weder als dauerhafte Topologieidentität noch als fachliche Abstammung interpretiert.

## 3. Rand- und Gleichheitssemantik

Die autoritative Punktzuordnung verwendet eine feste semi-offene Randsemantik. Dadurch gehört auch jeder Punkt auf einer Kante oder einem Mehrfachvertex genau einer atomaren Face und folglich genau einem Plattengebiet und einer Krustenregion.

Andere Fragestellungen verwenden ausdrücklich benannte Semantiken:

- `semi_open`: eindeutige Eigentums- und Punktlokalisierungsabfrage,
- `closed`: Berührung der abgeschlossenen Geometrien einschließlich Rand,
- `open`: reine Innenraumabfrage ohne Rand.

Topologische Nachbarschaft ist unabhängig von der semi-offenen Eigentümerschaft symmetrisch und besteht genau bei einer gemeinsamen Kante positiver Länge. Abstand kleiner als ein Epsilon und bloße Vertex-Berührung begründen keine Nachbarschaft.

Toleranz wirkt ausschließlich während des kontrollierten Snappings. Danach gilt Gleichheit autoritativer Vertices, Kanten, Faces und Snapshots als strukturelle Gleichheit ihrer kanonischen Repräsentation. Ein zusätzlicher nachträglicher Epsilonvergleich ist verboten.

## 4. Robuste Numerik und SnapPolicy

Topologische Entscheidungen verwenden gefilterte robuste S2-Prädikate mit Hochpräzisions- beziehungsweise Exact-Fallback und nötigenfalls symbolischer Perturbation. Selbstgebaute Entscheidungen der Form `abs(det) < epsilon` sind im autoritativen Pfad verboten.

Jeder Lauf besitzt genau eine unveränderliche `SnapPolicy`. Sie gilt gemeinsam für sämtliche von einer konstruktiven Operation betroffenen Besitz- und Feature-Layer. Lokale, ereignisabhängige oder je Geometrietyp abweichende Toleranzen sind unzulässig.

### 4.1 Globales Snapraster

Die Policy verwendet die Zentren eines festen globalen S2-Zellrasters als kanonische Ziele. S2-Zellen sind hierbei ausschließlich ein Snapraster und keine Raster- oder Flächenrepräsentation des Zustands.

Die Laufkonfiguration enthält eine minimale auflösbare Winkelbreite `W_min`. Der garantierte maximale Snapversatz darf höchstens `W_min / 4` betragen. Gewählt wird deterministisch das gröbste S2-Level, das diese Schranke einhält. Das gewählte Level und die daraus folgende garantierte Versatzschranke gehören zur Laufkonfiguration.

Faces, Löcher, Engstellen oder Featuregeometrien, deren maßgebliche Winkelbreite `W_min` unterschreitet, sind als autoritative Eingaben ungültig. Wenn eine Operation trotz gültiger Eingabe durch Konstruktion und Snapping eine fachlich relevante Face, Grenzkante, Engstelle, ein Loch oder eine Feature-Bindung kollabieren lässt, scheitert sie mit einer strukturierten Kollapsdiagnose. Sie darf das betroffene Objekt weder stillschweigend löschen noch fachlich umdeuten.

Der konkrete Standardwert von `W_min` und daraus abgeleitete physische Qualitätsbudgets werden in #13 anhand der Weltgröße und adversarial Tests kalibriert. Das hier festgelegte Verhältnis und die laufweite Unveränderlichkeit sind davon unabhängig.

### 4.2 Kraton-Zwangskonturen

Nach dem initialen Snapping und der Validierung wird jede Kratongrenze als unveränderliche geometrische Zwangskontur geführt. Spätere Operationen dürfen ihre Punktmenge weder verschieben noch löschen. Zusätzliche Vertices dürfen einen vorhandenen Bogen nur geometrieerhaltend unterteilen.

Eine Operation, deren Schnitt oder Snapping die Kontur, Fläche oder Zusammenhängigkeit eines Kratons ändern würde, scheitert. Zulässig ist ausschließlich die atomare Änderung der Plattengebietsreferenz aller Kraton-Faces beim vollständigen Trägerwechsel.

## 5. Tektonische Feature-Layer

Tektonische Features bleiben separate überlagernde Punkt-, Linien- oder Flächenlayer. Sie werden nicht grundsätzlich in die Besitzanordnung eingebaut, weil sie einander überlagern dürfen und keine dritte Besitzpartition bilden.

Feature-Geometrien verwenden dieselben normalisierten Punkte, geodätischen Bögen, robusten Prädikate, dieselbe `SnapPolicy` und dieselben ausdrücklich benannten Randmodelle. Bei einer Transaktion werden betroffene Feature-Layer gemeinsam mit der Besitzanordnung gesnappt und an deren Grenzen segmentiert. Dadurch sind Feature-Bindungen ohne toleranzbasierte Näherung prüfbar.

Erst wenn eine Ereigniskurve die Besitzpartition tatsächlich verändert, etwa beim erfolgreichen Aufbrechen durch ein Rift, wird sie ausdrücklich als Kante in die gemeinsame Anordnung eingefügt. Eine Linien-Mengendifferenz allein teilt keine Face.

## 6. Opaker SurfaceSnapshot

Der autoritative Zustand wird als opaker, unveränderlicher `SurfaceSnapshot` gekapselt. Half-Edge-Container, Engine-Typen und mutierbare Traversierungszeiger sind kein Wissen der Aufrufer.

Die geplante caller-facing Schnittstelle umfasst konzeptionell:

- semi-offene Punktzuordnung zu Plattengebiet und Krustenregion,
- ausdrücklich offene oder geschlossene geometrische Abfragen,
- Kantennachbarschaft und gemeinsame Grenzsegmente,
- Geometrie einer fachlichen Identität,
- Feature-Geometrie und geprüfte Feature-Bindungen,
- abgeleitete Winkel-, Flächen- und Zusammenhangsabfragen,
- Start einer atomaren Overlay-Transaktion,
- Validierungs- und Diagnoseansichten ohne Mutationszugriff.

Kinematik (#7), Ereignislogik (#8/#9), Persistenz (#11) und Bevy sind vorgesehene Aufrufer. Tests verwenden dieselben Queries und Transaktionen wie diese Aufrufer.

`SurfaceSnapshot` ist ein **Deepening Candidate**, kein bereits akzeptiertes tiefes Modul: Die Spezifikation zeigt die beabsichtigte kleine Schnittstelle und die dahinter verborgene Topologiekomplexität; erst Implementierung, Caller und Schnittstellentests können die architektonischen Akzeptanzkriterien belegen. Beim Löschen dieses Kandidaten müssten gemeinsame Randsemantik, Topologiepflege, Validierung, Kanonisierung und Engine-Übersetzung in mehrere Aufrufer ausweichen.

## 7. Atomare Overlay-Transaktion

Aufrufer dürfen die Half-Edge-Struktur nicht direkt verändern. Jede Änderung erfolgt über eine atomare Overlay-Transaktion gegen genau einen Eingabe-Snapshot.

Eine Transaktion erhält mindestens:

1. die betroffene kanonische Geometrie und gegebenenfalls neue Ereigniskurven,
2. die unveränderliche Lauf- und `SnapPolicy`,
3. eine deklarative fachliche Zuordnungsregel für jede mögliche neue Face,
4. den geplanten Fortbestand oder Wechsel fachlicher Identitäten und Tragbeziehungen, soweit für die Validierung nötig.

Sie führt als eine Operation aus:

1. betroffene Besitzgrenzen, Zwangskonturen, Ereigniskurven und Feature-Layer sammeln,
2. Eingaben und Kurvenbeschränkungen validieren,
3. echte Kreuzungen robust finden und als gemeinsame Vertices materialisieren,
4. alle betroffenen Layer gemeinsam auf das globale Raster snappen,
5. Randzyklen und atomare Faces rekonstruieren,
6. Face-Zuordnungen ausschließlich durch die deklarativen fachlichen Regeln bestimmen,
7. Featuresegmente und Bindungen übertragen,
8. technische geometrische Provenienz erzeugen,
9. Ergebnis kanonisieren und sämtliche Repräsentations- und Zustandsinvarianten validieren.

Das Ergebnis ist entweder

- ein vollständig gültiger, unveränderlicher neuer `SurfaceSnapshot` samt technischer Provenienz und Diagnosen oder
- ein strukturierter Fehler; der Eingabe-Snapshot bleibt unverändert.

Halbfertige Zwischenschritte sind keine gültigen Snapshots und dürfen die Schnittstelle nicht verlassen.

### 7.1 Keine implizite Eigentumsentscheidung

Die Geometrie-Engine darf Eigentum neuer Faces niemals anhand von Nähe, Flächengröße, Eingabereihenfolge oder internem Winding erraten. Jede neue Face muss durch die deklarative Regel genau ein Plattengebiet und genau eine Krustenregion erhalten.

Unzugeordnete Faces, widersprüchliche Ansprüche oder ein teilweiser Trägerwechsel ohne den erforderlichen Krustenregions-Split brechen die Transaktion ab. Welche fachliche Regel bei Bewegung, Wachstum oder Verbrauch gewinnt, bleibt #9 vorbehalten.

### 7.2 Technische Provenienz

Für jede neue Face, Kante und jedes neue Featuresegment wird die geometrische Herkunft aus Eingabeelementen vollständig ausgegeben, beispielsweise als Rest einer Eingabe-Face, Schnitt mehrerer Eingaben oder Segment einer Ereigniskurve.

Diese Provenienz erlaubt der Ereignislogik eine nachvollziehbare Fortbestands- und Nachfolgerentscheidung, ist aber selbst keine Platten-, Krusten- oder Feature-Abstammung. Fehlende oder mehrdeutige Provenienz verhindert den Commit.

## 8. Geometrie-Engine als hypothetische Seam

Die projektdefinierte Half-Edge-/Face-Anordnung bleibt unabhängig von der verwendeten S2-Implementierung autoritativ. Eine schmale, projektdefinierte Geometrie-Engine-Schnittstelle übersetzt normalisierte Punkte, Bögen, Operanden, Randmodell und `SnapPolicy` in robuste Prädikate, Overlay- und Builder-Aufrufe und übersetzt Ergebnisse und Fehler zurück.

Geplant sind ein `s2rst`-Adapterkandidat und ein S2-C++-Adapterkandidat hinter `cxx`. Keine S2-, C++-, FFI- oder Bibliotheks-DTOs dürfen Wissen von `SurfaceSnapshot`-Aufrufern werden. Die Produktionsauswahl folgt ausschließlich dem in der Recherche definierten Bake-off und Gate.

Diese Schnittstelle ist bis zum Implementierungsnachweis eine **hypothetische Seam**; die beiden Integrationen sind **Adapterkandidaten**. Erst vorhandene Adapter, gerichtete Abhängigkeiten, Übersetzungstests und gemeinsam durchlaufene Schnittstellentests können daraus eine bewiesene Seam beziehungsweise akzeptierte Adapter machen.

## 9. Kanonisierung und Determinismus

Jeder erfolgreiche Transaktionsentwurf wird vor Validierung und Commit kanonisiert. Die Kanonisierung normalisiert mindestens:

- Positionen auf das festgelegte S2-Zellzentrum,
- Half-Edge-Orientierung und Gegenkantenpaarung,
- Orientierung und Startvertex jedes Randzyklus,
- Reihenfolge mehrerer Randzyklen einer Face,
- Reihenfolge atomarer Faces, Vertices, Kanten und Featuresegmente anhand geometrischer und fachlicher Schlüssel,
- Darstellung des randlosen Full-Sphere-Sonderfalls.

Bei gleicher kanonischer Eingabe, fachlich relevanter Operationsreihenfolge, Engine-Version und Laufkonfiguration einschließlich `SnapPolicy` muss derselbe kanonische Snapshot-Hash entstehen. Engine-Kennung und -Version, `SnapPolicy` sowie die fachlich relevante Operationsreihenfolge gehen ausdrücklich in den Hash ein. HashMap-Reihenfolge, Traversierungsreihenfolge, Parallelisierung und Einfügereihenfolge dürfen ihn nicht beeinflussen.

Ein Wechsel von Engine-, Compiler- oder Plattformversion garantiert nicht ungeprüft Bitidentität. Gepinnte Versionen und die plattformübergreifenden Differential- und Determinismustests aus der Recherche bleiben Freigabebedingung.

Räumliche Indizes, vorberechnete Messwerte, Triangulierungen und Bevy-Meshes sind abgeleitete Caches. Sie dürfen verworfen und rekonstruiert werden und bestimmen weder Gültigkeit noch kanonischen Hash. Die konkrete dauerhafte Serialisierung des Snapshots entscheidet #11.

## 10. Commit-Invarianten

Eine Transaktion darf nur committen, wenn der vollständige Ergebnis-Snapshot mindestens folgende Bedingungen nachweist:

### Geometrie

1. Alle Vertices sind endlich, normalisiert und liegen auf erlaubten Rasterzentren.
2. Jede Kante ist ein eindeutiger kürzerer Großkreisbogen mit verschiedenen, nicht antipodalen Endpunkten.
3. Es bestehen keine ungesplitteten echten Kreuzungen, unbeabsichtigten T-Knoten oder unerlaubten Degenerierungen.
4. Die Mindestbreiten- und Snapversatzbedingungen sind erfüllt.
5. Kraton-Zwangskonturen sind geometrisch unverändert.

### Topologie

1. Sämtliche Twin-, Next-, Vertex-, Face- und Randzyklusinzidenzen sind vollständig und wechselseitig konsistent.
2. Die Anordnung ist eine geschlossene orientierte 2-Mannigfaltigkeit oder der definierte randlose Full-Sphere-Sonderfall.
3. Jede Kante trennt mindestens eine unterschiedliche fachliche Face-Referenz.
4. Jede gemeldete Nachbarschaft besitzt eine gemeinsame Kante positiver Länge und umgekehrt.
5. Geeignet berechnete Euler- und Flächenkonsistenzprüfungen stimmen; sie sind Zusatzprüfungen und kein alleiniger Abdeckungsbeweis.

### Oberflächenbelegung

1. Jede atomare Face hat genau eine Krustenregions- und eine Plattengebietsreferenz.
2. Die semi-offene Punktlokalisierung bildet die gesamte Kugel eindeutig auf genau eine Face ab.
3. Explizite Platten und Krustenregionen sind über Kanten positiver Länge zusammenhängend; nur der Hintergrund darf mehrteilig sein.
4. Jede Krustenregion wird vollständig von genau einem Plattengebiet getragen.
5. Punktberührende oder getrennte Komponenten teilen keine Identität, sofern sie nicht zum Hintergrund gehören.

### Features und Herkunft

1. Jedes aktuelle Feature besitzt eine gültige, gemeinsam gesnappte räumliche Bindung.
2. Grenzgebundene Features liegen auf den bezeichneten aktuellen Plattengebietsgrenzen; krustengebundene Features liegen auf oder in den bezeichneten tragenden Krustenregionen.
3. Technische Provenienz ist vollständig und von fachlicher Abstammung getrennt.
4. Fachliche Identitäten, Fortbestands- und Nachfolgerpläne erfüllen die Invarianten des Zustandsmodells.

Eine Implementierung darf diese Zusicherung inkrementell statt durch vollständigen Neuaufbau berechnen, muss aber dieselbe Aussage wie eine globale Vollvalidierung liefern. Ein ungeprüfter oder nur lokal plausibler Zustand darf die Schnittstelle nicht verlassen.

## 11. Bewusst vertagte Entscheidungen

Diese Spezifikation entscheidet ausdrücklich nicht:

- welche Eulerpole, Geschwindigkeiten oder Kurventessellierungsbudgets eine Bewegung erzeugt (#7),
- wann ein Feature beginnt, endet, seinen Typ wechselt oder geteilt wird (#8),
- welche fachliche Face-Zuordnungsregel bei konkurrierenden Ereignissen gewählt wird (#9),
- ob Snapshots vollständig oder differenziell persistiert und wie sie abgefragt werden (#11),
- welcher konkrete Standardwert für `W_min`, welche Weltgröße oder welche absoluten Laufzeit- und Speicherbudgets gelten (#13),
- ob `s2rst` oder S2 C++ das Produktions-Gate gewinnt; diese Wahl bleibt dem bereits spezifizierten Bake-off vorbehalten.

Damit sind die Repräsentation und ihr Interface festgelegt, ohne Entscheidungen der nachfolgenden Wayfinder-Tickets vorwegzunehmen.
