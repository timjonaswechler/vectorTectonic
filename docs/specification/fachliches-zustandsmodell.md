# Fachliches Zustandsmodell der Kugeloberfläche

Status: gemeinsam bestätigt für das MVP

Wayfinder-Ticket: [#5](https://github.com/timjonaswechler/vectorTectonic/issues/5)

## Zweck und Abgrenzung

Dieses Dokument legt die fachlichen Entitäten, Identitäten, Beziehungen und Invarianten des jeweils gültigen Oberflächenzustands fest. Es beschreibt weder Datenstrukturen noch sphärische Algorithmen, Bewegungsformeln, Ereignistrigger, Persistenzformate oder numerische Toleranzen. Diese Entscheidungen bleiben den nachfolgenden Wayfinder-Tickets vorbehalten.

## 1. Zwei gekoppelte Flächengliederungen

Jeder Punkt der Kugeloberfläche gehört jederzeit genau

1. einer aktuellen **Krustenregion** und
2. einem aktuellen **Plattengebiet**.

Beide Gliederungen sind lückenlos und flächig überlappungsfrei. Sie beschreiben verschiedene Sachverhalte:

- Krustenregionen unterscheiden materielle Herkunftskörper kontinentaler oder ozeanischer Kruste.
- Plattengebiete unterscheiden die Bewegungszuordnung zu einer expliziten Platte oder während der Initialisierung zum nicht aufgelösten Hintergrund.

Die Gliederungen sind durch eine Tragbeziehung gekoppelt: Eine Krustenregion wird vollständig von genau einem Plattengebiet getragen. Eine Platte darf viele Krustenregionen beider Krustenarten tragen. Wird nur ein Teil einer Krustenregion einem anderen Plattengebiet zugeordnet, endet die Region durch Teilung und ihre getrennten Nachfolger erhalten jeweils eine eindeutige Tragbeziehung.

Tektonische Features überlagern diese Gliederungen. Sie erzeugen keine dritte Besitzpartition.

## 2. Identität und Herkunft

Platten, Krustenregionen und tektonische Features besitzen innerhalb eines Laufs dauerhaft eindeutige, niemals wiederverwendete fachliche Identitäten. Gleichartige oder geometrisch gleiche spätere Objekte erhalten neue Identitäten.

Abstammungsbeziehungen sind azyklisch:

- **Platten** können durch Teilung mehrere Nachfolger oder durch Verschmelzung einen Nachfolger mit mehreren Vorgängern erzeugen.
- **Krustenregionen** können durch Teilung mehrere Nachfolger erzeugen, verschmelzen jedoch niemals. Jeder Nachfolger besitzt deshalb höchstens einen unmittelbaren Vorgänger.
- **Features** können durch Typwechsel, Teilung oder Vereinigung neue Nachfolger mit mehreren Vorgängern oder Nachfolgern erzeugen.

Beendete Entitäten gehören nicht mehr zur aktuellen Oberflächenbelegung. Ein minimaler Herkunftsnachweis erhält ihre Identität, Art, Beginn, Ende sowie Vorgänger und Nachfolger, aber keine frühere Geometrie. Eine vollständige Simulationshistorie ist nicht Teil dieses Zustandsmodells.

## 3. Platten und Plattengebiete

Eine explizite Platte ist stets flächenhaft zusammenhängend und bewegt sich als Einheit auf der Kugel. Flächen-, Richtungs- und Geschwindigkeitsänderungen erhalten ihre Identität.

Eine Platte endet durch:

- vollständigen Verbrauch,
- Verschmelzung zugunsten einer neuen Nachfolgerplatte oder
- Teilung in flächenhaft getrennte Nachfolger.

Eine Verbindung nur über einen Punkt reicht nicht als Zusammenhang. Numerische Mindestbreiten einer noch flächigen Verbindung sind dagegen keine fachliche Eigenschaft dieses Modells.

Der nicht aufgelöste Hintergrund ist keine Platte. Er besitzt weder Plattenidentität noch Plattenabstammung oder Eigenbewegung und darf aus mehreren getrennten Restflächen bestehen. Während der Initialisierung dient er als unbewegte Referenz für konvergente, divergente oder transformante Relativbewegungen expliziter Platten.

## 4. Krustenregionen

Eine Krustenregion ist ein identifizierbarer, flächenhaft zusammenhängender Herkunftskörper. Eine bloße Punktberührung verbindet keine Teile derselben Region. Räumliche Trennung erzeugt neue Nachfolgerregionen mit gemeinsamer Herkunft.

Es bestehen genau zwei materielle Krustenarten:

- kontinentale Kruste,
- ozeanische Kruste.

„Nicht aufgelöst“ ist keine dritte Krustenart, sondern bezeichnet unbekannte Herkunft, unbekanntes Alter und unbekannte historische Plattenzugehörigkeit anfänglich vorhandener Ozeankruste.

### 4.1 Kontinentale Kruste

Ein **Kraton** ist eine spezialisierte kontinentale Krustenregion. Er behält während des gesamten Laufs Identität, Fläche und Krustenart. Er darf weder geteilt noch verbraucht oder umgewandelt werden und kann nur als Ganzes die tragende Platte wechseln. Seine Ankunft an einer Subduktionszone erzwingt Kollision oder Grenzreorganisation.

Nichtkratonische kontinentale Kruste darf durch Rifting geteilt und vollständig zwischen Platten übertragen werden. Für das MVP gilt ausdrücklich die Modellannahme, dass die gesamte anfängliche kontinentale Krustenfläche materiell erhalten bleibt: Kontinentale Kruste wird weder neu erzeugt noch subduziert oder in ozeanische Kruste umgewandelt. Diese Vereinfachung ist kein allgemeines geologisches Naturgesetz.

Ein **Kontinent** ist der maximal zusammenhängende Verbund kontinentaler Krustenregionen, die über Grenzsegmente positiver Länge verbunden sind. Die Plattenzugehörigkeit ist dafür unerheblich. Punktberührung verbindet keine Kontinente; durchgehende ozeanische Kruste trennt sie.

Ein **Superkontinent** ist ein Kontinent, der alle anfänglich gesetzten Kratone vereint. Er ist eine abgeleitete Klassifikation und kein eigenständig bewegtes Objekt. Nach einer Trennung kann diese Klassifikation durch spätere Vereinigung erneut entstehen.

### 4.2 Ozeanische Kruste

Aufgelöste ozeanische Kruste besitzt bekannte Herkunft an einem bestimmten Rücken und eine bekannte Entstehungszeit oder ein bekanntes Entstehungsintervall. Ihr Alter wird daraus abgeleitet.

Nicht aufgelöste alte Ozeankruste wird während der Initialisierung physisch verbraucht und durch Kruste bekannter Herkunft ersetzt. Bloße Zuordnung zu einer expliziten Platte löst sie nicht auf. Getrennte Restflächen sind eigenständige Krustenregionen mit demselben unbekannten Status.

Ozeanische Krustenregionen können erzeugt, geteilt, verkleinert, vollständig zwischen Platten übertragen oder verbraucht werden. Verschiedene Herkunftskörper verschmelzen auch bei späterem Kontakt nicht zu einer Region.

## 5. Grenzen

Eine **Plattengebietsgrenze** ist die aus der aktuellen Oberflächenbelegung abgeleitete Trennlinie zweier Plattengebiete. Zwischen zwei expliziten Platten ist sie zugleich eine **Plattengrenze**; eine Grenze zum nicht aufgelösten Hintergrund ist keine Plattengrenze zwischen zwei bewegten Platten.

Plattengebietsgrenzen besitzen keine dauerhafte Identität, Abstammung oder zwingende aktive Klassifikation. Sie dürfen sich umformen und segmentieren. Ein Abschnitt darf ohne aktives Feature bestehen.

Tektonische Grenzfeatures sind eigenständige fachliche Objekte. Mehrere Featuretypen dürfen verschiedene Abschnitte derselben Grenze belegen. Relikte wie Suturen oder Fracture Zones dürfen weiterbestehen, nachdem sie keine aktuelle Plattengebietsgrenze mehr begleiten.

## 6. Tektonische Features

Ein tektonisches Feature ist eine identifizierbare, klassifizierte Punkt-, Linien- oder Flächenerscheinung. Ein bloßer Ereigniskandidat ist noch kein Feature; die Feature-Identität entsteht erst mit tatsächlichem Prozessbeginn.

Ein Feature besitzt:

- einen fachlichen Typ und Zustand,
- eine aktuelle räumliche Bindung,
- benannte Beziehungen zu beteiligten Plattengebieten und Krustenregionen sowie
- gegebenenfalls Vorgänger und Nachfolger.

Es besitzt keine einzelne Eigentümerplatte. Beziehungen drücken stattdessen Rollen aus, beispielsweise betroffene Platte, angrenzende Seite, Oberseite, Unterseite oder aktueller Träger eines Relikts.

Zustandswechsel und reine Geometrieänderungen erhalten die Feature-Identität. Ein fachlicher Typwechsel, eine Teilung oder eine Vereinigung beendet sie zugunsten neuer Nachfolgerfeatures.

Jedes aktuelle Feature ist entweder

- **grenzgebunden** auf einer aktuellen Plattengebietsgrenze oder
- **krustengebunden** auf beziehungsweise innerhalb aktuell tragender Krustenregionen.

Ein aktuelles Feature darf nicht ohne räumliche Bindung bestehen. Vollständiger Verbrauch seiner tragenden Kruste beendet ein krustengebundenes Feature. Teilweiser Verbrauch verkleinert es; räumliche Trennung erzeugt neue Nachfolgerfeatures.

Features dürfen sich räumlich überlagern. Beispielsweise darf eine Sutur innerhalb eines Orogenpolygons liegen oder ein Rift eine Reliktsutur aufgreifen. Identität, Typ, Zustand und Beziehungen halten sie unterscheidbar.

Für den Kern gelten zusätzlich bereits folgende Beziehungen:

- Ein Rift liegt innerhalb kontinentaler Kruste einer expliziten Platte und wird erst nach erfolgreichem Aufbrechen zum Vorgänger eines Rückens.
- Eine Subduktionszone kann während der Initialisierung nicht aufgelöste alte Ozeankruste an der Grenze zu einer expliziten Platte verbrauchen.
- Kontinentankunft beendet lokalen ozeanischen Verbrauch zugunsten von Kollision oder Grenzreorganisation.

Die vollständigen Zustände, Übergänge und Lebenszyklen der einzelnen Featuretypen werden in Ticket #8 festgelegt.

## 7. Seed-Zustand und Phasen

Die Simulationszeit beginnt monoton bei null und wird beim Phasenwechsel nicht zurückgesetzt.

Im Seed-Zustand:

- trägt genau eine explizite Platte den gesamten anfänglichen Superkontinenten,
- entspricht ihr Gebiet zunächst der kontinentalen Fläche,
- besteht die übrige Kugeloberfläche aus nicht aufgelöster alter Ozeankruste im unbewegten Hintergrund,
- liegen zwischen ausgewählten Kratonen Reliktsuturen in nichtkratonischer kontinentaler Kruste und
- bestehen noch keine aktiven tektonischen Features.

Die Reliktsuturen haben keine simulierten Vorgänger. Sie entstehen fachlich bei Simulationszeit null und vertreten eine vormodellierte tektonische Vorgeschichte.

Die Initialisierungsphase endet genau dann und unumkehrbar, wenn keine nicht aufgelöste Fläche mehr besteht und jeder Punkt einer expliziten Platte zugeordnet ist. Nicht aufgelöste alte Ozeankruste kann danach nie wieder entstehen.

Die Reifephase beginnt ohne Zurücksetzen der Simulationszeit und enthält ausschließlich explizite Platten sowie Kruste bekannter Herkunft.

## 8. Gültigkeit und Ergebnis

Ein gültiger Oberflächenzustand erfüllt gleichzeitig:

1. vollständige, flächig überlappungsfreie Krusten- und Plattengebietsbelegung,
2. genau eine Tragbeziehung je aktueller Krustenregion,
3. flächenhaften Zusammenhang jeder Platte und Krustenregion,
4. starke Kraton- und kontinentale Erhaltungsregeln,
5. gültige räumliche Bindungen und Rollen aller aktuellen Features,
6. eindeutige, nicht wiederverwendete Identitäten und
7. azyklische Abstammungsbeziehungen.

Tektonische Ereignisse wechseln atomar von einem gültigen Zustand zum nächsten. Halbfertige Rechenschritte mit Lücken, Doppelzuordnungen oder verwaisten Features gehören nicht zum fachlichen Zustand und dürfen nicht als Zwischen- oder Endergebnis gelten.

Ein Lauf ist ungültig und erzeugt keine finale Feature-Map, wenn eine Zustandsinvariante verletzt wird oder das Initialisierungszeitlimit von standardmäßig 250 Myr bei verbleibender nicht aufgelöster Fläche erreicht ist. Der letzte gültige Zustand darf ausschließlich zur Diagnose dienen.

Eine finale Feature-Map entsteht nur, wenn der Lauf die Initialisierung vollständig abschließt und anschließend die konfigurierte Reifephase in gültigen Zuständen beendet. Sie ist der letzte gültige Oberflächenzustand. Ein bestimmtes geologisches Endmuster, insbesondere ein neuer Superkontinent, ist kein Erfolgskriterium.

## 9. Bewusst vertagte Entscheidungen

Dieses Zustandsmodell legt ausdrücklich nicht fest:

- sphärische Repräsentation, Randsemantik, Snap-Toleranzen oder Polygonoperationen (#6),
- Eulerpole, Geschwindigkeiten und deren Aktualisierung (#7),
- vollständige Zustände und Lebenszyklen der Kernfeatures (#8),
- Ereignisbedingungen, Priorisierung und deterministische Auswahl (#9),
- Persistenz und Abfragen der finalen Feature-Map (#11) oder
- konkrete Validierungs- und Akzeptanzgrenzen (#13).
