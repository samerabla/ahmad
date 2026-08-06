# PoWi-Briefing

Tägliches Politik- und Wirtschaftsbriefing, automatisch aktualisiert.

- **Seite:** https://claude.ai/code/artifact/60a98a31-78ca-4828-8d0f-4a4ea83b9190
- **Lauf:** täglich 07:00 Uhr (deutsche Zeit), per Routine
- **Vorlage:** `briefing.html`

## Aufbau

`briefing.html` ist eine eigenständige Seite ohne externe Abhängigkeiten (CSS inline,
keine Webfonts, keine Skripte). Sie dient dem täglichen Lauf gleichzeitig als
Design-Vorlage und als Ausgabedatei: Struktur und Styles bleiben, nur die Inhalte
werden ersetzt.

Struktur:

| Block | Inhalt |
|---|---|
| `.masthead` | Titel, Datum, Ausgabennummer, Lesezeit |
| `.glance` | 4–5 Kennzahlen als Chips |
| `.feed` | 6–8 Meldungen, je mit Bereichs-Tag, Text, PoWi-Bezug und Quellen |
| `.weekly` | Thema der Woche mit Lehrplanbezügen |
| `.colophon` | Datum und Hinweis auf Quellenprüfung |

Die Bereichs-Tags sind `Deutschland`, `Europa`, `International` und `Wirtschaft`.

## Redaktionelle Regeln

- Kein Ereignis ohne Quellenlink.
- Der `PoWi-Bezug` nennt ein konkretes Lehrplanthema, keine Allgemeinplätze.
- Zahlen nur nennen, wenn sie belegt sind — lieber weglassen als schätzen.
- Ausgabennummer bei jedem Lauf um eins hochzählen.
