# Tektonische Kernfeatures: wissenschaftliche Grundlage für das MVP

## Zusammenfassung

Das MVP sollte tektonische Features **aus der lokalen relativen Plattenkinematik und der bereits vorhandenen Grenztopologie ableiten**, nicht aus behaupteten Spannungen oder Mantelprozessen. Eine Grenzlinie ist divergent, konvergent oder transform, wenn die relative Geschwindigkeit in ihrer lokalen sphärischen Normal-/Tangentialbasis entsprechend zerlegt ist; Rücken, Graben, Vulkanbogen, Sutur und Orogen sind davon abhängige, zeitlich zustandsbehaftete Features. Starre Platten und Euler-Rotationen sind dafür die wissenschaftlich etablierte kinematische Grundlage, während reale diffuse Grenzen eine ausdrücklich dokumentierte Auflösungsabstraktion bleiben ([McKenzie & Parker 1967](https://doi.org/10.1038/2161276a0); [Morgan 1968](https://doi.org/10.1029/JB073i006p01959); [Gordon 1998](https://doi.org/10.1146/annurev.earth.26.1.615)).

Entscheidend sind folgende Trennungen: Ein kontinentales Rift ist noch keine Plattengrenze und kein mittelozeanischer Rücken; bei versetzten Rückensegmenten ist nur der aktive Abschnitt zwischen den Achsen eine Transformgrenze, seine intraplattenseitigen Fortsetzungen sind Fracture Zones; ein Graben braucht Polarität, der Vulkanbogen liegt auf der Oberplatte; Kontinentankunft beendet ozeanische Subduktion nicht augenblicklich, führt aber regelbasiert zu Kollision, Sutur und einer breiten Orogenzone. Aktive Linien können binnen weniger Myr reorganisiert werden, ihre inaktiven Oberflächenspuren dagegen über viele Zehner bis Hunderte Myr überdauern. Deshalb braucht die finale Feature-Map neben Geometrie immer Zustand, Entstehungs-/Endzeit, Elternbeziehungen und Evidenz-/Annahmestatus.

## Scope und Evidenzsprache

**Produktgrenze.** Alle Geometrien liegen auf der Kugel und sind Punkte, Linien oder sphärische Polygone. Das Modell ist deterministisch und rein oberflächenkinematisch. Es berechnet keine Rasterhöhen, Mantelkonvektion, Spannungsfelder, Rheologie, Isostasie, thermische Struktur, Schmelzproduktion oder quantitative Erosion. Kratone sind gemäß Produktregel unteilbar; dies ist kein universelles geologisches Gesetz.

**Befund** bezeichnet Beobachtung, Datensatz oder robuste kinematische Folgerung. **Plausible MVP-Abstraktion** bezeichnet eine implementierbare Vereinfachung, die Befunde respektiert, aber nicht eindeutig aus ihnen folgt. **Modellannahme** bezeichnet eine frei zu kalibrierende Produktentscheidung ohne naturgesetzlichen Anspruch. Zahlen ohne robuste globale Verteilung werden nicht als harte Naturgrenzen verwendet.

Dieses Dokument ergänzt `CONTEXT.md` und `docs/research/platten-und-superkontinentzyklus.md`: Dort bereits begründete Euler-Kinematik, Rift-/Subduktionsinitiierung und Krustenbilanz werden hier nicht erneut hergeleitet, sondern zu Feature-Geometrien, Lebensläufen und Map-Attributen operationalisiert.

## Gemeinsame sphärische Geometrie

Für jeden Punkt einer orientierten Grenz-Polyline seien `r` die radiale Einheitsnormale, `t` die Linientangente, `plate_left = A`, `plate_right = B` und `n = t × r` die tangentiale Normale von links nach rechts. Aus `v_rel = v_B − v_A` folgen `v_n = v_rel·n` und `v_t = v_rel·t`; `v_n > 0` bedeutet Öffnung, `v_n < 0` Schließung. Wird die Polyline umgekehrt, müssen Seiten-IDs, `t`, `n` und `v_rel` gemeinsam wechseln; `v_n` bleibt dadurch invariant. Beispiel: Bewegen sich linke Platte mit `-5n` und rechte mit `+5n`, dann ist `v_rel = 10n` und die Grenze öffnet sich mit 10 mm/a.

- **Befund:** Relative Bewegung auf der Kugel folgt Kleinkreisen um den relativen Eulerpol; Transformverläufe sind lokal parallel zur Relativbewegung. Eine Grenze kann entlang ihres Verlaufs ihre kinematische Klasse wechseln ([Morgan 1968](https://doi.org/10.1029/JB073i006p01959); [Wilson 1965](https://doi.org/10.1038/207343a0)).
- **Plausible MVP-Abstraktion:** Zuerst numerischen Stillstand über Geschwindigkeitstoleranzen behandeln. Danach `f_n = |v_n| / hypot(v_n,v_t)` berechnen: Bei `f_n ≤ 0,2` ist das Segment transform, sonst bestimmt das Vorzeichen von `v_n` divergent oder konvergent. Bei `f_n < 0,8` und zugleich `|v_n| > eps_speed` wird zusätzlich `substate=oblique` gesetzt; bei positivem `v_n` transtensional, bei negativem transpressional. Kurze Nachbarstücke gleicher Klasse werden topologisch zusammengeführt.
- **Modellannahme:** `eps_speed = 1 mm/a`, Obliquitätsgrenzen 0,2/0,8 und Mindestsegmentlänge 25 km verhindern numerisches Flattern. Werte sind Auflösungsparameter, keine geologischen Schwellen.

## 1. Divergente Grenzen, kontinentale Rifte und mittelozeanische Rücken

### Entstehung, Fortsetzung und Ende

- **Befund:** Kontinentales Rifting verläuft von verteilter Extension über Lokalisierung und Krustenverdünnung bis gegebenenfalls zum Break-up. Viele Rifte scheitern; Vererbung, Plattenrandkräfte, Lithosphärenstruktur und Magmatismus beeinflussen den Verlauf, bestimmen Erfolg aber nicht einzeln. Reale Riftzonen sind von mehreren Zehnern bis mehreren Hundert Kilometern breit ([Brune et al. 2023](https://doi.org/10.1038/s43017-023-00391-3)).
- **Befund:** Ein mittelozeanischer Rücken ist die **ozeanische Akkretionsachse einer divergenten Plattengrenze**. Magnetische Altersbänder belegen beidseitige Neubildung; moderne volle Spreizungsraten reichen grob von ultraslow unter 20 bis schnell etwa 75–180 mm/a, ohne universelle Prozessschwellen ([Vine & Matthews 1963](https://doi.org/10.1038/199947a0); [Seton et al. 2020](https://doi.org/10.1029/2020GC009214)).
- **Plausible MVP-Abstraktion:** Ein Riftkorridor darf nur nicht-kratonische Kontinentpolygone durchziehen. Das Feature `rift` durchläuft die Substates `incipient` und `localized`, bis eine durchgehende Grenze zwei eigenständig bewegte Platten trennt und ozeanische Kruste erzeugt; erst dann entsteht der Nachfolger vom Typ `spreading_ridge`. Gescheiterte Rifte wechseln in `state=relict, substate=abandoned`, statt gelöscht zu werden.
- **Fortsetzung:** Rücken bleibt aktiv, solange die normale Öffnung den Toleranzwert übersteigt und ozeanische Neubildung topologisch möglich ist. **Modellannahme:** Das MVP verteilt die volle Öffnung symmetrisch und erzeugt pro Schritt auf jeder Seite einen Streifen der Breite `0,5 × v_n × dt` mit `birth_time = now`; reale Spreizung kann asymmetrisch sein.
- **Ende/Übergang:** Bei numerisch neutraler Normalbewegung wird die Achse inaktiv oder bei hinreichender Tangentialbewegung transform; bei `v_n < -eps_speed` folgt ein konvergenter Typwechsel. Bei Rücken–Graben-Kollision endet der betroffene Rückenabschnitt; ein Sprung der Akkretionsachse erzeugt eine neue Rücken-ID und lässt die alte mit `state=relict, substate=abandoned` nachleben.

### Geometrie und Größenordnung

- **Rift:** als zentrale Linie plus sphärisches Korridorpolygon. MVP-Breite 30–300 km, Default 100 km; dies bildet schmale und breite natürliche Rifte nur kartographisch ab, nicht deren Deformation. Länge 500–5.000 km ist eine sinnvolle Weltgenerator-Kalibrierung, aber keine belastbare Naturverteilung.
- **Rücken:** segmentierte Linienkette auf der gemeinsamen Plattengrenze. Segmente sind durch Transformen oder nicht-transforme Offsets getrennt; Rückensegmente reichen beobachtet von Zehnern bis Hunderten Kilometern ([Carbotte et al. 2016](https://doi.org/10.1144/SP420.5)). MVP: 50–500 km Segmentlänge, Default 200 km.
- **Breite:** Die Rückenachse selbst ist eine Linie. Ein optionales `ridge_zone`-Polygon von 20–100 km Breite ist reine Darstellungsabstraktion; es darf nicht als bathymetrische Rückenbreite oder Höhe ausgegeben werden.
- **Lebensdauer:** Erfolgreiches Rifting benötigt häufig viele bis einige Zehnermillionen Jahre. Lang bestehende Ozeanbecken zeigen, dass Rückenachsen prinzipiell viele Zehnermillionen Jahre bestehen können, ihre Segmente aber reorganisiert werden; eine globale Lebensdauerverteilung ist nicht belastbar. MVP-Rift 20–60 Myr bis Break-up (Kalibrierung aus dem Vorgängerdokument), aktive Rücken ohne festen Age-out.

### Erforderliche Attribute

Zusätzlich zum gemeinsamen Attributrahmen: `plate_left`, `plate_right`, `parent_rift_id`, `full_rate_mm_yr`, `normal_rate_mm_yr`, `segment_order`, `offset_connector_ids`, `corridor_width_km` und `crust_transition`. Kanonische Typen sind `rift` und `spreading_ridge`; `abandoned` ist ein Substate, kein eigener Typ.

## 2. Subduktionszonen, Gräben und Vulkanbögen

### Entstehung, Fortsetzung und Ende

- **Befund:** Subduktionsinitiierung ist nicht durch eine universelle Alters-, Geschwindigkeits- oder Dichteschwelle determiniert. Induzierte und spontane Initiierung sind Endglieder; känozoische Kandidaten liegen häufig an vorhandenen Schwächen, Kontinenträndern oder bestehenden Subduktionssystemen ([Stern 2004](https://doi.org/10.1016/j.epsl.2004.08.007); [Lallemand & Arcay 2021](https://doi.org/10.1016/j.earscirev.2021.103779)).
- **Plausible MVP-Abstraktion:** Initiierung nur an länger konvergenten Segmenten mit mindestens einer ozeanischen Seite und einer gespeicherten Schwäche (`fracture_zone`, passiver Rand, alte Sutur oder Fortsetzung eines Grabens). Alter erhöht den Score, löst aber nie allein aus. Diese Regel übernimmt die im Vorgängerdokument empfohlenen Gewichte ab 40/80 Ma.
- **Befund zur Polarität:** Naturfälle, insbesondere Ozean–Ozean-Systeme, sind nicht allein aus dem Krustenalter entscheidbar; die Rollen von Unter- und Oberplatte müssen explizit repräsentiert werden.
- **Modellannahme zur Polarität:** Bei Ozean–Kontinent-Konvergenz ist standardmäßig der Ozean Unterplatte. Bei Ozean–Ozean bestimmt das MVP die Polarität aus Schwächezonenlage, Altersscore und stabiler ID als Tiebreaker. `subducting_plate_id` und `overriding_plate_id` sind zwingend.
- **Fortsetzung:** Ein etablierter Graben konsumiert ausschließlich die geometrisch anliegende Unterplatte um `max(0, -v_n) × dt`; Transformanteile erzeugen keinen Verbrauch. Kurze neutrale Phasen dürfen mit 1–5 Myr Hysterese überbrückt werden.
- **Ende/Übergang:** Rückenankunft kann den Graben lokal beenden oder reorganisieren; vollständiger Beckenverbrauch und Kontinentankunft schalten `trench[active]` in `trench[waning, collision_pending]`. Kontinent darf im MVP nicht wie Ozeankruste gelöscht werden. Trench rollback, Slabbruch oder Polaritätswechsel dürfen ohne zusätzliche Modellentscheidung nicht behauptet werden.

### Geometrie und Größenordnung

- **Graben:** orientierte Linie exakt auf der konvergenten Grenze; Zähne/Normalen zeigen kartographisch zur Oberplatte. Subduktionszonen bilden global mehrere 10.000 km Grenzlänge; einzelne Systeme reichen von Hunderten bis mehreren Tausend Kilometern ([Stern 2002](https://doi.org/10.1029/2001RG000108)). MVP-Segment 200–5.000 km, aber kein Mindestwert als Naturtrigger.
- **Forearc/Vulkanbogen:** Vulkanbogen ist eine oberplattenseitige, ungefähr grabenparallele Linie oder Kette von Punktzentren. Global liegen Bögen typischerweise über Slabs in ungefähr 65–130 km Tiefe; die Oberflächendistanz zum Graben hängt stark vom Slabwinkel ab und ist nicht konstant ([Syracuse & Abers 2006](https://doi.org/10.1029/2005GC001045); [England et al. 2004](https://doi.org/10.1111/j.1365-246X.2003.02132.x)).
- **Plausible MVP-Abstraktion:** Ohne Slabgeometrie Vulkanbogen durch sphärischen Offset auf der Oberplatte: 100–300 km vom Graben, Default 180 km; Enden 50–150 km innerhalb der Grabensegmentenden kürzen. Bei Platzmangel clippen, nie auf die Unterplatte verschieben. Ein 50–150-km-Korridor ist Darstellungszone, keine Schmelzvorhersage.
- **Lebensdauer/Nachleben:** Bögen entstehen nicht augenblicklich mit der ersten Konvergenz. Modellannahme: 3–10 Myr Reifeverzögerung, Ende 0–5 Myr nach Grabenende; `volcanic_arc` und `trench` wechseln danach in `state=relict` und bleiben unbegrenzt in der Historie sowie standardmäßig 100 Myr sichtbar. Es gibt keine robuste universelle Löschzeit.

### Erforderliche Attribute

Zusätzlich zum gemeinsamen Attributrahmen: `subducting_plate_id`, `overriding_plate_id`, `polarity`, `normal_convergence_mm_yr`, `obliquity_deg`, `initiation_basis[]`, `consumed_area_km2`, `arc_id`, `arc_offset_km` und `collision_successor_id`.

## 3. Transformgrenzen und Fracture Zones

### Fachliche Trennung und Lebenslauf

- **Befund:** Transformgrenzen sind aktive, überwiegend seitenverschiebende Plattengrenzen, die Rücken–Rücken, Rücken–Graben oder Graben–Graben verbinden. Sie folgen auf der Kugel näherungsweise Kleinkreisen des relativen Eulerpols und erzeugen bzw. vernichten idealisiert keine Fläche ([Wilson 1965](https://doi.org/10.1038/207343a0); [Morgan 1968](https://doi.org/10.1029/JB073i006p01959)).
- **Befund:** Bei einer Ridge–Ridge-Transform ist nur das Stück **zwischen den beiden aktiven Rückenspitzen** Plattengrenze. Die gleichgerichteten Spuren außerhalb sind Fracture Zones innerhalb derselben Platte: normalerweise keine aktive Plattengrenze, aber langlebige morphologische/strukturelle Lineamente. Systeme können Tausende Kilometer lang sein; Romanche erreicht >5.000 km Systemlänge und etwa 950 km Offset ([Hensen et al. 2019](https://doi.org/10.3389/feart.2019.00039)).
- **Plausible MVP-Abstraktion:** Aus jedem Ridge–Transform–Ridge-Knotenpaar entstehen (a) eine aktive `transform_fault` zwischen den Achsen und (b) mit fortschreitender Spreizung beidseits advectierte `fracture_zone`-Linien. Nur dieser Rückenoffset erzeugt automatisch Fracture Zones. Transformbreite im Modell ist null; optionaler 5–30-km-Stilkorridor darf nicht als reale Deformationsbreite interpretiert werden.
- **Weitere Connectoren:** Ridge–Trench- und Trench–Trench-Transformen verbinden die jeweiligen aktiven Endknoten und bestehen nur, solange beide Endfeatures und überwiegend tangentiale Relativbewegung bestehen. Endet oder reorganisiert ein Endfeature, endet der Connector oder wird mit neuer ID neu verknüpft; seine alte Spur ist `transform_fault[state=inactive]`, nicht automatisch `fracture_zone`.
- **Kontinentale Transformen:** können mehrere parallele Stränge und breite diffuse Zonen besitzen; die Alpine Fault belegt einen lokalisierten Hauptstrang, ist aber keine globale Breitenverteilung ([Norris & Toy 2014](https://doi.org/10.1016/j.jsg.2014.03.003)). Das MVP erzeugt eine Hauptlinie, wenn eine kontinentale Grenzstrecke nach Obliquitätsregel überwiegend tangential ist, plus optionales Korridorpolygon. Sie endet oder wird neu klassifiziert, sobald Endknoten verschwinden oder die Kinematik dauerhaft divergent/konvergent wird.
- **Fortsetzung/Ende:** aktiv bei tangential dominierter Relativbewegung. Änderung des Eulerpols kann Transpression oder Transtension erzeugen; die Grundklasse bleibt nach `f_n` transform oder wechselt divergent/konvergent, während `oblique` den Mischanteil kennzeichnet. Die alte Linie bleibt `transform_fault[state=inactive]` oder bei Ridge–Ridge-Herkunft `fracture_zone`.
- **Nachleben:** Fracture Zones altern mit der Kruste und können bis zur Subduktion den ganzen Ozean queren. Keine feste Lebensdauer; Ende nur durch geometrischen Verbrauch, Überprägung oder explizite Sichtbarkeitsregel. Reaktivierung ist möglich, aber nicht automatisch.

### Geometrie und Größenordnung

Oceanische Transformabstände überlappen breit; ein Review nennt etwa 400 ± 200 km an langsamen und 600 ± 300 km an schnellen Rücken ([Hensen et al. 2019](https://doi.org/10.3389/feart.2019.00039)). Große ridge-transform Offsets sind >30 km; kleine Offsets können nicht-transform sein ([Howe et al. 2022](https://doi.org/10.1029/2020JB020017)). MVP: Offset 20–500 km, selten bis 1.000 km; unter 30 km bevorzugt `non_transform_offset`. Dies ist eine Erzeugungsverteilung, kein Ausschluss echter kleiner Transformen.

### Erforderliche Attribute

Zusätzlich zum gemeinsamen Attributrahmen: `connector_type`, `node_a`, `node_b`, `plate_left`, `plate_right`, `sense` (`dextral|sinistral`), `tangential_rate_mm_yr`, `normal_rate_mm_yr`, `offset_km`, `source_transform_id`, `crust_birth_range` und `reactivated`. Kanonische Typen sind `transform_fault`, `fracture_zone` und `non_transform_offset`; `inactive` und `oblique` sind State beziehungsweise Substate.

## 4. Kontinentale Kollision, Sutur und Orogenzone

### Entstehung, Fortsetzung und Ende

- **Befund:** Kontinentkollision folgt gewöhnlich auf Ozeanschließung und Ankunft positiv auftriebiger kontinentaler Kruste am Graben; Subduktion verlangsamt sich, Konvergenz kann jedoch durch Unterfahrung, Verkürzung, laterale Extrusion und diffuse Deformation weiter aufgenommen werden. Zeitpunkt und Mechanismus sind selbst für Indien–Asien umstritten ([Yin & Harrison 2000](https://doi.org/10.1146/annurev.earth.28.1.211); [van Hinsbergen et al. 2019](https://doi.org/10.1016/j.earscirev.2019.102908)).
- **Plausible MVP-Abstraktion:** Der fachliche Substate wird `collision_pending`, sobald entlang eines aktiven Grabens auf einer zusammenhängenden Länge von mindestens 100 km Kontinent der Unterplatte ankommt; nach 1–3 Myr anhaltendem Kontakt entsteht der Feature-Typ `collision`. Der ehemalige Graben wird nicht sofort gelöscht, sondern mit gemeinsamer Eltern-ID in Sutur überführt.
- **Sutur:** zentrale, langlebige Linie des geschlossenen Ozeans, topologisch aus der letzten Grabengeometrie plus kollidierenden Kontinenträndern abgeleitet. Sie ist kein aktiver Grenztyp und darf später intraplattenintern liegen.
- **Orogen:** sphärisches Polygon/Korridor um die Kollisionsfront, vorzugsweise asymmetrisch mit Ober-/Unterplattenseite und an Segmentenden verjüngt. Es ist eine **deformierte Zone**, keine Höhenfläche.
- **Fortsetzung:** Solange kontinentaler Kontakt und Konvergenz bestehen, wächst ein kumulativer `shortening_proxy_km = ∫max(0,-v_n)dt`; das ist Kinematik, keine rekonstruierte Krustenverkürzung. Indien–Asien zeigt, dass Plattenkonvergenz und kartierte Verkürzung stark differieren können ([van Hinsbergen et al. 2011](https://doi.org/10.1029/2011TC002908)).
- **Ende/Nachleben:** Bei neutraler/divergenter Bewegung endet aktive Orogenese nach Hysterese; Sutur bleibt als fossile Linie erhalten. Orogengürtel können noch Zehnermillionen Jahre verbreitert/exhumiert werden und als geologische Gürtel Hunderte Myr überliefert sein. Ohne Erosionsmodell darf das MVP kein deterministisches „Abtragen“ oder Höhenzerfall berechnen.

### Geometrie und Größenordnung

- Kollisionsorogene reichen von einigen Hundert Kilometern breiten alpinen Gürteltypen bis zu >1.000-km-breiten Plateaus; Tibet nahm seit Kollisionsbeginn mindestens ~1.400 km N–S-Verkürzung auf ([Yin & Harrison 2000](https://doi.org/10.1146/annurev.earth.28.1.211)). Ein global universeller Breite–Konvergenz-Faktor existiert nicht.
- **Modellannahme:** anfängliche Gesamtbreite 100–300 km; danach `total_width = initial_width + f_total × shortening_proxy`, mit `f_total = 0,1–0,3` (Default 0,2), gekappt auf 1.000 km Gesamtbreite. Bei symmetrischem Puffer wächst jede Seite um die Hälfte; die Formel ist keine physische Breitenvorhersage.
- Sutur als Linie, optionaler 20–100-km-`suture_zone`-Korridor nur zur Visualisierung komplexer Mischzonen. Keine fixe natürliche Suturbreite behaupten.
- **Lebensdauer:** aktive Kollisionsphase 10–100+ Myr plausibel, ohne harten Timeout. Inaktive Orogenzone standardmäßig 100 Myr sichtbar und danach nur Stilklasse `relict`; Sutur niemals allein wegen Alters löschen.

### Erforderliche Attribute

Zusätzlich zum gemeinsamen Attributrahmen: `continent_a`, `continent_b`, `precursor_trench_id`, `closed_ocean_id`, `active_end_time`, `suture_id`, `orogen_id`, `vergence` (`double|a_over_b|b_over_a|unknown`), `normal_convergence_mm_yr`, `shortening_proxy_km`, `orogen_width_km`, `width_rule` und `reactivated`.

## Featureübergreifender Lebenszyklus und Übergänge

1. `continental_interior → rift[candidate] → rift[active]` nur außerhalb von Kratonen.
2. `rift[active] → rift[relict, abandoned]` bei ausbleibender lokalisierter Divergenz; Nachleben als intraplatteninternes Lineament.
3. `rift[active] → breakup → spreading_ridge[active] + passive_margins`; Rücken erzeugt Altersbänder.
4. Rückensegmentierung erzeugt `transform_fault`; Spreizung advectiert dessen Spur als `fracture_zone`.
5. Konvergente Reorganisation an geeigneter Schwäche kann `subduction_initiation → trench → volcanic_arc` erzeugen. Fracture Zone ist Kandidat, kein Triggerbeweis.
6. `ridge_trench_collision` beendet/reorganisiert lokale Achsen; `continent_arrival → collision_pending → suture + orogen[active]`.
7. Typwechsel erzeugt immer ein neues Feature mit `predecessor_ids`; Historie wird nicht überschrieben. Inaktive Features werden nur durch Subduktion geometrisch entfernt oder in der Darstellung generalisiert.

**Konfliktauflösung pro Tick:** etablierte Subduktion/Verbrauch; Kontinentankunft und Kollision; Rücken–Graben-Ereignisse; Grenzklassifikation; neue Initiierungen; Riftfortschritt; Rückenproduktion; abgeleitete Bögen/Orogene; Topologieprüfung. Diese Reihenfolge ist Modellannahme, verhindert aber doppelte Flächennutzung.

## Implementierbare MVP-Regeln

1. Relative Geschwindigkeit an jedem Grenzvertex berechnen und längengewichtet pro Segment klassifizieren.
2. Kein Riftpolygon darf ein Kraton schneiden; Kandidatenpfade müssen um Kratone geführt oder verworfen werden.
3. Rücken nur bei ozeanischer Krustenproduktion; divergente Kontinentalgrenze bleibt Rift bis Break-up.
4. Transformsegmente ändern die Kugelflächenbilanz nicht. Aktive Transformlinie exakt auf der Plattengrenze; Fracture Zone intraplattenintern.
5. Graben immer orientiert und mit expliziter Unter-/Oberplatte. Verbrauch nur auf Unterplatte.
6. Vulkanbogen als geodätischer Offset auf Oberplatte, nicht als symmetrischer Grabenpuffer.
7. Kontinentankunft stoppt ozeanischen Verbrauch lokal; erzeugt Sutur/Orogen, keine Löschung kontinentaler Polygone.
8. Der gemeinsame Attributrahmen gilt für alle Featuretypen: `feature_id`, `feature_type`, `state`, `substate`, `geometry`, `carrier_plate_ids`, `start_time`, `end_time`, `predecessor_ids`, `successor_ids`, `generation_rule_version`, `evidence_class` und `confidence`. `feature_type` bezeichnet die fachliche Klasse (`rift`, `spreading_ridge`, `trench`, `volcanic_arc`, `transform_fault`, `fracture_zone`, `non_transform_offset`, `collision`, `suture`, `orogen`); `state ∈ {candidate, active, waning, inactive, relict, consumed}` ausschließlich den generischen Lebenszyklus. Phasen wie `incipient`, `collision_pending`, `abandoned` oder `oblique` stehen in `substate`.
9. Geometrische Maße (`length_km`, `area_km2`, `mean_width_km`) werden aus sphärischer Geometrie abgeleitet; gespeicherte Parameter bleiben getrennt.
10. Determinismus: stabile Sortierung `(priority, score desc, stable_id asc)`; Seed nur für dokumentierte Gleichstände, nie für nachträgliche Geometrieperturbation.

## Parameter- und Unsicherheitstabelle

| Parameter | MVP-Bereich / Default | Status und Unsicherheit |
|---|---:|---|
| Geschwindigkeitstoleranz / Obliquitätsgrenzen | 1 mm/a / 0,2 und 0,8 | Modellannahme, auf Zeitschritt/Auflösung testen |
| Riftkorridorbreite | 30–300 / 100 km | evidenzgestützte Größenordnung; keine Deformationsberechnung |
| Rift bis Break-up | 20–60 / 35 Myr | Kalibrierung, kein Naturtimer |
| Rückensegmentlänge | 50–500 / 200 km | plausible Abstraktion aus breiter Hierarchie |
| volle Spreizungsrate | 20–80 / 40 mm/a; selten 160 | evidenzgestützt, nicht vollständig |
| Ridge–Ridge-Offset | 20–500 / 100 km; selten 1.000 | Erzeugungsverteilung |
| kleiner nicht-transformer Offset | bevorzugt <30 km | deskriptiv, kein harter Ausschluss |
| Transformabstand | 200–900 / 450 km | grobe überlappende Naturverteilung |
| Subduktions-Startaltergewicht | 40/80 Ma | plausible Abstraktion, nie Alleintrigger |
| Startkonvergenz | 5 mm/a über 5 Myr | Modellannahme aus Vorgängerdokument |
| Subduktions-Hysterese | 1–5 / 3 Myr | Modellannahme |
| Bogenreife | 3–10 / 5 Myr | Modellannahme |
| Graben–Bogen-Offset | 100–300 / 180 km | plausible Oberflächenabstraktion ohne Slabwinkel |
| Kollisionskontakt | ≥100 km, 1–3 / 2 Myr | numerische Modellannahme |
| Orogenbreite initial | 100–300 / 200 km | plausible Darstellungsgröße |
| Orogenbreitenwachstum | 0,1–0,3 / 0,2 × Konvergenzproxy | Modellannahme; nicht reale Verkürzung |
| aktive Kollision | 10–100+ Myr, kein Timeout | Größenordnung, stark fallabhängig |
| Sichtbarkeit reliktischer Bögen/Orogene | 100 Myr | Produktentscheidung |
| Sutur/FZ-Nachleben | bis Verbrauch; kein Age-out | geologisch begründet, Darstellung generalisierbar |

## Validierungshinweise

- **Topologie:** lückenlose, überlappungsfreie Kugelpartition; Grenzlinien deckungsgleich mit angrenzenden Polygonkanten; aktive Transformen nie intraplattenintern, Fracture Zones nie irrtümlich als aktive Grenze.
- **Kinematik:** `ridge ⇒ v_n>eps_speed`; `trench ⇒ v_n<-eps_speed`; `transform ⇒ f_n≤0,2` bei nicht ruhender Grenze. Beim Umkehren der Polyline müssen Seiten-IDs und Basis gemeinsam wechseln und `v_n` invariant bleiben. Flächenzuwachs am Rücken und Verbrauch am Graben müssen der integrierten Normalbewegung entsprechen.
- **Semantik:** kein `spreading_ridge` auf rein kontinentalen Seiten; jeder aktive Bogen referenziert genau einen Graben und liegt auf dessen Oberplatte; jede Sutur referenziert eine Kollision; keine Kontinentfläche wird am Graben gelöscht.
- **Zeit:** `start≤end`; Nachfolger startet nicht vor Vorgänger; inaktive Features bewegen sich mit ihrer Trägerplatte; Fracture-Zone-Alter konsistent mit angrenzendem Krustenalter.
- **Determinismus:** gleicher Seed/Config erzeugt identische IDs, Zustandsfolgen und kanonisch äquivalente sphärische Geometrien.
- **Plausibilitätsensemble statt Einzel-Erde:** Verteilungen von Plattenzahl, Grenzlängenanteilen, Spreizungsraten, Ozeanalter, Rückenoffsets, Graben–Bogen-Abständen und Feature-Lebensdauern über viele Seeds prüfen. MAPRIDGES bietet aktuelle globale Rücken-/Transformgeometrien als Referenzdatensatz ([Sautter et al. 2024](https://doi.org/10.17882/99981)); Seton et al. liefern Ozeanalter/Spreizung.

## Ausdrücklich nicht belastbare Aussagen

- Krustenalter, Konvergenzrate, passiver Rand, Fracture Zone oder Plume **garantieren** jeweils allein weder Rift-Erfolg noch Subduktionsstart.
- Rücken sind nicht einfach jede divergente Grenze; kontinentale Rifte können über lange Zeit divergent bleiben oder scheitern.
- Transformstörungen stehen nicht überall exakt rechtwinklig zum Rücken; sphärische Kinematik und Reorganisation erzeugen Obliquität.
- Fracture Zones sind nicht pauschal aktive Transformgrenzen und auch nicht garantiert seismisch tot.
- Ein Vulkanbogen liegt nicht in einem universell festen Abstand zum Graben; ohne Slabwinkel ist jeder Offset eine Abstraktion.
- Ozean–Ozean-Subduktionspolarität folgt nicht deterministisch allein aus dem Alter; Polaritätswechsel darf nicht spontan aus Oberflächengeometrie behauptet werden.
- Kontinentkontakt bedeutet weder augenblicklichen Slabbruch noch sofortiges Ende aller Subduktion/Magmatik.
- Orogenbreite oder -höhe folgt nicht eindeutig aus integrierter Plattenkonvergenz. Das MVP liefert keine Höhe.
- Suturen sind keine stets scharfen, gleich breiten Linien; die Linie ist kartographische Generalisierung einer komplexen Zone.
- Eine feste 100- oder 200-Myr-Löschfrist für alte Features ist wissenschaftlich nicht begründbar.

## Bibliografie

- Brune, S. et al. (2023). *Geodynamics of continental rift initiation and evolution*. Nature Reviews Earth & Environment 4, 235–253. https://doi.org/10.1038/s43017-023-00391-3
- Carbotte, S. M. et al. (2016). *Tectonic and magmatic segmentation of the Global Ocean Ridge System*. Geological Society, London, Special Publications 420. https://doi.org/10.1144/SP420.5
- England, P. et al. (2004). *Systematic variation in the depths of slabs beneath arc volcanoes*. GJI 156. https://doi.org/10.1111/j.1365-246X.2003.02132.x
- Gordon, R. G. (1998). *The plate tectonic approximation*. Annual Review of Earth and Planetary Sciences 26. https://doi.org/10.1146/annurev.earth.26.1.615
- Hensen, C. et al. (2019). *Marine Transform Faults and Fracture Zones*. Frontiers in Earth Science 7. https://doi.org/10.3389/feart.2019.00039
- Howe, M. et al. (2022). *Marine Vertical Gravity Gradients Reveal the Global Distribution and Tectonic Significance of “Seesaw” Ridge Propagation*. JGR Solid Earth. https://doi.org/10.1029/2020JB020017
- Lallemand, S. & Arcay, D. (2021). *Subduction initiation from the earliest stages to self-sustained subduction*. Earth-Science Reviews 221. https://doi.org/10.1016/j.earscirev.2021.103779
- McKenzie, D. P. & Parker, R. L. (1967). *The North Pacific: an example of tectonics on a sphere*. Nature 216. https://doi.org/10.1038/2161276a0
- Morgan, W. J. (1968). *Rises, trenches, great faults, and crustal blocks*. JGR 73. https://doi.org/10.1029/JB073i006p01959
- Norris, R. J. & Toy, V. G. (2014). *Continental transforms: A view from the Alpine Fault*. Journal of Structural Geology 64. https://doi.org/10.1016/j.jsg.2014.03.003
- Sautter, B. et al. (2024, rev. 2026). *MAPRIDGES: Global Database of Mid-Oceanic Ridge Segments and Transform Faults*. SEANOE. https://doi.org/10.17882/99981
- Seton, M. et al. (2020). *A Global Data Set of Present-Day Oceanic Crustal Age and Seafloor Spreading Parameters*. G3 21. https://doi.org/10.1029/2020GC009214
- Stern, R. J. (2002). *Subduction zones*. Reviews of Geophysics 40. https://doi.org/10.1029/2001RG000108
- Stern, R. J. (2004). *Subduction initiation: spontaneous and induced*. EPSL 226. https://doi.org/10.1016/j.epsl.2004.08.007
- Syracuse, E. M. & Abers, G. A. (2006). *Global compilation of variations in slab depth beneath arc volcanoes*. G3 7. https://doi.org/10.1029/2005GC001045
- van Hinsbergen, D. J. J. et al. (2011). *Restoration of Cenozoic deformation in Asia and the size of Greater India*. Tectonics 30. https://doi.org/10.1029/2011TC002908
- van Hinsbergen, D. J. J. et al. (2019). *Geological, geophysical and plate kinematic constraints for models of the India–Asia collision*. Earth-Science Reviews 197. https://doi.org/10.1016/j.earscirev.2019.102908
- Vine, F. J. & Matthews, D. H. (1963). *Magnetic anomalies over oceanic ridges*. Nature 199. https://doi.org/10.1038/199947a0
- Wilson, J. T. (1965). *A new class of faults and their bearing on continental drift*. Nature 207. https://doi.org/10.1038/207343a0
- Yin, A. & Harrison, T. M. (2000). *Geologic Evolution of the Himalayan–Tibetan Orogen*. Annual Review of Earth and Planetary Sciences 28. https://doi.org/10.1146/annurev.earth.28.1.211

## Sources kept/dropped

**Kept:** Peer-reviewte Primärarbeiten und Reviews mit DOI zur sphärischen Kinematik, globalen Ozeanaltern/Spreizungsraten, Riftentwicklung, Subduktionsinitiierung, Transform-/Fracture-Zone-Unterscheidung, Vulkanbogengeometrie und Kontinentkollision. MAPRIDGES wurde als offizieller, versionierter Geometriedatensatz behalten. Repository-Quellen `CONTEXT.md` und `docs/research/platten-und-superkontinentzyklus.md` bestimmten Scope und verhinderten widersprüchliche Trigger.

**Dropped:** EGU-Abstracts 2021/2025/2026, Academia.edu, DocsLib, Paperity, Presse-/Blogtexte und Lehrseiten — vorläufig, sekundär oder redundant. Einzelne 2-D/3-D Vollgeodynamikmodelle wurden nicht zur direkten Triggerkalibrierung genutzt, weil Rheologie und Manteldynamik außerhalb des MVP liegen. Extremwerte einzelner Naturbeispiele wurden nur als Reichweite, nicht als Default übernommen.

## Gaps

1. Es existiert keine robuste globale Wahrscheinlichkeitsverteilung für Länge, Breite und Lebensdauer aller Featureklassen; insbesondere Rückenlebensdauer, Transformzonenbreite und Orogen-Nachleben sind hier nur fallbasierte Größenordnungen oder Produktregeln. Die vorgeschlagenen Erzeugungsbereiche müssen gegen Ensembles, nicht gegen Einzelfälle kalibriert werden.
2. Ohne Slabgeometrie bleibt der Bogenoffset nur kartographisch plausibel. Eine spätere, weiterhin vektorielle Erweiterung könnte pro Grabensegment einen kategorialen `slab_dip_class` führen.
3. Diffuse Plattengrenzen und Mikroplatten sind mit einer einzigen Linie nur generalisiert. Wenn die finale Karte breite Deformationszonen zeigen soll, braucht sie zusätzliche Korridorpolygone ohne Änderung der Plattenpartition.
4. Orogenbreitenentwicklung ist am schwächsten fundiert, weil sie Rheologie, Erosion und dreidimensionale Fluchtbewegungen integriert. Sie muss ausdrücklich als Stil-/Proxyregel erscheinen.
5. Issue-Map-Inhalt war in den gelesenen Repository-Dokumenten nicht als separate Datei verfügbar; die genannten Wayfinder-Produktgrenzen wurden daher direkt aus dem Auftrag und `CONTEXT.md` angewendet.

## Entscheidungsreife Empfehlungen

1. **Feature-Ontologie verbindlich festschreiben:** Rift ≠ divergente Grenze ≠ Rücken; Transform ≠ Fracture Zone; Graben mit Polarität; Kollision erzeugt Sutur plus Orogenpolygon.
2. **Zustandsbehaftete Historie statt Überschreiben:** aktive und fossile Features mit Vorgänger/Nachfolger, Zeiten und Trägerplatte; nur Subduktion entfernt Geometrie physisch.
3. **Geometrie-Regeln implementieren:** sphärische Normal-/Tangentialzerlegung, geodätische Oberplattenoffsets für Bögen, asymmetrische Korridore für Orogene, advektierte Fracture Zones.
4. **Defaults aus der Tabelle übernehmen, aber als Modellannahmen serialisieren.** Besonders Rift-Timer, Subduktionsscore, Bogenoffset und Orogenbreitenregel dürfen nicht als wissenschaftlich deterministische Trigger dokumentiert werden.
5. **Finale Map-Mindestattribute:** ID, Typ, Zustand, Geometrie, Seiten-/Plattenrollen, Start/Ende, Rate/Polarität, Eltern-/Nachfolger, Erzeugungsregelversion, Evidenzklasse und Unsicherheit.
6. **Akzeptanz über Invarianten und Verteilungen:** keine Lücken/Überlappungen, korrekte Flächenbilanz, kein Kratonriss, korrekte Ober-/Unterplattenrollen, bit-deterministische Historie sowie ensemblebasierte Plausibilitätsmetriken.

## Modellannahmen

- Harte Unteilbarkeit der Kratone.
- Linienrepräsentation aktiver Grenzen trotz real diffuser Deformation.
- Numerische Kinematikschwellen und Mindestsegmentlängen.
- Rift-Erfolgszeit, Subduktionsscore/-hysterese und deterministische Tiebreaks.
- Standardpolarität Ozean unter Kontinent und regelbasierte Ozean–Ozean-Polarität.
- Konstanter bzw. kategorialer Graben–Bogen-Offset ohne Slabmodell.
- Kollisionskontakt-Schwelle und Kollision ohne Vernichtung kontinentaler Fläche.
- Orogenbreite als gekappte Funktion eines Konvergenz-Proxys, nicht als physische Gebirgsbreite/-höhe.
- Sichtbarkeitsalter für Reliktbögen/Orogene; Suturen und Fracture Zones ohne geologischen Age-out.
