# Kinematisches Plattenbewegungsmodell

Status: gemeinsam bestätigt für das MVP

Wayfinder-Ticket: [#7](https://github.com/timjonaswechler/vectorTectonic/issues/7)

Grundlagen: [fachliches Zustandsmodell](fachliches-zustandsmodell.md), [Geometrie- und Topologierepräsentation](geometrie-und-topologierepraesentation.md), [Platten- und Superkontinentzyklus](../research/platten-und-superkontinentzyklus.md) und [tektonische Kernfeatures](../research/tektonische-kernfeatures.md)

## Zweck und Abgrenzung

Dieses Dokument legt fest, wie explizite Platten als starre Körper auf der Kugel bewegt, ihre Eulerrotationen aus regelbasierten Einflüssen aktualisiert und relative Grenzbewegungen bestimmt werden. Das Modell ist deterministisch, kinematisch und bewusst nicht prädiktiv. Slab Pull, Ridge Push und Kollisionswiderstand sind kalibrierte Einflüsse auf gewünschte Oberflächengeschwindigkeiten, keine berechneten physikalischen Kräfte.

Dieses Dokument legt Beginn, Ende, Auswahl und fachliche Face-Zuordnung tektonischer Ereignisse nicht selbst fest; diese sind in den Feature-Lebenszyklen und den [automatischen tektonischen Ereignisregeln](automatische-tektonische-ereignisregeln.md) getrennt spezifiziert. Nicht festgelegt werden Persistenz (#11) oder die konkreten numerischen Integrations- und Akzeptanzbudgets (#13). Mantelkonvektion, Spannungsfelder, Rheologie, Massenträgheit und quantitative Kraftbilanzen bleiben außerhalb des MVPs.

## 1. Autoritativer Bewegungszustand

Jeder gültige Simulationszeitpunkt besitzt neben dem `SurfaceSnapshot` genau einen kanonischen `KinematicState`. Er enthält für jede aktuelle explizite Platte genau einen Bewegungsdatensatz und keinen für den nicht aufgelösten Hintergrund oder beendete Platten. Snapshot und Bewegungszustand beziehen sich auf dieselbe Simulationszeit und dieselbe Menge aktueller Plattenidentitäten. Eine Abweichung ist ungültig.

Die kanonische Bewegung einer Platte ist ein dreidimensionaler Winkelgeschwindigkeitsvektor `ω`:

- Seine Richtung bezeichnet die orientierte Rotationsachse.
- Sein Betrag ist die Winkelgeschwindigkeit.
- Eulerpol und Rotationsrate sind abgeleitete Diagnose- und Darstellungswerte und werden nicht redundant autoritativ gespeichert.
- Für einen Einheitsvektor `r` auf der Kugel mit physischem Radius `R` ist die lineare Oberflächengeschwindigkeit `v(r) = R (ω × r)`.
- Der Nullvektor bezeichnet kanonischen Stillstand und besitzt keinen Eulerpol.

Der kanonische Bewegungszustand geht in den deterministischen Zustands-Hash ein. Seine dauerhafte Serialisierung bleibt #11 vorbehalten.

## 2. Bezugssysteme

### 2.1 Initialisierungsphase

Während der Initialisierung ist der nicht aufgelöste Hintergrund die unbewegte Referenz mit `v = 0`. Er erhält weder Plattenidentität noch `ω`, Einflussbeiträge oder einen sonstigen Bewegungszustand. Grenzkinematik zwischen expliziter Platte und Hintergrund wird trotzdem nach denselben relativen Formeln wie zwischen zwei Platten berechnet.

Die Seed-Platte erhält einmalig einen isotrop aus Welt-Seed und stabiler Plattenidentität bestimmten Eulerpol. Der Betrag ihres Vektors wird so gewählt, dass `R|ω|` ihrer konfigurierten Startgeschwindigkeit entspricht; diese beträgt standardmäßig 40 mm/a und darf zwischen 20 und 60 mm/a liegen. Der Pol wird nicht nachträglich neu ausgelost. Eine ungeeignete Initialisierung muss durch Ereignis- oder Akzeptanzregeln erkennbar werden.

### 2.2 Reifephase

Nach Ende der Initialisierung gilt ein flächenintegriertes No-Net-Rotation-Bezugssystem. Für eine Platte `P` ist ihr sphärischer Formtensor

`J(P) = ∫_P (I − r rᵀ) dA`.

Die globale Best-Fit-Starrrotation wird aus den Formtensoren und Plattenvektoren bestimmt und entfernt. Ein bloß nach Plattenflächen gewichtetes Mittel der `ω`-Vektoren genügt nicht. Das Entfernen einer gemeinsamen Starrrotation verändert keine relative Grenzbewegung.

No-Net-Rotation, Höchstgeschwindigkeiten und kanonischer Stillstand werden als gemeinsame deterministische Least-Squares-Projektion gelöst. Für vorgeschlagene Vektoren `ω̂_i` minimiert jeder Projektionslauf

`E_project = Σ_i ∫_{P_i} |R((Ω_i − ω̂_i) × r)|² dA`

unter `R|Ω_i| ≤ v_max` und in der Reifephase zusätzlich `Σ_i J(P_i) Ω_i = 0`. Während der Initialisierung entfällt nur diese No-Net-Rotation-Gleichung.

Stillstand wird durch einen monotonen aktiven Satz bestimmt: Zuerst wird die Aufgabe ohne Stillstandsfixierungen eindeutig gelöst. Alle Platten mit `R|Ω_i| < 1 mm/a` werden gleichzeitig, nach stabiler Platten-ID protokolliert, auf `Ω_i = 0` fixiert und die vollständige Aufgabe erneut gelöst. Neu unter die Schwelle fallende Platten werden ergänzt; bereits fixierte werden nicht wieder freigegeben. Das endet nach höchstens der Zahl aktueller Platten. Existiert kein eindeutiges endliches Minimum oder ist der jeweilige Bedingungssatz unlösbar, scheitert der Tick strukturiert.

Die flächenintegrierte quadratische Metrik ist dieselbe für die Projektion nach `prepare_step` und nach `reconcile_after_commit`. Bedingungen dürfen nicht nacheinander so angewandt werden, dass eine spätere Korrektur eine frühere wieder verletzt.

## 3. Kinematische Einflüsse

### 3.1 Gemeinsames Prinzip

Eine Platte behält ohne aktive Einflüsse ihre zuletzt gültige Bewegung. Es gibt keine periodische Zufallsbewegung und keine Dämpfung gegen ein absolutes Ruhesystem.

Aktive Einflüsse werden als gewünschte lokale Grenzgeschwindigkeiten formuliert. Eine geometrisch gewichtete Least-Squares-Ausgleichsrechnung bestimmt daraus den Zielvektor `ω_target`, während eine flächenintegrierte Regularisierung die aktuelle Bewegung bevorzugt. Die Normalform dieser Rechnung entspricht einem längenintegrierten kinematischen Drehmoment, das über den Formtensor der Platte in eine Rotation übersetzt wird.

Dadurch gelten gleichzeitig:

- längere aktive Grenzsegmente tragen stärker bei,
- große und unterschiedlich geformte Platten reagieren entsprechend ihrer gesamten Geometrie,
- konstante Einflüsse führen zu einem begrenzten kinematischen Zielzustand statt zu fortlaufender Beschleunigung und
- ohne Einflüsse ist der aktuelle Vektor selbst wieder das Ziel.

Ein gemeinsamer `driver_speed_scale` übersetzt die dimensionslosen Gewichte in eine gewünschte lineare Geschwindigkeit. Er beträgt standardmäßig 20 mm/a und darf zwischen 5 und 40 mm/a liegen. Er wird pro Lauf fest konfiguriert und nicht anhand des laufenden Ergebnisses nachgeregelt.

Das MVP besitzt einen geschlossenen Einflusskatalog:

1. Slab Pull,
2. Ridge Push und
3. Kollisionswiderstand.

Bewegungsvererbung bei Plattenabstammung ist eine gesonderte Zustandsübergangsregel und kein Einflussbeitrag. Transformgrenzen, passive Ränder, Suturen und Fracture Zones erzeugen keinen eigenen Bewegungsbeitrag. Sie dürfen spätere Ereigniskandidaten beeinflussen, aber keine Scheinkraft darstellen.

### 3.2 Slab Pull

Slab Pull wirkt ausschließlich auf die explizite Unterplatte eines aktiven Grabensegments. Die gewünschte Bewegungsrichtung zeigt lokal innerhalb der Tangentialebene normal von der Unterplatte zum Graben. Die Oberplatte erhält keinen entgegengesetzten Slab-Pull-Beitrag.

Der längengewichtete Beitrag hat das Grundgewicht `1,0` und wird mit Krustenalter und Subduktionsreife multipliziert:

- Alter unter 40 Ma: Faktor `0,5`,
- 40 bis 80 Ma: linearer Anstieg von `0,5` auf `1,0`,
- ab 80 Ma: Faktor `1,0`.

Nicht aufgelöste alte Ozeankruste behält fachlich unbekanntes Alter, wird für diesen Einfluss aber mit einem konfigurierten Ersatzalter von 80 Ma bewertet.

Nach Beginn einer Subduktionszone steigt das Reifegewicht linear von null auf eins. Die Reifezeit beträgt standardmäßig 10 Myr und darf zwischen 5 und 20 Myr liegen. Der Umgang mit kurzen Unterbrechungen folgt der in Feature-Lebenszyklen und Ereignisregeln festgelegten Feature-Hysterese. Alter und Slab Pull können eine Subduktion nicht selbst initiieren; sie beeinflussen nur eine bereits aktive, rollenbestimmte Zone.

### 3.3 Ridge Push

Ridge Push wirkt an einem aktiven Rückensegment auf beide angrenzenden expliziten Platten jeweils lokal normal von der Rückenachse weg. Beide Seiten erhalten dieselbe Einflussdichte pro Segmentlänge. Das Grundgewicht beträgt `0,25`, also ein Viertel des voll ausgereiften Slab-Pull-Gewichts.

Spreizungsrate und Krustenalter verstärken diesen Beitrag nicht zusätzlich. Damit entsteht keine positive Rückkopplung, bei der hohe Öffnung automatisch noch höhere Öffnung erzwingt. Ridge Push gilt nur für ein bereits aktives Rückenfeature; ein Rift oder eine bloß divergente Grenze ist noch kein Rücken.

### 3.4 Kollisionswiderstand

Eine aktive Kontinentkollision wirkt gekoppelt auf beide beteiligten Platten und setzt ausschließlich für deren vorhergesagte konvergente Normalkomponente ein abweichendes Ziel. Ihr tangentiales Ziel bleibt gegenüber der vorläufigen Bewegung unverändert. Das Kollisionsziel selbst kehrt die Normalkomponente niemals um und erzeugt keine Divergenz; die gekoppelte Ausgleichsrechnung, andere Einflüsse und globale Nebenbedingungen können die tatsächlich resultierenden Komponenten dennoch verändern. Ziel und Ist werden deshalb getrennt diagnostiziert und spätere Folgen ausschließlich durch Feature-Lebenszyklen und Ereignisregeln entschieden.

Für den kumulativen `shortening_proxy` in Kilometern ist der entgegenwirkende Anteil

`c = 0,5 + 0,4 · clamp(shortening_proxy / 500 km, 0, 1)`.

Er beträgt somit zu Beginn 50 %, steigt linear und erreicht bei 500 km höchstens 90 %. Die Beiträge werden über die aktuelle kollidierende Grenzlänge integriert. Kontaktlänge, Prozentsätze und Verkürzungsmaß sind Modellannahmen und keine rheologische Berechnung.

### 3.5 Eindeutige Zielrechnung

Alle linearen Geschwindigkeiten werden in derselben physischen Einheit ausgewertet; Winkelgeschwindigkeiten werden dafür mit `R` in Oberflächengeschwindigkeiten übersetzt. Für Kandidatenvektoren `Ω_i` minimiert die Zielrechnung auf der Einheitskugel die folgende vollständig bestimmte quadratische Abweichung:

`E = Σ_i ∫_{P_i} |R((Ω_i − ω_i) × r)|² dA + Σ_{d∈D} w_d ∫_{Γ_d} (u_d · R(Ω_i × r) − s_d)² dl + Σ_{c∈C} ∫_{Γ_c} [((v_rel(Ω) · n) − s_c)² + ((v_rel(Ω) · t) − q_c)²] dl`.

Dabei gelten ohne zusätzliche Normalisierung:

- `d` ist eine Slab-Pull- oder Ridge-Push-Beobachtung auf genau einer Platte, `u_d` ihre bestätigte lokale Richtung und `s_d = driver_speed_scale` ihre gewünschte Geschwindigkeit.
- `w_d` ist für Slab Pull `1,0 · Altersfaktor · Reifefaktor` und für Ridge Push `0,25`.
- Flächen `dA` werden in Steradiant und Längen `dl` im Winkelmaß der Einheitskugel eingesetzt. Weil beide dimensionslos sind, legt die Formel selbst das Verhältnis von Flächenpersistenz und Grenzbeitrag fest; es gibt keinen verborgenen Normierungs- oder Mobilitätsparameter.
- `D` und `C` enthalten jedes aktive, rollenbestimmte Segment genau einmal je vorgesehener Plattenwirkung.

Für Kollisionssegmente wird zuerst dieselbe Minimierung ohne den letzten Summanden gelöst. Aus diesem eindeutigen vorläufigen Ziel folgen die lokalen Komponenten `v_n⁰` und `v_t⁰`. Ist `v_n⁰` konvergent, lautet die gewünschte Kollisionsrate `s_c = (1 − c) v_n⁰`; andernfalls ist `s_c = v_n⁰`, sodass die Kollision keine Divergenz erzeugt. Das tangentiale Ziel ist stets `q_c = v_t⁰`. Danach wird die vollständige oben stehende Zielfunktion einmal gelöst. Der Kollisionssummand hat Gewicht `1,0`; eine weitere Iteration findet nicht statt.

Die Integrale werden über die vollständige aktuelle Plattenfläche beziehungsweise aktive Grenzlänge gebildet. Ihre Normalgleichungen liefern die zuvor beschriebene Formtensor-/Drehmomentdarstellung. Besitzt die quadratische Form ohne die anschließend geltenden globalen Nebenbedingungen kein eindeutiges endliches Minimum, entsteht ein strukturierter Fehler. Konkrete Solver- und Quadraturverfahren bleiben Implementierungsdetails, müssen aber dieses mathematische Ergebnis innerhalb der in #13 festgelegten Budgets reproduzieren.

## 4. Aktualisierung pro Tick

### 4.1 Ziel und Relaxation

Aus aktuellem Bewegungszustand, gültigem Snapshot, aktiven Feature-Rollen und Laufkonfiguration wird genau einmal zu Tickbeginn der Zielzustand berechnet. Der aktuelle Vektor nähert sich ihm exponentiell:

`α = 1 − exp(−Δt / τ)`

`ω_relaxed = ω_current + α (ω_target − ω_current)`.

Die Relaxationszeit `τ` beträgt standardmäßig 5 Myr und darf zwischen 2 und 10 Myr liegen. Sie bezeichnet kinematische Glättung, nicht physikalische Trägheit. Die Formel hält das Verhalten über zulässige Zeitschritte von 0,25 bis 1 Myr mit 0,5 Myr Default vergleichbar.

Nach der Relaxation werden die gemeinsamen Bezugssystem-, Höchstgeschwindigkeits- und Stillstandsbedingungen gelöst. Als maximale Oberflächengeschwindigkeit einer Platte gilt die größte Geschwindigkeit ihres Rotationsfelds auf der gesamten Kugel, also `R|ω|`, nicht nur das Maximum über ihrem aktuellen Polygon. Sie beträgt standardmäßig 100 mm/a und darf zwischen 50 und 150 mm/a konfiguriert werden. Entsprechend bezeichnet kanonischer Plattenstillstand `R|ω| < 1 mm/a`. Eine nötige Begrenzung erhält die bevorzugte Rotationsachse soweit mit den globalen Nebenbedingungen vereinbar.

Der resultierende Vektor bleibt während des gesamten Ticks konstant. Es gibt keine Rückkopplungsiteration innerhalb desselben Ticks.

### 4.2 Endliche Rotation

Die räumliche Fortschreibung erfolgt durch die exakte endliche Rotation `exp(ω Δt)`, nicht durch lineare Aktualisierung einzelner Vertices. Eine gemeinsame Rotation erhält Kugellage, Winkel, Großkreisbögen und innere Plattengeometrie.

Unterschiedlich bewegte Platten liefern konkurrierende Randvorschläge an die atomare Overlay- und Ereignisauflösung. Die Kinematik entscheidet weder neue Face-Zuordnungen noch Ereignisprioritäten. Die Ereignisregeln dürfen ausschließlich an qualifizierender featureloser Konvergenz und aktiver Kontinentkollision eine lokale flächenneutrale Akkommodation anwenden; die tatsächliche relative Bewegung bleibt dabei für Zeitnachweis beziehungsweise Verkürzungsproxy maßgeblich. Nur ein vollständig gültiger atomarer Commit darf den nächsten `SurfaceSnapshot` erzeugen.

Konfigurierte Rift- und Spreizungsraten sind Zielraten. Grenzklassifikation, Krustenerzeugung und Krustenverbrauch verwenden ausschließlich die nach Relaxation und Nebenbedingungen tatsächlich erreichte relative Bewegung. Ziel-/Ist-Abweichungen werden diagnostiziert und nicht durch zusätzliche Geometrie erzwungen.

## 5. Plattenabstammung und Bewegungsvererbung

Reine Flächen- oder Formänderungen einer fortbestehenden Platte verwenden ihren im abgeschlossenen Tick tatsächlich gültigen `ω`-Vektor als unverändertes Vererbungsziel. Die gemeinsame Abstammungsrechnung und anschließende globale Projektion dürfen davon nur abweichen, soweit gleichzeitig bestätigte Zielraten oder Bezugssystem-, Stillstands- und Geschwindigkeitsbedingungen dies erfordern; jede Abweichung wird diagnostiziert. Beendete Platten verlieren ihren Bewegungsdatensatz; vollständig verbrauchte Platten besitzen keinen Nachfolgerzustand.

### 5.1 Teilung

Bei einer Teilung erben alle Nachfolger zunächst den Vektor der Vorgängerplatte. Bei erfolgreichem Break-up werden zusätzlich entgegengesetzte Rotationsanteile durch eine deterministische Least-Squares-Anpassung über die gesamte neue Grenze bestimmt. Sie reproduzieren die ereignisbedingte volle Zielöffnungsrate bestmöglich und werden symmetrisch auf beide Nachfolger verteilt. Gemeinsame Drift bleibt dadurch erhalten.

### 5.2 Verschmelzung

Bei einer Verschmelzung erhält der neue Nachfolger den Vektor, der die unmittelbar zuvor gültigen Oberflächengeschwindigkeiten aller Vorgänger über die vereinigte Fläche flächengewichtet bestmöglich approximiert. Eulerpole werden niemals direkt gemittelt.

### 5.3 Gemeinsame Auflösung

`reconcile_after_commit` verarbeitet sämtliche Teilungen, Verschmelzungen und ereignisbedingten Zielraten eines atomaren Commits in einer einzigen deterministischen Ausgleichsrechnung. Für jeden Nachfolger `j` und Vorgänger `p` bezeichnet `Q_pj` den durch die technische Provenienz eindeutig ausgewiesenen Teil der neuen Plattenfläche, der von `p` stammt. Kandidatenvektoren `Ω_j` minimieren

`E_reconcile = Σ_j Σ_{p∈pred(j)} ∫_{Q_pj} |R(Ω_j × r) − R(ω_p^step × r)|² dA + Σ_e ∫_{Γ_e} (((v_right(Ω) − v_left(Ω)) · n) − s_e)² dl`.

Dabei gelten folgende feste Regeln:

- `ω_p^step` ist der im abgeschlossenen `KinematicStep` tatsächlich verwendete Vektor des Vorgängers.
- `e` läuft über jede vom Commit bestätigte kinematische Zielrate genau einmal; `s_e` ist ihre signierte normale Zielrate. Alle Zielraten haben dasselbe Gewicht pro Winkelstrecke, sofern die Ereignisregeln sie im selben Commit zugelassen haben.
- Bei einem binären Split mit Vorgänger `p` und Nachfolgern `a`, `b` gilt zusätzlich die harte Symmetriebedingung `Ω_a − ω_p^step = −(Ω_b − ω_p^step)` vor der globalen Bezugssystemprojektion.
- Ein unverändert fortbestehender Plattenteil wird durch seine eigene Provenienz wie jeder andere Vorgänger behandelt. Jede Fläche eines Nachfolgers muss genau einer Vorgängerprovenienz zugeordnet sein; eine vorgängerlose neue Platte außerhalb des Seed-Zustands ist ungültig.
- Mehrere Provenienzanteile einer Verschmelzung werden ausschließlich durch ihre tatsächlichen Flächenintegrale gewichtet; Eulerpole werden nicht gemittelt.

Anschließend werden die für die Phase geltenden Bezugssystem-, Stillstands- und Geschwindigkeitsbedingungen gemeinsam angewandt. Eine sequentielle, von Container- oder Traversierungsreihenfolge abhängige Vererbung ist unzulässig. Besitzt die quadratische Aufgabe unter ihren harten Bedingungen kein eindeutiges endliches Minimum oder widersprechen sich harte Bedingungen, verhindert ein strukturierter Fehler den Commit.

Wann eine Teilung oder Verschmelzung eintritt und welche Änderungen gemeinsam in einem Commit erlaubt sind, legen die automatischen Ereignisregeln fest; eine generische Reorganisation ist unzulässig.

## 6. Relative Grenzkinematik

Für einen Punkt `r` auf einer kanonisch orientierten Plattengebietsgrenze seien `t` die Linientangente, links Platte oder Gebiet `A`, rechts Platte oder Gebiet `B` und

`n = t × r`

​​die tangentiale Normale von links nach rechts. Mit `v_A` und `v_B` aus den tatsächlichen Tickrotationen gilt

`v_rel = v_B − v_A`,

`v_n = v_rel · n`,

`v_t = v_rel · t`.

`v_n > 0` bedeutet Öffnung, `v_n < 0` Schließung. Bei Umkehr der Grenzorientierung wechseln Seiten, Tangente, Normale und relative Geschwindigkeit gemeinsam; `v_n` bleibt invariant. Der Hintergrund geht mit `v = 0` ein.

Die Kinematik liefert signierte Rohwerte, längengewichtete Segmentstatistiken und die rein kinematische Klassifikation:

1. Unter 1 mm/a Gesamtgeschwindigkeit gilt das Segment als ruhend.
2. Sonst ist `f_n = |v_n| / hypot(v_n, v_t)`.
3. Bei `f_n ≤ 0,2` ist es transform.
4. Andernfalls ist es nach dem Vorzeichen von `v_n` divergent oder konvergent.
5. Bei `f_n < 0,8` und nicht ruhender Normalbewegung erhält es zusätzlich den Zustand `oblique`, mit transtensionaler beziehungsweise transpressionaler Richtung.

Die Schwellen sind versionierte Auflösungsparameter und keine Naturgrenzen. Rohwerte bleiben unabhängig von der Klasse erhalten.

Eine lange Grenze wird an Nullstellen und Klassifikationsschwellen deterministisch geteilt. Pro zusammenhängender Grenzkomponente werden Abschnitte unter 25 km anschließend mit folgender festen Regel beseitigt:

1. Wähle den aktuell kürzesten zu kurzen Abschnitt; Gleichstände entscheidet sein orientierungsunabhängiger kanonischer Geometrieschlüssel.
2. Vereinige ihn mit dem längeren seiner unmittelbar angrenzenden Abschnitte und übernimm dessen Klasse. Bei gleichen Nachbarlängen gewinnt der kleinere orientierungsunabhängige Geometrieschlüssel; am offenen Ende existiert nur ein Kandidat.
3. Fasse danach direkt benachbarte Abschnitte gleicher Klasse zusammen und wiederhole, bis kein zu kurzer Abschnitt verbleibt.
4. Besteht eine geschlossene oder offene Komponente nur aus einem Abschnitt, bleibt dieser unabhängig von seiner Länge erhalten.

Verschiedene Abschnitte derselben Plattengrenze dürfen unterschiedliche Klassen besitzen.

## 7. Zeitintegrierter Grenzfluss

Die erwartete Öffnungs- oder Schließungsfläche eines Ticks ergibt sich aus der zeitlichen und räumlichen Integration der tatsächlichen relativen Normalbewegung über eine kanonische symmetrische Referenzkurve. Für jeden Zeitpunkt `t` wird die kürzeste eindeutige Rotationsinterpolation in `SO(3)` zwischen den endlichen Rotationen der linken und rechten Seite gebildet; für den Hintergrund steht die Identitätsrotation. Ihre Mitte advektiert die gemeinsame Startgrenze `Γ₀` zu `Γ_m(t)`. Relative Geschwindigkeit, Tangente, Normale und Linienelement werden an derselben Kurve ausgewertet, wobei `v_i(r) = R(ω_i × r)` das überall definierte Rotationsfeld der jeweiligen Seite bezeichnet.

Die autoritative signierte Flussfläche ist

`F = ∫₀^Δt ∫_{Γ_m(t)} (v_B(r,t) − v_A(r,t)) · n_m(r,t) dl dt`.

Die zulässigen Zeitschritte und Geschwindigkeitsgrenzen garantieren, vorbehaltlich des Qualitätsbudgets aus #13, die eindeutige kürzeste Rotationsinterpolation. Ist sie nicht eindeutig oder kann ihre Eindeutigkeit nicht nachgewiesen werden, scheitert der Tick. Der Tickanfangswert `v_n Δt` allein ist nicht autoritativ, obwohl `ω` innerhalb des Ticks konstant bleibt.

Der `KinematicStep` liefert für jedes Segment:

- den signierten integrierten Normalfluss,
- getrennte Öffnungs- und Schließungsanteile, falls die Klasse innerhalb des Integrationsintervalls wechselt,
- die verwendete deterministische Unterteilung und
- eine numerische Fehlerabschätzung.

Die Overlay-/Ereignisauflösung muss erzeugte beziehungsweise verbrauchte Fläche gegen diesen Fluss prüfen. Tangentiale Bewegung trägt exakt null zum normalen Flächenfluss bei. Das konkrete adaptive Integrations- und Fehlerbudget wird in #13 festgelegt.

## 8. Mitbewegung von Kruste und Features

Krustengebundene Geometrien werden durch die exakte Rotation ihres eindeutigen aktuellen Trägers starr advektiert. Das gilt auch für intraplatteninterne Relikte wie Suturen und Fracture Zones. Einzige Ausnahme sind die in den Ereignisregeln festgelegten lokalen flächenneutralen Akkommodationen, die Krustenfläche, Kratone und Zwangskonturen erhalten müssen. Ein vollständiger Trägerwechsel bindet sie im nächsten gültigen Zustand an die Bewegung des neuen Trägers; eine räumliche Teilung folgt den bereits festgelegten fachlichen Nachfolgerregeln.

Grenzgebundene Features besitzen keinen eigenen Bewegungsvektor und keine Eigentümerplatte. Die angrenzenden Platten liefern Randvorschläge; die neue gemeinsame Feature-Geometrie und ihre Bindung werden erst atomar mit der Topologie- und Ereignisauflösung festgelegt.

## 9. Modulkandidat und Interface

`PlateKinematics` ist ein **Deepening Candidate**. Seine geplante kleine caller-facing Schnittstelle verbirgt Eulerkinematik, Formtensoren, Einflussintegration, Relaxation, globale Nebenbedingungen, Grenzprofile und Flussintegration hinter zwei konzeptionellen Operationen:

### `prepare_step(...) → KinematicStep`

Eingaben:

- unveränderlicher gültiger `SurfaceSnapshot`,
- dazu passender unveränderlicher `KinematicState`,
- aktive, rollenbestimmte Feature-Beziehungen,
- unveränderliche versionierte Kinematikkonfiguration,
- aktuelle Simulationszeit und `Δt`.

Vollständiges Ergebnis:

- tatsächlicher `ω`-Vektor und endliche Rotation jeder aktuellen Platte,
- Ziel- und Ist-Bewegungen,
- relative Grenzprofile, Klassen und zeitintegrierte Flüsse,
- advektierte Geometrievorschläge und
- vollständige Diagnosen.

### `reconcile_after_commit(...) → KinematicState`

Eingaben:

- vorheriger Bewegungszustand,
- der vollständig berechnete `KinematicStep` mit den während des Ticks tatsächlich verwendeten Vektoren,
- vollständig gültiger atomar committierter oder für eine Ereignisprobe committierbarer Nachfolge-Snapshot,
- bestätigte Plattenabstammungen und
- durch den Commit festgelegte kinematische Zielraten.

Ergebnis ist genau ein vollständiger Bewegungszustand für den neuen Snapshot oder ein strukturierter Fehler. Wegen der reinen Schnittstelle darf die Ereignisauflösung dieselbe Rechnung gegen einen vollständig validierten Probe-Nachfolger verwenden, um Öffnungsziele vor Auswahl zu prüfen und zu bewerten. Ein solches Probe-Ergebnis wird nicht veröffentlicht und darf für verschiedene Kandidaten beliebig oft rein ausgewertet werden; nach dem tatsächlichen Commit wird die Rechnung genau einmal als kanonische Reconciliation für den veröffentlichten Nachfolger ausgeführt. Der im `KinematicStep` tatsächlich verwendete Vektor bildet das Vererbungsziel jeder fortbestehenden Platte; Abstammungsregeln und die gemeinsame Projektion transformieren diesen Tickendzustand nachvollziehbar für den neuen Gesamtzustand.

Beide Operationen sind rein bezüglich ihrer Eingaben: Sie verändern weder Snapshot noch Bewegungszustand. Halbfertige Ergebnisse verlassen die Schnittstelle nicht. Aufrufer und Tests verwenden dieselbe Schnittstelle.

Diese Architekturbezeichnung bleibt bis zu Implementierung, realen Callern und Schnittstellentests bewusst ein Kandidatenstatus. Beim Löschen des Modulkandidaten würden Bezugssystemlogik, Einflussintegration, sphärische Geschwindigkeitsberechnung, Nebenbedingungen und Grenzflussberechnung in Ereignislogik, Topologie und Tests ausweichen.

## 10. Fehlervertrag und Diagnosen

Folgende Bedingungen erzeugen mindestens einen strukturierten Fehler und niemals eine stille Korrektur:

- Snapshot und Bewegungszustand haben verschiedene Zeitpunkte oder Plattenmengen,
- eine aktuelle explizite Platte besitzt keinen oder mehr als einen Bewegungsdatensatz,
- Hintergrund oder beendete Platte besitzt einen Bewegungsdatensatz,
- nicht endliche Vektoren, Geschwindigkeiten, Tensoren oder Integrale,
- fehlende oder widersprüchliche Ober-/Unterplatten- oder Kollisionsrollen,
- singuläre beziehungsweise außerhalb des Qualitätsbudgets schlecht konditionierte Formtensoren,
- unlösbare Abstammungs-, Zielraten-, No-Net-Rotation-, Stillstands- oder Geschwindigkeitsbedingungen,
- unerreichbare erforderliche Integrationsgenauigkeit und
- Parameter außerhalb ihrer bestätigten Bereiche.

Jeder erfolgreiche `KinematicStep` diagnostiziert mindestens:

- Ausgangs-, Ziel- und Ist-`ω` je Platte,
- Beiträge nach Feature-ID und Einflussart,
- Alters- und Reifegewichte,
- Relaxationsanteil,
- Bezugssystem-, Stillstands- und Geschwindigkeitskorrekturen,
- Ziel-/Ist-Raten und Nebenbedingungsresiduen,
- Grenzprofile, Segmentzusammenfassungen und integrierte Flüsse sowie
- deterministische Sortier-, Unterteilungs- und Fehlerangaben.

Diese Diagnose gehört nicht zur fachlichen Identität oder finalen Feature-Map, ist aber Teil der Erklärbarkeit eines Laufs.

## 11. Parameter

| Parameter | zulässiger Bereich | Default | Status |
|---|---:|---:|---|
| Zeitschritt `Δt` | 0,25–1 Myr | 0,5 Myr | numerische Modellannahme |
| Relaxationszeit `τ` | 2–10 Myr | 5 Myr | kinematische Modellannahme |
| Seed-Plattengeschwindigkeit | 20–60 mm/a | 40 mm/a | Modellannahme |
| maximale Plattengeschwindigkeit | 50–150 mm/a | 100 mm/a | Modellannahme |
| kanonischer Stillstand | fest 1 mm/a | 1 mm/a | Auflösungsannahme |
| `driver_speed_scale` | 5–40 mm/a | 20 mm/a | Modellannahme |
| Slab-Pull-Grundgewicht | fest | 1,0 | Modellannahme |
| Ridge-Push-Grundgewicht | fest | 0,25 | Modellannahme |
| Slab-Pull-Reifezeit | 5–20 Myr | 10 Myr | Modellannahme |
| Ersatzalter nicht aufgelöster Kruste | fest | 80 Ma | Modellannahme ohne Alterszuschreibung |
| Kollisionswiderstand | 50–90 % | 50 %, linear bis 90 % bei 500 km | Modellannahme |
| Transformgrenze `f_n` | fest | ≤ 0,2 | Auflösungsannahme |
| Obliquität | fest | `f_n < 0,8` | Auflösungsannahme |
| minimale eigenständige Abschnittslänge | fest | 25 km | Auflösungsannahme |

Sämtliche konfigurierbaren Bereiche sind harte Validierungsgrenzen. Werte werden weder still geklemmt noch automatisch korrigiert. Die gesamte Konfiguration trägt eine Regelversion und ist Bestandteil der deterministischen Laufeingabe.

## 12. Determinismus- und Gültigkeitsinvarianten

Für gleiche kanonische Eingaben, Welt-Seed, Regelversion und gepinnte numerische Umgebung gelten mindestens:

1. genau derselbe kanonische `KinematicState`,
2. dieselben Ziel- und Ist-Rotationen,
3. dieselben Einflussbeiträge und Abstammungsanpassungen,
4. dieselben Grenzsegmente, Klassen und integrierten Flüsse,
5. keine Abhängigkeit von HashMap-, Traversierungs-, Parallelisierungs- oder Einfügereihenfolge,
6. keine Zufallsziehung außerhalb der einmaligen, stabil adressierten Seed-Erzeugung,
7. starre innere Plattenbewegung ohne Integrationsdrift,
8. No-Net-Rotation nach der Initialisierung,
9. Hintergrundstillstand während der Initialisierung,
10. keine Geschwindigkeit außerhalb der konfigurierten Grenzen und
11. Flächenproduktion beziehungsweise -verbrauch ausschließlich aus der tatsächlichen integrierten Normalbewegung.

Kandidaten und Gleichstände werden mit stabilen fachlichen Identitäten kanonisch sortiert. Plattformübergreifende Bitidentität, konkrete Residuen, Konditionsgrenzen und Integrationsbudgets werden in #13 als Akzeptanzstandard festgelegt.

## 13. Bewusst vertagte Entscheidungen

Diese Spezifikation entscheidet ausdrücklich nicht:

- wann Rifte, Rücken, Gräben, Kollisionen oder andere Features beginnen, enden oder ihren Zustand wechseln (#8),
- wie Bewegungszustände, Diagnosen oder Zwischenstände dauerhaft gespeichert werden (#11),
- welche absoluten Integrationsfehler, Tensor-Konditionen, Laufzeiten oder Ensembleverteilungen akzeptiert werden (#13) und
- welche konkrete interne Datenstruktur oder numerische Solverimplementierung `PlateKinematics` verwendet.

Damit ist das kinematische Plattenbewegungsmodell festgelegt; die darauf aufbauende Ereignisdynamik ist in den [automatischen tektonischen Ereignisregeln](automatische-tektonische-ereignisregeln.md) getrennt spezifiziert, ohne Persistenz oder Implementierung vorwegzunehmen.
