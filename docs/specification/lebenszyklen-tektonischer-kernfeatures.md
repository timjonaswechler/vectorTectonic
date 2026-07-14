# Lebenszyklen der tektonischen Kernfeatures

Status: gemeinsam bestätigt für das MVP

Wayfinder-Ticket: [#8](https://github.com/timjonaswechler/vectorTectonic/issues/8)

Grundlagen: [fachliches Zustandsmodell](fachliches-zustandsmodell.md), [Geometrie- und Topologierepräsentation](geometrie-und-topologierepraesentation.md), [kinematisches Plattenbewegungsmodell](kinematisches-plattenbewegungsmodell.md) und [Recherche zu tektonischen Kernfeatures](../research/tektonische-kernfeatures.md)

## Zweck und Abgrenzung

Dieses Dokument legt für die neun tektonischen Kernfeatures Entstehung, Veränderung, Ende, Erhaltung, autoritative Geometrie und fachliche Herkunft fest. Es beschreibt zulässige Lebensläufe und die Wirkungen bestätigter Übergänge, aber weder deren globale Auswahl noch die Konfliktauflösung konkurrierender Ereignisse.

Die [automatischen tektonischen Ereignisregeln](automatische-tektonische-ereignisregeln.md) bestimmen konkrete Trigger-Defaults, Scores, Prioritäten, Gleichstände und die atomare Auswahl miteinander konkurrierender Ereignisse. Ticket #11 bestimmt Persistenz und Abfragen, Ticket #13 numerische Akzeptanzbudgets. Dieses Dokument führt keine Implementierungsdatenstrukturen, vollständige Simulationshistorie, Raster, Höhen, Erosion, Rheologie oder Mantelprozesse ein.

## 1. Verbindliche Feature-Taxonomie und Geometrie

Das MVP besitzt genau neun Kernfeature-Typen:

| Feature-Typ | Autoritative Geometrie | Fachliche Bedeutung |
|---|---|---|
| `rift` | zentrale geodätische Linie | aktives Auseinanderbrechen kontinentaler Kruste innerhalb einer Platte |
| `spreading_ridge` | geodätische Linie | ozeanische Akkretionsachse einer divergenten Plattengrenze |
| `trench` | orientierte geodätische Linie | Oberflächenspur aktiver Subduktion mit fester Polarität |
| `volcanic_arc` | geodätische Linie | grabenparallele, oberplattengebundene Spur abgeleiteten Vulkanismus |
| `transform_fault` | geodätische Linie | aktive, tangential dominierte Verbindung tektonischer Endfeatures |
| `fracture_zone` | geodätische Linie | krustengebundene konjugierte Spur eines Ridge–Ridge-Transforms |
| `collision` | geodätische Linie | aktive kontinentale Kollisionsfront |
| `suture` | geodätische Linie | erhaltene Narbe eines geschlossenen Ozeans |
| `orogen` | sphärisches Polygon | deformierte Zone einer aktiven oder vergangenen Kollision |

Alle Linien sind segmentierte Folgen kürzerer Großkreisbögen und verwenden dieselbe `SnapPolicy` wie die Oberflächenanordnung. Die Grabenorientierung trägt Ober-/Unterplattenrollen; die Orientierung aller anderen Linien ist kanonisch, aber nicht Teil ihrer fachlichen Identität.

Rift-, Bogen-, Transform- und Suturkorridore, eine optionale Rückenstilzone sowie einzelne Vulkanzentren werden deterministisch aus der autoritativen Geometrie und ihren Parametern abgeleitet. Sie sind weder zusätzliche Feature-Geometrien noch eigene Kernfeatures. Passive Kontinentränder und `non_transform_offset` bleiben abgeleitete Geometrien beziehungsweise spätere Taxonomie und besitzen in diesem MVP keinen eigenen Lebenszyklus.

## 2. Gemeinsamer Lebenszyklus

### 2.1 Zustände

Ein aktuelles Feature besitzt genau einen der folgenden generischen Zustände:

- `active`: Sein tektonischer Prozess oder, bei einer Fracture Zone, seine Spurbildung läuft.
- `waning`: Eine Endbedingung liegt vor, die bestätigte Hysterese oder Ausklangfrist ist aber noch nicht abgelaufen.
- `relict`: Der erzeugende Prozess ist beendet; die geometrische Spur bleibt fachlich erhalten.

Ein Ereigniskandidat ist kein Feature und erhält weder Feature-Identität noch Geometrie im aktuellen Zustand. `candidate`, `inactive` und `consumed` sind deshalb keine Feature-Zustände. Vollständiger Verbrauch, ein Typwechsel oder das ersatzlose Ende entfernt eine Identität aus dem aktuellen Zustand und erzeugt einen Herkunftsnachweis mit Endzeit und Endursache.

`relict` ist bezüglich Aktivität terminal. Rückkehr aus `waning` nach `active` erhält die Identität; eine spätere Reaktivierung eines Relikts erzeugt dagegen eine neue Identität mit Vorgängerbeziehung. Die neue Aktivitätsperiode beginnt mit eigenen Reife-, Hysterese- und Aktivitätsakkumulatoren. Das Relikt darf ihre Auswahl als geerbte Schwäche beeinflussen, überträgt aber keine abgelaufenen Timer.

### 2.2 Identität und Abstammung

Reine Zustands-, Rollen-unverändernde Geometrie- und Parameteränderungen erhalten die Feature-Identität. Ein fachlicher Typwechsel, eine räumliche Trennung, eine fachliche Vereinigung oder eine Reaktivierung eines Relikts erzeugt neue Nachfolgeridentitäten. Bei Teilungen eines noch laufenden Features wird nicht rekonstruierbarer fachlicher Fortschritt konservativ auf die Nachfolger übertragen; bei Vereinigungen muss die Übergangsregel den Umgang mit jedem Akkumulator ausdrücklich bestimmen.

Berührung, Überlappung, gleiche Geometrie oder gemeinsames Snapping vereinigen Features niemals automatisch. Ebenso erzeugen technische Segmentierung, Tessellierung oder der Austausch snapshot-lokaler Topologiehandles keine fachliche Abstammung. Eine Vereinigung erfordert einen ausdrücklich bestätigten fachlichen Übergang.

### 2.3 `waning` und Prozesswirkungen

Sobald die typspezifische Aktivitätsbedingung nicht mehr erfüllt ist, wechselt ein aktives Feature nach `waning`. Ab diesem Zeitpunkt pausieren alle typspezifischen Flächenänderungen und kinematischen Einflussbeiträge:

- ein Rücken erzeugt keine ozeanische Kruste und keinen Ridge Push,
- ein Graben verbraucht keine Kruste und erzeugt keinen Slab Pull,
- eine Kollision erhöht weder Verkürzungsproxy noch Kollisionswiderstand,
- ein Rift und eine Fracture Zone erhöhen ihren Reife- beziehungsweise Wachstumsfortschritt nicht.

Kehrt die passende Bedingung innerhalb der Hysterese zurück, wird dieselbe Identität wieder `active`. Andernfalls endet sie gemäß ihrem typspezifischen Lebenslauf. Die konkrete Dauer nicht wissenschaftlich eingegrenzter Hysteresen legen die automatischen Ereignisregeln fest.

### 2.4 Bindung und Erhaltung

Aktive grenzgebundene Features folgen der atomar bestätigten aktuellen Grenze und besitzen keinen eigenen Bewegungsvektor. Bei Linienfeatures liegt die Geometrie auf der Grenze. Eine aktive oder abklingende Orogenzone ist als einziges grenzgebundenes Flächenfeature über eine aus ihrer Quellkollision abgeleitete Bindungsachse verankert; ihr Polygon wird bei jedem gültigen Übergang vollständig aus Achse, eingefrorenem oder wachsendem Verkürzungsproxy, Breitenregel und bestätigter Vergenz bestimmt. Krustengebundene Features werden mit ihrem eindeutigen Träger starr advektiert. Ausschließlich die in den Ereignisregeln festgelegten flächenneutralen Qualifikations- und Kollisionsakkommodationen dürfen lokale Geometrie unter Erhaltung von Krustenfläche, Kratonen und Zwangskonturen deformieren. Wechselt ein vollständiger Träger, bleibt die Feature-Identität erhalten; verteilt sich die Geometrie anschließend auf unterschiedlich bewegte Träger, entstehen räumlich getrennte Nachfolger.

Alle geometrisch noch vorhandenen Relikte gehören unabhängig von ihrem Alter zur autoritativen Feature-Map. Eine Sichtbarkeitsfrist, Stiländerung oder Generalisierung darf ihre fachliche Geometrie nicht löschen. Vollständiger Verbrauch der tragenden Kruste beendet ein Relikt; teilweiser Verbrauch verkleinert es, und eine für eindeutige Trägerbewegung erforderliche räumliche Trennung erzeugt Nachfolger. Spätere Features dürfen Relikte überlagern, ohne sie zu verdrängen.

## 3. Gemeinsame fachliche Provenienz

Jedes aktuelle Feature und jeder Herkunftsnachweis führt mindestens:

- dauerhafte Feature-Identität und Feature-Typ,
- Entstehungszeit sowie unmittelbare Vorgänger und Nachfolger,
- eine kanonische Entstehungsart, beispielsweise `rift_breakup`, `ridge_relocation`, `subduction_initiation`, `boundary_reclassification`, `collision`, `source_trace` oder `reactivation`,
- Version der erzeugenden Regel und
- deren Evidenzklasse `befund`, `plausible_mvp_abstraktion` oder `modellannahme`.

Ein aktuelles Feature führt zusätzlich Zustand, typspezifischen Unterzustand, Zeitpunkt des letzten Zustandswechsels, autoritative Geometrie, aktuelle Feature-Bindung und sämtliche aktuellen benannten Rollen. Nur nicht aus dem aktuellen Gesamtzustand rekonstruierbare Lebenszykluswerte sind autoritativ. Dazu gehören gewählte Erzeugungsparameter, Reife- und Hysteresezustand, kumulativer Grabenverbrauch, Rift-Reifegrad, Kollisions-/Orogen-Verkürzungsproxy, Bogenoffset und Orogenbreitenregel.

Ein Herkunftsnachweis führt zusätzlich Endzeit und Endursache, aber weder frühere Geometrie noch eine aktuelle räumliche Bindung oder aktuelle Rollen. Ein numerischer Vertrauenswert pro Feature ist nicht vorgesehen. Ereignisscores, Tiebreaks und numerische Residuen sind Diagnosen, keine fachliche Eigenschaft des erzeugten Features.

Länge, Fläche, tatsächliche Breite, aktuelle Normal-/Tangentialrate, Obliquität und Krustenalter werden aus kanonischer Geometrie, Bewegungszustand und Krustenherkunft abgeleitet.

## 4. Rift

### 4.1 Entstehung

Ein `rift` entsteht erst mit bestätigtem Beginn des Auseinanderbrechens in zusammenhängender nichtkratonischer kontinentaler Kruste einer expliziten Platte. Die Auswahl kann eine Reliktsutur oder ein früheres Rift als geerbte Schwäche verwenden, doch der bloße Kandidat bleibt außerhalb des Feature-Zustands. Die autoritative Riftlinie und ihr abgeleiteter Korridor dürfen keine Kraton-Zwangskontur schneiden. Ein nicht gültig um Kratone führbarer Verlauf wird nicht erzeugt.

Das Rift beginnt als `active` mit Unterzustand `incipient`, eigener Entstehungszeit, betroffenen Krustenbeziehungen, auslösender Schwäche und einem Reifegrad von null. Der abgeleitete Riftkorridor verwendet eine bei Entstehung gewählte Breite von 30–300 km; 100 km ist der in den automatischen Ereignisregeln bestätigte Default.

### 4.2 Veränderung

Weil die noch ungeteilte Platte starr ist, wird keine fiktive innerplattige Relativbewegung integriert. Stattdessen wächst ein dimensionsloser, ausdrücklich als Modellannahme markierter Reifegrad mit kumulierter `active`-Riftzeit. Die gesamte Reifedauer bis zum Break-up-Kandidaten liegt im Bereich 20–60 Myr. Die in den automatischen Ereignisregeln festgelegte Zwischenschwelle wechselt den Unterzustand von `incipient` nach `localized`.

Zusammenhängende Endfortpflanzung und Formänderung der Riftlinie erhalten die Identität, solange dieselbe Schwäche und derselbe beabsichtigte Trennverlauf fortbestehen. Technische Kurvenänderungen dürfen weder Kratone schneiden noch einen fachlich anderen Trennpfad unter derselben Identität verbergen.

Verliert das Rift seine Aktivitätsbedingungen, wechselt es nach `waning`; sein Reifegrad pausiert. Erholt es sich innerhalb der Hysterese, setzt es denselben Lebenslauf fort.

### 4.3 Ende und Erhaltung

Ein Break-up ist nur als vollständiger atomarer Übergang zulässig. Er muss gleichzeitig

1. die gesamte bestätigte Riftachse als neue Plattengrenze einsetzen,
2. die Vorgängerplatte gültig teilen,
3. einen oder mehrere aktive Rücken-Nachfolger erzeugen und
4. die bestätigte Öffnungszielkurve an die Bewegungsvererbung übergeben.

Am exakten Break-up-Zeitpunkt entsteht noch keine ozeanische Fläche positiver Größe. Die erste ozeanische Kruste entsteht im folgenden Tick ausschließlich aus dem tatsächlich integrierten positiven Rückenfluss.

Ein abschnittsweiser Rift-/Rücken-Hybrid ist im MVP unzulässig. Scheitert die atomare Topologieänderung, bleibt der vorherige gültige Riftzustand bestehen. Läuft die Hysterese ohne Break-up oder Erholung ab, wechselt dasselbe Rift mit Unterzustand `abandoned` nach `relict`; Linie, Korridorparameter, Reifegrad und Herkunft bleiben erhalten.

## 5. Rücken

### 5.1 Entstehung

Ein `spreading_ridge` entsteht genau in einem atomaren Übergang, der eine divergente Grenze zwischen zwei eigenständig bewegten Platten und ozeanische Krustenproduktion herstellt. Zulässige Entstehungsarten sind:

- erfolgreicher kontinentaler Break-up mit Rift-Vorgänger,
- bestätigter Achsensprung mit Rücken-Vorgänger oder
- bestätigte ozeanische Plattenteilung oder Grenzreklassifikation.

Auf kontinentaler Kruste ist ein Rift zwingende Vorstufe. Eine bloß divergent klassifizierte Grenze ohne ozeanische Akkretion ist kein Rücken.

### 5.2 Veränderung

Die Identität ist an die kontinuierlich fortbestehende Akkretionsachse gebunden, nicht an ein bestimmtes Plattenpaar oder eine unveränderliche interne Segmentierung. Advektion, Formänderung, wandernde Endpunkte, wechselnde Rate und interne Segmentierung erhalten die Identität. Räumliche Trennung oder Vereinigung folgt den allgemeinen Nachfolgerregeln.

Solange der Rücken `active` ist, erzeugt er aus dem tatsächlich integrierten positiven Normalfluss ozeanische Kruste auf beiden Seiten und liefert Ridge Push. Tangentiale Bewegung erzeugt keine Fläche. Die symmetrische Verteilung des vollen Öffnungsflusses auf beide Seiten ist eine versionierte Modellannahme. Transformconnectoren und kleine nicht-transforme Offsets sind nicht Teil der Rückenidentität.

Bei fehlender normaler Öffnung oder unmöglicher gültiger Krustenproduktion wechselt der Rücken nach `waning`; Produktion und Ridge Push pausieren sofort. Ein Achsensprung ist keine Geometrieänderung, sondern beendet die alte Achse zugunsten eines neuen aktiven Rücken-Nachfolgers.

### 5.3 Ende und Erhaltung

Nach abgelaufener Hysterese endet die aktive grenzgebundene Rückenidentität. Ein bestätigter neuer Grenztyp erhält eine eigene Nachfolgeridentität. Zusätzlich entstehen zwei zunächst deckungsgleiche `spreading_ridge[relict]`-Nachfolger, je einer gebunden an die unmittelbar angrenzende ozeanische Kruste jeder Seite. Sie bewahren die konjugierte aufgegebene Achse und werden anschließend unabhängig mit ihren Trägern advektiert.

## 6. Graben

### 6.1 Entstehung

Ein `trench` entsteht erst durch bestätigte Subduktionsinitiierung an einem länger konvergenten Grenzsegment mit mindestens einer ozeanischen Seite und einer zulässigen geerbten Schwäche. Weder Krustenalter noch Konvergenz noch eine Fracture Zone lösen ihn allein aus. Die konkrete Kombination und Auswahl entscheiden die automatischen Ereignisregeln.

Bei Entstehung werden Linienorientierung, Unterplatte, Oberplatte und Polarität atomar festgelegt. Bei Ozean–Kontinent-Konvergenz ist im MVP standardmäßig der Ozean Unterplatte; Ozean–Ozean-Polarität folgt der in den automatischen Ereignisregeln festgelegten deterministischen Auswahl. Die Polarität bleibt während der gesamten Grabenidentität unveränderlich. Ein bestätigter Polaritätswechsel beendet sie und erzeugt einen neuen Graben-Nachfolger; ein spontaner Wechsel ist verboten.

### 6.2 Veränderung

Ein aktiver Graben verbraucht ausschließlich den geometrisch anliegenden Anteil der Unterplatte entsprechend dem tatsächlich integrierten negativen Normalfluss. Kontinentale Kruste wird nicht verbraucht. Der Graben akkumuliert die nicht aus dem Endzustand rekonstruierbare verbrauchte Fläche und liefert nur als `active`-Feature den rollenbestimmten Slab-Pull-Einfluss.

Seine Slab-Pull-Reife wächst ausschließlich mit kumulierter aktiver Grabenzeit von null bis zur konfigurierten Reifezeit von 5–20 Myr. Formänderung, Endfortpflanzung und veränderliche Obliquität erhalten die Identität, solange Polarität und Prozesskontinuität bestehen. Bei fehlender Konvergenz wechselt der Graben nach `waning`; die Subduktionshysterese liegt zwischen 1 und 5 Myr. Währenddessen pausiert auch die Reifeakkumulation. Rückkehr der passenden Kinematik erhält die Identität und setzt die bisherige Reife fort.

### 6.3 Ende ohne Kollision

Endet der Graben durch dauerhaft ausbleibende Konvergenz, Rückenankunft oder bestätigte Grenzreklassifikation, endet seine aktive grenzgebundene Identität. Es entsteht ein einzelner `trench[relict]`-Nachfolger auf der damaligen Oberplattenkruste. Er übernimmt letzte Geometrie, Polarität und Herkunft, besitzt aber keine aktuellen Unter-/Oberplattenrollen mehr. Bei nur abschnittsweisem Ende entstehen fortbestehende aktive und reliktische Nachfolger.

### 6.4 Kontinentankunft

Erreicht kontinentale Kruste der Unterplatte den Graben auf einem in den automatischen Ereignisregeln bestätigten zusammenhängenden Abschnitt, wechselt dieser nach `waning` mit Unterzustand `collision_pending`. Ozeanischer Verbrauch und Slab Pull pausieren lokal sofort. Der Kontakt muss 1–3 Myr anhalten; löst er sich vorher gültig, darf der Graben zur selben aktiven Identität zurückkehren.

Bei bestätigter Kollision endet der betroffene Grabenabschnitt atomar zugunsten einer aktiven Kollision und einer reliktischen Sutur; gleichzeitig entsteht die aktive Orogenzone. Erfordert der geschlossene Ereigniskatalog stattdessen eine Grenzreklassifikation, gilt der Lebenslauf des Endes ohne Kollision.

## 7. Vulkanbogen

### 7.1 Entstehung und Geometrie

Ein `volcanic_arc` entsteht ausschließlich aus genau einem ausreichend reifen aktiven Quellgraben nach 3–10 Myr Reifezeit. Er erhält den Quellgraben und dessen Oberplatte als unveränderliche Herkunftsrollen. Die Bogenlinie wird als geodätischer Offset von 100–300 km auf der Oberplatte erzeugt, an Grabenenden um 50–150 km gekürzt und bei Platzmangel auf der Oberplatte geclippt. Sie darf niemals auf die Unterplatte verschoben werden.

### 7.2 Veränderung und Ende

Form-, Offset- und Clippingänderungen erhalten die Identität, solange derselbe Quellgraben besteht. Ein Wechsel zu einem anderen Graben, eine Teilung oder Vereinigung erzeugt Nachfolger. Der Bogen folgt seinem Quellgraben verzögert: Wechselt der Graben nach `waning`, wechselt auch der Bogen nach `waning`; erholt sich der Graben, werden beide unter ihren bestehenden Identitäten wieder aktiv.

Endet der Quellgraben, klingt der Bogen 0–5 Myr aus und wechselt anschließend unter derselben, nun krustengebundenen Identität nach `relict`. Der letzte gültige Linienverlauf und die Oberplattenbindung bleiben unabhängig vom Alter erhalten. Ein neuer Quellgraben kann einen neuen Bogen mit Vorgängerbeziehung zum Relikt erzeugen, reaktiviert aber nicht dessen Identität.

## 8. Transformstörung

### 8.1 Entstehung und Identität

Eine `transform_fault` entsteht durch einen bestätigten, tangential dominierten Connector zwischen genau zwei aktiven tektonischen Endfeatures. Zulässige Connector-Typen sind Ridge–Ridge, Ridge–Trench und Trench–Trench; eine kontinentale, tangential dominierte Hauptlinie ist der entsprechende kontinentale Typ. Die konkrete automatische Auswahl ist in den automatischen Ereignisregeln festgelegt.

Die Identität ist an das fachliche Paar aktiver Endpunkte und den Connector-Typ gebunden. Diese Endpunkte werden durch Beziehungen zu Feature-Identitäten und deren geometrischen Enden beschrieben, niemals durch dauerhafte technische Vertexhandles. Formänderung, Längenänderung und wechselnde Obliquität erhalten die Identität. Der Ersatz eines Endfeatures oder Connector-Typs beendet sie zugunsten eines neuen Nachfolgers.

### 8.2 Veränderung und Ende

Eine aktive Transformstörung erzeugt und verbraucht keine Fläche und liefert keinen eigenen kinematischen Einfluss. Bei nicht mehr tangential dominierter Bewegung oder beim Ende eines Endfeatures wechselt sie nach `waning`. Nach bestätigtem Klassenwechsel entsteht ein neuer Grenzfeature-Typ; nach ersatzlosem Ende gelten folgende Erhaltungsregeln:

- Eine Ridge–Ridge-Transformstörung hinterlässt außer ihren bereits erzeugten Fracture Zones kein zusätzliches Transformrelikt.
- Ridge–Trench-, Trench–Trench- und kontinentale Transformen erzeugen je angrenzender Krustenseite einen krustengebundenen `transform_fault[relict]`-Nachfolger.

Diese Relikte werden niemals allein durch Inaktivität zu Fracture Zones. Eine spätere Reaktivierung erzeugt eine neue Transformidentität.

## 9. Fracture Zone

Eine aktive Ridge–Ridge-Transformstörung erzeugt mit der ersten benachbarten ozeanischen Krustenproduktion genau zwei `fracture_zone`-Features, eines pro Spreizungsseite. Jede Fracture Zone ist von Beginn an an ihre ozeanische Trägerkruste gebunden und referenziert dieselbe Quelltransform.

Ihr Zustand `active` bezeichnet laufende Spurbildung, nicht eine aktive Plattengrenze. Solange Quelle und Spreizung aktiv sind, wächst ihre Linie proximal mit neu gebildeter Kruste; ältere Teile werden starr mit dem Träger advektiert. Sie besitzt keinen eigenen kinematischen Einfluss und keinen Flächenfluss.

Wechselt Quelle oder Spreizung nach `waning`, pausiert das Wachstum. Endet die Quelle oder der Rücken endgültig, wechselt die Fracture Zone unter derselben Identität nach `relict`. Sie hat keinen geologischen Age-out und endet nur durch vollständigen Verbrauch ihrer Trägergeometrie; Teilverbrauch und Trennung folgen den allgemeinen Regeln. Andere Transformtypen erzeugen niemals automatisch Fracture Zones.

## 10. Kollision

### 10.1 Entstehung

Eine `collision` entsteht nach dem in Abschnitt 6.4 beschriebenen bestätigten Kontinentkontakt atomar mit Sutur und Orogen. Sie ist eine aktive grenzgebundene Kollisionsfront zwischen den beteiligten kontinentalen Krusten. Ihre Herkunft referenziert mindestens den Vorgängergraben, die beteiligten kontinentalen Krustenregionen, die Herkunftsnachweise der am geschlossenen Becken beteiligten ozeanischen Krustenregionen und die aktuellen Plattenrollen. Sie vernichtet oder erzeugt keine kontinentale Fläche.

### 10.2 Veränderung

Solange kontinentaler Kontakt und tatsächliche Konvergenz bestehen, akkumuliert die Kollision

`shortening_proxy_km = ∫ max(0, -v_n) dt`.

Der Proxy ist integrierte Plattenkonvergenz und keine rekonstruierte Krustenverkürzung. Nur die aktive Kollision liefert den in der Kinematikspezifikation festgelegten Kollisionswiderstand. Formänderung und zusammenhängende Endfortpflanzung erhalten die Identität; räumliche Trennung und Vereinigung erzeugen Nachfolger.

Bei neutraler oder divergenter Bewegung wechselt die Kollision nach `waning`; Proxy und Kollisionswiderstand pausieren. Erholt sich die Konvergenz innerhalb der Hysterese, setzt sie dieselbe Identität fort.

### 10.3 Ende

Nach abgelaufener Hysterese endet die Kollisionsidentität ersatzlos und bleibt nur als Herkunftsnachweis erhalten. Eine Kollision besitzt kein eigenes Relikt, weil Sutur und Orogen ihre dauerhaften Oberflächenspuren darstellen. Erneute Konvergenz erzeugt eine neue Kollision mit Beziehungen zu diesen Relikten.

## 11. Sutur

Bei bestätigter Kollision entsteht eine `suture[relict]` als Nachfolgerin des beendeten Grabenabschnitts. Ihre Linie wird aus der letzten Grabengeometrie und dem bestätigten kontinentalen Kontakt abgeleitet und bezeichnet die Narbe des geschlossenen Ozeanbeckens. Ihre Herkunft referenziert den Vorgängergraben, die Quellkollision, die beteiligten kontinentalen Krustenregionen und die Herkunftsnachweise der betroffenen ozeanischen Krustenregionen; eine eigenständige Ozeanidentität wird nicht eingeführt. Die Sutur ist kein aktiver Grenztyp und besitzt weder Flächenfluss noch kinematischen Einfluss.

Solange sie auf einer aktuellen Plattengebietsgrenze liegt, darf sie grenzgebunden bleiben. Wird diese Grenze durch Plattenverschmelzung intraplattenintern, wechselt sie ohne Identitätsverlust zur krustengebundenen Spur auf der gemeinsamen Trägerplatte. Würden ihre Teile anschließend verschiedenen Trägerbewegungen folgen, entstehen entsprechende reliktische Nachfolger.

Seed-Reliktsuturen sind der einzige Entstehungssonderfall: Sie beginnen bei Simulationszeit null ohne simulierten Vorgänger und mit Entstehungsart `seeded_prehistory`. Suturen besitzen keinen Age-out. Sie werden nicht selbst aktiv; ihre Nutzung als Schwäche erzeugt ein neues Rift-, Graben-, Transform- oder Kollisionsfeature mit Vorgängerbeziehung, während die Sutur erhalten bleibt.

## 12. Orogenzone

### 12.1 Entstehung und Wachstum

Ein `orogen` entsteht atomar mit seiner Quellkollision als aktives, grenzgebundenes und die Kollisionsfront überlagerndes sphärisches Polygon. Seine aus der Quellkollision abgeleitete Bindungsachse liegt auf der aktuellen Plattengebietsgrenze; das Polygon darf die angrenzenden Plattengebiete überlagern, besitzt aber keine Eigentümerplatte. Es referenziert genau eine Quellkollision und die zugehörige Sutur. Die anfängliche Gesamtbreite liegt zwischen 100 und 300 km.

Während die Quellkollision aktiv ist, folgt die Gesamtbreite der versionierten Modellregel

`total_width = min(1_000 km, initial_width + width_factor × shortening_proxy_km)`

mit `width_factor` zwischen 0,1 und 0,3. Diese Breite ist ein kartographischer Deformationsproxy, keine Höhen-, Rheologie- oder Erosionsberechnung. Eine Asymmetrie wird nur aus bestätigter Vergenz abgeleitet; ohne bestätigte Vergenz wächst die Zone symmetrisch. Formänderung, Wachstum und Clipping erhalten die Identität.

### 12.2 Ende und Erhaltung

Wechselt die Quellkollision nach `waning`, wechselt auch das Orogen nach `waning`; Verkürzungsproxy und Breite frieren ein, während Bindungsachse und daraus reproduzierte Fläche der atomar bestätigten Kollisionsgrenze folgen. Erholt sich die Kollision, werden beide wieder aktiv. Endet sie, endet auch die grenzgebundene Bindung; ein zeit- oder erosionsbasierter Rückbau findet nicht statt.

Überspannt die letzte Orogenfläche beim Kollisionsende mehrere unterschiedlich bewegte Platten, wird sie entlang der aktuellen Plattengebietsgrenzen in eindeutig getragene `orogen[relict]`-Nachfolgerpolygone geteilt. Liegt die gesamte Zone bereits auf einer gemeinsamen Nachfolgerplatte, behält sie ihre Identität und wechselt mit krustengebundener Geometrie nach `relict`. Sämtliche Nachfolger bewahren Quellkollision, Sutur, letzten Verkürzungsproxy und Breitenregel.

## 13. Übergangsmatrix

| Ausgang | Bestätigte Bedingung | Ergebnis |
|---|---|---|
| kein Feature | Riftinitiierung | `rift[active, incipient]` |
| `rift[active]` | Reifeschwelle | `rift[active, localized]`, gleiche Identität |
| `rift[active|waning]` | vollständiger Break-up | Rift endet; aktive Rücken-Nachfolger |
| `rift[waning]` | Hysterese abgelaufen | `rift[relict, abandoned]`, gleiche Identität |
| kein Feature/Rücken | bestätigte ozeanische Akkretion oder Achsensprung | neuer `spreading_ridge[active]` |
| `spreading_ridge[waning]` | Hysterese abgelaufen | aktive Identität endet; zwei krustengebundene Rückenrelikte |
| kein Feature | bestätigte Subduktionsinitiierung | `trench[active]` mit fester Polarität |
| `trench[waning]` | Ende ohne Kollision | Oberplatten-`trench[relict]`-Nachfolger |
| `trench[waning, collision_pending]` | bestätigter Kontinentkontakt | Graben endet; `collision[active]`, `suture[relict]`, `orogen[active]` |
| reifer `trench[active]` | Bogenreife erreicht | `volcanic_arc[active]` |
| `volcanic_arc[waning]` | Quelle endet und Ausklang läuft ab | `volcanic_arc[relict]`, gleiche Identität |
| kein Feature | bestätigter Transformconnector | `transform_fault[active]` |
| Ridge–Ridge-Transform + Akkretion | erste Spurbildung | zwei `fracture_zone[active]` |
| Ridge–Ridge-Transform endet | Hysterese abgelaufen | Transform endet; vorhandene Fracture Zones bleiben |
| anderer Transform endet | Hysterese abgelaufen | zwei krustengebundene Transformrelikte |
| `fracture_zone[active]` | Quelle/Spreizung endet | `fracture_zone[relict]`, gleiche Identität |
| `collision[waning]` | Hysterese abgelaufen | Kollision endet ohne eigenes Relikt; Orogen wird reliktisch |
| beliebiges `relict` | bestätigte Reaktivierung | neues aktives Feature mit Relikt als Vorgänger |
| krustengebundenes Feature | vollständiger Trägerverbrauch | Identität endet mit Herkunftsnachweis |
| Feature | fachliche räumliche Trennung/Vereinigung | neue Nachfolgeridentitäten |

## 14. Lebenszyklusinvarianten

Jeder gültige Zustand erfüllt zusätzlich zu den bereits bestätigten Oberflächeninvarianten:

1. Jeder Ereigniskandidat liegt außerhalb des Feature-Zustands.
2. Jedes aktuelle Feature hat genau einen Zustand aus `active`, `waning` oder `relict`.
3. Kein Relikt kehrt unter derselben Identität nach `active` zurück.
4. Typwechsel, fachliche Teilung, Vereinigung und Reaktivierung erzeugen neue Identitäten; technische Geometrieänderungen nicht.
5. Während `waning` findet keine typspezifische Flächenproduktion, kein Verbrauch und kein kinematischer Einfluss statt.
6. Jeder aktive Rücken erzeugt ozeanische Kruste; jede ozeanische Akkretion referenziert einen aktiven Rücken.
7. Jeder aktive Graben besitzt unveränderliche Polarität und genau eine Unter- und Oberplattenrolle; nur die Unterplatte wird verbraucht.
8. Kein Graben verbraucht kontinentale Kruste.
9. Jeder Vulkanbogen referenziert genau einen Quellgraben und liegt vollständig auf dessen Oberplatte.
10. Nur Ridge–Ridge-Transformen erzeugen automatisch Fracture Zones; diese sind niemals aktive Plattengrenzen.
11. Jede aktive Kollision besitzt eine Sutur und genau eine aktive oder abklingende Orogenbeziehung; die Kollision selbst hinterlässt kein Relikt.
12. Jede Sutur ist reliktisch und referenziert beteiligte kontinentale Krustenregionen, ozeanische Krustenherkunft und Kollision oder den Seed-Sonderfall.
13. Jedes krustengebundene Feature beziehungsweise jeder seiner Nachfolger besitzt eine eindeutige Trägerbewegung; eine aktive oder abklingende Orogenzone ist stattdessen über ihre Kollisionsachse grenzgebunden.
14. Geometrischer Kontakt allein verändert keine Identität oder Abstammung.
15. Alter allein löscht kein Relikt aus der autoritativen Feature-Map.
16. Start-, Zustandswechsel- und Endzeiten sind monoton; ein Nachfolger beginnt nicht vor dem Übergang seiner Vorgänger.

Ein atomarer Übergang scheitert strukturiert, wenn er eine erforderliche Bindung, Rolle, Provenienz oder eindeutige Trägerbewegung nicht herstellen kann. Der vorherige gültige Zustand bleibt dann unverändert.

## 15. Architekturstatus

Die Lebenszyklusregeln konzentrieren genügend Verhalten, um `FeatureLifecycle` als **Deepening Candidate** für ein künftiges Modul zu benennen. Seine mögliche caller-facing Verantwortung wäre, aus gültigem Snapshot, passendem `KinematicStep`, bestätigten Ereignissen, Zeit und versionierter Konfiguration einen vollständigen deklarativen Lebenszyklus-Übergangsplan oder einen strukturierten Fehler zu liefern. Topologieänderung, Eventauswahl und Persistenz wären Aufrufer beziehungsweise Nachbarn, nicht Teile seiner Implementierung.

Der Kandidat ist ausdrücklich kein akzeptiertes tiefes Modul und seine Schnittstelle keine nachgewiesene Seam. Reale Aufrufer, Implementierung und Schnittstellentests fehlen; den Vertrag bestätigter Ereignisse legen die automatischen Ereignisregeln fest. Es werden weder Adapter noch zusätzliche Ports eingeführt. Die Löschprobe begründet nur den Kandidatenstatus: Ohne eine solche Konzentration würden Zustandsautomaten, Abstammung, Reliktbindung und Provenienz voraussichtlich in Ereignisauflösung, Topologietransaktionen, Persistenz und Tests dupliziert.

## 16. Bewusst vertagte Entscheidungen

Diese Spezifikation entscheidet ausdrücklich nicht:

- dauerhafte Serialisierung, Sichtbarkeitsabfragen und Darstellungsfilter (#11),
- Bevy-Stil, Interaktion oder Mesh-Erzeugung (#12),
- absolute numerische Toleranzen, Ensembleziele und Leistungsbudgets (#13),
- konkrete Datenstrukturen, Algorithmen oder die endgültige Modulschnittstelle.

Damit sind Entstehung, Veränderung, Ende und Erhaltung der tektonischen Kernfeatures festgelegt; die automatische Ereignisentscheidung ist in ihrer nachfolgenden Spezifikation getrennt geregelt, ohne eine Implementierung vorwegzunehmen.
