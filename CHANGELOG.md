# Änderungen

## 2.0 · das große Update

**KI und Kosten**
- Sparmodus (Standard: an): kürzerer Verlauf, knappes Gedächtnis, Befehle wie Timer ohne KI-Rückfrage, Bots antworten direkt
- Websuche standardmäßig aus, weil jede Suche extra kostet
- Monatslimit in US-Dollar für Cloud-KI; ist es erreicht, wird nichts mehr gesendet
- Kosten-Widget: Verbrauch heute und im Monat, Limit, selbst eingetragenes Guthaben abzüglich Verbrauch
- Token werden so gezählt, wie der Anbieter sie meldet; Listenpreise der Claude-Modelle sind hinterlegt, eigene Preise lassen sich eintragen

**Bots**
- Eigenes KI-Team: jeder Bot mit Auftrag, Modell, Schlüssel und Adresse
- Ansprechen mit `@Name`, „Name, …“, „Frag den Name …“ oder Stichwort; Jarvis kann Aufgaben selbst abgeben
- Bots kreisen sichtbar um Jarvis: 14 Formen, Hover-Karte, Dashboard mit Übersicht und Einstellungen
- Für Bots gelten Privat-Weiche und „Strikt lokal“ wie für Jarvis

**Oberflächen**
- 25 Oberflächen (vorher 7), darunter Avatar (Gesicht aus 3D-Würfeln) und Kosmos 3D in WebGL, Phantom und Retro als Arbeitsübersicht
- Ladescreen mit den echten Startschritten, Vorschaubilder in der Auswahl
- Fenster und Knöpfe in Milchglas-Optik
- Maus-Effekte: Partikel reagieren, Klick-Welle, Ansicht neigt sich; alles abschaltbar
- Hintergrundklang und Effekt-Töne mit Reglern
- Widgets: Wetter, Notizen, Uhr, System, Kosten; verschiebbar und in der Größe änderbar

**Steuerung**
- PC-Steuerung (Standard: aus): freigegebene Ordner und Programme, Textdateien und Skripte lesen und schreiben, mit Nachfrage; kein Löschen, kein Ausführen
- Home Assistant (Standard: aus): Geräte per Sprache und Routine schalten, ohne KI und ohne Token
- Webhook-Routinen für eigene Automationen

**Sicherheit und Datenschutz**
- Schutz-Kopfzeilen: kein Einbetten durch fremde Seiten, Oberfläche lädt und sendet nur an Jarvis selbst
- Server liefert nur noch `core`, `oberflaechen`, `sounds` und `jarvis.html` aus
- „Lokal“ wird an der echten Adresse geprüft (eigener PC und Heimnetz), nicht am Wortlaut
- Der Hauptschlüssel geht nie an die eigene Adresse eines Bots
- Skripte werden nie ohne Nachfrage geschrieben und nie gestartet
- Chatverlauf löschbar oder abschaltbar; Bot-Verlauf abschaltbar
- Auf Linux und macOS sind die Dateien mit Schlüsseln und Notizen nur für den eigenen Benutzer lesbar

## 1.1
- Datenschutz-Modus: Anzeige LOKAL/CLOUD, Privat-Weiche, Strikt lokal

## 1.0
- Erste Version: Ollama, Claude, OpenAI-kompatibel, 7 Oberflächen
