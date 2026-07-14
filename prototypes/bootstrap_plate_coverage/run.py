#!/usr/bin/env python3
"""PROTOTYPE ONLY: line-oriented TUI for the bootstrap state model."""

import sys

sys.dont_write_bytecode = True

from model import invariant_report, initial_state, reduce, suggested_action

BOLD = "\033[1m"
DIM = "\033[2m"
RESET = "\033[0m"

KEYS = {
    "n": "next",
    "r": "rift",
    "s": "subduction",
    "t": "tick",
    "x": "cut_craton",
}


def render(state):
    print("\033[2J\033[H", end="")
    print(f"{BOLD}PROTOTYP — Bootstrap zur vollständigen Plattenabdeckung{RESET}")
    print(f"{DIM}Normalisierte Flächen sind Ablaufmarker, keine Geometrie oder Kalibrierung.{RESET}\n")
    print(f"{BOLD}Zeit / Phase{RESET}  {state['time_myr']} Myr / {state['phase']}")
    print(
        f"{BOLD}Fläche{RESET}        Kontinent {state['continental_percent']:5.1f} % | "
        f"Ozean bekannt {state['resolved_ocean_percent']:5.1f} % | "
        f"Hintergrund {state['unresolved_ocean_percent']:5.1f} %"
    )
    print(f"{BOLD}Hintergrund{RESET}   {state['background_components']} Komponente(n)")
    print(f"{BOLD}Rift{RESET}          {state['rift']}")
    print(f"{BOLD}Rücken{RESET}        {state['ridge']['state']}")
    print(f"{BOLD}Graben{RESET}        {state['trench']}")
    print(f"{BOLD}Platten{RESET}")
    for plate in state["plates"]:
        print(
            f"  {plate['id']}: {plate['area_percent']:5.1f} %, "
            f"Kratone={','.join(plate['cratons'])}, Vorgänger={plate['predecessors'] or '-'}"
        )
    print(f"{BOLD}Beendete Platten{RESET}  {[p['id'] for p in state['ended_plates']] or '-'}")
    print(f"{BOLD}Invarianten{RESET}    {invariant_report(state)}")
    print(f"{BOLD}Letzter Übergang{RESET}  {state['last_transition']}")
    suggestion = suggested_action(state)
    print(f"\n{BOLD}Nächster Vorschlag{RESET}  {suggestion or 'Ziel erreicht — Reaktion festhalten'}")
    print(
        f"\n{BOLD}[n]{RESET} {DIM}Vorschlag ausführen{RESET}  "
        f"{BOLD}[r]{RESET} {DIM}Rift starten{RESET}  "
        f"{BOLD}[s]{RESET} {DIM}Subduktion qualifizieren{RESET}  "
        f"{BOLD}[t]{RESET} {DIM}5 Myr{RESET}  "
        f"{BOLD}[x]{RESET} {DIM}illegalen Kratonschnitt versuchen{RESET}  "
        f"{BOLD}[q]{RESET} {DIM}beenden{RESET}"
    )


def main():
    state = initial_state()
    while True:
        render(state)
        key = input("\n> ").strip().lower()[:1]
        if key == "q":
            return
        action = KEYS.get(key)
        if action == "next":
            action = suggested_action(state)
            if action is None:
                state = reduce(state, "tick")
                continue
        state = reduce(state, action or key)


if __name__ == "__main__":
    main()
