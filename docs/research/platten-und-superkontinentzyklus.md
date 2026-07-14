# Research: Platten- und Superkontinentzyklus wissenschaftlich fundieren

## Zusammenfassung

Für das MVP ist ein kinematisches, flächendeckendes Plattenmodell wissenschaftlich vertretbar: starre sphärische Platten bewegen sich über Euler-Rotationen; an divergenten Grenzen entsteht Kruste, an konvergenten Grenzen wird sie entsprechend der relativen Normalbewegung entfernt. Rift-, Subduktions- und Kollisionsereignisse müssen regelbasiert ausgelöst werden, dürfen aber nicht als Ergebnis berechneter Mantelkonvektion oder Spannungen ausgegeben werden. Ein Lauf von 200 Myr kann große Ozeane erzeugen und schließen sowie praktisch den gesamten Ozeanboden umsetzen, garantiert jedoch weder den vollständigen Ersatz jedes Restbeckens noch einen neuen Superkontinenten.

## Abgrenzung und Evidenzsprache

Dieses Dokument trennt:

- **Befund**: Beobachtung oder robuste kinematische Folgerung aus hochrangiger Literatur.
- **Plausible Abstraktion**: vereinfachte Regel, die Befunde respektiert, aber nicht eindeutig aus ihnen folgt.
- **Modellannahme**: bewusst gesetzte Entscheidung ohne Anspruch auf naturgesetzliche oder konsensuale Gültigkeit.

Nicht simuliert werden Mantelkonvektion, quantitative Spannungsfelder, Rheologie, Isostasie, thermische Entwicklung oder Schmelzbildung. Entsprechend sind Ereignis-Trigger keine physikalischen Vorhersagen.

## Findings

### 1. Repräsentation und globale Invarianten

1. **Plattenkinematik auf der Kugel ist die richtige Basisebene.** Starre Platten lassen sich durch Euler-Pole und Winkelgeschwindigkeiten bewegen; relative Bewegung an einer Grenzlinie liefert divergent, konvergent oder transform. Das ist die klassische sphärische Formulierung der Plattentektonik. [McKenzie & Parker 1967](https://doi.org/10.1038/2161276a0), [Morgan 1968](https://doi.org/10.1029/JB073i006p01959)
2. **Starre Innenbereiche und schmale Grenzen sind nützliche, aber nicht universell wahre Näherungen.** Reale diffuse Grenzen können über 1.000 km breit sein und etwa 15 % der Erdoberfläche umfassen. Im MVP sind Liniengrenzen daher eine explizite Auflösungsabstraktion. [Gordon 1998](https://doi.org/10.1146/annurev.earth.26.1.615)
3. **Implementierbare Invarianten:** Zu jedem Zeitschritt bilden nicht überlappende sphärische Polygone eine vollständige Partition der Kugel; jedes Polygon besitzt genau eine `plate_id`, `crust_kind` (`continental`, `oceanic_resolved`, `oceanic_unresolved`) und ein Alter beziehungsweise `unknown`; jede Grenze besitzt Typ, Orientierung und bei Subduktion Polarität. Neue Fläche an Rücken und entfernte Fläche an Gräben werden ausschließlich aus der relativen Normalgeschwindigkeit bestimmt. Das ist eine kinematische Bilanzregel, keine Geodynamiksimulation.

### 2. Superkontinentzerfall und Rifting

1. **Rifte lokalisieren bevorzugt strukturelle Schwächezonen, ihr Erfolg ist aber nicht deterministisch aus einer Einzelursache ableitbar.** Vererbte Suturen und Scherzonen beeinflussen die Lokalisierung; manche Rifte scheitern. Plumes, Fernkräfte, Lithosphärenstruktur und Oberflächenprozesse wirken zusammen, und die Kontrolle erfolgreichen gegenüber gescheitertem Rifting bleibt umstritten. [Brune et al. 2023](https://doi.org/10.1038/s43017-023-00391-3), [Buiter & Torsvik 2014](https://doi.org/10.1016/j.gr.2014.02.007)
2. **Riftentwicklung ist mehrphasig und diachron.** Globale Rekonstruktionen zeigen typischerweise langsame anfängliche Extension und eine abrupte Beschleunigung vor dem Break-up. Beim Südatlantik blieb die volle Divergenzrate in den ersten etwa 25 Myr unter 10 mm/a und stieg dann innerhalb von rund 6 Myr auf über 35 mm/a; das ist ein Fallbeispiel, keine universelle Uhr. [Brune et al. 2016](https://doi.org/10.1038/nature18319), [Ulvrová et al. 2019](https://doi.org/10.1029/2018GL080387)
3. **Pangaea zerfiel nicht in einem einzigen radialen Ereignis**, sondern in einer Folge regional unterschiedlicher Rift- und Driftphasen. Ein synchroner „Superkontinent platzt“-Trigger wäre daher irreführend. [de Lamotte et al. 2015](https://doi.org/10.1002/2014TC003760), [Müller et al. 2016](https://doi.org/10.1146/annurev-earth-060115-012211)
4. **Plausible Abstraktion:** Erzeuge mehrere kandidatengestützte Riftlinien in nicht-kratonischer kontinentaler Kruste. Gewichte alte Suturen/mobile Gürtel stark, bevorzuge zusammenhängende lange Pfade und verhindere Durchschneiden eines Kratonpolygons. Lasse nur einen Teil nach 20–60 Myr zu durchgehenden Plattengrenzen werden; übrige Pfade werden als gescheiterte Rifte markiert. Der Bereich ist eine MVP-Kalibrierung an „mehreren bis vielen Zehnermillionen Jahren“, kein Naturgrenzwert.
5. **Empfohlene Riftkinematik:** volle Öffnungsrate zunächst 2–10 mm/a, bei erfolgreichem Break-up Übergang über 3–10 Myr auf 20–60 mm/a. Höhere Werte bleiben möglich, sollten aber selten sein. Diese Parameter bilden beobachtete langsame Frühphasen und den Südatlantik-Fall ab, ohne ihn zur Universalregel zu machen. [Brune et al. 2016](https://doi.org/10.1038/nature18319)

### 3. Ozeanische Krustenneubildung

1. **An mittelozeanischen Rücken entsteht beidseits neue ozeanische Lithosphäre; magnetische Isochronen belegen die zeitlich geordnete Spreizung.** [Vine & Matthews 1963](https://doi.org/10.1038/199947a0), [Seton et al. 2020](https://doi.org/10.1029/2020GC009214)
2. **Heutige volle Spreizungsraten decken eine große Bandbreite ab.** Der globale Datensatz verwendet ungefähr 20–55 mm/a für langsame und 75–180 mm/a für schnelle Systeme; die Kategorien sind deskriptiv, keine Prozessschwellen. Das heutige mittlere Ozeankrustenalter beträgt 64,2 Myr. [Seton et al. 2020](https://doi.org/10.1029/2020GC009214)
3. **Plausible Abstraktion:** Ein aktiver Rücken erzeugt pro Schritt zwei sphärische Krustenstreifen mit `birth_time = now`; deren Breite ist `half_rate × dt` je Seite. Empfohlene volle Rate: 20–80 mm/a, Default 40 mm/a, selten bis 160 mm/a. Transformsegmente erzeugen oder vernichten keine Fläche. Alter ist stets `now - birth_time`, nicht zufällig.
4. **Topologische Regel:** Divergenz darf keine Lücke auf der Kugel erzeugen. Die neu entstehende Fläche wird atomar in explizite Ozeanpolygone überführt; Rücken bleibt gemeinsame Grenzlinie der Tochterplatten. Flächengewinn muss bis auf numerische Toleranz der integrierten Divergenz entsprechen.

### 4. Subduktionsbeginn, Fortsetzung und Verbrauch

1. **Subduktionsbeginn ist wissenschaftlich nicht abschließend geklärt.** „Induzierte“ Initiation durch bestehende Konvergenz und „spontane“ Initiation durch gravitative Instabilität sind Endglieder. Die Auswertung von 70 känozoischen Kandidaten findet Initiationen häufig nahe Kontinenten, bestehenden Subduktionszonen und Schwächezonen; eindeutig spontane känozoische Beispiele sind selten oder fehlen. [Stern 2004](https://doi.org/10.1016/j.epsl.2004.08.007), [Lallemand & Arcay 2021](https://doi.org/10.1016/j.earscirev.2021.103779)
2. **Alter erhöht negative Auftriebskraft, ist aber kein hinreichender Startschalter.** Ozeanische Lithosphäre wird beim Abkühlen dichter; Schätzungen für den Übergang zu negativer Auftriebskraft liegen etwa bei 20–50 Myr. Gleichzeitig wird alte Lithosphäre stärker, weshalb intakte passive Ränder nicht einfach abkippen. [Stern 2004](https://doi.org/10.1016/j.epsl.2004.08.007)
3. **Robuste Startbedingung für das MVP:** Neue Subduktion nur, wenn (a) die relative Bewegung über mindestens ein zusammenhängendes Grenzsegment konvergent ist, (b) eine vorbestehende Schwäche vorliegt (Transform-/Frakturzone, passiver Rand, ehemalige Sutur oder Fortsetzung eines Grabens) und (c) mindestens eine Seite ozeanisch ist. Alter dient als Gewicht, nicht als alleiniger Schwellenwert. Bevorzugung ab 40 Myr, starkes Gewicht ab 80 Myr; erzwungene Initiation darf jüngere Kruste betreffen.
4. **Polarität:** Bei Ozean–Kontinent-Konvergenz taucht standardmäßig die ozeanische Seite ab. Bei Ozean–Ozean entscheidet deterministisch die Kombination aus höherem Alter und Schwächezonen-Orientierung; ein Seed-Tiebreaker ist zulässig. Dies ist eine Modellannahme, weil Naturfälle nicht durch Alter allein entschieden werden.
5. **Fortsetzung benötigt Hysterese.** Selbsttragende Subduktion ist erreicht, wenn Slab Pull Widerstände überwindet; Start und Fortsetzung haben deshalb nicht dieselben Bedingungen. Im Oberflächenmodell bleibt ein einmal etablierter Graben aktiv, solange die mittlere Normalbewegung konvergent bleibt und subduzierbare ozeanische Fläche anliegt. Kurze 1–5-Myr-Intervalle schwacher/neutraler Bewegung dürfen überbrückt werden; der konkrete Wert ist Modellannahme. [Lallemand & Arcay 2021](https://doi.org/10.1016/j.earscirev.2021.103779)
6. **Verbrauch:** Entferne am Graben einen Streifen der abtauchenden Platte mit Breite `convergence_normal × dt`. Da Kruste vom Rücken weg altert, wird geometrisch häufig alte Kruste zuerst erreicht; eine globale Regel „lösche stets die älteste Kruste“ wäre jedoch falsch. Stoppe oder reorganisiere bei Rücken–Graben-Kollision, vollständigem Beckenverbrauch oder Kontinentankunft. Kontinent–Kontinent-Kollision erzeugt Sutur/Orogenlinie und darf im MVP keine kontinentale Fläche vernichten.

### 5. Ersatz der anfänglich unaufgelösten alten Ozeankruste

1. **Die erhaltene heutige Ozeankruste ist überwiegend jung, aber 200 Myr ist keine harte physikalische Obergrenze.** Der globale Mittelwert liegt bei 64,2 Myr; Rekonstruktionen verlorener Ozeane sind nicht eindeutig, weil subduzierte Kruste nicht direkt erhalten ist. [Seton et al. 2020](https://doi.org/10.1029/2020GC009214), [Williams et al. 2021](https://doi.org/10.1016/j.gsf.2020.06.004)
2. **Sehr alte Restbecken sind möglich und Alterszuweisungen können umstritten sein.** Für Kruste unter dem östlichen Mittelmeer wurde ein paläozoisches Alter von bis zu etwa 340 Ma vorgeschlagen. Das widerlegt eine automatische Löschung bei 180 oder 200 Ma. [Granot 2016](https://doi.org/10.1038/ngeo2784)
3. **Heutige Größenordnung der Produktion:** Der aktualisierte Alters-/Spreizungsdatensatz und globale Rekonstruktionen stützen eine breite Ratenverteilung und nahezu ausgeglichene langfristige Produktion/Destruktion; die zeitliche Entwicklung der globalen Produktionsrate bleibt wegen fehlender alter Kruste unsicher. Deshalb darf aus einer globalen Mittelrate keine lokale Austauschgarantie abgeleitet werden. [Seton et al. 2020](https://doi.org/10.1029/2020GC009214), [Cogné & Humler 2006](https://doi.org/10.1029/2005GC001148)
4. **Initialisierungsabschluss muss topologisch, nicht zeitlich definiert sein:** abgeschlossen genau dann, wenn `area(oceanic_unresolved) = 0` (innerhalb Flächentoleranz) und jede Oberflächenzelle/-polygonfläche einer expliziten Platte angehört. Aufgelöste alte Kruste darf älter als 200 Ma überleben.
5. **Plausible Abstraktion zum garantierten Abschluss:** Lege initial Subduktionsränder so an, dass jede zusammenhängende unaufgelöste Ozeanregion einen kinematischen Pfad zu mindestens einem konsumierenden Graben hat; priorisiere dort Verbrauch gegenüber bereits neu erzeugter Kruste. Falls nach einer konfigurierbaren Obergrenze (empfohlen 250 Myr) unaufgelöste Taschen isoliert bleiben, ist der Lauf als ungültige Initialisierung zurückzuweisen – nicht durch kommentarloses „Altern und Löschen“ zu reparieren.

### 6. Was eine Reifephase von 200 Myr leisten kann

1. **Kinematische Reichweite:** Bei 20–80 mm/a relativer Bewegung entsprechen 200 Myr etwa 4.000–16.000 km kumulativer Relativverschiebung. Damit sind breite Ozeane, lange passive Ränder, etablierte Subduktionssysteme, Inselbögen und mehrere Kollisionen plausibel. Die Multiplikation ist eine direkte kinematische Folgerung aus beobachteten Ratenbereichen. [Seton et al. 2020](https://doi.org/10.1029/2020GC009214)
2. **Pangaea liefert eine Größenordnung, keine Vorlage:** Seit dem mesozoischen Zerfall entstanden heutige Ozeanbecken und globale Plattenreorganisationen; mittlere absolute Plattengeschwindigkeiten lagen phasenweise etwa bei 4–10 cm/a. [Müller et al. 2016](https://doi.org/10.1146/annurev-earth-060115-012211)
3. **200 Myr garantieren keinen neuen Superkontinenten.** Superkontinentzyklen werden häufig in der Größenordnung mehrerer hundert Myr diskutiert; Introversion, Extroversion und Orthoversion sind konkurrierende Organisationsmodelle, keine deterministische Sequenz. [Murphy et al. 2021](https://doi.org/10.1038/s43017-021-00160-0), [Mitchell et al. 2012](https://doi.org/10.1038/nature10800)
4. **MVP-Auslegung:** Die Reifephase beginnt erst nach vollständiger Plattenpartition und läuft standardmäßig 200 Myr. Ihr Ziel ist eine geologisch lesbare Feature Map, nicht ein vorgeschriebener Endzustand. Sinnvolle Endmerkmale: Krustenalterbänder und Rücken, passive Ränder/Riftreste, Transformlinien, Gräben mit Polarität, Vulkanbogen-/Orogen-Puffer, Suturen und kollidierte Terrane. Eine Superkontinent-Neubildung ist zulässiges emergentes Ergebnis, aber kein Akzeptanzkriterium.

### 7. Determinismus, Parameter und Validierung

| Parameter | empfohlener MVP-Bereich | Default | Status |
|---|---:|---:|---|
| Zeitschritt | 0,25–1 Myr | 0,5 Myr | numerische Modellannahme |
| frühe volle Riftrate | 2–10 mm/a | 5 mm/a | evidenzgestützte Größenordnung |
| Rift-Beschleunigung | 3–10 Myr | 6 Myr | kalibrierte Abstraktion |
| volle Spreizungsrate | 20–80 mm/a, selten bis 160 | 40 mm/a | evidenzgestützter Bereich, eingeschränkter Default |
| erfolgreiche Riftphase | 20–60 Myr | 35 Myr | Modellkalibrierung, kein Naturgrenzwert |
| Altergewicht Subduktionsstart | ab 40 Myr; stark ab 80 Myr | 40/80 | plausible Abstraktion |
| Konvergenz für Start | ≥5 mm/a über ≥5 Myr | 5/5 | Modellannahme; keine universelle Schwelle |
| Fortsetzungs-Hysterese | 1–5 Myr | 3 Myr | Modellannahme |
| Initialisierungs-Timeout | 150–250 Myr | 250 Myr | Modellannahme |
| Reifephase | konfigurierbar 100–400 Myr | 200 Myr | Produktvorgabe/Modellannahme |

**Deterministische Reihenfolge je Tick:** (1) Euler-Bewegungen aus aktuellem Zustandsgraph bestimmen; (2) relative Grenzkinematik klassifizieren; (3) etablierte Subduktion fortführen; (4) Kollisionen/Rücken-Graben-Ereignisse auflösen; (5) neue Subduktionen nach sortiertem Score initiieren; (6) Riftfortschritt und Break-up; (7) neue Rückenkruste erzeugen; (8) sphärische Polygone reparieren und Invarianten prüfen; (9) Features protokollieren. Kandidaten werden stabil nach `(score desc, stable_id asc)` sortiert; Zufall wird ausschließlich aus Welt-Seed plus stabiler Ereignis-ID abgeleitet.

**Pflichttests für spätere Implementierung:** identischer Seed/Config ⇒ bit-identische Feature-IDs und äquivalente Geometrie; keine Polygonüberlappung oder -lücke; kein Rift schneidet einen Kraton; Krustenalter nie negativ und am Rücken null; Flächenbilanz pro Tick; Transformfläche null; Subduktion entfernt nur die Unterplatte; Initialisierung endet nur bei null unaufgelöster Fläche; Parametergrenzen erzeugen erklärbare Validierungsfehler statt stiller Korrektur.

## Unsicherheit und ausdrücklich nicht belastbare Aussagen

- Literatur stützt keine universelle Kombination aus Krustenalter, Geschwindigkeit und Grenztyp, die Subduktion sicher startet.
- Es besteht kein Konsens, dass Superkontinente primär durch Plumes, Rand-Subduktion, „Insulation“ oder einen festen geometrischen Modus zerfallen beziehungsweise neu entstehen. [Murphy et al. 2021](https://doi.org/10.1038/s43017-021-00160-0)
- Ein Alter von 200 Ma ist weder Lebensdauergrenze ozeanischer Kruste noch Dauer eines vollständigen Wilson-Zyklus.
- Aus Oberflächengeometrie allein lassen sich reale Euler-Pole, Riftpfade, Subduktionspolarität und Plattenreorganisationen nicht eindeutig vorhersagen.
- „Kratone können nie gespalten werden“ ist eine Produktregel. Reale Riftlokalisierung wird zwar von starken Kratonkernen und schwächeren Rändern beeinflusst, ein absolutes geologisches Verbot folgt daraus nicht. [Yoshida & Yoshizawa 2021](https://doi.org/10.1146/annurev-earth-091620-113028)

## Bibliografie

- Brune, S. et al. (2016). *Abrupt plate accelerations shape rifted continental margins*. Nature 536, 201–204. https://doi.org/10.1038/nature18319
- Brune, S. et al. (2023). *Geodynamics of continental rift initiation and evolution*. Nature Reviews Earth & Environment 4, 235–253. https://doi.org/10.1038/s43017-023-00391-3
- Buiter, S. J. H. & Torsvik, T. H. (2014). *A review of Wilson Cycle plate margins*. Gondwana Research 26, 627–653. https://doi.org/10.1016/j.gr.2014.02.007
- Cogné, J.-P. & Humler, E. (2006). *Trends and rhythms in global seafloor generation rate*. G3 7. https://doi.org/10.1029/2005GC001148
- de Lamotte, D. F. et al. (2015). *Style of rifting and the stages of Pangea breakup*. Tectonics 34. https://doi.org/10.1002/2014TC003760
- Gordon, R. G. (1998). *The plate tectonic approximation*. Annual Review of Earth and Planetary Sciences 26, 615–642. https://doi.org/10.1146/annurev.earth.26.1.615
- Granot, R. (2016). *Palaeozoic oceanic crust preserved beneath the eastern Mediterranean*. Nature Geoscience 9, 701–705. https://doi.org/10.1038/ngeo2784
- Lallemand, S. & Arcay, D. (2021). *Subduction initiation from the earliest stages to self-sustained subduction*. Earth-Science Reviews 221, 103779. https://doi.org/10.1016/j.earscirev.2021.103779
- McKenzie, D. P. & Parker, R. L. (1967). *The North Pacific: an example of tectonics on a sphere*. Nature 216, 1276–1280. https://doi.org/10.1038/2161276a0
- Mitchell, R. N. et al. (2012). *Supercontinent cycles and the calculation of absolute palaeolongitude in deep time*. Nature 482, 208–211. https://doi.org/10.1038/nature10800
- Morgan, W. J. (1968). *Rises, trenches, great faults, and crustal blocks*. JGR 73, 1959–1982. https://doi.org/10.1029/JB073i006p01959
- Müller, R. D. et al. (2016). *Ocean Basin Evolution and Global-Scale Plate Reorganization Events Since Pangea Breakup*. Annual Review of Earth and Planetary Sciences 44, 107–138. https://doi.org/10.1146/annurev-earth-060115-012211
- Murphy, J. B. et al. (2021). *The supercontinent cycle*. Nature Reviews Earth & Environment 2, 358–374. https://doi.org/10.1038/s43017-021-00160-0
- Seton, M. et al. (2020). *A Global Data Set of Present-Day Oceanic Crustal Age and Seafloor Spreading Parameters*. G3 21. https://doi.org/10.1029/2020GC009214
- Stern, R. J. (2004). *Subduction initiation: spontaneous and induced*. EPSL 226, 275–292. https://doi.org/10.1016/j.epsl.2004.08.007
- Ulvrova, M. M. et al. (2019). *Breakup Without Borders*. GRL 46, 1338–1347. https://doi.org/10.1029/2018GL080387
- Vine, F. J. & Matthews, D. H. (1963). *Magnetic anomalies over oceanic ridges*. Nature 199, 947–949. https://doi.org/10.1038/199947a0
- Williams, S. et al. (2021). *Reconstructing seafloor age distributions in lost ocean basins*. Geoscience Frontiers 12. https://doi.org/10.1016/j.gsf.2020.06.004
- Yoshida, M. & Yoshizawa, K. (2021). *Continental drift with deep cratonic roots*. Annual Review of Earth and Planetary Sciences 49, 117–139. https://doi.org/10.1146/annurev-earth-091620-113028

## Sources

- **Kept:** Peer-reviewte Original- und Reviewliteratur mit DOI; besonders globale Alters-/Spreizungsdaten, sphärische Plattenkinematik, Rift- und Subduktionsreviews. Sie trägt die Regeln und quantitativen Größenordnungen.
- **Dropped:** Academia.edu, New Scientist, Phys.org, Pressemitteilungen und Lehrbuchauszüge — tertiär, redundant oder ohne ausreichende Primärbelegfunktion.
- **Dropped:** Einzelne numerische Mantelmodelle als direkte Parameterquelle — außerhalb des Surface-only-Scope und nicht beobachtungsäquivalent.

## Gaps

Globale Datensätze liefern keine eindeutige Wahrscheinlichkeit für Rift-Erfolg oder Subduktionsstart eines beliebigen Fantasy-Layouts. Vor Implementierung sollte deshalb eine kleine Sensitivitätsmatrix über Riftanzahl, Spreizungsrate, Initiationsscore und Timeout definiert werden; die vorgeschlagenen Defaults sind anschließend gegen Flächenalter-Histogramm, Plattenzahl, Grenzlängen und Anteil ungelöster Läufe zu kalibrieren, nicht gegen einen einzigen „erdähnlichen“ Endzustand.

## Entscheidungsreife Empfehlungen

1. **Zustandsmodell festschreiben:** vollständige sphärische Polygonpartition; starre Euler-Platten; Grenzlinien mit Typ/Polarität; Krustenart und Geburtszeit pro Polygon; etwa zehn unteilbare Kratone im anfänglichen, etwa 25 % der Oberfläche großen Superkontinenten.
2. **Zerfall:** mehrere diachrone Riftkandidaten nur in nicht-kratonischer Kruste; bevorzugt Suturen/mobile Gürtel; gescheiterte Rifte zulassen. Früh 2–10 mm/a, nach erfolgreichem Break-up 20–60 mm/a; Erfolg nach 20–60 Myr regelbasiert bewerten.
3. **Neubildung:** an Rücken beidseitig exakt so viel Kruste erzeugen, wie relative Divergenz öffnet; empfohlene volle Spreizung 20–80 mm/a, Default 40; Alter deterministisch aus Geburtszeit.
4. **Subduktion:** nur bei Konvergenz plus Schwächezone plus ozeanischer Seite initiieren. Alter ab 40/80 Myr als steigendes Gewicht, nie als alleinigen Schalter verwenden. Etablierte Gräben mit Hysterese fortsetzen; Verbrauch geometrisch am Graben, nicht global „älteste zuerst“.
5. **Vollständiger Ersatz:** Initialisierung ausschließlich bei `unresolved_area == 0` beenden. Jede anfängliche Ozeanregion muss einen Pfad zu einem Graben besitzen; Timeout Default 250 Myr, danach Lauf verwerfen. Kein Age-Cutoff und kein unsichtbares Löschen.
6. **Reifephase:** danach 200 Myr standardmäßig, konfigurierbar 100–400 Myr. Diese Dauer kann Ozeanöffnung/-schluss, reife Subduktion und Kollision plausibel erzeugen, garantiert aber keinen neuen Superkontinenten.
7. **Aussagegrenze:** Ausgabe als „plausible kinematische Geschichte“, nicht als physikalische Rekonstruktion oder Vorhersage kennzeichnen.

### Modellannahmen

- Startfläche des Superkontinents (~25 %) und Zahl der Kratone (~10).
- Kratone sind absolut unteilbar.
- Linien statt diffuser Deformationszonen und starre Platteninnenräume.
- Kandidaten-Scoring, Seed-Tiebreaks sowie konkrete Rift-, Initiations-, Hysterese- und Timeout-Schwellen.
- Standardpolarität Ozean unter Kontinent und altersgewichtete Entscheidung bei Ozean–Ozean.
- Bevorzugter Verbrauch anfänglich unaufgelöster Kruste zur garantierbaren Initialisierung.
- 250-Myr-Initialisierungstimeout und anschließende 200-Myr-Reifephase.
- Kollision vernichtet im MVP keine kontinentale Fläche; Orogene und Vulkanbögen sind abgeleitete Oberflächenfeatures ohne quantitative Höhen-/Magmenphysik.
