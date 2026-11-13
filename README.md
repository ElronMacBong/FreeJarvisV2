# FreeJarvis · JARVIS YouTube-Edition 2.0

Persönlicher KI-Assistent im Browser: mit Stimme, Gedächtnis, Automation, **25 Oberflächen** und einem **Team aus Bots**, das sichtbar um Jarvis kreist.

Keine Pakete zu installieren, nur Python. Start per Doppelklick.

**Eine Version für alles:** lokal mit **Ollama** (kostenlos, ohne Schlüssel), **Claude** per API oder **OpenAI-kompatible** Dienste (OpenAI, Groq, OpenRouter, LM Studio). Gewechselt wird oben rechts per Klick, ohne Code zu ändern.

## Start

1. Python 3.8 oder neuer installieren (unter Windows den Haken bei „Add python.exe to PATH“ setzen)
2. Ordner entpacken, `start.bat` doppelklicken (Mac/Linux: `sh start.sh`)
3. Der Browser öffnet `http://localhost:5050`. Warten, bis der Ladescreen fertig ist, dann **STARTEN**
4. Oben links **Einstellungen** → KI einrichten

Die komplette Anleitung mit Fehlerbehebung steht in **`anleitung.txt`**.

## Was Jarvis kann

| Bereich | Inhalt |
| --- | --- |
| **Sprechen und Zuhören** | Browser-Stimme oder ElevenLabs, Mikrofon mit Weckwort |
| **Gedächtnis** | „Merk dir …“ und „Vergiss …“, bleibt standardmäßig auf dem PC |
| **Befehle ohne KI** | Timer, Webseiten, Wetter, Systemwerte, Weck-Briefing, eigene Routinen |
| **Bots** | Eigene KI-Helfer mit Auftrag, Modell und Schlüssel. Ansprechen mit `@Name`, „Name, …“ oder Stichwort. Jarvis kann Aufgaben selbst abgeben |
| **Oberflächen** | 25 Designs mit Vorschau, darunter Avatar und Kosmos 3D in WebGL und Retro als Arbeitsübersicht |
| **Widgets** | Wetter, Notizen, Uhr, System, Kosten. Verschiebbar, Größe änderbar |
| **Kostenkontrolle** | Sparmodus, Monatslimit, Kosten-Widget mit Verbrauch und Guthaben |
| **PC-Steuerung** | Freigegebene Ordner und Programme, Textdateien und Skripte lesen und schreiben. Standardmäßig aus, mit Nachfrage |
| **Home Assistant** | Geräte per Sprache und Routine schalten, dazu Webhooks. Standardmäßig aus |

## Kosten

Ollama kostet nichts. Bei Claude und anderen API-Diensten zahlt man nach Verbrauch. Dafür gibt es:

- **Sparmodus** (an): kürzerer Verlauf, Befehle wie Timer ohne KI-Rückfrage, Bots antworten direkt
- **Websuche** standardmäßig aus, weil jede Suche extra kostet
- **Monatslimit** in US-Dollar: Ist es erreicht, sendet Jarvis nichts mehr an eine Cloud-KI
- **Kosten-Widget**: Verbrauch heute und im Monat, Limit, Guthaben

Das echte Guthaben lässt sich mit einem normalen API-Schlüssel nicht abfragen. Jarvis rechnet deshalb mit dem Guthaben, das man einträgt, und zieht den eigenen Verbrauch ab. Verbindlich ist die Abrechnung beim Anbieter. Details in `anleitung.txt`, Abschnitt 21.

## Home Assistant

Adresse und langlebigen Zugriffstoken eintragen (Einstellungen → Automation), dann gehen Sätze wie „Schalte das Wohnzimmerlicht an“, „Stehlampe auf 40 Prozent“ oder „Ist der Fernseher an?“. Das läuft ohne KI. Per Routine lassen sich beliebige Dienste und Webhooks auslösen, zum Beispiel für Node-RED. Schlösser und Alarmanlagen sind absichtlich nicht freigegeben. Details in Abschnitt 22.

## Datenschutz

- Anzeige **LOKAL / CLOUD** oben rechts
- **Privat-Weiche:** Nachrichten mit Telefonnummer, IBAN, Passwort oder eigenen Stichwörtern gehen an die lokale KI statt in die Cloud
- Das **Gedächtnis** bleibt standardmäßig lokal
- **Strikt lokal** sperrt alle Online-Dienste
- Das gilt auch für Bots

`einstellungen.json`, `gedaechtnis.json`, `bot_statistik.json` und `kosten.json` enthalten persönliche Daten oder Schlüssel. Sie entstehen erst beim Benutzen und werden durch `.gitignore` nicht mit hochgeladen. Details in Abschnitt 15.

## Sicherheit

- Der Server ist nur über localhost erreichbar.
- Schreibende Anfragen müssen an genau diesen Port gehen, einen passenden `Origin` haben und die Kopfzeile `X-Jarvis: 1` tragen. CORS-Vorabfragen werden mit 403 beantwortet. Damit kann weder eine fremde Webseite noch ein anderer Dienst auf dem Rechner Jarvis steuern. (Bis Version 1.1 war das möglich. Danke an Pascal für den Hinweis.)
- Schutz-Kopfzeilen verhindern, dass fremde Seiten Jarvis einbetten, und erlauben der Oberfläche nur Verbindungen zu Jarvis selbst.
- Ausgeliefert werden nur `core`, `oberflaechen`, `sounds` und `jarvis.html`.
- Skripte schreibt Jarvis nur nach Nachfrage und startet sie nie.

Jarvis ist ein Hobby-Projekt ohne Gewähr, Nutzung auf eigene Verantwortung. Wer eine Lücke findet: bitte direkt melden statt öffentlich posten. Details in Abschnitt 16.

## Dateien

| Datei | Zweck |
| --- | --- |
| `start.bat`, `start.sh` | Start |
| `server.py` | der Server: verbindet Oberfläche und KI, nur Python-Standardbibliothek |
| `jarvis.html` | Oberfläche Vortex |
| `oberflaechen/` | die 24 weiteren Oberflächen und ihre Vorschaubilder |
| `core/core.js`, `core/core.css` | gemeinsame Technik und Aussehen |
| `core/bots.js` | Bots auf der Oberfläche und der Reiter „Bots“ |
| `core/widgets.js` | Widgets |
| `core/klang.js`, `sounds/` | Klang |
| `core/maus.js` | Maus-Effekte |
| `core/pc.js` | PC-Steuerung |
| `anleitung.txt` | die ausführliche Anleitung |
| `einstellungen.beispiel.json` | alle Einstellungen mit Standardwerten, nur zum Nachschlagen |

## Eigene Oberfläche bauen

Eine Datei aus `oberflaechen/` kopieren und den Teil in `Jarvis.registerTheme({...})` ändern. Siehe `anleitung.txt`, Abschnitt 17.

## Lizenz

MIT, siehe `LICENSE`.
