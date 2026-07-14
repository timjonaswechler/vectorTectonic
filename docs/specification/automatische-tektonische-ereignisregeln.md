# Automatische tektonische Ereignisregeln

Status: gemeinsam bestätigt für das MVP

Wayfinder-Ticket: [#9](https://github.com/timjonaswechler/vectorTectonic/issues/9)

Grundlagen: [fachliches Zustandsmodell](fachliches-zustandsmodell.md), [Geometrie- und Topologierepräsentation](geometrie-und-topologierepraesentation.md), [kinematisches Plattenbewegungsmodell](kinematisches-plattenbewegungsmodell.md), [Lebenszyklen der tektonischen Kernfeatures](lebenszyklen-tektonischer-kernfeatures.md), [Platten- und Superkontinentzyklus](../research/platten-und-superkontinentzyklus.md) und [tektonische Kernfeatures](../research/tektonische-kernfeatures.md)

## Zweck und Abgrenzung

Dieses Dokument legt den geschlossenen Katalog automatischer tektonischer Ereignisse, ihre Vorbedingungen, ihre Konflikte sowie ihre deterministische Auswahl fest. Es bestimmt außerdem die fachliche Face-Zuordnung für bewegte Plattenvorschläge und die Reihenfolge eines vollständigen Ticks.

Die Regeln sind deterministische, oberflächenkinematische Modellannahmen. Sie behaupten weder Spannungsberechnung noch Mantelkonvektion, Rheologie oder eine physikalisch eindeutige Vorhersage. Persistenz und Abfragen (#11), Bevy-Interaktion (#12), numerische Akzeptanzbudgets und Ensemblekalibrierung (#13) sowie konkrete Datenstrukturen und Algorithmen bleiben außerhalb dieses Tickets.

## 1. Ereignisbegriff und geschlossener Katalog

Ein **tektonisches Ereignis** ist ein fachlich atomarer Gesamtübergang zwischen zwei gültigen Oberflächenzuständen. Es umfasst unteilbar sämtliche für seine Gültigkeit erforderlichen Topologie-, Identitäts-, Bewegungsziel- und Featurefolgen. Einzeln auswählbare technische Teiloperationen sind unzulässig.

Das MVP besitzt genau elf Ereignistypen:

| Ereignistyp | Klasse | Fachlicher Gesamtübergang |
|---|---|---|
| `rift_initiation` | auswählbare Initiierung | Beginn eines kontinentalen Rifts |
| `rift_breakup` | zwingende Folge | reifes Rift teilt die Platte und erzeugt Rücken |
| `oceanic_plate_split` | auswählbare Initiierung | schwächengebundene ozeanische Plattenteilung mit Rücken |
| `connectivity_split` | zwingender Abschluss | getrennte Restkomponenten erhalten Nachfolgerplatten |
| `subduction_initiation` | auswählbare Initiierung | Beginn eines Grabens mit fester Polarität |
| `trench_propagation` | auswählbare Initiierung | Fortpflanzung eines Grabens auf einen Nachbarabschnitt |
| `boundary_reclassification` | zwingende Folge | Ende, zulässiger Typwechsel oder Achsensprung eines Grenzfeatures |
| `ridge_trench_encounter` | zwingende Folge | lokale Begegnung und Beendigung von Rücken und Graben |
| `continental_collision` | zwingende Folge | bestätigter Kontinentkontakt erzeugt Kollision, Sutur und Orogen |
| `plate_merger` | zwingende Folge | vollständig verriegelte Kollisionsplatten verschmelzen |
| `complete_plate_consumption` | zwingender Abschluss | letzte ozeanische Fläche einer Unterplatte wird verbraucht |

Ein Übergang ohne Katalogtyp ist unzulässig. Insbesondere gibt es kein generisches Plattenreorganisationsereignis.

Reine Wechsel zwischen `active`, `waning` und `relict`, Bogenreife, Fracture-Zone-Wachstum und typspezifischer Ausklang sind deterministische Lebenszyklusfolgen des Tickzustands oder eines bestätigten Ereignisses. Sie sind keine zusätzlichen tektonischen Ereignistypen.

### 1.1 Zwingende Folgen und auswählbare Initiierungen

Zwingende Folgeereignisse erhalten Invarianten oder stellen eine unausweichliche Folge eines bereits bestätigten Prozesses her. Sie werden nicht gegen Initiierungen bewertet. Auswählbare Initiierungen sind geologisch zulässige, aber nicht erzwungene Prozessbeginne oder Ausbreitungen. Nur sie besitzen Ereigniskandidaten und Fachscores.

`rift_breakup` wird erst zwingend, nachdem das Rift seine Reife- und Gültigkeitsbedingungen erfüllt. Ein nicht erfolgreiches Rift klingt stattdessen gemäß seinem Lebenszyklus zum Relikt ab.

`connectivity_split` und `complete_plate_consumption` sind benannte Abschlussereignisse desselben atomaren Bündels, das Zusammenhangsverlust oder vollständigen Flächenverlust verursacht. Sie konkurrieren nie selbstständig um Auswahl. Ein ungültiger Zwischenzustand mit unverbundener oder flächenloser Platte darf nicht entstehen.

## 2. Versionierte Default-Konfiguration

| Regel | Zulässiger Bereich | Default | Status |
|---|---:|---:|---|
| Riftkorridor | 30–300 km | 100 km | plausible MVP-Abstraktion |
| aktive Rift-Reife | 20–60 Myr | 35 Myr | Modellannahme |
| Rift-Lokalisierung | innerhalb der Rift-Reife | 50 % | Modellannahme |
| Rift-Hysterese | fest für Regelversion | 5 Myr | Modellannahme |
| anfängliche volle Öffnungszielrate | 2–10 mm/a | 5 mm/a | evidenzgestützte Größenordnung |
| Beschleunigung nach Teilung | 3–10 Myr | 6 Myr | kalibrierte Abstraktion |
| volle Spreizungszielrate | 20–80 mm/a | 40 mm/a | evidenzgestützte Größenordnung |
| Subduktions-Mindestlänge | fest für Regelversion | 200 km | Auflösungsannahme |
| Initiationskonvergenz | fest für Regelversion | 5 mm/a über 5 Myr | Modellannahme |
| Grenzfeature-Hysterese | 1–5 Myr | 3 Myr | Modellannahme |
| Bogenreife | 3–10 Myr | 5 Myr | Modellannahme |
| Graben–Bogen-Offset | 100–300 km | 180 km | plausible Oberflächenabstraktion |
| Kürzung an Bogenenden | 50–150 km | 100 km | Modellannahme |
| Bogen-Ausklang | 0–5 Myr | 2 Myr | Modellannahme |
| Kollisionskontakt | mindestens 100 km, 1–3 Myr | 100 km über 2 Myr | Modellannahme |
| Orogen-Anfangsbreite | 100–300 km | 200 km | plausible Darstellungsgröße |
| Orogen-Breitenfaktor | 0,1–0,3 | 0,2 | Modellannahme |
| Orogen-Maximalbreite | fest | 1.000 km | Modellannahme |
| Verschmelzungsverriegelung | fest für Regelversion | <1 mm/a über 5 Myr | Modellannahme |
| Initialisierungs-Timeout | 150–250 Myr | 250 Myr | Modellannahme |

Alle Werte sind Teil einer unveränderlichen, versionierten Laufkonfiguration. Werte außerhalb bestätigter Bereiche werden abgelehnt und niemals still geklemmt. Die Worldbuilding-Pasta-Richtwerte von 10–20 cm/a für subduzierende Ozeanplatten, 5–10 cm/a nach jüngerer Kollision, 2–5 cm/a für Kontinente mit aktivem Rand und unter 1 cm/a für Kontinente mit passivem Rand dienen nur der Plausibilisierung tatsächlicher Plattenbewegung. Sie werden nicht als laterale Grabenausbreitungsgeschwindigkeit interpretiert.

## 3. Kandidaten, Zeitnachweis und stabile Schlüssel

### 3.1 Endliche Kandidatenmenge

Kandidaten entstehen ausschließlich aus kanonischer aktueller Geometrie und fachlicher Provenienz:

- Riftachsen aus endlichen Kombinationen vorhandener Schwächen und zulässiger Randanschlüsse,
- Subduktionskandidaten aus kanonischen konvergenten Grenzsegmenten und ihren Schwächenschnitten,
- Grabenausbreitung nur aus dem unmittelbar benachbarten geeigneten Segment eines Grabenendes,
- ozeanische Teilungen aus reliktischen Rücken, Fracture Zones und Beziehungen instabiler Triple Junctions.

Es gibt keine Zufallsabtastung während der Ereignisentscheidung und keine nachträgliche Geometrieperturbation. Der Welt-Seed beeinflusst Seed-Zustand und anfängliche Kinematik, nicht Ereignisscores oder Gleichstände.

Jeder Kandidat besitzt einen stabilen technischen Schlüssel aus Regelversion, Ereignistyp, Quellidentitäten, Rollen, kanonischem Quellintervall und Geometrieprovenienz. Reine Advektion oder technische Neusegmentierung erhält ihn. Fachliche Teilung, Vereinigung, Rollenwechsel oder Quellenverlust beendet ihn und erzeugt gegebenenfalls neue Kandidaten. Der Schlüssel ist keine fachliche Identität.

### 3.2 Kandidatengedächtnis

Zeitabhängige Vorbedingungen verwenden ein kanonisches Kandidatengedächtnis. Es speichert je fortgeführtem Kandidaten ausschließlich:

- akkumulierte qualifizierende Dauer,
- ersten und letzten Prüfzeitpunkt und
- deterministische Vorbedingungsdiagnosen.

Unterschreitet eine harte Vorbedingung ihre Schwelle, wird die qualifizierende Dauer sofort auf null gesetzt. Initiierungskandidaten besitzen keine Hysterese. Kandidaten bleiben außerhalb des Feature-Zustands und werden nie zu Relikten. Das Gedächtnis ist Teil des reproduzierbaren Laufzustands, aber nicht der finalen fachlichen Feature-Map.

## 4. Riftinitiierung und Break-up

### 4.1 Vorbedingungen für `rift_initiation`

Ein Kandidat ist genau dann zulässig, wenn:

1. seine gesamte Achse in zusammenhängender nichtkratonischer Kontinentalkruste genau einer expliziten Platte liegt,
2. Achse und abgeleiteter Korridor keine Kraton-Zwangskontur schneiden,
3. die probeweise Fortsetzung bis zu Kontinental- beziehungsweise Plattenrand beim späteren Break-up genau zwei flächig zusammenhängende Nachfolgerplatten erzeugt,
4. beide Nachfolger Mindestbreiten, auflösbare Fläche und sämtliche Zustandsinvarianten erfüllen,
5. weder Korridor noch Wirkungsbereich ein aktives oder abklingendes Rift oder eine aktive Kollisionsfront überlagern und
6. mindestens eine zulässige Schwäche berührt wird: Reliktsutur, reliktisches Rift oder nichtkratonischer Verbindungsgürtel zwischen Kratonen.

Aktuelle innerplattige Dehnung ist keine Vorbedingung. Die starre Vorgängerplatte besitzt keine solche Relativkinematik.

### 4.2 Rift-Fachscore

Zulässige Kandidaten werden lexikografisch absteigend verglichen nach:

1. längengewichteter Schwächenunterstützung mit `relict_rift = 3`, `relict_suture = 2`, `noncratonic_connector = 1`, geteilt durch dreifache Gesamtlänge,
2. Teilungsbalance `4 A₁ A₂ / (A₁ + A₂)²` der kontinentalen Nachfolgerflächen,
3. Pfadwirkungsgrad aus kürzester sphärischer Endpunktdistanz geteilt durch tatsächliche Achslänge,
4. kürzerer Achslänge und
5. kanonischem Kandidatenschlüssel.

Es gibt keine gewichtete Mischsumme.

### 4.3 Reife und `rift_breakup`

Ein bestätigtes Rift beginnt `active, incipient`. Nur aktive Zeit erhöht seinen Reifegrad. Bei 50 % der konfigurierten Zielreife wird es unter derselben Identität `localized`.

Bei erreichter Zielreife wird `rift_breakup` zwingend, sofern die probeweise Teilung weiterhin gültig ist. Verliert das Rift zuvor seine topologische Eignung, wechselt es bis zu fünf Myr nach `waning`; Erholung setzt Identität und Reife fort. Nach Ablauf endet es als `relict, abandoned` ohne tektonisches Ereignis.

Der Break-up:

1. beendet das Rift zugunsten seiner Nachfolger,
2. teilt die Vorgängerplatte,
3. setzt die Riftachse als gemeinsame Plattengrenze ein,
4. erzeugt aktive Rücken und
5. setzt die Zielkurve für die Bewegungsvererbung.

Am exakten Break-up-Zeitpunkt entsteht noch keine ozeanische Fläche positiver Größe. Die erste aufgelöste ozeanische Kruste entsteht im folgenden Tick ausschließlich aus tatsächlich integriertem positiven Normalfluss. Dadurch bleiben Flächenbilanz und starre Vorgängerkinematik widerspruchsfrei.

Die volle Zielöffnungsrate beginnt mit 5 mm/a und steigt über sechs Myr linear auf 40 mm/a. Tatsächliche Öffnung folgt ausschließlich aus `PlateKinematics`; die Zielkurve erzwingt keine Geometrie.

## 5. Ozeanische Plattenteilung

### 5.1 Vorbedingungen für `oceanic_plate_split`

Ein Kandidat ist zulässig, wenn:

1. seine Achse vollständig in aufgelöster ozeanischer Kruste genau einer expliziten Platte liegt,
2. sie einer zusammenhängenden Fracture Zone, einem reliktischen Rücken oder einer konjugierten aufgegebenen Rückenstruktur folgt,
3. ihre Enden verschiedene Punkte der bestehenden Plattengrenze erreichen,
4. der probeweise Schnitt genau zwei flächig zusammenhängende, auflösbare Nachfolger erzeugt,
5. keine Seite Mindestbreite oder Mindestfläche unterschreitet,
6. der Wirkungsbereich keine aktiven oder abklingenden Gräben, Rücken oder Kollisionen überlagert und
7. die probeweise Bewegungsvererbung mit der bestehenden reinen `PlateKinematics`-Reconciliation eine positive Öffnung erreichen kann.

Eine **instabile Triple Junction** liegt für diese Regel genau dann vor, wenn drei explizite ozeanische Plattengrenzen an einem kanonischen Junction-Vertex zusammentreffen, ihre tatsächlichen Tickrotationen ohne Teilung probeweise eine mindestens `W_min` breite, durch bestehende Rückenflüsse nicht zuordenbare Lücke am Vertex erzeugen und der Teilungskandidat diese Lücke unter allen Invarianten schließt. Dieser binäre, aus derselben atomaren Probe abgeleitete Nachweis darf den Kandidaten begünstigen, erzeugt aber niemals eine vorgängerlose Platte. Neue Platten besitzen stets eindeutige Vorgängerprovenienz.

Die Teilung erzeugt Grenze und aktive Rücken; positive neue Ozeanfläche entsteht erst aus dem folgenden tatsächlichen Öffnungsfluss. Sie verwendet dieselbe Öffnungszielkurve wie der kontinentale Break-up.

### 5.2 Fachscore

Kandidaten werden lexikografisch absteigend verglichen nach:

1. längengewichtetem Anteil auf reliktischem Rücken,
2. Anteil auf Fracture Zones,
3. binärem Nachweis einer instabilen Triple-Junction-Beziehung,
4. erreichbarer positiver Öffnungsrate der probeweisen Bewegungsvererbung,
5. Flächenbalance der Nachfolger,
6. kürzerer gültiger Achse und
7. kanonischem Kandidatenschlüssel.

Die Öffnungsrate wird durch dieselbe reine Reconciliation gegen den vollständig validierten Probe-Nachfolger bestimmt, die nach einem tatsächlichen Commit den Bewegungszustand erzeugt. Die Ereignisauflösung besitzt dafür keinen eigenen Kinematiksolver; reine Probeauswertungen dürfen beliebig oft erfolgen, werden weder veröffentlicht noch als Commit behandelt und zählen nicht als die genau einmalige kanonische Nach-Commit-Reconciliation des Ticks.

## 6. Subduktionsinitiierung, Polarität und Grabenausbreitung

### 6.1 Vorbedingungen für `subduction_initiation`

Ein Kandidat ist zulässig, wenn:

1. ein zusammenhängender kinematisch konvergenter Grenzabschnitt mindestens 200 km lang ist,
2. seine längengewichtete normale Konvergenz mindestens 5 mm/a während fünf Myr ohne Unterbrechung beträgt,
3. mindestens eine geometrisch anliegende Seite ozeanische Kruste trägt,
4. der Abschnitt eine Fracture Zone, einen abgeleiteten passiven Kontinentrand oder eine Reliktsutur berührt,
5. er keinen aktiven oder abklingenden Graben, Rücken, Transformconnector oder Kollisionsabschnitt überlagert,
6. eine eindeutige zulässige Polarität bestimmt werden kann und
7. probeweiser Verbrauch ausschließlich ozeanische Unterplattenkruste entfernt, ohne Kraton-, Kontinental- oder Mindestbreiteninvarianten zu verletzen.

Während der Initialisierung darf die ozeanische Unterseite zum nicht aufgelösten Hintergrund gehören. Alter ist nur Scorefaktor und niemals Vorbedingung.

### 6.2 Polarität

Die Polarität wird ohne Seed-Zufall nach fester Rangfolge bestimmt:

1. Nicht aufgelöste alte Ozeankruste des Hintergrunds ist Unterseite.
2. Bei Ozean–Kontinent-Konvergenz ist die ozeanische Seite Unterplatte.
3. Bei Ozean–Ozean-Konvergenz bleiben nur Polaritäten mit vollständig ozeanischer Unterseite zulässig.
4. Danach gewinnt eine auf der vorgesehenen Unterseite liegende geerbte Schwäche.
5. Danach gewinnt das höhere längengewichtete Krustenalter; unbekannte Hintergrundkruste verwendet nur für diese Bewertung 80 Ma.
6. Danach entscheidet der kleinere kanonische Schlüssel aus Plattengebiets-ID und orientierungsunabhängiger Segmentgeometrie.

Die bestätigte Polarität bleibt für die gesamte Grabenidentität unveränderlich.

### 6.3 Subduktions-Fachscore

Innerhalb derselben Prioritätsklasse werden Kandidaten lexikografisch absteigend verglichen nach:

1. Anteil der Kandidatenlänge auf mindestens einer zulässigen Schwäche,
2. mittlerer normaler Konvergenz, auf 5–150 mm/a normalisiert,
3. Unterplatten-Altersfaktor `0` unter 40 Ma, linear `0–1` zwischen 40 und 80 Ma und `1` ab 80 Ma,
4. zusammenhängender Kandidatenlänge und
5. kanonischem Kandidatenschlüssel.

Fracture Zone, passiver Rand und Reliktsutur besitzen untereinander keinen unbelegten Qualitätsrang.

### 6.4 Qualifikationsakkommodation

Während des fünf Myr langen Zeitnachweises wird ein konvergenter featureloser Abschnitt flächenneutral verriegelt, sofern bereits alle Initiationsbedingungen außer der Dauer erfüllt sind. Die vorhergesagte tatsächliche Konvergenz zählt für das Kandidatengedächtnis, aber es wird noch keine Kruste verbraucht oder einer Seite zugeschlagen.

Verliert der Kandidat seine Voraussetzungen, endet die Akkommodation. Featurelose Konvergenz ohne qualifizierenden Subduktionskandidaten, Kollision oder andere zulässige Auflösung ist ein strukturierter Tickfehler. Die konkrete Geometriekonstruktion bleibt Implementierungsdetail; sie muss Flächen, Kratone, Zwangskonturen und lückenlose Abdeckung bewahren.

### 6.5 `trench_propagation`

Ein Kandidat:

1. beginnt genau an einem Ende eines aktiven Grabens,
2. umfasst ausschließlich den unmittelbar benachbarten geeigneten kinematischen Grenzabschnitt,
3. behält Unterseite, Oberseite und Polarität unverändert,
4. weist dort mindestens 5 mm/a normale Konvergenz während fünf Myr auf,
5. besitzt auf der Unterseite durchgehend ozeanische Kruste und
6. wird durch keine Kollision, Rückenbegegnung oder konkurrierendes aktives Grenzfeature blockiert.

Das Grabenende selbst ist die erforderliche Schwäche. Es gibt weder ein künstliches Längenfenster noch eine unabhängige laterale Ausbreitungsgeschwindigkeit.

Kandidaten werden lexikografisch absteigend verglichen nach:

1. kleinerem Richtungswechsel am Grabenende,
2. höherer normaler Konvergenz,
3. aktivem Reifeanteil des Quellgrabens bis zur Slab-Pull-Reifezeit,
4. Unterplattenalter nach der 40/80-Ma-Funktion,
5. Länge des benachbarten geeigneten Abschnitts und
6. kanonischem Kandidatenschlüssel.

## 7. Grenztypwechsel

`boundary_reclassification` wird nach drei Myr verlorener Aktivitätsbedingung zwingend. Erholt sich die Bedingung innerhalb der Hysterese, kehrt dieselbe Identität von `waning` nach `active` zurück.

Die geschlossene Übergangsmatrix erlaubt:

- `kein Feature → spreading_ridge`, im selben Tick, in dem eine ozeanische oder unmittelbar aus Break-up stammende Grenze positiven Öffnungsfluss besitzt und Akkretion möglich ist,
- `kein Feature → transform_fault`, nach drei Myr tangential dominierter Bewegung, sofern zulässige Endfeatures verbunden werden oder eine zusammenhängende kontinentale Transform-Hauptlinie besteht,
- `spreading_ridge → transform_fault`, wenn die Transformvorbedingungen erfüllt sind,
- `transform_fault → spreading_ridge`, bei anhaltender Divergenz und möglicher ozeanischer Akkretion,
- `spreading_ridge → spreading_ridge` als Achsensprung mit neuer Identität und Entstehungsart `ridge_relocation`,
- `spreading_ridge | transform_fault | trench | collision → kein aktives Grenzfeature`, wenn kein zulässiger Nachfolgertyp besteht.

Ein konvergenter Rücken oder Transformabschnitt wird niemals automatisch zum Graben. `trench → collision` erfolgt ausschließlich über `continental_collision`; Rücken–Graben-Interaktionen ausschließlich über `ridge_trench_encounter`. Ein Polaritätswechsel beendet den alten Graben und benötigt später eine neue `subduction_initiation`.

Rücken entstehen ohne Initiierungsscore, weil positiver Öffnungsfluss an einer expliziten ozeanischen Plattengrenze sonst eine Lücke erzeugen würde. Transformstörungen erzeugen und verbrauchen keine Fläche und liefern keinen kinematischen Einfluss.

Ein Achsensprung ist nur zulässig, wenn die bestehende Achse keine gültige Akkretion mehr herstellen kann. Eine probeweise neue Achse muss positiven Öffnungsfluss, gültige ozeanische Krustenproduktion und dieselben angrenzenden Platten ermöglichen. Die alte Rückenidentität endet mit zwei krustengebundenen Reliktnachfolgern; die neue Achse erhält eine neue aktive Rückenidentität. Mehrere Ersatzachsen werden nach höherer erreichbarer Öffnung, kürzerer Achse und kanonischem Schlüssel gewählt. Existiert eine gültige Ersatzachse, ist der Sprung zwingend; andernfalls endet der Rücken nach seiner Hysterese.

Vulkanbögen folgen dem Quellgraben durch dessen Hysterese und klingen nach seinem endgültigen Ende zwei Myr aus. Fracture Zones folgen ohne eigenen Timer Quelle und Spreizung.

## 8. Rücken–Graben-Begegnung

`ridge_trench_encounter` wird zum frühesten Zeitpunkt innerhalb des Ticks zwingend, an dem der bestätigte Grabenfluss einen aktiven Rücken erreicht.

Der Übergang:

1. beendet den tatsächlich zusammentreffenden Rückenabschnitt,
2. erzeugt bei räumlicher Trennung Nachfolger für verbleibende Rückenstücke,
3. beendet den betroffenen Grabenabschnitt und erzeugt dessen Oberplattenrelikt und
4. erhält benachbarte Grabenabschnitte nur, wenn dieselbe Unterplatte, Polarität und anliegende ozeanische Kruste fortbestehen.

Subduktion springt nicht automatisch über den Rücken. Slab Window, Polaritätswechsel, Back-Arc-Spreading und spontaner Grabenversatz liegen außerhalb des MVPs. Jenseits des ehemaligen Rückens ist eine neue `subduction_initiation` erforderlich.

## 9. Kontinentkollision und Plattenverschmelzung

### 9.1 `continental_collision`

Sobald der nächste Grabenverbrauch kontinentale Kruste berühren würde, pausieren lokaler Verbrauch und Slab Pull sofort. Ein zusammenhängender Kontakt von mindestens 100 km wechselt zu `collision_pending`. Bleibt er zwei Myr bestehen und behält mehr als 1 mm/a längengewichtete Konvergenz, wird `continental_collision` zwingend.

Das Ereignis beendet den Grabenabschnitt und erzeugt atomar aktive Kollision, reliktische Sutur und aktives Orogen. Es verbraucht keine kontinentale Fläche. Die Regel gilt unverändert für Kratonankunft.

Löst sich der Kontakt vor zwei Myr, darf derselbe Graben reaktiviert werden, sofern wieder ozeanische Unterplattenkruste anliegt. Bleibt ein nicht auflösbarer Kontakt unter 100 km bestehen, endet der lokale Graben nach drei Myr durch Grenzreklassifikation, statt eine Mikrokollision zu erfinden oder Kontinent zu verbrauchen.

### 9.2 Flächenneutrale Kollisionsakkommodation

Während aktiver Kollision erhöht tatsächliche normale Konvergenz weiterhin `shortening_proxy` und Orogenbreite. Sie überträgt oder löscht jedoch keine kontinentale Face. Der kollidierende Rand wird innerhalb der Orogenzone deterministisch deformiert, sodass beide kontinentalen Krustenflächen, sämtliche Kratone und die globale Abdeckung erhalten bleiben. Tangentialbewegung darf Kontakt und Form verändern.

Diese lokale, flächenneutrale Deformation ist eine ausdrückliche Ausnahme von rein starrer Krustenadvektion und bleibt auf die Orogenzone beschränkt. Kann keine gültige flächen- und zwangskonturerhaltende Geometrie hergestellt werden, scheitert der Tick strukturiert.

### 9.3 `plate_merger`

Eine Verschmelzung wird zwingend, wenn:

1. genau zwei explizite Platten eine Grenze teilen, deren gesamte Länge aus aktiver oder abklingender Kontinentkollision besteht,
2. zwischen demselben Paar kein Rücken-, Graben- oder Transformabschnitt besteht,
3. die relative Gesamtgeschwindigkeit entlang der gesamten Grenze fünf Myr unter 1 mm/a bleibt,
4. die vereinigte Fläche kantenzusammenhängend und gültig ist und
5. kein Feature weiterhin zwei verschiedene Plattenrollen benötigt.

Die Kollision endet, die alte Grenze wird intraplattenintern, die Sutur bleibt erhalten und das Orogen wird krustengebunden reliktisch. Beide Vorgängerplatten enden zugunsten einer neuen Nachfolgeridentität. Bewegungsgleiche Ozeanplatten und nur teilweise kollidierende Platten verschmelzen nicht automatisch.

## 10. Zusammenhangstrennung und vollständiger Verbrauch

### 10.1 `connectivity_split`

Verliert eine Platte durch ein anderes Ereignis ihren Zusammenhang über Kanten positiver Länge, endet sie im selben Ereignisbündel. Jede verbleibende flächige, auflösbare Komponente erhält eine Nachfolgerplatte. Punktberührung verbindet keine Komponenten. Identitäten, Krusten-Tragbeziehungen, Features und Bewegungen folgen den bestätigten Abstammungsregeln.

### 10.2 `complete_plate_consumption`

Vollständiger Verbrauch ist nur zulässig, wenn:

1. die Platte ausschließlich ozeanische Kruste trägt,
2. jede entfernte Face genau einem aktiven Graben mit dieser Platte als Unterplatte zugeordnet ist,
3. die integrierten negativen Normalflüsse des Bündels sämtliche verbleibenden Faces tatsächlich abdecken und
4. keine Fläche lediglich durch Snapping oder Unterschreiten von `W_min` verschwindet.

Die Plattenidentität endet ohne Nachfolger. Sämtliche vollständig mitverbrauchte krustengebundene Features enden mit Herkunftsnachweis. Frei werdende Oberflächenanteile gehen lokal an die jeweilige Oberplatte. Treffen mehrere Oberplatten aufeinander, entsteht zwischen ihnen eine neue abgeleitete Plattengrenze; daraus folgt kein generisches Reorganisationsereignis.

Ein unzureichend durch Fluss gedeckter, aber kollabierender Rest erzeugt eine strukturierte Kollapsdiagnose und verhindert den Commit.

## 11. Hintergrund während der Initialisierung

Der nicht aufgelöste Hintergrund ist während der Initialisierung ein flächenbilanzierter Residualträger:

1. Er füllt jede verbleibende Fläche, die weder von einer expliziten Platte noch zulässiger Rückenproduktion beansprucht wird.
2. An konvergenten Rändern verliert er gleichzeitig die von der Oberplatte überdeckte Fläche.
3. Eine reine Rotation einer expliziten Platte tauscht gleich große führende und nachlaufende Hintergrundflächen aus und erzeugt netto keine neue nicht aufgelöste Kruste.
4. Rückenproduktion ersetzt Hintergrundfläche dauerhaft durch aufgelöste ozeanische Kruste expliziter Platten.
5. Hintergrundkomponenten dürfen sich teilen und vereinigen, bleiben aber ohne Plattenidentität oder Eigenbewegung.
6. Die Hintergrundfläche darf pro Tick nur um bestätigte explizite Überdeckung und Rückenproduktion abnehmen, niemals zunehmen.

Nach Ende der Initialisierung existiert dieser Residualfall nicht mehr.

### 11.1 Auflösungsperspektive

Nach jedem Tick muss für jede zusammenhängende Hintergrundkomponente mindestens eine nachweisbare Auflösungsperspektive bestehen:

- Kontakt zu einem aktiven konsumierenden Graben,
- Kontakt zu einem fortschreitend qualifizierenden Subduktionskandidaten,
- Erreichbarkeit eines solchen Grabens oder Kandidaten durch dieselbe zusammenhängende Hintergrundfläche oder
- eine aktive Rift-/Rückenentwicklung, deren probeweise Geometrie die Komponente in Reichweite einer solchen Verbrauchsfront bringt.

Ein auswählbares Ereignis, das die letzte Perspektive beseitigt, ist unzulässig. Erzeugt ein zwingendes Ereignis eine perspektivlose Komponente und lässt sich im selben Bündel keine zulässige Folge herstellen, wird der Lauf sofort ungültig. Das Initialisierungs-Timeout bleibt zusätzliche Sicherheitsgrenze.

## 12. Fachliche Face-Zuordnung

Nach Bewegung und Ereignisauswahl gilt ein geschlossener Satz deklarativer Zuordnungsregeln:

1. **Genau ein Anspruch:** bestehende Plattengebiets- und Krustenreferenz bleibt erhalten.
2. **Lücke an aktivem Rücken:** ausschließlich dort entsteht aufgelöste ozeanische Kruste; der tatsächliche integrierte Öffnungsfluss wird symmetrisch den angrenzenden Platten zugeordnet.
3. **Überlappung am aktiven Graben:** Die Oberplatte gewinnt die Oberflächenbelegung; ausschließlich geometrisch anliegende ozeanische Unterplattenkruste wird entsprechend dem tatsächlichen Schließungsfluss verbraucht.
4. **Kontinentkollision:** Es gilt die flächenneutrale Kollisionsakkommodation.
5. **Qualifizierende featurelose Konvergenz:** Es gilt die flächenneutrale Qualifikationsakkommodation.
6. **Transformabschnitt:** Die gemeinsame Grenze wird flächenneutral aus den tangentialen Vorschlägen rekonstruiert.
7. **Nicht aufgelöster Hintergrund:** Es gilt ausschließlich die Residualregel der Initialisierungsphase.

Jede andere Lücke, Mehrfachbeanspruchung oder nicht eindeutige Face verhindert den Commit. Nähe, Flächengröße, Eingabereihenfolge, Engine-Winding und technische Handle-Reihenfolge sind niemals Eigentumsregeln.

## 13. Wirkungsbereiche und Konflikte

Jedes Ereignis deklariert vor Auswahl:

- gelesene Platten, Krustenregionen, Features und Grenzsegmente,
- exklusiv veränderte Faces, Kanten, Identitäten und Rollen,
- erzeugte und beendete Entitäten,
- verlangte kinematische Zielraten und
- eine vollständig probeweise validierte Nachfolgegeometrie.

Zwei Ereignisse stehen in Konflikt, wenn:

1. sich ihre exklusiven Änderungen überschneiden,
2. eines eine Vorbedingung des anderen ungültig macht,
3. sie derselben Identität widersprüchliche Nachfolger geben,
4. sie unvereinbare Rollen oder Zielraten setzen oder
5. ihre gemeinsame Probe keinen gültigen atomaren Snapshot ergibt.

Bloße Endpunktberührung ist kein Konflikt, sofern die gemeinsame Probe gültig bleibt.

### 13.1 Konflikte zwingender Ereignisse

Zwingende Ereignisse werden zuerst nach ihrem frühesten nachweisbaren Kontakt- oder Schwellenzeitpunkt innerhalb des Ticks geordnet. Zeitgleiche kompatible Ereignisse werden gemeinsam gebündelt. Bei zeitgleichem Konflikt gilt folgende Schutzpriorität:

1. `continental_collision`,
2. `ridge_trench_encounter`,
3. `complete_plate_consumption`,
4. `connectivity_split`,
5. `plate_merger`,
6. `boundary_reclassification`.

Danach werden verursachte Abschlussereignisse bis zu einem geschlossenen gültigen Bündel ergänzt. Verdrängte zwingende Ereignisse werden mit Gewinner, Konfliktgrund und neu zu prüfender Nachfolgebedingung diagnostiziert.

## 14. Auswahl auswählbarer Ereignisse

### 14.1 Typpriorität

Während der Initialisierung gilt:

1. `trench_propagation`, wenn dadurch nicht aufgelöste Kruste erreichbar wird,
2. `subduction_initiation` mit nicht aufgelöster Unterseite,
3. `rift_initiation`,
4. sonstige `trench_propagation`,
5. sonstige `subduction_initiation`,
6. `oceanic_plate_split`.

Während der Reifephase gilt:

1. `trench_propagation`,
2. `subduction_initiation`,
3. `rift_initiation`,
4. `oceanic_plate_split`.

Fachlich verschiedene Scores werden nicht zu einer globalen Mischzahl verrechnet. Innerhalb desselben Typs entscheiden Fachscore und anschließend Kandidatenschlüssel.

### 14.2 Greedy maximaler konfliktfreier Satz

Nach Abschluss des zwingenden Bündels werden alle zulässigen Kandidaten nach Typpriorität, Fachscore absteigend und kanonischem Schlüssel sortiert. Jeder Kandidat wird genau einmal in dieser Reihenfolge probeweise hinzugefügt. Er wird angenommen, wenn er mit allen bereits angenommenen Ereignissen konfliktfrei ist und die gemeinsame atomare Probe gültig bleibt; andernfalls wird er mit Konfliktgrund abgelehnt.

Es gibt kein pauschales globales Ereignislimit. Räumlich und fachlich unabhängige Initiierungen dürfen im selben Tick eintreten.

## 15. Tickreihenfolge

Jeder Tick verwendet genau folgende Reihenfolge:

1. `SurfaceSnapshot`, `KinematicState`, Feature-Zustand und Kandidatengedächtnis auf gemeinsamen Zeitpunkt und Gültigkeit prüfen.
2. `PlateKinematics.prepare_step` genau einmal ausführen.
3. Zwingende Ereignisse samt Zeitpunkten erkennen, Konflikte entscheiden und Abschlussereignisse schließen.
4. Kandidatengedächtnis mit tatsächlich beobachteter Kinematik aktualisieren.
5. Initiierungskandidaten erzeugen, bewerten und den maximalen konfliktfreien Satz auswählen.
6. `FeatureLifecycle` erzeugt aus bestätigten Ereignissen und Tickbedingungen den vollständigen Lebenszyklusplan.
7. Bewegung, Flüsse, Face-Zuordnung, Ereignisse und Lebenszyklusfolgen in einer atomaren Overlay-Transaktion anwenden.
8. Den vollständigen Nachfolge-Snapshot validieren.
9. `PlateKinematics.reconcile_after_commit` genau einmal als kanonische Nach-Commit-Reconciliation für sämtliche Abstammungen und Zielraten ausführen.
10. Erst danach den neuen gültigen Gesamtzustand veröffentlichen.

Kinematik wird innerhalb desselben Ticks nicht auf neu entstandene Features rückgekoppelt. Halbfertige Ergebnisse verlassen keine Schnittstelle.

## 16. Fehlervertrag und deterministischer Rückzug

Scheitert eine gemeinsame Transaktion unter Beteiligung auswählbarer Initiierungen, wird der rangniedrigste am strukturierten Fehler beteiligte Initiierungskandidat entfernt. Auswahl und gemeinsame Probe werden mit unveränderten Eingaben erneut ausgeführt. Das wiederholt sich, bis die Transaktion gelingt oder nur das zwingende Bündel verbleibt.

Jede Rücknahme wird mit Kandidat, Rang und Fehler diagnostiziert. Sie erzeugt weder Feature noch Relikt. Scheitert das zwingende Bündel allein, ist der Tick strukturell fehlgeschlagen und der Lauf ungültig.

Verboten sind insbesondere:

- Toleranzerhöhung oder Schwellenlockerung,
- Geometrieperturbation,
- zufällige Wiederholung,
- stilles Löschen kollabierter Details,
- Überspringen zwingender Ereignisse und
- Umdeutung in eine generische Reorganisation.

## 17. Determinismus und Diagnosen

Regelversion, Konfiguration und Kandidatengedächtnis gehören zum kanonischen Laufzustand und Zustands-Hash. Jede bestätigte Ereignisinstanz erhält eine deterministische ID aus:

- Vorgängerzustands-Hash,
- Simulationszeit,
- Ereignistyp,
- Kandidatenschlüssel beziehungsweise zwingender Ursache und
- kanonischer Bündelposition.

Abschlussereignisse werden nach ihrem verursachenden Hauptereignis kanonisch sortiert. HashMap-, Traversierungs-, Parallelisierungs-, Einfügungs- und Probeausführungsreihenfolge dürfen Auswahl, IDs oder Ergebnis nicht beeinflussen. Änderungen der Semantik erfordern eine neue Regelversion.

Jeder Tick diagnostiziert mindestens:

- sämtliche geprüften Vorbedingungen und Zeitnachweise,
- Fachscore-Komponenten,
- Typpriorität und kanonische Sortierschlüssel,
- Wirkungsbereiche und Konfliktkanten,
- angenommene, abgelehnte und zurückgenommene Kandidaten,
- zwingende Ursachen, Zeitpunkte und Verdrängungen,
- Face-Zuordnungsgrund je veränderter Face,
- Abschlussereignisse und Abstammungen sowie
- strukturierte Probe-, Kollaps- und Commitfehler.

Diese Diagnosen sind kein Bestandteil der fachlichen Feature-Map.

## 18. Architekturstatus

`TectonicEventResolution` ist ein **Deepening Candidate** für ein künftiges Modul. Seine mögliche caller-facing Operation ist:

### `decide_tick(...) → EventDecision`

Eingaben:

- gültiger unveränderlicher `SurfaceSnapshot`,
- passender unveränderlicher `KinematicStep`,
- aktuelle Feature-Zustände und Beziehungen,
- Kandidatengedächtnis,
- Simulationszeit und `Δt` sowie
- unveränderliche versionierte Ereigniskonfiguration.

Vollständiges Ergebnis:

- kanonisch geordnetes zwingendes Ereignisbündel,
- ausgewählter konfliktfreier Initiierungssatz,
- deklarative Wirkungsbereiche, Face-Regeln und Zielraten,
- fortgeschriebenes Kandidatengedächtnis,
- vollständige Auswahl-, Konflikt- und Ablehnungsdiagnosen

oder ein strukturierter Fehler.

`FeatureLifecycle` bleibt für Lebenszyklus- und Provenienzpläne verantwortlich. `SurfaceSnapshot` bleibt für atomare Geometrieänderung und Validierung verantwortlich. `PlateKinematics` bleibt für Bewegung, Flüsse und Abstammungsreconciliation verantwortlich. Es werden keine zusätzlichen Adapter oder Ports eingeführt.

Der Kandidat ist kein akzeptiertes tiefes Modul und seine Schnittstelle keine bewiesene Seam. Implementierung, reale Caller und Schnittstellentests fehlen. Die Löschprobe begründet nur den Kandidatenstatus: Ohne diese Konzentration würden Vorbedingungen, Scores, Konfliktauflösung, Prioritäten und Diagnosen in Simulationsablauf, Feature-Lebenszyklus, Topologietransaktion und Tests dupliziert.

## 19. Ereignisinvarianten

Jeder erfolgreiche Tick erfüllt zusätzlich zu allen bestehenden Zustands-, Geometrie-, Kinematik- und Lebenszyklusinvarianten:

1. Jedes tektonische Ereignis gehört genau einem Typ des geschlossenen Katalogs an.
2. Jede Ereignisinstanz ist ein atomarer Gesamtübergang; technische Teiloperationen sind nicht einzeln auswählbar.
3. Kein auswählbares Ereignis verdrängt eine kompatible zwingende Folge höherer Schutzpriorität.
4. Jede Initiierung erfüllt sämtliche harten Vorbedingungen und ihren Zeitnachweis.
5. Scores entscheiden nur zwischen bereits zulässigen Kandidaten desselben Typs.
6. Alter allein initiiert keine Subduktion.
7. Kein Rift schneidet einen Kraton; keine Teilung hinterlässt punktverbundene oder unauflösbare Platten.
8. Ozeanische Kruste entsteht nur aus tatsächlichem positiven Rückenfluss.
9. Ozeanische Kruste wird nur an einem aktiven Graben der geometrisch anliegenden Unterplatte verbraucht.
10. Kontinentale Fläche und Kratone werden weder verbraucht noch durch Face-Zuordnung übertragen.
11. Transformbewegung erzeugt und verbraucht exakt keine Fläche.
12. Vollständiger Plattenverbrauch folgt tatsächlichem Fluss, niemals Snapping.
13. Hintergrundfläche nimmt während der Initialisierung niemals zu und besitzt nach deren Ende exakt Fläche null.
14. Jede verbleibende Hintergrundkomponente besitzt eine Auflösungsperspektive.
15. Ereignisauswahl, IDs, Abstammungen und kanonischer Ergebniszustand sind bei gleichen Eingaben identisch.
16. Ein strukturierter Fehler lässt den letzten gültigen Zustand unverändert.

## 20. Bewusst vertagte Entscheidungen

Diese Spezifikation entscheidet ausdrücklich nicht:

- konkrete interne Datenstrukturen, Solver, Overlay- oder Akkommodationsalgorithmen,
- dauerhafte Serialisierung von Kandidatengedächtnis, Ereignissen oder Diagnosen (#11),
- Bevy-Darstellung und Interaktion (#12),
- numerische Residuen, Flächenfehler, Konditions-, Laufzeit- und Speicherbudgets (#13),
- ensemblebasierte Zielverteilungen und endgültige Kalibrierung der Modellannahmen (#13),
- Slab Windows, Slabbruch, Back-Arc-Spreading, Trench Rollback, spontane Polaritätswechsel oder Mantelprozesse und
- zusätzliche Feature- oder Ereignistypen jenseits des geschlossenen MVP-Katalogs.

Damit sind zulässige automatische Ereignisse, ihre Vorbedingungen, Konfliktauflösung und deterministische Auswahl spezifiziert, ohne ihre Implementierung vorwegzunehmen.
