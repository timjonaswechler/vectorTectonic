# Vector Tectonics

Dieser Kontext beschreibt die fachlichen Objekte einer prozeduralen, vektorbasierten Plattentektonik-Simulation auf einer Kugeloberfläche.

## Language

**Feature-Map**:
Der finale, vektorbasierte Zustand der simulierten Kugeloberfläche mit Platten, Krustenregionen und tektonischen Features.
_Avoid_: Heightmap, Terrainkarte

**Platte**:
Eine explizite, zusammenhängende Region der Lithosphärenoberfläche, die sich als Einheit auf der Kugel bewegt.
_Avoid_: Mantelplatte, Rasterzelle

**Kraton**:
Eine dauerhaft unteilbare Region alter kontinentaler Kruste innerhalb eines Kontinents; ein Rift darf sie niemals durchschneiden.
_Avoid_: beliebige Kontinentalfläche

**Nicht aufgelöste alte Ozeankruste**:
Der anfängliche ozeanische Hintergrund, dessen historische Plattenzuordnung unbekannt ist und der während der Initialisierungsphase vollständig durch explizite Platten ersetzt wird.
_Avoid_: Ozeanplatte, leerer Raum

**Initialisierungsphase**:
Der Simulationsabschnitt, der endet, sobald jeder Punkt der Kugeloberfläche einer expliziten Platte zugeordnet ist.
_Avoid_: vollständige Simulation

**Reifephase**:
Der anschließende Simulationsabschnitt, in dem sich die vollständig aufgelöste Plattenkonfiguration bis zum finalen Zustand weiterentwickelt.
_Avoid_: Initialisierungsphase

**Tektonisches Feature**:
Eine fachlich klassifizierte Punkt-, Linien- oder Flächengeometrie, die aus dem tektonischen Zustand hervorgeht.
_Avoid_: Pixel, Höhenwert

**Kugeloberfläche**:
Die zweidimensionale Oberfläche, auf der sämtliche Simulationsgeometrie liegt.
_Avoid_: Mantel, flache Karte
