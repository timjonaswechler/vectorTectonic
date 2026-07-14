# Vector Tectonics

Dieser Kontext beschreibt die fachlichen Objekte einer prozeduralen, vektorbasierten Plattentektonik-Simulation auf einer Kugeloberfläche.

## Language

**Fachliche Identität**:
Die innerhalb eines Simulationslaufs dauerhaft eindeutige Kennzeichnung einer Platte, Krustenregion oder eines tektonischen Features. Beendete Identitäten werden nie wiederverwendet; ähnliche spätere Entitäten erhalten neue Identitäten.
_Avoid_: wiederverwendbare laufende Nummer, Gleichheit aufgrund gleicher Geometrie

**Feature-Map**:
Der letzte gültige Oberflächenzustand eines erfolgreichen Laufs nach vollständig abgeschlossener Initialisierungs- und Reifephase. Ein bestimmtes geologisches Endmuster ist nicht vorgeschrieben.
_Avoid_: Zwischenzustand, Ergebnis eines ungültigen Laufs, Heightmap, Terrainkarte

**Platte**:
Eine identifizierbare, stets flächenhaft zusammenhängende Region der Lithosphärenoberfläche, die sich als Einheit auf der Kugel bewegt. Flächen- und Bewegungsänderungen erhalten ihre Identität; Verschmelzung, vollständiger Verbrauch oder Trennung bis auf bloße Punktberührung beenden sie zugunsten neuer Nachfolger beziehungsweise ihres Endes.
_Avoid_: Bewegungszustand als Identität, punktverbundene oder unverbundene Flächensammlung, Mantelplatte, Rasterzelle

**Plattenabstammung**:
Die azyklische, stammbaumartige Herkunft einer Platte aus Teilungen und Verschmelzungen. Teilung erlaubt mehrere Nachfolger, Verschmelzung mehrere Vorgänger; kein Nachfolger darf eigener Vorfahr sein.
_Avoid_: strenger Baum, Krustenabstammung, bloße Flächenänderung

**Plattengebiet**:
Die eindeutige Bewegungszuordnung eines Oberflächenpunkts zu einer expliziten Platte oder, während der Initialisierungsphase, zum nicht aufgelösten Hintergrund. Plattengebiete bilden eine vollständige Flächengliederung der Kugeloberfläche und tragen jeweils vollständig enthaltene Krustenregionen.
_Avoid_: Krustenregion, tektonisches Feature

**Plattengebietsgrenze**:
Die aus der aktuellen Oberflächenbelegung abgeleitete topologische Trennlinie zweier Plattengebiete. Sie besitzt keine dauerhafte Identität oder zwingende aktive Klassifikation; tektonische Grenzfeatures können abschnittsweise auf ihr liegen, und erst zwischen zwei expliziten Platten ist sie zugleich eine Plattengrenze.
_Avoid_: dauerhaftes Grenzobjekt, immer aktives Grenzfeature, Sutur, Fracture Zone

**Plattengrenze**:
Eine Plattengebietsgrenze zwischen zwei expliziten, eigenständig bewegten Platten.
_Avoid_: Grenze zum nicht aufgelösten Hintergrund, tektonisches Grenzfeature

**Krustenregion**:
Ein identifizierbarer, flächenhaft zusammenhängender Herkunftskörper kontinentaler oder ozeanischer Art, der vollständig von genau einem Plattengebiet getragen wird. Trennung bis auf bloße Punktberührung erzeugt Nachfolger; Regionen können außerdem erzeugt, verkleinert oder verbraucht werden, verschmelzen aber niemals miteinander.
_Avoid_: punktverbundener oder verschmolzener Herkunftsmix, Platte, Plattengebiet, tektonisches Feature

**Tragbeziehung**:
Die veränderliche, eindeutige Zuordnung einer vollständigen Krustenregion zu genau einem Plattengebiet. Ein vollständiger Trägerwechsel erhält ihre Identität; ein teilweiser Wechsel setzt ihre Teilung in Nachfolgerregionen voraus.
_Avoid_: Krustenabstammung, bloße Überlappung

**Krustenabstammung**:
Die azyklische Herkunft einer Krustenregion aus ihrer Erzeugung oder Teilung. Da Krustenregionen niemals verschmelzen, besitzt jede Nachfolgerregion höchstens einen unmittelbaren Vorgänger.
_Avoid_: Verschmelzung, bloße Flächennachbarschaft, Plattenzugehörigkeit

**Krustenart**:
Die materielle Einordnung einer Krustenregion als kontinental oder ozeanisch. „Nicht aufgelöst“ ist keine Krustenart, sondern bezeichnet unbekannte Herkunft und historische Plattenzugehörigkeit alter Ozeankruste.
_Avoid_: nicht aufgelöst, Plattenzugehörigkeit

**Kraton**:
Eine spezialisierte kontinentale Krustenregion und dauerhaft unteilbarer, nicht verbrauchbarer Kern alter Kruste, der Identität, Fläche und Krustenart über den gesamten Lauf bewahrt. Er liegt nicht innerhalb einer anderen Krustenregion, darf nur als Ganzes die tragende Platte wechseln und erzwingt bei Ankunft an einer Subduktionszone Kollision oder einen zulässigen Grenztypwechsel.
_Avoid_: eingebettete Zusatzfläche, gesamte Kontinentalkruste, beliebige Kontinentalfläche

**Nichtkratonische kontinentale Kruste**:
Kontinentale Kruste außerhalb der dauerhaft unteilbaren Kratone. Sie verbindet oder umgibt Kratone und kann durch Rifting geteilt werden, wird im MVP jedoch weder erzeugt, verbraucht noch in ozeanische Kruste umgewandelt.
_Avoid_: Kraton, ozeanische Kruste

**Kontinentale Erhaltungsregel**:
Die ausdrückliche MVP-Modellannahme, dass die gesamte anfängliche kontinentale Krustenfläche materiell erhalten bleibt. Kontinentankunft beendet lokalen ozeanischen Verbrauch zugunsten von Kollision, ohne zu behaupten, kontinentale Kruste sei in der Natur grundsätzlich unverbrauchbar.
_Avoid_: allgemeines geologisches Naturgesetz, kontinentale Subduktionssimulation

**Kontinent**:
Ein maximal zusammenhängender Verbund kontinentaler Krustenregionen, die über Grenzsegmente positiver Länge verbunden sind, unabhängig von ihrer Plattenzugehörigkeit. Bloße Punktberührung verbindet keine Kontinente; durchgehende ozeanische Kruste trennt sie.
_Avoid_: punktberührender Verbund, Platte, einzelne Krustenregion, Landmasse

**Superkontinent**:
Ein Kontinent, der alle anfänglich gesetzten Kratone vereint. Im Seed-Zustand trägt ihn die einzige explizite Platte; nach einer Trennung kann die Klassifikation durch spätere Vereinigung erneut entstehen, bleibt aber stets ein abgeleiteter Verbund statt einer eigenständig bewegten Entität.
_Avoid_: Großplatte, dauerhaftes Einzelobjekt

**Seed-Zustand**:
Der Zustand bei Simulationszeit null: Eine einzige explizite Platte trägt den gesamten anfänglichen Superkontinenten, während die übrige Kugeloberfläche aus nicht aufgelöster alter Ozeankruste im unbewegten Hintergrund besteht. Zwischen ausgewählten Kratonen liegen Reliktsuturen, aktive tektonische Features bestehen noch nicht.
_Avoid_: vollständig aufgelöste Plattenkonfiguration, Reifephase

**Aufgelöste ozeanische Kruste**:
Ozeanische Kruste mit bekannter Entstehung an einem bestimmten Rücken und bekannter Entstehungszeit oder bekanntem Entstehungsintervall. Ihr Krustenalter ergibt sich aus dieser Herkunft.
_Avoid_: nicht aufgelöste alte Ozeankruste, eigenständige Krustenart

**Nicht aufgelöste alte Ozeankruste**:
Anfänglich vorhandene ozeanische Kruste unbekannter Herkunft, unbekannten Alters und unbekannter historischer Plattenzugehörigkeit, die während der Initialisierung als flächenbilanzierter Residualträger dient, physisch verbraucht und ersetzt wird. Bloße Zuordnung zu einer expliziten Platte löst sie nicht auf; getrennte Restflächen sind eigenständige Krustenregionen und dienen gemeinsam als unbewegte Referenz.
_Avoid_: umbenannte aufgelöste Kruste, eine unverbundene Krustenregion, dritte Krustenart, Ozeanplatte, leerer Raum

**Referenzbewegung**:
Die als null gesetzte Bewegung des nicht aufgelösten Hintergrunds, gegen die Bewegungen expliziter Platten während der Initialisierung als konvergent, divergent oder transformant eingeordnet werden.
_Avoid_: Eigenbewegung einer Platte, Plattenidentität

**Plattenbewegungszustand**:
Die vollständige Bewegungszuordnung eines gültigen Simulationszeitpunkts mit genau einem Winkelgeschwindigkeitsvektor für jede aktuelle explizite Platte. Der nicht aufgelöste Hintergrund und beendete Platten besitzen keinen Plattenbewegungszustand.
_Avoid_: gespeicherter Eulerpol zusätzlich zum Vektor, Bewegung des Hintergrunds, unvollständige Plattenzuordnung

**Winkelgeschwindigkeitsvektor**:
Die kanonische gerichtete Rotationsbewegung einer expliziten Platte auf der Kugel. Eulerpol, Rotationsrate und lokale Oberflächengeschwindigkeit werden daraus abgeleitet; der Nullvektor bezeichnet Stillstand ohne Eulerpol.
_Avoid_: translatorische Plattenbewegung, redundant gespeicherter Eulerpol, physikalischer Drehimpuls

**No-Net-Rotation-Bezugssystem**:
Das Bezugssystem der Reifephase, in dem die flächenintegrierte gemeinsame Starrrotation aller expliziten Platten null ist, ohne ihre relativen Grenzbewegungen zu verändern. Während der Initialisierung gilt stattdessen die Referenzbewegung des Hintergrunds.
_Avoid_: fixierte Ankerplatte, ruhender Mantel, einfaches Mittel der Eulerpole

**Kinematischer Einflussbeitrag**:
Ein regelbasierter, längengewichteter Beitrag einer bereits feststehenden tektonischen Beziehung zur bevorzugten Plattenbewegung. Slab Pull, Ridge Push und Kollisionswiderstand sind im MVP solche kalibrierten Einflüsse und ausdrücklich keine berechneten physikalischen Kräfte.
_Avoid_: Kraft, Beschleunigung, Mantelströmung, Ereignisauslöser

**Bewegungsvererbung**:
Die deterministische Überleitung der zuletzt gültigen Plattenbewegungen auf Nachfolger einer Plattenteilung oder -verschmelzung. Sie ist eine Regel des Plattenabstammungsübergangs und kein kinematischer Einflussbeitrag.
_Avoid_: zufällige Neubewegung, Mittelung von Eulerpolen, tektonischer Antrieb

**Relative Grenzkinematik**:
Die aus den aktuellen Bewegungen beider angrenzender Plattengebiete abgeleitete lokale Normal- und Tangentialbewegung einer orientierten Grenze. Sie kann abschnittsweise öffnend, schließend, transform oder ruhend sein und ist noch keine Entscheidung über ein tektonisches Feature oder Ereignis.
_Avoid_: absoluter Plattenvektor, Grenzfeature, Ereignisentscheidung

**Simulationszeit**:
Die monotone fachliche Zeitachse eines Laufs, beginnend bei null im Seed-Zustand. Entstehung und Ende aktueller oder beendeter Entitäten beziehen sich auf sie; der Wechsel in die Reifephase setzt sie nicht zurück.
_Avoid_: zurückgesetzte Phasenzeit, reale geologische Datierung

**Initialisierungsphase**:
Der Simulationsabschnitt, der genau dann und unumkehrbar endet, wenn keine nicht aufgelöste Fläche mehr besteht und jeder Punkt einer expliziten Platte zugeordnet ist.
_Avoid_: vollständige Simulation, zeitbasierter Phasenwechsel

**Reifephase**:
Der anschließende Simulationsabschnitt mit ausschließlich expliziten Platten und Kruste bekannter Herkunft. Nicht aufgelöste alte Ozeankruste kann in dieser Phase nicht erneut entstehen.
_Avoid_: Initialisierungsphase

**Ungültiger Lauf**:
Ein Lauf, der keine finale Feature-Map hervorbringt, weil eine Zustandsinvariante verletzt wurde oder das Initialisierungszeitlimit bei verbleibender nicht aufgelöster Fläche erreicht ist. Sein letzter gültiger Zustand dient ausschließlich der Diagnose.
_Avoid_: unvollständige Feature-Map, erfolgreicher Lauf mit Warnung

**Tektonisches Feature**:
Eine identifizierbare, fachlich klassifizierte Punkt-, Linien- oder Flächenerscheinung mit benannten Beziehungen zu beteiligten Plattengebieten und Krustenregionen. Zustands- und Geometrieänderungen erhalten ihre Identität; Features dürfen sich überlagern und bilden keine exklusive Flächenpartition.
_Avoid_: Plattengebietsgrenze, Eigentum genau einer Platte, exklusive Flächenklasse, Pixel, Höhenwert

**Feature-Lebenszyklus**:
Die zusammenhängende Aktivitäts- und Erhaltungsperiode genau einer Feature-Identität in den Zuständen aktiv, abklingend oder reliktisch. Ereigniskandidaten sind noch keine Features; Typwechsel, Reaktivierung eines Relikts sowie fachliche Teilung oder Vereinigung erzeugen Nachfolgeridentitäten.
_Avoid_: wiederholte Aktivitätsperioden derselben Identität, technischer Geometrieverlauf, vollständige Simulationshistorie

**Aktives Feature**:
Ein aktuelles Feature, dessen tektonischer Prozess oder, bei einer Fracture Zone, dessen Spurbildung läuft.
_Avoid_: Ereigniskandidat, Feature während der Hysterese, Relikt

**Abklingendes Feature**:
Ein aktuelles Feature, dessen Aktivitätsbedingung nicht mehr erfüllt ist, dessen bestätigte Hysterese oder Ausklangfrist aber noch läuft. Prozesswirkungen pausieren; eine rechtzeitige Erholung erhält die Identität.
_Avoid_: weiterhin verdeckt aktives Feature, Relikt, beendete Identität

**Reliktisches Feature**:
Ein aktuelles, erhaltenes Feature nach Ende seines erzeugenden Prozesses. Seine Identität kann nicht wieder aktiv werden; spätere Reaktivierung erzeugt ein neues Feature mit Vorgängerbeziehung.
_Avoid_: inaktiver Ereigniskandidat, gelöschte Geometrie, erneut aktive gleiche Identität

**Feature-Beziehung**:
Die benannte Rolle einer Platte, eines Plattengebiets oder einer Krustenregion an einem tektonischen Feature, etwa Oberseite, Unterseite, angrenzende Seite oder aktueller Träger eines Relikts.
_Avoid_: ungerichtete Beteiligung, Eigentümerplatte

**Feature-Bindung**:
Die räumliche Verankerung eines aktuellen tektonischen Features an einer aktuellen Plattengebietsgrenze oder auf beziehungsweise innerhalb aktuell tragender Krustenregionen. Grenzgebundene Flächenfeatures verwenden eine Bindungsachse auf der Grenze; vollständiger Trägerverbrauch beziehungsweise räumliche Trennung krustengebundener Geometrie beendet sie oder erzeugt Nachfolger.
_Avoid_: bloße historische Beteiligung, Eigentümerplatte eines grenzgebundenen Features, schwebendes Feature

**Feature-Abstammung**:
Die azyklische Herkunft eines tektonischen Features aus Typwechsel, Teilung oder Vereinigung anderer Features. Sie darf mehrere Vorgänger oder Nachfolger besitzen; bloße Zustands- und Geometrieänderungen führen dieselbe Identität fort.
_Avoid_: strenger Baum, Plattenabstammung, unverbindliche räumliche Nähe

**Herkunftsnachweis**:
Der minimale bleibende Nachweis einer beendeten Platte, Krustenregion oder eines beendeten Features mit Identität, Art, Beginn, Ende sowie Vorgängern und Nachfolgern. Er gehört nicht zur aktuellen Oberflächenbelegung und bewahrt keine frühere Geometrie.
_Avoid_: vollständige Simulationshistorie, aktuelles Oberflächenobjekt

**Tektonisches Ereignis**:
Ein fachlich atomarer Gesamtübergang zwischen zwei gültigen Oberflächenzuständen, der sämtliche für seine Gültigkeit erforderlichen Topologie-, Identitäts- und Featurefolgen unteilbar umfasst. Es ist entweder ein zwingendes Folgeereignis oder ein auswählbares Initiierungsereignis und gehört genau einem Typ des geschlossenen Ereigniskatalogs an.
_Avoid_: einzeln auswählbare Teiloperation, halbfertiger Rechenschritt, bloßer Feature-Zustandswechsel, Ereigniskandidat, generische Plattenreorganisation

**Ereigniskatalog**:
Die vollständige, versionierte Menge zulässiger Typen tektonischer Ereignisse. Ein Übergang ohne passenden Katalogtyp ist unzulässig und darf nicht als generische Reorganisation ausgeführt werden.
_Avoid_: offene Ereigniserweiterung, sonstiges Ereignis, Reorganisations-Fallback

**Ereignisbündel**:
Die atomar gemeinsam anzuwendende Menge kompatibler tektonischer Ereignisse einschließlich aller von ihnen erzwungenen Abschlussereignisse. Nur das vollständig geschlossene Bündel darf einen neuen gültigen Zustand erzeugen.
_Avoid_: sequenziell sichtbare Teilereignisse, halbfertiger Zwischenzustand

**Kandidatengedächtnis**:
Das nicht zur Feature-Map gehörende Laufwissen über die ununterbrochene Dauer zeitabhängiger Vorbedingungen stabil fortgeführter Ereigniskandidaten. Es verleiht Kandidaten keine fachliche Identität und erzeugt bei ihrem Ende kein Relikt.
_Avoid_: tektonisches Feature, vollständige Ereignishistorie, fachliche Kandidatenidentität

**Zwingendes Folgeereignis**:
Ein tektonisches Ereignis, das bei erfüllten Vorbedingungen zur Erhaltung fachlicher Invarianten oder als unausweichliche Folge eines bereits bestätigten Prozesses eintreten muss und nicht gegen Initiierungen bewertet wird.
_Avoid_: optionaler Kandidat, scorebasierter Neuanfang

**Auswählbares Initiierungsereignis**:
Ein geologisch zulässiger, aber nicht erzwungener Beginn oder eine nicht zwingende Ausbreitung eines tektonischen Prozesses, der als Ereigniskandidat bewertet und konfliktfrei ausgewählt werden muss.
_Avoid_: zwingende Zustandsfolge, Invariantenreparatur

**Ereigniskandidat**:
Eine noch nicht eingetretene Möglichkeit für ein auswählbares Initiierungsereignis. Er gehört zur Ereignisentscheidung und wird erst mit tatsächlichem Prozessbeginn zu einem tektonischen Feature.
_Avoid_: zwingendes Folgeereignis, tektonisches Feature, Relikt

**Reliktsutur**:
Eine im Seed-Zustand erzeugte Sutur zwischen ausgewählten Kratonen in nichtkratonischer kontinentaler Kruste. Sie besitzt keinen simulierten Vorgänger, vertritt eine vormodellierte tektonische Vorgeschichte und kann spätere Ereigniskandidaten begünstigen.
_Avoid_: aktive Kollision, Kratongrenze, Plattengrenze

**Rift**:
Das aktive Auseinanderbrechen kontinentaler Kruste innerhalb einer expliziten Platte, repräsentiert durch eine zentrale Linie. Erst vollständiger erfolgreicher Break-up erzeugt getrennte Platten und Rücken; nicht aufgelöste alte Ozeankruste kann selbst nicht riften.
_Avoid_: beliebige Divergenz, Rücken, Riftkorridor als zweite autoritative Geometrie

**Rücken**:
Die linienförmige ozeanische Akkretionsachse einer divergenten Grenze zwischen eigenständig bewegten Platten oder ihre konjugierte erhaltene Spur. Nur ein aktiver Rücken erzeugt ozeanische Kruste; kontinentale Divergenz bleibt bis zum Break-up ein Rift.
_Avoid_: beliebige divergente Grenze, Rift, breite Höhenzone

**Subduktion**:
Der aktive konvergente Prozess, bei dem ozeanische Kruste der Unterplatte verbraucht wird und der im Modell durch einen Graben repräsentiert ist.
_Avoid_: eigener Feature-Typ Subduktionszone, jede konvergente Bewegung, Verbrauch kontinentaler Kruste

**Graben**:
Das orientierte linienförmige Grenzfeature aktiver Subduktion mit unveränderlicher Unter-/Oberplattenpolarität. Nur die geometrisch anliegende Unterplatte wird an ihm verbraucht.
_Avoid_: Subduktion als zweites Feature, jede konvergente Grenze, Kollisionsfront

**Vulkanbogen**:
Eine aus genau einem reifen Graben abgeleitete, grabenparallele Linie auf dessen Oberplatte. Er ist eine Oberflächenabstraktion ohne berechnete Slabgeometrie oder Schmelzproduktion.
_Avoid_: symmetrischer Grabenpuffer, unabhängiger Vulkanismus, Höhenprofil

**Transformstörung**:
Ein aktives linienförmiges Grenzfeature, das zwei tektonische Endfeatures bei tangential dominierter Relativbewegung verbindet und keine Fläche erzeugt oder verbraucht.
_Avoid_: Fracture Zone, jeder schräge Grenzabschnitt, diffuse Deformationszone

**Fracture Zone**:
Eine krustengebundene konjugierte Spur, die allein durch fortgesetzte Spreizung beiderseits einer Ridge–Ridge-Transformstörung entsteht. Sie ist keine aktive Plattengrenze und bleibt bis zum Verbrauch ihrer Trägerkruste erhalten.
_Avoid_: aktive Transformstörung, beliebige alte Störung, Ridge–Trench-Connector

**Kollision**:
Der aktive linienförmige Kontaktprozess kontinentaler Kruste nach Ozeanschließung. Er verbraucht keine kontinentale Fläche und hinterlässt als dauerhafte Spuren Sutur und Orogenzone statt eines eigenen Kollisionsrelikts.
_Avoid_: jede Konvergenz, Subduktion, reliktische Kollisionslinie

**Sutur**:
Die reliktische linienförmige Narbe eines geschlossenen Ozeans, abgeleitet aus dem beendeten Graben und dem bestätigten Kontinentkontakt. Sie ist kein aktiver Grenztyp und besitzt keinen altersbedingten Ablauf.
_Avoid_: aktive Kollision, beliebige Kontinentalgrenze, Kratongrenze

**Orogenzone**:
Das sphärische Polygon der durch eine Kollision deformierten Zone, dessen Breite im MVP ein kinematischer Proxy und keine Höhen- oder Erosionsaussage ist.
_Avoid_: Gebirgshöhe, Terrain, exklusive Krustenfläche

**Gültiger Oberflächenzustand**:
Ein fachlicher Zeitpunkt, an dem Oberflächenbelegung, Zusammenhänge, Trag- und Feature-Beziehungen sowie Abstammungen sämtliche Invarianten erfüllen. Tektonische Ereignisse wechseln atomar zwischen gültigen Zuständen; halbfertige Rechenschritte gehören nicht zum fachlichen Zustand.
_Avoid_: persistierter Zwischenzustand mit Lücken, Doppelzuordnungen oder verwaisten Features

**Oberflächenbelegung**:
Die lückenlose und überlappungsfreie Zuordnung jedes Punkts der Kugeloberfläche zu genau einer aktuellen Krustenregion und genau einem aktuellen Plattengebiet. Tektonische Features dürfen diese beiden Flächengliederungen überlagern, bilden aber keine weitere Besitzpartition.
_Avoid_: Feature-Layer als Flächenbesitz, unzugeordnete Fläche

**Kugeloberfläche**:
Die zweidimensionale Oberfläche, auf der sämtliche Simulationsgeometrie liegt.
_Avoid_: Mantel, flache Karte
