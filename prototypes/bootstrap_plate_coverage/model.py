"""PROTOTYPE ONLY: pure bootstrap state transitions, not production simulation."""

from copy import deepcopy

CRATONS = [f"K{i:02d}" for i in range(1, 11)]
RIFT_MATURITY_MYR = 35
RIFT_LOCALIZATION_MYR = RIFT_MATURITY_MYR / 2
STEP_MYR = 5
RESOLUTION_PER_STEP_PERCENT = 5.0


def initial_state():
    return {
        "time_myr": 0,
        "phase": "initialisierung",
        "continental_percent": 25.0,
        "resolved_ocean_percent": 0.0,
        "unresolved_ocean_percent": 75.0,
        "background_components": 1,
        "plates": [
            {
                "id": "P01",
                "status": "active",
                "area_percent": 25.0,
                "cratons": CRATONS.copy(),
                "predecessors": [],
            }
        ],
        "ended_plates": [],
        "rift": {"state": "none", "active_myr": 0},
        "ridge": {"state": "none"},
        "trench": {"state": "none", "qualifying_myr": 0},
        "last_transition": "Seed: ein Superkontinent auf P01; alter Ozean liegt im Hintergrund.",
    }


def _set_last(state, message):
    state["last_transition"] = message
    return state


def _break_up(state):
    predecessor = state["plates"][0]
    predecessor["status"] = "ended_by_rift_breakup"
    state["ended_plates"].append(predecessor)
    state["plates"] = [
        {
            "id": "P02",
            "status": "active",
            "area_percent": 12.5,
            "cratons": CRATONS[:5],
            "predecessors": ["P01"],
        },
        {
            "id": "P03",
            "status": "active",
            "area_percent": 12.5,
            "cratons": CRATONS[5:],
            "predecessors": ["P01"],
        },
    ]
    state["rift"]["state"] = "ended_by_breakup"
    state["ridge"]["state"] = "active"


def _resolve_ocean_band(state):
    amount = min(RESOLUTION_PER_STEP_PERCENT, state["unresolved_ocean_percent"])
    state["unresolved_ocean_percent"] -= amount
    state["resolved_ocean_percent"] += amount
    each = amount / len(state["plates"])
    for plate in state["plates"]:
        plate["area_percent"] += each

    if state["unresolved_ocean_percent"] == 0:
        state["background_components"] = 0
        state["phase"] = "reife"
        state["last_transition"] = (
            f"{amount:g} % letzte Hintergrundfläche ersetzt; vollständige explizite "
            "Plattenabdeckung erreicht. Initialisierung endet unumkehrbar."
        )
    else:
        state["last_transition"] = (
            f"{amount:g} % Hintergrund im gekoppelten Rücken/Graben-Band durch "
            "aufgelöste ozeanische Kruste ersetzt."
        )


def reduce(state, action):
    """Return a new state after one user action; never performs I/O."""
    next_state = deepcopy(state)

    if action == "rift":
        if next_state["phase"] != "initialisierung":
            return _set_last(next_state, "ABGELEHNT: Der Prototyp endet mit der Initialisierung.")
        if next_state["rift"]["state"] != "none":
            return _set_last(next_state, "ABGELEHNT: Ein Riftlauf besteht bereits.")
        next_state["rift"] = {"state": "active_incipient", "active_myr": 0}
        return _set_last(
            next_state,
            "Rift entlang nichtkratonischer Schwäche initiiert; alle Kratone bleiben ungeschnitten.",
        )

    if action == "subduction":
        if next_state["phase"] != "initialisierung":
            return _set_last(next_state, "ABGELEHNT: Kein Hintergrund für Bootstrap-Subduktion.")
        if next_state["trench"]["state"] != "none":
            return _set_last(next_state, "ABGELEHNT: Qualifikation oder Graben besteht bereits.")
        next_state["trench"] = {"state": "qualifying", "qualifying_myr": 0}
        return _set_last(
            next_state,
            "Konvergenter Seed-Rand qualifiziert mit altem Ozean als Unterseite.",
        )

    if action == "cut_craton":
        return _set_last(
            next_state,
            "ABGELEHNT: Kraton K05 ist eine unveränderliche Zwangskontur und darf nicht geteilt werden.",
        )

    if action != "tick":
        return _set_last(next_state, f"ABGELEHNT: unbekannte Aktion {action!r}.")

    if next_state["phase"] == "reife":
        return _set_last(next_state, "STOPP: Zielzustand erreicht; keine Reifephase wird prototypisiert.")

    rift_state = next_state["rift"]["state"]
    ridge_was_active = next_state["ridge"]["state"] == "active"
    trench_state = next_state["trench"]["state"]
    if rift_state == "none" and trench_state == "none":
        return _set_last(
            next_state,
            "ABGELEHNT: Keine Auflösungsperspektive. Zuerst Rift oder Subduktionsqualifikation starten.",
        )
    if next_state["ridge"]["state"] == "active" and trench_state == "none":
        return _set_last(
            next_state,
            "ABGELEHNT: Rücken allein verbraucht den Hintergrund nicht; konsumierende Front qualifizieren.",
        )

    next_state["time_myr"] += STEP_MYR
    messages = []

    if trench_state == "qualifying":
        next_state["trench"]["qualifying_myr"] += STEP_MYR
        if next_state["trench"]["qualifying_myr"] >= 5:
            next_state["trench"]["state"] = "active"
            messages.append("Subduktionszeitnachweis erfüllt; Graben aktiv")

    if rift_state.startswith("active"):
        next_state["rift"]["active_myr"] += STEP_MYR
        active_myr = next_state["rift"]["active_myr"]
        if active_myr >= RIFT_MATURITY_MYR:
            _break_up(next_state)
            messages.append("Rift atomar aufgebrochen: P01 endet, P02/P03 und Rücken entstehen")
        elif active_myr >= RIFT_LOCALIZATION_MYR:
            next_state["rift"]["state"] = "active_localized"
            messages.append("Rift lokalisiert")

    if ridge_was_active and next_state["trench"]["state"] == "active":
        _resolve_ocean_band(next_state)
    elif messages:
        next_state["last_transition"] = "; ".join(messages) + ". Hintergrundfläche unverändert."
    else:
        next_state["last_transition"] = "Zeit fortgeschrieben; Hintergrundfläche unverändert."

    return next_state


def suggested_action(state):
    if state["phase"] == "reife":
        return None
    if state["trench"]["state"] == "none":
        return "subduction"
    if state["rift"]["state"] == "none":
        return "rift"
    return "tick"


def invariant_report(state):
    explicit = sum(plate["area_percent"] for plate in state["plates"])
    all_cratons = [craton for plate in state["plates"] for craton in plate["cratons"]]
    budget = explicit + state["unresolved_ocean_percent"]
    return {
        "surface_budget_100": abs(budget - 100.0) < 1e-9,
        "explicit_equals_known_crust": abs(
            explicit - state["continental_percent"] - state["resolved_ocean_percent"]
        ) < 1e-9,
        "ten_unique_whole_cratons": sorted(all_cratons) == CRATONS,
        "background_never_negative": state["unresolved_ocean_percent"] >= 0,
        "phase_matches_coverage": (state["phase"] == "reife")
        == (state["unresolved_ocean_percent"] == 0),
    }
