# Deutsche Lokalisierung für PMCA-RE RELOADED.
# Nur die rechte Seite (nach dem =) übersetzen. IDs niemals ändern.

app-title = PMCA-RE RELOADED – Kamera-Toolkit

# Navigation / Tabs
nav-camera-info = Kamera-Infos
nav-install-app = Apps
nav-tweaks = Tweaks
nav-network = Netzwerk
nav-firmware = Firmware
nav-updater = Updater-Shell
nav-service = Service-Shell
nav-tools = Dateiwerkzeuge
nav-settings = Einstellungen
nav-license = Lizenz

# Willkommensseite: die Seite, mit der das Fenster startet. Sie sagt, wofür das
# Toolkit da ist, was zuerst zu tun ist und welche Seite welche Frage beantwortet.
#
# „Sprache“ bedeutet in diesem Programm zweierlei: die Sprache dieses Fensters
# und die Sprachen, die das Menü der Kamera anbietet. Beide bekommen eine eigene
# Überschrift statt nur eines Nebensatzes, denn genau diese Verwechslung soll
# die Seite verhindern.
nav-welcome = Willkommen
welcome-intro-heading = Was dieses Programm kann
welcome-intro-body = Dieses Programm hilft, den Funktionsumfang Ihrer Sony-Kamera zu erhalten und an Ihre Bedürfnisse anzupassen. Nachdem Sony den App-Store für PlayMemories-Kamera-Apps abgeschaltet hatte, gab es keine Möglichkeit, gekaufte oder bereits installierte Apps zu sichern oder wieder zu installieren. Ein Reset der Kamera löscht die installierten Apps unwiederbringlich. Dieses Programm bietet die Möglichkeit, vorhandene Apps zu sichern und wieder zu installieren, und kann zusätzlich frei verfügbare Community-Apps bereitstellen. Sie können auch eigene Apps (*.apk-Dateien) installieren. Bei Problemen liest das Programm Kamera-Informationen und interne Protokolle zur Analyse aus. Einstellungen, die die Kamera bereits besitzt, in ihrem Menü aber nicht anbietet, lassen sich umschalten und wieder zurücknehmen. Das Programm verbindet sich über das USB-Kabel direkt mit der Kamera. Welche Modelle unterstützt werden, ist auf der Seite „Kamera-Infos“ aufgeführt.
welcome-start-heading = Der erste Schritt
welcome-start-body = Schalten Sie die Kamera zuerst ein und stellen Sie im Menü unter „USB-Verbindung“ Massenspeicher oder MTP ein, solange das Kabel noch nicht steckt. „Auto“ funktioniert meist ebenfalls, weil sich die Kamera dann von selbst für eines von beiden entscheidet. Auf die Reihenfolge kommt es an: Sobald das Kabel steckt, ist das Menü der Kamera nicht mehr erreichbar. Verbinden Sie die Kamera erst danach per USB, öffnen Sie „Kamera-Infos“ und drücken Sie „Kamera-Infos abrufen“. Sobald Modell und Firmware-Version erscheinen, ist die Verbindung in Ordnung und alle weiteren Funktionalitäten dieses Programms stehen zur Verfügung.
welcome-start-driver = Erscheint stattdessen ein Treiberhinweis, folgen Sie zuerst diesem. Unter Windows belegt ein Systemtreiber die Kamera; er muss ersetzt werden, bevor der direkte USB-Zugriff möglich ist, den dieses Toolkit braucht. Das Programm erledigt das selbst, doch das Zuweisen eines Treibers ist ein Recht, das Windows nur Administratoren einräumt. In diesem Fall bietet Ihnen das Programm die Möglichkeit, es als Administrator neu zu starten, und Windows bittet Sie um Bestätigung des Vorgangs. Am Ende der Sitzung wird die Zuweisung wieder aufgehoben. Unter Linux und macOS ist das nicht erforderlich.
welcome-start-driver-linux = Erscheint stattdessen ein Treiberhinweis, folgen Sie zuerst diesem. Unter Linux muss nichts ersetzt werden: Die Kamera ist unmittelbar erreichbar. Einmalig nötig sind das Recht, das Gerät zu öffnen — eine udev-Regel für Sonys Hersteller-Kennung — und für den Massenspeichermodus das Lösen des Kerneltreibers usb-storage. Beides wird außerhalb dieses Programms mit Root-Rechten eingerichtet; das Skript scripts/linux-prepare.sh im Quelltext erledigt beides. Dieses Fenster selbst fordert solche Rechte nie an.
welcome-start-driver-macos = Erscheint stattdessen ein Treiberhinweis, folgen Sie zuerst diesem. Unter macOS muss für den direkten USB-Zugriff nichts ersetzt werden. Nur der Massenspeichermodus läuft über Sonys eigenen Camera Driver, der gesondert installiert wird und dabei wie jedes Installationsprogramm nach einem Administratorkennwort fragt. Dieses Fenster selbst fordert solche Rechte nie an.
welcome-tweaks-heading = Tweaks und Sprachen
welcome-tweaks-body = Tweaks sind Einstellungen, die die Kamera bereits besitzt, in ihrem Menü aber nicht anbietet: die Aufnahmezeitbegrenzung, die 4K-Begrenzung, die Wahl zwischen PAL und NTSC, den USB-App-Installer — und die Menüsprachen.
welcome-tweaks-languages-heading = Die Sprachen der Kamera
welcome-tweaks-languages-body = Sony-Kameras zeigen nur die Menüsprachen an, die in ihrer Verkaufsregion üblich sind. Wer im Ausland lebt, findet seine eigene Sprache darum oft nicht und muss sich mit einer fremden behelfen. Weitere Sprachen stecken aber bereits in der Firmware. Dieses Programm holt sie ins Menü, damit Sie Ihre Kamera in einer vertrauten Sprache bedienen können.
welcome-tweaks-ui-heading = Die Sprache dieses Programms
welcome-tweaks-ui-body = Davon getrennt ist die Auswahl oben. Sie ändert nur die Sprache dieses Programms und rührt die Kamera nicht an.
welcome-tweaks-safety = Jeder Tweak wird in den Sicherungsspeicher der Kamera geschrieben, und jeder lässt sich mit demselben Schalter wieder zurücknehmen. Ein unterbrochener Schreibvorgang kann die Kamera dauerhaft beschädigen — ziehen Sie währenddessen weder das Kabel ab noch den Akku heraus.
welcome-privacy-heading = Ihre Daten
welcome-privacy-where = Auf der Seite „Lizenz“ steht dies alles noch einmal vollständig, zusammen mit den Lizenzen aller Bibliotheken, aus denen dieses Programm gebaut ist.
welcome-where-heading = Wo finde ich was
welcome-where-info = Modell, Seriennummer und Firmware-Version. Zugleich der Verbindungstest.
welcome-where-install = Apps aus der Community-Liste oder eine eigene .apk-Datei.
welcome-where-tweaks = Die Einstellungen, die das Kameramenü nicht erreicht, darunter die Sprachen.
welcome-where-network = GPS-Assistenzdaten sowie Zugangspunkte und Dienste für das Live-Streaming.
welcome-where-firmware = Ein Sony-Firmware-Abbild auf die Kamera schreiben.
welcome-where-tools = Firmware- und Paketdateien auspacken und ansehen. Ganz ohne Kamera.
welcome-where-shells = Direkter Zugriff auf Speicher und Dateisystem der Kamera. Vorher in den Einstellungen einblenden.
welcome-where-settings = Design, erweiterter Modus und das Bearbeiten der Übersetzung dieses Programms.
welcome-where-license = Woher diese Portierung stammt, auf wessen Arbeit sie aufbaut und unter welchen Bedingungen sie steht.
welcome-where-hint = Keine Seite greift von sich aus auf die Kamera zu. Es wird nichts gelesen oder geschrieben, bevor Sie eine Schaltfläche drücken, die das ankündigt.

# Schaltflächen
btn-get-info = Kamera-Infos abrufen
btn-refresh = Aktualisieren
btn-load-apps = Verfügbare Apps laden
btn-list-tweaks = Verfügbare Tweaks anzeigen
btn-install = Ausgewählte App installieren
btn-install-file = .apk-Datei installieren…
btn-apply = Übernehmen
btn-save = Auf Kamera speichern
btn-add = Hinzufügen
btn-remove = Entfernen
btn-reveal = Anzeigen
btn-update-gps = GPS-Assistenzdaten auf das Gerät übertragen
btn-cancel = Abbrechen
btn-browse = Durchsuchen…
btn-switch-msc = In den Massenspeichermodus wechseln
btn-start-driver-helper = Treiberhilfe starten

# Bezeichnungen der Dateitypfilter in den Dateidialogen
file-filter-apk = Android-Anwendung (*.apk)
file-filter-firmware = Sony-Firmware-Image (*.dat)
file-filter-usage-log = Nutzungsprotokoll (*.bin)
file-filter-catalog = Übersetzungsdatei (*.ftl)
file-filter-spk = Sony-App-Paket (*.spk)
file-filter-any = Alle Dateien
file-filter-gps = GPS-Assistenzdaten (*.dat)

# Status / Meldungen
msg-usage-log-written = Nutzungsprotokoll geschrieben nach { $path }
msg-connecting = Verbinde…
msg-apps-found = { $count } Apps gefunden
msg-switch-mode = Bitte die Kamera in den MTP- oder Massenspeicher-Modus schalten.
msg-no-tweaks = Für diese Kamera sind keine Tweaks verfügbar.
msg-done = Fertig.

# USB-Moduswechsel der Kamera (MTP -> Massenspeicher). Wird unter einer
# "falscher Modus"-Meldung angeboten; für die Gegenrichtung gibt es keinen Befehl.
mode-switching = Kamera wird in den Massenspeichermodus geschaltet…
mode-switch-wait = Warte darauf, dass sich die Kamera neu anmeldet…
camera-confirm-hint = Hinweise an der Kamera (z. B. auf www.sony.net/pm) können weggeklickt werden; sie stören den Vorgang nicht.
mode-switch-done = Die Kamera ist jetzt im Massenspeichermodus.
mode-switch-timeout = Die Kamera ist nicht im Massenspeichermodus erschienen. Bitte aus- und wieder einschalten und es erneut versuchen.
mode-switch-impossible = Dieser Modus lässt sich nicht per USB umschalten. Bitte die Kamera aus- und wieder einschalten.
mode-switch-hint = Nur eine Kamera im MTP-Modus lässt sich per USB umschalten. Der Wechsel ist vorübergehend: Die im Kameramenü eingestellte USB-Verbindungsart bleibt unverändert. Die Kamera aus- und wieder einschalten oder das Kabel neu anstecken stellt den vorherigen Modus wieder her — einen USB-Befehl für den Rückweg gibt es nicht.

# Kamera-Infofelder
info-model = Modell
info-product-code = Produktcode
info-serial = Seriennummer
info-firmware = Firmware-Version
info-lens = Objektiv
info-region = Region
info-mode = Modus
mode-mass-storage = Massenspeicher
mode-mtp = MTP
mode-updater = Updater-Modus
mode-app-install = App-Installationsmodus
mode-service = Servicemodus
mode-unknown = Unbekannt
info-android-version = Android-Plattformversion
info-photographer = Fotograf
info-copyright = Copyright
info-photographer-1 = Fotograf (Eintrag 1)
info-photographer-2 = Fotograf (Eintrag 2)
info-copyright-1 = Copyright (Eintrag 1)
info-copyright-2 = Copyright (Eintrag 2)
diagnose-services-heading = Kamera wird nicht erreicht?
diagnose-services-hint = Einige Windows-Dienste können eine Kamera im MTP-Modus belegen, sodass diese Anwendung sie nicht erreicht. Die Prüfung fragt Windows nur, welche Dienste laufen; sie verändert nichts.
btn-diagnose-services = Windows-Dienste prüfen
diagnose-services-none = Kein bekannter störender Dienst läuft.
diagnose-services-found = Diese laufenden Dienste können den Zugriff auf die Kamera stören:
diagnose-services-advice = Beenden Sie den Dienst unter „Dienste“ (services.msc) oder mit folgendem Befehl als Administrator und verbinden Sie die Kamera danach neu:

# Firmware-Updater / Shell
updater-model = Kameramodell
updater-firmware-current = Aktuelle Firmware-Version
updater-guard-ok = Firmware-Abbild passt zu dieser Kamera.
updater-no-fdat = Für das Modell { $model } ist keine mitgelieferte Updater-Firmware verfügbar.
updater-model-required = Die Kamera ist bereits im Updater-Modus und kann dort ihr Modell nicht melden. Schalten Sie die Kamera aus und wieder ein, damit sie im normalen Modus startet, und versuchen Sie es dann erneut. Alternativ geben Sie eine Payload-Datei an (--fdat-file) oder, in der Konsole, das Modell (--model <MODELL>).
updater-already-updater = Kamera ist bereits im Updater-Modus.
updater-flash-required = Änderung der Kamera ohne Bestätigung wird abgelehnt. Führen Sie den Befehl mit --flash erneut aus, um in den Updater-Modus zu wechseln und die Payload zu schreiben.
updater-flashing = Updater-Payload wird auf die Kamera geschrieben…
updater-switch-wait = Bitte folgen Sie den Anweisungen auf dem Kamerabildschirm. Warte auf Wechsel in den Updater-Modus…
updater-switch-timeout = Zeitüberschreitung beim Warten auf die Kamera. Bitte führen Sie den Befehl erneut aus, sobald sie wieder verbunden ist.
updater-shell-ready = Updater-Payload läuft. 'help' für Befehle, 'exit' zum Beenden.
updater-shell-done = Shell-Sitzung beendet.
updater-session-panic = Die Updater-Sitzung wurde unerwartet beendet. Schalten Sie die Kamera aus und wieder ein, bevor Sie es erneut versuchen.

# Antworten der Updater-Shell. Die Fehler beim Zerlegen einer Zeile sind nur
# einmal formuliert, in den service-*-Schlüsseln: beide Shells teilen sich den
# Zerleger, also scheitern ein Pfad in Anführungszeichen und ein falscher
# Hexwert in beiden auf dieselbe Weise.
updater-backup-synced = Backup in den Flash-Speicher geschrieben.
updater-backup-locked = Die geschützten Backup-Einstellungen sind jetzt gesperrt.
updater-backup-writable = Die geschützten Backup-Einstellungen sind jetzt beschreibbar.
updater-shell-console-only = Die Root-Shell braucht ein Terminal und steht nur auf der Konsole zur Verfügung: Rufen Sie pmca-console updatershell --flash auf und geben Sie dort 'shell' ein.

# Die Seite der Updater-Shell.
updater-warning = Achtung: Um diese Shell zu erreichen, startet die Kamera in den Updater-Modus neu und es wird eine Payload auf sie geschrieben – ein kleines Programm dieses Projekts, das die Kamera wie ein Firmware-Update annimmt und das anschließend die Befehle dieser Shell beantwortet. Die Payload verschwindet wieder, sobald die Kamera ausgeschaltet wird, aber ihre Befehle schreiben in den dauerhaften Einstellungsspeicher der Kamera, und ein abgebrochener Schreibvorgang kann die Kamera dauerhaft beschädigen. Außerdem werden dabei alle Einstellungen der Kamera zurückgesetzt – anders als bei der Service-Shell.
updater-console-heading = Die Updater-Shell auf der Konsole ausführen
updater-driver-hint = Die Updater-Shell braucht die Kamera am hauseigenen Massenspeichertreiber von Windows, USBSTOR, denn darüber wird die Payload übertragen. Das ist genau das Gegenteil dessen, was die Service-Shell braucht. Die Anwendung weist den WinUSB-Treiber selbst zu und gibt ihn selbst wieder frei, es ist also nichts von Hand vorzubereiten – ein von einem anderen Werkzeug zugewiesener Treiber muss aber zuerst zurückgenommen werden. Eine Kamera im Updater-Modus meldet sich unter einer eigenen Produkt-ID an und darf ebenfalls nicht umgebunden werden.
updater-fdat-label = Payload-Datei (optional)
updater-confirm = Die Kamera startet in den Updater-Modus neu und es wird eine Payload auf sie geschrieben. Fortfahren?
updater-cancelled = Abgebrochen. Die Kamera wurde nicht verändert.

btn-shell-primer = Wissenswertes über diese Shell
btn-shell-primer-hide = Erklärung ausblenden
shell-primer-compare-heading = Zwei Shells, zwei Protokolle
shell-primer-compare = Beide Seiten bieten eine Befehlszeile zur Kamera, aber es ist nicht dieselbe Shell. Die Service-Shell spricht Sonys eigenes Werks-Serviceprotokoll, das die Kamera nach der Anmeldung selbst beantwortet; dafür wird nichts auf die Kamera geschrieben. Die Updater-Shell gibt es nur, weil dieses Projekt ein eigenes Programm in die Kamera schreibt: Die Kamera startet in den Updater-Modus neu, nimmt die Payload wie ein Firmware-Abbild an, und diese Payload beantwortet dann die unten aufgeführten Befehle. Es kann immer nur eine der beiden geöffnet sein.
shell-primer-payload = „Payload“ (englisch für Nutzlast) bezeichnet den eigentlichen Inhalt, den ein Transportweg befördert. Bei der Updater-Shell ist das ein kleines Programm dieses Projekts: Die Kamera nimmt es auf demselben Weg entgegen wie ein Firmware-Update, behält es aber nur bis zum Ausschalten. Die Service-Shell braucht keine Payload, weil die Kamera deren Befehle mit ihrer eigenen Firmware beantwortet. Bei Firmware-Dateien meint der Begriff dasselbe: das eigentliche Firmware-Abbild im Container der .dat-Datei.
shell-primer-when-service = Die Service-Shell ist für alles zuständig, was die gespeicherten Einstellungen der Kamera liest oder ändert, während sie ihre eigene Firmware weiterlaufen lässt – den HASP-Block lesen, eine Rücksicherung des Backup-Bereichs anlegen, einzelne Einträge ändern, Dateien von der Kamera holen. Sie steckt auch unter der Seite „Tweaks“; alles von dort lässt sich hier von Hand erledigen.
shell-primer-when-updater = Die Updater-Shell ist dann die richtige, wenn die Kamera anders nicht erreichbar ist oder wenn Sie statt einzelner Befehle eine Shell auf dem laufenden System brauchen. Nur dieser Weg bietet eine echte Root-Shell, und er funktioniert auch bei einer Kamera, die den Service-Modus verweigert. Er kostet einen Neustart, einen Schreibvorgang und die Kamera-Einstellungen, ist also die zweite und nicht die erste Wahl.
shell-primer-driver-heading = Treiber: Beide brauchen das Gegenteil
shell-primer-driver = Die Service-Shell spricht rohes USB und braucht unter Windows eine Kamera, die an WinUSB gebunden ist. Die Anwendung weist diesen Treiber auf dem Weg in den Servicemodus selbst zu und gibt ihn am Sitzungsende wieder frei; ein separates Werkzeug ist nicht beteiligt. Die Updater-Shell läuft über Windows' eigenen Massenspeichertreiber USBSTOR und funktioniert nicht, solange die Kamera an WinUSB gebunden ist – das Freigeben gehört deshalb zum Abschluss einer Servicesitzung und ist kein Nachgedanke. Das Zuweisen eines Treibers erfordert Administratorrechte. Unter Linux ist für keine von beiden ein Treiberwechsel nötig, nur der Zugriff auf den Geräteknoten.
shell-primer-risk-heading = Was schiefgehen kann
shell-primer-risk = Die unten markierten Befehle schreiben in den dauerhaften Einstellungsspeicher der Kamera. Ein abgebrochener Schreibvorgang kann dazu führen, dass die Kamera nicht mehr startet, und nichts davon lässt sich rückgängig machen – genau dafür gibt es bk d: Legen Sie eine Kopie des Backup-Bereichs an, bevor Sie etwas ändern. Die Updater-Payload selbst bleibt nicht auf der Kamera, beim Ausschalten verschwindet sie wieder. Der Weg über den Updater-Modus setzt allerdings alle Einstellungen der Kamera zurück.
shell-primer-commands-heading = Befehle dieser Shell
shell-primer-writes = Mit (WRITES) markierte Befehle verändern die Kamera. Alles andere liest nur.
shell-cmd-hasp = Liest den HASP-Block, den geschützten Identitätsbereich der Kamera.
shell-cmd-pull = Holt eine Datei von der Kamera. Ohne lokalen Namen wird der Name auf der Kamera übernommen.
shell-cmd-push = Kopiert eine lokale Datei auf die Kamera und ersetzt dabei, was dort liegt. (WRITES)
shell-cmd-rm = Löscht eine Datei auf der Kamera. (WRITES)
shell-cmd-rmem = Schreibt einen Bereich des Gerätespeichers in eine lokale Datei. Offset und Länge sind hexadezimal.
shell-cmd-bk-r = Liest einen Eintrag des Backup-Bereichs und zeigt ihn als Hex-Dump.
shell-cmd-bk-d = Sichert den gesamten Backup-Bereich in eine Datei. Tun Sie das, bevor Sie etwas ändern: Es ist der einzige Weg zurück.
shell-cmd-bk-dump-txt = Sichert den gesamten Backup-Bereich als <file>.bin und eine lesbare Auflistung als <file>.txt.
shell-cmd-bk-stat = Zeigt den Backup-Statusblock der Kamera einschließlich der Region.
shell-cmd-bk-w = Schreibt einen Eintrag des Backup-Bereichs, angegeben als hexadezimale Bytes mit Leerzeichen dazwischen. (WRITES)
shell-cmd-bk-s-sub = Sichert den Backup-Bereich eines Subsystems dauerhaft in den Flash, damit die Änderung einen Neustart übersteht. (WRITES)
shell-cmd-bk-s = Sichert den Backup-Bereich dauerhaft in den Flash, damit die Änderung einen Neustart übersteht. (WRITES)
shell-cmd-bk-lock = Setzt oder entfernt den Schreibschutz der geschützten Backup-Einträge; manche Einstellungen lassen sich bei gesetztem Schreibschutz nicht ändern. (WRITES)
shell-cmd-term = Schaltet das Debug-Terminal der Kamera ein oder aus. (WRITES)
shell-cmd-info = Liest den Backup-Statusblock, der angibt, ob der Bereich überhaupt beschreibbar ist.
shell-cmd-shell = Öffnet eine Root-Shell auf der Kamera. Sie braucht ein Terminal und steht deshalb nur in der Konsole zur Verfügung; shell tcp <port> bietet sie stattdessen auf 127.0.0.1 für einen anderen Client an. (WRITES)
shell-cmd-help = Listet die Befehle auf, die diese Shell annimmt.
shell-cmd-exit = Beendet die Sitzung und verlässt die Shell.
shell-primer-diagnostics-heading = Diagnose und Reparatur
shell-cmd-bk-trace = Zeichnet Paket für Paket in eine Datei auf, wie die Kamera den gesamten Backup-Bereich über USB sendet. Zur Untersuchung großer Backups; kann die Kamera mitten in der Antwort hängen lassen. Dann das Kabel trennen und die Kamera aus- und einschalten, notfalls den Akku entnehmen.
shell-cmd-bk-rewrite = Liest den Backup-Bereich, schreibt ihn unverändert zurück und liest ihn erneut. Zeigt, ob die Kamera ein ganzes Abbild annimmt, ohne eine Einstellung zu ändern. (WRITES)
shell-cmd-bk-restore-attrs = Setzt die Attribute aller Backup-Einträge, etwa den Schreibschutz, auf den Stand einer mit bk d oder bk dump-txt gesicherten Kopie zurück. Die Werte bleiben unverändert. Nur mit einer Kopie derselben Kamera verwenden. (WRITES)
shell-primer-diagnostics-env = Drei Umgebungsvariablen wirken auf die Konsole wie auf die grafische Oberfläche, wenn sie vor dem Start gesetzt werden, etwa in PowerShell mit $env:PMCA_TWEAKS_DIRECT = "1". PMCA_TWEAKS_DIRECT lässt die Tweaks einzelne Werte schreiben, auch wenn das Backup kleiner als 1 MB ist. PMCA_TWEAKS_IMAGE lässt sie das ganze Abbild schreiben, auch wenn es größer als 1 MB ist; sind beide gesetzt, gilt PMCA_TWEAKS_DIRECT. PMCA_TLS_DUMP=<Datei> speichert bei einer App-Installation die erste TLS-Nachricht der Kamera (ClientHello), um einen gescheiterten Verbindungsaufbau zu untersuchen.

# App-Installation
install-switching = Wechsle in den App-Installationsmodus…
install-disabled = Die Kamera hat den Wechsel in den App-Installationsmodus abgelehnt. Meist ist daran die Einstellung „USB-App-Installer aktivieren“ schuld: Sie muss unter „Tweaks“ eingeschaltet sein.
install-stays-app-mode = Die Kamera bleibt im App-Installationsmodus, in dem weitere Apps installiert werden können. Für alle anderen Funktionen das USB-Kabel trennen und die Kamera aus- und wieder einschalten.
err-usb-app-install-mode = Die Kamera befindet sich noch im App-Installationsmodus und nimmt darin nur App-Installationen an. Trennen Sie das USB-Kabel und schalten Sie die Kamera dann aus und wieder ein — bei angeschlossenem Kabel startet sie erneut in diesen Modus.
install-switch-wait = Warte, bis die Kamera im App-Installationsmodus neu verbindet…
install-switch-timeout = Zeitüberschreitung beim Warten auf den App-Installationsmodus. Bitte führen Sie den Befehl erneut aus, sobald die Kamera wieder verbunden ist.
install-bad-mode = Die Kamera kann in diesem Modus nicht verwendet werden. Bitte in den MTP- oder Massenspeichermodus wechseln.
install-starting = Installation wird gestartet…
install-error = Kommunikationsfehler bei der Installation
install-server-failed = Der lokale Installationsserver hat die Verbindung der Kamera nicht angenommen:
install-done = Aufgabe erfolgreich abgeschlossen.

# APK-Prüfung
apk-package = Paket:
apk-unreadable = Warnung: Die .apk-Datei konnte nicht analysiert werden.
apk-incompatible = Warnung: Diese App verlangt ein SDK-Level, das die Kamera möglicherweise nicht bietet.
apk-unsigned = Warnung: Die .apk-Datei enthält keine Signatur.

# Community-App-Store
store-loading = App-Liste wird geladen…
store-resolving = Neueste Version wird ermittelt…
store-downloading = Wird heruntergeladen
store-unknown-app = Unbekannte App
store-no-release = Keine herunterladbare Version für diese App
store-not-installable = keine Version
msg-no-apps = Keine Apps gefunden.
store-heading = Community-App-Store
store-discontinued = Sony hat den offiziellen PlayMemories-Camera-Apps-Store abgeschaltet; die Server antworten nicht mehr, Apps können dort also nicht mehr gekauft oder heruntergeladen werden.
store-use-own-apk = Eigene .apk-Dateien lassen sich weiterhin installieren: Datei unten auswählen oder in dieses Fenster ziehen. Der Community-App-Store weiter unten funktioniert ebenfalls noch.
btn-store-sync = Mit Kamera abgleichen
btn-install-selected = Ausgewählte installieren
btn-store-stop = Nach dieser App anhalten
btn-store-stopping = Wird nach dieser App angehalten…
btn-store-retry = Fehlgeschlagene erneut versuchen
btn-continue = Fortsetzen
btn-reinstall = Erneut installieren
btn-skip = Überspringen
btn-confirm-updater = Neu starten und schreiben
btn-confirm-tweaks = Tweaks schreiben
btn-confirm-languages = Sprachen aktivieren
btn-confirm-wifi = Zugangspunkte ersetzen
btn-confirm-stream = Einstellungen überschreiben
btn-confirm-show-shells = Shells einblenden
btn-confirm-discard = Übersetzung verwerfen
store-select-all = Alle Apps auswählen
store-col-app = App
store-col-version = Version
store-col-author = Autor
store-col-camera = App-Version Kamera
store-col-status = Status
store-camera-absent = nicht installiert
store-version-unknown = unbekannt
store-batch-query = Installierte Apps werden von der Kamera gelesen…
store-sync-done = Kamera abgeglichen. Installierte Apps aus der Liste: { $count }
store-batch-downloading = App { $current } von { $total } wird heruntergeladen…
store-batch-installing = App { $current } von { $total }: { $name } wird installiert…
store-reinstall-confirm = { $name } ist bereits auf der Kamera installiert. Installierte Version: { $installed }. Version im App-Store: { $store }. Soll die App erneut installiert werden?
store-state-queued = wartet
store-state-installed = installiert
store-state-failed = fehlgeschlagen
store-state-skipped = übersprungen
store-state-not-run = nicht ausgeführt
store-batch-summary = Installiert: { $done } · fehlgeschlagen: { $failed } · übersprungen: { $skipped } · nicht ausgeführt: { $notrun }

# Servicemodus (Senser)
usb-mode-advice = Für Tweaks, Backups und die Service-Shell den USB-Verbindungsmodus der Kamera auf „Auto“ stellen, nicht fest auf Massenspeicher. Fragt die Kamera nach einer Sitzung nach dem Modus, „Bildübertragung (MSC)“ bzw. „Massenspeicher“ wählen. Bei fest eingestelltem Massenspeicher verweigert die Kamera den nächsten Wechsel in den Servicemodus häufig.
service-switch-refused = Die Kamera hat den Wechsel in den Servicemodus nicht vollzogen und sich nicht neu am USB angemeldet; an der Kamera steht dann meist „Überprüfen Sie das angeschlossene Gerät“. Bitte das USB-Kabel trennen, wieder verbinden und es erneut versuchen.
service-switching = Wechsle in den Servicemodus…
service-switch-wait = Warte darauf, dass sich die Kamera im Servicemodus neu verbindet…
service-switch-timeout = Die Kamera ist nicht im Servicemodus erschienen. Sie meldet sich dort als eigenständiges USB-Gerät (Produkt-ID 0336) an, dem ebenfalls der WinUSB-Treiber zugewiesen werden muss; prüfe den Geräte-Manager auf ein Sony-Gerät ohne Treiber. Ein Aus- und Einschalten der Kamera bringt sie in den Normalmodus zurück.
service-authenticating = Authentifiziere…
# Wird angezeigt, wenn die Kamera noch aus einem früheren Lauf im Servicemodus
# steht — und wenn sich das nicht auflösen ließ.
service-stranded-reset = Die Kamera steht noch aus einem früheren Lauf im Servicemodus. Diese Sitzung wird zuerst beendet…
service-stranded-failed = Die Kamera hängt aus einem früheren Lauf im Servicemodus fest und hat auf die Aufforderung, ihn zu verlassen, nicht reagiert. Schalten Sie die Kamera aus und wieder ein und versuchen Sie es erneut.
service-bad-mode = Die Kamera kann in diesem Modus nicht verwendet werden. Bitte in den Massenspeichermodus wechseln.
service-driver-required = Direkter USB-Zugriff wurde verweigert, und der WinUSB-Treiber konnte nicht zugewiesen werden. Das Zuweisen eines Treibers erfordert Administratorrechte.
service-warning = Warnung: Der Servicemodus erlaubt direkten Zugriff auf Speicher, Dateisystem und Kalibrierdaten der Kamera. Schreibbefehle können die Kamera dauerhaft beschädigen.
service-shell-ready = Service-Shell bereit. 'help' zeigt die Befehle, 'exit' beendet.
service-shell-done = Service-Shell-Sitzung beendet.
# Wird beim Start gemeldet, wenn ein früherer Lauf abgebrochen wurde, bevor er
# den zugewiesenen Treiber wieder freigeben konnte.
driver-swept = { $count } Treiberzuweisung(en) aus einem früheren Lauf wurden entfernt.
# Wird beim Start gemeldet, wenn derselbe abgebrochene Lauf die Kamera zudem im
# Servicemodus zurückgelassen hat, wo sie die nächste Sitzung verweigern würde.
driver-swept-camera = Eine Kamera, die ein früherer Lauf im Servicemodus zurückgelassen hat, wurde in ihren normalen Modus zurückgeschaltet.
driver-release-replug = Windows wartet beim Freigeben des Treibers auf die Kamera. Bitte das USB-Kabel trennen und wieder verbinden, damit das Aufräumen abgeschlossen werden kann.
# Ergebnisse des Befehls `driver`.
driver-bound = { $id } verwendet jetzt WinUSB (Treiberpaket { $package }).
driver-released = { $count } Treiberzuweisung(en) aufgehoben; die Kamera nutzt wieder die von Windows mitgelieferten Treiber.
driver-needs-elevation = Das Zuweisen eines Treibers erfordert Administratorrechte. Bitte diesen Befehl in einer als Administrator gestarteten Konsole ausführen.
driver-leftover-needs-elevation = Ein früherer Lauf hat der Kamera einen Treiber zugewiesen, der noch nicht wieder freigegeben ist. Solange er besteht, fordert die Kamera dazu auf, das angeschlossene Gerät zu überprüfen, und ihr Massenspeicher funktioniert nicht. Das Freigeben erfordert Administratorrechte; bitte die Abfrage von Windows bestätigen.
driver-binding = WinUSB-Treiber wird der Kamera zugewiesen…
driver-bind-wait = Warte darauf, dass sich die Kamera mit dem neuen Treiber neu meldet…
driver-bind-timeout = Die Kamera ist nach der Treiberzuweisung nicht wieder aufgetaucht. Bitte abziehen, erneut anschließen und noch einmal versuchen.
driver-bind-failed = Der WinUSB-Treiber konnte nicht zugewiesen werden: { $detail }
# Antworten der Shell. Sie stammen aus dem Backend, damit Konsole und Fenster
# dieselben Worte verwenden.
service-usage = Aufruf: { $usage }
service-bad-hex = Kein Hexadezimalwert: { $value }
service-unknown-command = Unbekannter Befehl: { $command }. 'help' zeigt die Liste.
service-unterminated-quote = Die Zeile endet innerhalb eines Arguments in Anführungszeichen.
service-help = Befehlsübersicht:
service-read-bytes = { $count } Bytes gelesen.
service-pulled = { $remote } von der Kamera nach { $local } kopiert.
service-pushed = { $local } nach { $remote } auf die Kamera kopiert.
service-removed = { $remote } auf der Kamera gelöscht.
service-dumped = { $size } Bytes ab { $offset } nach { $local } ausgelesen.
service-backup-saved = Backup-Abbild ({ $count } Bytes) nach { $local } gesichert.
service-backup-text-saved = Backup-Abbild ({ $count } Bytes) nach { $bin } und Auflistung nach { $txt } gesichert.
service-backup-status = Backup-Status, Region: { $region }
service-backup-rewritten = Abbild ({ $count } Bytes) unverändert zurückgeschrieben und erneut gelesen; nicht dauerhaft gespeichert.
service-backup-attrs-restored = Attribute von { $count } Einträgen nach { $local } zurückgesetzt.
service-backup-written = Backup-Eintrag { $id } geschrieben.
service-backup-persisted = Backup des Subsystems { $id } dauerhaft gespeichert.
service-terminal-on = Das Debug-Terminal ist jetzt eingeschaltet.
service-terminal-off = Das Debug-Terminal ist jetzt ausgeschaltet.

# Die Service-Seite ohne erweiterten Modus: wie die Shell über die Konsole
# erreichbar ist und was dafür vorbereitet sein muss.
service-console-heading = Die Service-Shell in der Konsole starten
service-console-hint = Öffnen Sie eine Eingabeaufforderung im Ordner mit pmca-console und führen Sie diesen Befehl aus:
service-driver-heading = Vorbereitung
service-driver-hint = Der Servicemodus benötigt direkten USB-Zugriff, den Windows über den Massenspeichertreiber nicht gewährt. Die Anwendung weist den WinUSB-Treiber selbst zu – zweimal, einmal während die Kamera im Massenspeichermodus ist und ein zweites Mal, nachdem sie im Servicemodus unter einer zweiten Produkt-ID wieder erscheint – und gibt beide Zuweisungen am Sitzungsende wieder frei. Das Zuweisen eines Treibers erfordert Administratorrechte, die Konsole also als Administrator starten; ein weiteres Werkzeug wird nicht benötigt.
driver-note-linux = Hier muss nichts installiert werden: libusb erreicht die Kamera unmittelbar, ohne einen Kerneltreiber zu verdrängen. Nötig sind lediglich das Recht, das Gerät zu öffnen, und eine Arbeitsumgebung, die es nicht bereits belegt. Richten Sie eine udev-Regel für Sonys Hersteller-Kennung 054c ein und lösen Sie den Treiber usb-storage, bevor Sie den Massenspeichermodus verwenden. Das Skript scripts/linux-prepare.sh im Quelltext erledigt beides und beendet zusätzlich die gvfs-Datenträgerdienste.
driver-note-macos = Für den unmittelbaren USB-Zugriff muss nichts installiert werden: libusb erreicht die Kamera direkt. Der Massenspeichermodus ist die Ausnahme, denn er läuft über Sonys eigenen Camera Driver, der gesondert installiert werden muss. Diese Plattform ist die am wenigsten erprobte der drei; rechnen Sie eher damit, ein Problem zu melden, als es bereits beschrieben zu finden.
service-advanced-hint = Die Shell lässt sich auch hier einblenden, unter Einstellungen, Erweiterter Modus.
btn-copy-command = Befehl kopieren
msg-command-copied = Befehl in die Zwischenablage kopiert.

# Die Service-Seite mit eingeschaltetem erweitertem Modus.
btn-service-open = Verbinden
btn-service-close = Trennen
btn-service-run = Ausführen
btn-service-browse = Dateipfad einfügen…
shell-path-quote = Dieser Pfad lässt sich nicht einfügen: Er enthält ein Anführungszeichen, mit dem die Befehlszeile Anfang und Ende eines Arguments bestimmt. Benennen Sie die Datei um oder verschieben Sie sie an einen Pfad ohne dieses Zeichen.
service-command-label = Befehl
service-not-connected = Nicht verbunden.

# Nur in der GUI verwendet
install-apk-path = Pfad zu einer lokalen .apk-Datei (optional)
install-drop-hint = .apk-Datei hierher ziehen

# Beschreibung der Tweaks
tweak-rec-limit = Videoaufnahme ohne Zeitbegrenzung
tweak-rec-limit-4k = 4K-Videoaufnahme ohne Zeitbegrenzung
tweak-language = Alle Menüsprachen anzeigen
tweak-pal-ntsc = PAL/NTSC-Auswahl & Warnung aktivieren
tweak-pal-ntsc-mode = Videosystem (PAL / NTSC)
tweak-pal-ntsc-pal = PAL
tweak-pal-ntsc-ntsc = NTSC
tweak-pal-ntsc-note = Nach dem Wechsel des Videosystems verlangt die Kamera, die Speicherkarte zu formatieren. Sichern Sie vorher alle Daten — Bilder, Videos, LUTs. Am einfachsten ist es, die Karte vor dem Umschalten zu entnehmen.
tweaks-set-malformed = Erwartet wurde --set ID=WERT, erhalten: { $arg }
tweaks-set-unknown-value = Unbekannter Wert "{ $value }". Erwartet wird einer von: { $options }
tweaks-set-not-a-choice = { $id } ist ein Schalter, keine Auswahl. Bitte --enable oder --disable verwenden.
tweaks-needs-set = { $id } ist eine Auswahl, kein Schalter. Bitte --set { $id }=WERT mit einem von: { $options }
tweak-usb-installer = USB-App-Installer aktivieren

# Tweak-Ablauf
tweaks-warning = Warnung: Das Anwenden von Tweaks überschreibt den Backup-Speicher der Kamera. Ein unterbrochener Schreibvorgang kann die Kamera dauerhaft beschädigen.
tweaks-reading = Kameraeinstellungen werden gelesen…
tweaks-writing = Kameraeinstellungen werden geschrieben…
tweaks-done = Kameraeinstellungen aktualisiert.
tweaks-write-failed = Die Kamera hat die neuen Einstellungen nicht so gespeichert, wie sie gesendet wurden. Ein Teil kann trotzdem geschrieben worden sein: Bitte die Einstellungen erneut lesen und bei Bedarf zurücknehmen. Der sichere Rückweg ist eine Sicherung des Backups von vor der Änderung.
tweaks-confirm-required = Schreibvorgang ohne Bestätigung abgelehnt. Bitte nach dem Lesen der Warnung erneut mit --confirm ausführen.
tweaks-state-enabled = aktiviert
tweaks-state-disabled = deaktiviert
tweaks-full-read-failed = Die Kamera hat ihr Backup nicht vollständig geliefert ({ $detail }). Danach reagiert sie oft nicht mehr: Kamera ausschalten, den Akku einige Sekunden herausnehmen, wieder einsetzen und neu verbinden.
tweaks-direct-mode = Das Backup dieser Kamera ist größer als 1 MB. Die Einstellungen werden deshalb einzeln gelesen und geschrieben; vor jeder Änderung werden die alten Werte gesichert und lassen sich mit „Änderungen zurücknehmen“ zurückschreiben.
tweaks-direct-forced = Der Einzelwert-Weg ist über PMCA_TWEAKS_DIRECT erzwungen. Die Einstellungen werden deshalb einzeln gelesen und geschrieben; vor jeder Änderung werden die alten Werte gesichert und lassen sich mit „Änderungen zurücknehmen“ zurückschreiben.
tweaks-direct-refused = Die Kamera hat die Änderung abgelehnt ({ $detail }). Bereits geschriebene Werte wurden zurückgesetzt; es wurde nichts geändert. Manche Modelle, vor allem japanische, erlauben diese Einstellung nur über das ganze Backup-Abbild.
tweaks-direct-revert-failed = Die Kamera hat die Änderung abgelehnt ({ $detail }), und nicht alle bereits geschriebenen Werte ließen sich zurücksetzen. Die alten Werte sind gesichert: Bitte „Änderungen zurücknehmen“ versuchen.
tweaks-direct-restart = Kameraeinstellungen aktualisiert. Schalten Sie die Kamera aus und wieder ein, damit die Änderung wirksam wird.
tweaks-rollback-unsaved = Die aktuellen Werte konnten nicht zum Zurücknehmen gesichert werden. Es wurde nichts geändert.
tweaks-image-saved = Das Backup-Abbild von vor der Änderung wurde gespeichert: { $path }
tweaks-whole-image-hint = Um die Änderung stattdessen über das ganze Backup-Abbild zu schreiben, denselben Befehl mit --whole-image erneut ausführen. Das Abbild von vor der Änderung wird vorher gesichert.
tweaks-whole-image-confirm = Die Änderung über das ganze Backup-Abbild schreiben? Die Kamera hat sie als Einzelwert abgelehnt. Dabei wird das ganze Abbild mit der Änderung zurückgeschrieben; das Abbild von vor der Änderung wird vorher auf diesem Computer gesichert.
btn-whole-image = Ganzes Backup schreiben
tweaks-undo-none = Für diese Kamera sind keine gesicherten Werte vorhanden.
tweaks-undo-confirm = Die gesicherten Werte auf die Kamera zurückschreiben? Damit werden alle einzeln geschriebenen Änderungen seit der Sicherung zurückgenommen.
tweaks-undo-done = Die gesicherten Werte wurden zurückgeschrieben. Schalten Sie die Kamera aus und wieder ein.
tweaks-undo-available = Die Werte von vor der ersten Änderung sind gesichert und lassen sich zurückschreiben.
btn-undo-tweaks = Änderungen zurücknehmen
tweaks-identity-heading = Kamera-Identität (nur lesbar)
tweaks-caution-languages = Zwei Sprachen — vermutlich Chinesisch (vereinfacht) und Chinesisch (traditionell) — kann diese Kamera nicht anzeigen. Beim Blättern dorthin startet das Sprachmenü neu, deshalb bleiben sie vorsorglich ausgeschaltet.
tweaks-caution-accept = Diese beiden Sprachen trotzdem aktivieren (auf eigene Verantwortung)
tweaks-caution-confirm = Diese beiden Sprachen können das Sprachmenü der Kamera neu starten lassen. Es geht nichts kaputt und die Änderung lässt sich rückgängig machen, aber Sie müssen die Einstellung eventuell von vorn suchen. Trotzdem aktivieren?

# Einstellungen
settings-language = Sprache
settings-theme = Design
settings-theme-system = System
settings-theme-light = Hell
settings-theme-dark = Dunkel

# Erweiterter Modus. Ein Schutz vor dem Verklicken, nicht vor einem Angreifer:
# alles, was er einblendet, ist über die Konsole ohnehin erreichbar.
settings-advanced-heading = Erweiterter Modus
settings-advanced-hint = Der erweiterte Modus blendet die Service-Shell und die Updater-Shell auf den gleichnamigen Seiten ein. Der Servicemodus erlaubt direkten Zugriff auf den Speicher, das Dateisystem und die Kalibrierdaten der Kamera; ein vertippter Befehl kann die Kamera unbrauchbar machen. Lassen Sie ihn ausgeschaltet, solange Sie nicht wissen, was Sie schreiben werden.
settings-advanced-toggle = Service- und Updater-Shell einblenden
settings-advanced-on = Der erweiterte Modus ist eingeschaltet. Die Seiten Service-Shell und Updater-Shell bieten nun die Shells selbst an.
settings-advanced-off = Der erweiterte Modus ist ausgeschaltet. Die Seiten Service-Shell und Updater-Shell erklären stattdessen den Weg über die Konsole.
settings-advanced-confirm = Service- und Updater-Shell können Ihre Kamera dauerhaft beschädigen. Beide Shells einblenden?

# Bearbeiten einer Übersetzung. Sprachdateien lassen sich herunterladen, in
# einem beliebigen Texteditor ändern und wieder hochladen; eine hochgeladene
# Datei wird vor der Verwendung geprüft.
settings-catalog-heading = Übersetzung bearbeiten
settings-catalog-hint = Laden Sie eine Sprachdatei herunter, ändern Sie sie in einem beliebigen Texteditor und laden Sie sie wieder hoch. Übersetzt wird nur der Text rechts vom Gleichheitszeichen; weggelassene Einträge behalten ihren mitgelieferten Wortlaut. Die hier gewählte Sprache ändert nicht die Sprache der Anwendung.
settings-catalog-locale = Sprachdatei
settings-catalog-builtin = Es wird die mitgelieferte Übersetzung verwendet.
settings-catalog-custom = Es wird Ihre eigene Übersetzung verwendet:
btn-catalog-export = Herunterladen…
btn-catalog-import = Hochladen…
btn-catalog-reset = Mitgelieferte wiederherstellen
catalog-exported = Geschrieben nach
catalog-imported = Übersetzung übernommen.
catalog-reset-done = Die mitgelieferte Übersetzung wurde wiederhergestellt.
catalog-reset-none = Es gab keine eigene Übersetzung zum Wiederherstellen.
catalog-reset-confirm = Eigene Übersetzung für diese Sprache verwerfen?
catalog-rejected = Die Datei wurde nicht übernommen. Bitte korrigieren Sie diese Zeilen:
catalog-count-accepted = Übernommene Einträge
catalog-count-missing = Aus der mitgelieferten Übersetzung beibehaltene Einträge
catalog-count-untranslated = Einträge, die wörtlich dem englischen Original entsprechen
catalog-line = Zeile
catalog-problem-syntax = kein Eintrag der Form "id = Text"
catalog-problem-continuation = in der folgenden Zeile fortgesetzt, was nicht unterstützt wird
catalog-problem-duplicate = mehrfach definiert
catalog-problem-unknown-key = kein Eintrag, den diese Anwendung verwendet
catalog-problem-empty = ohne Text
catalog-problem-placeholder = Platzhalter geändert, erwartet:

# Herkunft, Danksagung und Lizenz. Wird auf der Lizenzseite und von
# `pmca-console license` angezeigt. Der Lizenztext selbst wird nicht übersetzt:
# Er ist ein Rechtsdokument und wird wortgetreu wiedergegeben.
about-thanks-heading = Danke
about-thanks-body-1 = Dieses Werkzeug gibt es dank ma1co, der herausgefunden hat, wie Sonys Kameras über USB kommunizieren, und der Sony-PMCA-RE 2015 veröffentlicht hat, und dank aller, die in den Jahren danach dazu und zu den OpenMemories-Projekten beigetragen haben.
about-thanks-body-2 = Die Kameraerkennung, das Protokoll für externe Befehle, der Ablauf des Firmware-Updaters und der Service-Modus sind allesamt ihre Arbeit, und diese Portierung führt diese Arbeit fort. Vielen Dank. Auf diesem Fundament bringt PMCA-RE RELOADED viele neue Funktionen mit, die masterflai beigesteuert hat. Allen voran die nun integrierte automatische Umschaltung des Gerätetreibers unter Windows: Der Kamera muss kein anderer Treiber mehr von Hand mit einem separaten Werkzeug zugewiesen werden — das macht den Ablauf bequemer, schneller und risikoärmer.
about-contributors-heading = Mitwirkende
about-contributors-body = Neben ma1co haben diese Personen zu Sony-PMCA-RE beigetragen oder in eigenen Abspaltungen Änderungen veröffentlicht, die in diese Portierung übernommen wurden:
about-purpose-heading = Warum es diese Portierung gibt
about-purpose-body-1 = Die ursprüngliche Anwendung wird nicht mehr weiterentwickelt. Sie wurde für Python 2 geschrieben, dessen Lebensdauer 2020 endete, und Teile davon setzen auf Dienste, die Sony inzwischen abgeschaltet hat, sodass sie auf heutigen Systemen nicht mehr uneingeschränkt läuft.
about-purpose-body-2 = Zweck dieser Portierung ist Bewahrung statt Ersatz und ebenso eine problemlose Nutzung heute: Diese Kameras sollen nutzbar bleiben, und das Programm soll auf aktuellen Computern ebenso wie mit neueren Kameramodellen ohne Schwierigkeiten funktionieren. Alles, was das Original konnte, soll auch hier weiterhin möglich sein.
about-origin-heading = Wie sie entstanden ist
about-origin-body-1 = Die Portierung von Python nach Rust wurde mit Hilfe von KI durchgeführt.
about-origin-body-2 = Was das nicht bedeutet, sei ebenso deutlich gesagt: Das Verhalten ist durch eine automatisierte Testsuite und durch einen Hardware-Testplan abgedeckt. Vor allem die Windows-Version wurde ausführlich geprüft, weil die Gerätetreiber sie zur aufwendigsten Plattform machen; macOS und Linux konnten dagegen nicht vollumfänglich getestet werden. Bitte betrachten Sie dies als experimentelle Software.
about-license-heading = Lizenz
about-license-body = Diese Portierung wird unter der MIT-Lizenz veröffentlicht, derselben Lizenz wie das ursprüngliche Projekt. Der ursprüngliche Copyright-Hinweis bleibt unverändert erhalten, wie es diese Lizenz verlangt.
about-license-full = Lizenztext
about-third-party-heading = Lizenzen der verwendeten Bibliotheken
about-third-party-body = Dieses Programm ist statisch gebunden: Der Code jeder verwendeten Bibliothek steckt in der ausführbaren Datei, und deren Lizenzen verlangen, dass die Urheberrechtshinweise mitgeliefert werden. Sie sind unten vollständig wiedergegeben, ein Eintrag je Paket.
about-third-party-toggle = Alle Drittlizenzen anzeigen
about-privacy-heading = Daten
about-privacy-body-1 = Dieses Programm läuft auf Ihrem eigenen Rechner. Es sendet keine Nutzungsdaten, es gibt keine Telemetrie und keinen Konto-Zwang. Nichts von dem, was Sie hier tun, wird irgendwohin gemeldet.
about-privacy-body-2 = Zwei Funktionen stellen jedoch von sich aus Verbindungen her, und dabei erfährt die Gegenstelle Ihre IP-Adresse: Die GPS-Assistenzdaten werden von Sony geladen und die App-Liste von GitHub. Ohne diese Funktionen bleibt das Programm vollständig offline.
about-privacy-body-3 = Was aus der Kamera ausgelesen wird — darunter ihre Seriennummer und gespeicherte WLAN-Kennwörter — verlässt Ihren Rechner nicht. Wenn Sie es in eine Datei schreiben, achten Sie darauf, an wen Sie diese Datei weitergeben.
about-privacy-body-4 = Hinter diesem Programm wird kein Dienst betrieben, und an seine Urheber gelangen keine Daten; es gibt hier also nichts, worüber Auskunft erteilt oder was gelöscht werden könnte. Was die zwei genannten Dienste mit den Adressen tun, die sie sehen, richtet sich nach deren eigenen Datenschutzerklärungen, nicht nach dieser.
about-legal-heading = Rechtliche Hinweise
about-legal-trademark = Dies ist ein unabhängiges Projekt. Es stammt nicht von Sony, ist von Sony weder unterstützt noch in irgendeiner Weise mit Sony verbunden. »Sony«, »PlayMemories« und die Modellbezeichnungen der Kameras sind Marken der Sony Group Corporation und werden hier ausschließlich genannt, um zu benennen, mit welchen Kameras dieses Werkzeug spricht.
about-legal-own-device = Dieses Programm ist für die Verwendung mit Kameras bestimmt, die Ihnen gehören.
about-legal-warranty = Was dieses Werkzeug tut, liegt außerhalb dessen, was Sony für diese Kameras vorsieht, und kann Ihre Gewährleistung beenden. Die Software selbst wird ohne jede Gewährleistung bereitgestellt; die Nutzung erfolgt auf eigenes Risiko. Ob und in welchem Umfang das Reverse Engineering eines Geräts, das Ihnen gehört, zulässig ist, hängt vom jeweiligen Land ab.
about-links-heading = Projekte
about-link-original = Sony-PMCA-RE — das ursprüngliche Projekt
about-link-fwtool = fwtool — die Firmware-Werkzeuge, auf denen die Erzeugung der Update-Abbilder beruht
about-link-tweak = OpenMemories: Tweak — die empfohlene App für Kameras, die sie unterstützen
about-link-applist = OpenMemories: App list — weitere Apps
about-link-platform = OpenMemories: Platform — der mitgelieferte Updater-Body
about-link-devices = Unterstützte Kameramodelle
about-link-nexhack = Nex-Hack — die Gemeinschaftsforschung, auf der dieses Projekt aufbaut
about-link-zve10 = zv-e10-re — Untersuchungen zu Schnittstellen und Einstellungsspeicher der ZV-E10
about-version = Version
btn-copy-url = Adresse kopieren
msg-url-copied = Adresse in die Zwischenablage kopiert.

# Sicherheitshinweis
notice-experimental = Dies ist experimentelle Software. Nutzung auf eigenes Risiko.

# Netzwerkeinstellungen (GPS / WLAN / Live-Streaming)
gps-downloading = GPS-Assistenzdaten werden heruntergeladen…
gps-writing = GPS-Assistenzdaten werden auf die Kamera geschrieben…
gps-bad-size = Die GPS-Assistenzdaten haben eine unerwartete Größe und wurden nicht geschrieben.
wifi-read = Zugangspunkte für das Live-Streaming werden von der Kamera gelesen…
wifi-written = Zugangspunkte für das Live-Streaming geschrieben.
stream-read = Live-Streaming-Einstellungen werden von der Kamera gelesen…
stream-written = Live-Streaming-Einstellungen geschrieben.
net-secret-warning = Achtung: Diese Datei enthält WLAN-Passwörter und Zugriffstoken im Klartext.
wifi-secret-warning = Die Kamera speichert diese Kennwörter unverschlüsselt, und sie werden hier genau so angezeigt, wie sie gespeichert sind. Sie bleiben auf diesem Rechner; wenn Sie sie in eine Datei sichern, achten Sie darauf, an wen Sie diese weitergeben.
gps-heading = GPS-Assistenzdaten
gps-hint = Datei hierher ziehen, Pfad eingeben oder das Feld leer lassen, um die aktuellen Daten von Sony zu beziehen.
btn-gps-devices = Kompatible Geräte
btn-gps-devices-hide = Kompatible Geräte ausblenden
gps-devices-note = Diese Modelle haben ein eingebautes GPS-Modul. Bei vielen hat es nur die Variante, deren Name auf V endet; Varianten ohne V haben kein GPS.
gps-devices-compact = Kompaktkameras (DSC)
gps-devices-slt = SLT-Kameras (A-Mount)
gps-devices-action = Action-Cams
gps-devices-camcorder = Camcorder (Handycam)
gps-col-variant = GPS-Variante
gps-col-arch = Architektur
gps-col-module = GPS-Modul
gps-col-apps = App-Support
gps-yes = ja
gps-no = nein
gps-unknown = unbekannt
gps-devices-untested = GPS vorhanden, Unterstützung ungetestet
gps-devices-untested-note = Diese Modelle haben ein eingebautes GPS-Modul, stehen aber nicht in der Geräteliste von OpenMemories. Ob diese Anwendung mit ihnen zusammenarbeitet, ist nicht getestet; Architektur und App-Support sind daher unbekannt.
wifi-heading = Zugangspunkte für das Live-Streaming
wifi-hint = Dies sind die Zugangspunkte der Live-Streaming-Funktion, nicht die WLAN-Einstellungen der Kamera. Ein Gehäuse ohne Streaming-Funktion lehnt das ab.
wifi-multi = Liste mit mehreren Zugangspunkten
wifi-none = Keine Zugangspunkte gespeichert.
wifi-writing = Zugangspunkte für das Live-Streaming werden auf die Kamera geschrieben…
wifi-confirm = Dies ersetzt alle auf der Kamera gespeicherten Zugangspunkte. Fortfahren?
wifi-empty-refused = Eine leere Liste wird nicht geschrieben, da sie alle gespeicherten Zugangspunkte löschen würde.
stream-heading = Live-Streaming
stream-writing = Live-Streaming-Einstellungen werden auf die Kamera geschrieben…
stream-confirm = Dies überschreibt die Live-Streaming-Einstellungen der Kamera. Fortfahren?
stream-invalid-json = Die Einstellungen sind kein gültiges JSON:

# Firmware-Update und offizieller Store
firmware-warning = Achtung: Dieser Vorgang überschreibt die Firmware der Kamera. Unterbrechen Sie ihn nach dem Start nicht: Ziehen Sie das USB-Kabel nicht ab und schalten Sie die Kamera nicht aus. Verwenden Sie nur eine Firmware-Datei, die für genau dieses Kameramodell bestimmt ist, und achten Sie auf einen ausreichend geladenen Akku. Andernfalls kann die Kamera dauerhaft unbrauchbar werden.
firmware-untested = Ungetestet: Das Flashen der Firmware ist portiert, aber nie an einer echten Kamera erprobt worden. Ein misslungener oder abgebrochener Schreibvorgang lässt sich nicht rückgängig machen; betrachten Sie daher alles auf dieser Seite als unbestätigt.
firmware-confirm-required = Ohne --confirm wird nicht geschrieben.
firmware-truncated = Die Firmware-Datei ist unvollständig: Die FDAT-Payload reicht über das Dateiende hinaus.
firmware-updating = Firmware-Aktualisierung:
firmware-invalid = Die Datei ist kein gültiges Sony-Firmware-Image
firmware-dat-path = Firmware-Datei (.dat)
firmware-drop-hint = .dat-Firmware-Datei hier ablegen
firmware-file-required = Bitte zuerst eine Firmware-Datei auswählen.
firmware-check-ok = Die Kamera akzeptiert dieses Image. Es wurde nichts geschrieben.
firmware-cancelled = Flashen abgebrochen. Die Kamera wurde nicht verändert.
firmware-current-version = Installierte Version
firmware-new-version = Version in diesem Image
firmware-models-hint = Kompatibel sind die Kameras, die sich über die Updater-Shell ansprechen lassen.
btn-firmware-check = Image prüfen
btn-firmware-flash = Firmware flashen
firmware-own-device = Bestätigen Sie, dass Sie sich dieser Risiken bewusst sind und ausschließlich Ihr eigenes Gerät aktualisieren.
btn-firmware-accept = Risiken verstanden – flashen

# Dateiwerkzeuge: die Vorgänge, die gar keine Kamera brauchen.
tools-hint = Diese Vorgänge arbeiten auf Dateien und rühren die Kamera nie an, hier kann also nichts kaputtgehen. Es sind dieselben Vorgänge, die pmca-console als apk2spk, spk2apk, print-backup, print-usagelog, guess-firmware und xpd-checksum anbietet.
tools-operation = Vorgang
tools-apk2spk = Eine .apk in einen .spk-Container verpacken
tools-spk2apk = Eine .spk wieder in eine .apk auspacken
tools-print-backup = Den Inhalt einer Backup.bin anzeigen
tools-print-usagelog = Den Inhalt einer Usagelog.bin anzeigen
tools-guess-firmware = Die Payload in einer Firmware-Datei (.dat) finden
tools-xpd-checksum = Die CIC-Prüfsumme einer Datei berechnen
tools-input = Eingabedatei
tools-file-required = Wählen Sie zuerst die Dateien aus, die dieser Vorgang braucht.
tools-apk-invalid = Die Datei ließ sich nicht als .spk verpacken
tools-spk-invalid = Die Datei ist kein gültiges .spk-Paket
tools-backup-invalid = Die Datei ist kein gültiges Backup.bin-Abbild
tools-usagelog-invalid = Die Datei ist kein gültiges Nutzungsprotokoll
tools-reset-value = Rücksetzwert:
tools-fdat-found = Payload bei Offset { $offset } gefunden, Größe { $size }.
tools-checksum = Prüfsumme
tools-written = Geschrieben nach { $path }

# Verweise auf die Projektdokumentation
docs-compatibility = Kamera-Kompatibilität
docs-compatibility-hint = Das ursprüngliche Projekt hat eine Übersicht geprüfter Kameras bereitgestellt. Sie umfasst vorwiegend ältere Kameramodelle und ist unter folgender Adresse zu finden:
devices-note = Angaben aus der Übersicht des ursprünglichen Projekts. Die Zahl beim App-Support ist die Version der App-Plattform der Kamera.
devices-more-note = Die folgenden Kameras wurden mit diesem Programm nicht ausdrücklich getestet. Ein Haken in der letzten Spalte bedeutet, dass die Community über ihren Einsatz berichtet hat. Ist zusätzlich die Service-Shell mit ✖ markiert, war der Einsatz erfolglos.
btn-more-models = Weitere Geräte
btn-more-models-hide = Weitere Geräte ausblenden
check-updater-yes = Ihr Gerät { $model } ist kompatibel: Laut Geräteliste lässt es sich über die Updater-Shell ansprechen.
check-updater-no = Ihr Gerät { $model } ist nicht kompatibel: Laut Geräteliste lässt es sich nicht über die Updater-Shell ansprechen.
check-updater-unknown = Die Kompatibilität Ihres Geräts { $model } ist unbekannt: Es steht in keiner Geräteliste dieses Programms, daher ist nicht bekannt, ob es sich über die Updater-Shell ansprechen lässt.
check-apps-no = Ihr Gerät { $model } ist nicht kompatibel: Laut Geräteliste bietet es keinen App-Support.
check-apps-untested = Die Kompatibilität Ihres Geräts { $model } ist unbekannt: Es steht in der Geräteliste, ob es App-Support bietet, ist aber nicht bestätigt.
check-apps-unknown = Die Kompatibilität Ihres Geräts { $model } ist unbekannt: Es steht in keiner Geräteliste dieses Programms, daher ist nicht bekannt, ob es App-Support bietet.
check-gps-no = Ihr Gerät { $model } ist nicht kompatibel: Es steht nicht in der Liste der Geräte mit GPS-Empfänger.
check-gps-untested = Die Kompatibilität Ihres Geräts { $model } ist unbekannt: Laut Geräteliste hat es einen GPS-Empfänger, die Übertragung ist mit diesem Modell aber ungetestet.
check-gps-unknown = Die Kompatibilität Ihres Geräts { $model } ist unbekannt: Es steht in keiner Geräteliste dieses Programms, daher ist nicht bekannt, ob es einen GPS-Empfänger hat.
check-unidentified = Das angeschlossene Gerät ließ sich nicht identifizieren, seine Kompatibilität ist daher unbekannt.
check-proceed = Möchten Sie trotzdem fortfahren?
check-cancelled = Vorgang abgebrochen. Das Gerät wurde nicht verändert.
devices-col-confirmed = Bestätigt
devices-group-ilc = Systemkameras (E- und A-Mount)
devices-group-remote = Live-View-Fernbedienungen
btn-copy-link = Link kopieren
shutter-heading = Auslösezähler
shutter-hint = Die Kamera zählt jede Auslösung des Verschlusses und schreibt den aktuellen Stand in jedes Foto, das sie speichert (ARW, JPEG oder HEIF). Wählen Sie das zuletzt aufgenommene Foto aus, um den aktuellen Stand auszulesen. Die Datei wird nur gelesen, nicht verändert.
shutter-source = Wo der Zähler in der Datei steht, unterscheidet sich je nach Modell. Die Positionen stammen aus öffentlich dokumentierten Untersuchungen (ExifTool) und sind nicht für jedes Modell bestätigt.
shutter-drop-hint = Foto (ARW, JPEG, HEIF) hierher ziehen
btn-shutter-read = Foto auswerten
shutter-file-required = Bitte zuerst ein Foto auswählen.
shutter-file = Datei
shutter-taken = Aufnahmedatum
shutter-count = Auslösungen
shutter-type = Verschluss
shutter-type-mechanical = Mechanisch
shutter-type-electronic = Elektronisch
shutter-electronic-hint = Dieses Foto wurde mit dem elektronischen Verschluss aufgenommen. Bei vielen Modellen zählen solche Aufnahmen nicht mit; der Wert zeigt dann den Stand der letzten mechanischen Auslösung.
shutter-not-sony = Das Foto stammt nicht von einer Sony-Kamera. Ein Auslösezähler lässt sich daraus nicht lesen.
shutter-unknown-model = Für dieses Kameramodell ist nicht bekannt, wo der Auslösezähler in der Datei steht. Es wird deshalb kein Wert angezeigt.
shutter-not-recorded = Die Datei enthält keinen lesbaren Auslösezähler. Das kommt bei Fotos vor, die mit einem Bildbearbeitungsprogramm gespeichert oder exportiert wurden, und bei manchen Modellen bei Aufnahmen mit elektronischem Verschluss. Wählen Sie die Originaldatei von der Speicherkarte.
shutter-model-mismatch = Das Foto stammt von einem anderen Kameramodell als die angeschlossene Kamera.
shutter-invalid = Die Datei ist kein lesbares Foto
file-filter-photo = Fotos (ARW, JPEG, HEIF)
usage-log-heading = Nutzungsprotokoll
usage-log-hint = Die Kamera führt ein internes Protokoll in undokumentiertem Format. Es wird unverändert gespeichert; die Kamera wird dabei nur gelesen.
usage-log-written = Geschrieben nach
btn-usage-log-info = Wissenswertes zum Nutzungsprotokoll
btn-usage-log-info-hide = Erklärung ausblenden
usage-log-info-what-heading = Was ist das?
usage-log-info-what = Sony nennt das Protokoll intern „KikiLog“. Die Kamera schreibt es selbst; dieses Programm liest es nur. Eine öffentliche Beschreibung des Formats ist nicht bekannt.
usage-log-info-read = Lesen verändert nichts: Mehrere Downloads hintereinander waren byteidentisch.
usage-log-info-flush-heading = Warum neue Einträge fehlen
usage-log-info-flush = Neue Einträge erscheinen oft nicht sofort: Die Kamera speichert sie unter Umständen erst beim Ausschalten, teils verteilt über mehrere Ein-/Aus-Zyklen. Kamera aus- und wieder einschalten, erneut laden und das wiederholen, bis keine neuen Einträge mehr hinzukommen.
usage-log-info-content-heading = Was man ablesen kann
usage-log-info-shutter = Auslösungen: Manche Ereignisse stehen für jeweils eine Aufnahme. Ihre Anzahl ergibt jedoch nicht den Auslösezähler: Das Protokoll hat eine feste Größe und deckt nicht zwingend die gesamte Nutzungsdauer der Kamera ab. Den Gesamtstand führt die Kamera an anderer Stelle und schreibt ihn in die Fotos, die sie speichert; er lässt sich im Bereich „Auslösezähler“ auslesen.
usage-log-info-clock = Zeitangaben sind nur so gut wie die Kamerauhr. Nach einem Akkuwechsel kann die Uhr zurückfallen und Einträge mit dem initialen Datum der Kamera erzeugen.
usage-log-info-unknown = Eine öffentliche Beschreibung der Ereignisnummern ist nicht bekannt. Das Programm zeigt die Zahlen ohne Deutung an.
usage-log-info-privacy-heading = Datenschutz
usage-log-info-privacy = Die Datei enthält die Seriennummern eingelegter Speicherkarten. Nicht unbedacht weitergeben oder öffentlich posten.
btn-usage-log = Protokoll speichern
usagelog-size = Protokollbereich
usagelog-cursor = Schreibposition
usagelog-spare = Zweiter Bereich
usagelog-count = Einträge
usagelog-events = Ereignisse nach Nummer
usagelog-entries = Einträge der Reihe nach
msg-link-copied = Link in die Zwischenablage kopiert.

# Fehlermeldungen des Backends, adressiert über den stabilen Fehlercode.
# Der variable Teil (Pfad, Statusbyte, Gerätemeldung) wird vom Frontend
# angehängt; die Texte sind daher auch ohne ihn vollständige Sätze.
# Ein `err-operation` gibt es bewusst nicht: dieser Code kennzeichnet eine
# bereits übersetzte Meldung, die unverändert angezeigt werden muss.
err-usb-no-device = Keine Kamera gefunden. Bitte eine Kamera anschließen und einschalten.
err-usb-camera-not-ready = Die Kamera ist angeschlossen, aber nicht bereit: Sie ist ausgeschaltet oder wartet am Display auf die Auswahl des USB-Modus. Bitte einschalten bzw. an der Kamera „Bildübertragung (MSC)“ bzw. „Massenspeicher“ wählen.
camera-choose-usb-mode = Die Kamera wartet auf die Auswahl des USB-Modus. Bitte an der Kamera „Bildübertragung (MSC)“ bzw. „Massenspeicher“ wählen.
err-usb-too-many-devices = Es ist mehr als eine Kamera angeschlossen. Bitte alle bis auf die gewünschte trennen.
err-usb-transfer = Die USB-Übertragung ist fehlgeschlagen. Bitte ein anderes Kabel oder einen anderen Anschluss verwenden und USB-Hubs vermeiden.
err-usb-stall = Die Kamera hat die USB-Übertragung abgebrochen.
err-usb-protocol = Die Kamera hat unerwartet geantwortet.
err-usb-msc-status = Die Kamera hat einen Speicherbefehl abgelehnt.
cli-error-prefix = Fehler
err-usb-driver = Windows hat die Kamera einem Treiber zugewiesen, der keinen direkten Zugriff erlaubt. Dieser Modus benötigt den WinUSB-Treiber, den die Anwendung selbst zuweisen kann – allerdings nur mit Administratorrechten.
err-needs-elevation = Für das Zuweisen oder Freigeben eines Treibers sind Administratorrechte nötig, und die Abfrage von Windows wurde abgelehnt. Mit „Treiberhilfe starten“ erneut versuchen; an der Kamera muss nichts geändert werden.
err-usb-no-mass-storage-volume = Die Kamera ist im Massenspeichermodus angeschlossen, stellt aber kein Laufwerk bereit. Bitte die Kamera aufwecken, eine Rückfrage zum USB-Modus am Display bestätigen und prüfen, ob eine Speicherkarte eingelegt ist. Den Treiber bitte nicht wechseln: Dieser Modus benötigt genau den Standardtreiber von Windows.
err-usb-msc-sense = Die Kamera hat einen Speicherfehler gemeldet.
err-usb-ptp-response = Die Kamera hat den Befehl abgelehnt.
err-usb-unsupported = Dieses Kameramodell unterstützt diese Funktion nicht.
err-usb-updater-status = Der Firmware-Updater der Kamera hat den Vorgang abgelehnt.
err-usb-senser = Ein Befehl im Servicemodus ist fehlgeschlagen.
err-market-core = Eine Kamera-Datendatei konnte nicht gelesen werden.
err-market-json = Die empfangenen Daten sind fehlerhaft.
err-market-missing-field = In der Antwort fehlt ein erwartetes Feld.
err-market-no-apk = Es ist keine App zur Installation ausgewählt.
err-market-http = Die Netzwerkanfrage ist fehlgeschlagen.
err-market-io = Der lokale Installationsserver ist fehlgeschlagen.
err-market-tls = Die verschlüsselte Verbindung zur Kamera kam nicht zustande.
err-apk = Die App-Datei konnte nicht gelesen werden.
err-io = Eine Datei konnte nicht gelesen oder geschrieben werden.
err-json = Die Kamera hat fehlerhafte Daten gesendet.
err-unknown-locale = Diese Sprache ist nicht verfügbar.
err-catalog-encoding = Die Übersetzungsdatei ist nicht als UTF-8-Text gespeichert. Bitte speichern Sie sie erneut mit UTF-8-Kodierung.
err-internal = Es ist ein interner Fehler aufgetreten. Das ist ein Programmfehler.

## Funktionsanalyse
nav-analysis = Analyse
analysis-title = Funktionsanalyse
analysis-experimental = Experimentell. Diese Funktion ist neu und erst an wenigen Kameras erprobt.
analysis-own-hardware = Nur an der eigenen Kamera verwenden. Schon der Servicemodus ist ein Eingriff, den Sony nicht vorsieht; die Verantwortung liegt bei Ihnen.
analysis-explanation-toggle = Erklärung: Zweck, Ablauf, Risiken
analysis-purpose-heading = Wofür?
analysis-purpose = Die Funktionsanalyse dokumentiert, in welchem Wert ihres Einstellungsspeichers (Backup) die Kamera eine ihrer eigenen Menüeinstellungen ablegt. Das Ergebnis beschreibt den Einstellungsspeicher dieser Kamera; es lässt sich mit anderen Kameras vergleichen, ändert aber nicht, was eine Kamera anbietet.
analysis-how-heading = Wie läuft sie ab?
analysis-how = Für jeden Menüwert, den Sie angeben, liest das Programm den kompletten Einstellungsspeicher einmal aus, jedes Mal in einer eigenen kurzen Servicemodus-Sitzung. Dazwischen ziehen Sie das Kabel ab und stellen die Einstellung im Kameramenü um; die Kamera bleibt dabei eingeschaltet. Zum Schluss wird der erste Wert noch einmal gelesen. Werte, die dem Menü folgen und am Ende wieder beim Anfangswert sind, sind Kandidaten. Werte, die sich von selbst ändern (Zähler, Uhrzeit), werden aussortiert.
analysis-requirements-heading = Voraussetzungen
analysis-requirements = Die Kamera ist per USB verbunden, ihr USB-Modus steht auf „Auto“, und nach jeder Sitzung wählen Sie an der Kamera „Bildübertragung (MSC)“. Unter Windows braucht das Programm Administratorrechte, weil es für den Servicemodus kurzzeitig einen Treiber zuweist. Akku laden; jede Sitzung dauert etwa eine Minute.
analysis-read-heading = Lesende Schritte
analysis-read = Die Abzüge lesen nur. An den Einstellungen der Kamera ändert sich dadurch nichts. Das Risiko ist gering, aber nicht null: Schon das Umschalten in den Servicemodus ist ein Eingriff, den Sony nicht vorsieht.
analysis-write-heading = Schreibender Schritt (Schreibtest)
analysis-write = Nur wenn Sie ihn ausdrücklich einschalten: Das Programm schreibt den gefundenen Wert in die Kamera, Sie prüfen nach einem Neustart im Menü, ob er wirkt, und danach wird der ursprüngliche Wert zurückgeschrieben. Geschrieben wird nur ein Wert, den die Kamera selbst gespeichert hat, und nur, wenn genau ein passender Kandidat gefunden wurde. Der alte Wert wird vorher gesichert. Trotzdem verändert dieser Schritt die Kamera; ein Fehler kann sie in einen Zustand bringen, den nur der Service beheben kann.
analysis-legal-heading = Rechtliches
analysis-legal = Die Software wird ohne jede Gewähr bereitgestellt (MIT-Lizenz). Eingriffe in die Kamera können die Garantie berühren. Verwenden Sie die Funktion nur an Kameras, die Ihnen gehören. Der Analysebericht enthält keine Seriennummer und nichts aus dem Einstellungsspeicher außer den gefundenen Kandidaten; er wird nie automatisch versendet.
analysis-start-heading = Neue Analyse
analysis-setting-label = Menüeinstellung, so wie die Kamera sie nennt
analysis-values-label = Menüwerte, durch Komma getrennt
analysis-values-hint = Der erste Wert ist der, der jetzt eingestellt ist. Zwei bis acht Werte; jeder kostet eine Servicemodus-Sitzung.
analysis-write-option = Danach Schreibtest durchführen (verändert die Kamera)
analysis-write-warning = Der Schreibtest verändert die Kamera: Er schreibt einen Wert in ihren Einstellungsspeicher und danach den ursprünglichen Wert zurück. Nur an der eigenen Kamera, auf eigenes Risiko.
analysis-write-consent = Die Kamera gehört mir, ich trage das Risiko.
btn-analysis-write-enable = Schreibtest aktivieren
btn-analysis-start = Analyse starten
btn-analysis-snapshot = Backup lesen
btn-analysis-new = Neue Analyse
analysis-discard-confirm = Die laufende Analyse verwerfen? Der bisherige Bericht bleibt gespeichert.
analysis-access-read = LESEND
analysis-access-write = SCHREIBEND
analysis-access-user = MANUELL
analysis-access-local = PC
analysis-step-snapshot = Backup lesen, die Einstellung steht auf „{ $value }“
analysis-step-change = Im Kameramenü auf „{ $value }“ umstellen
analysis-step-evaluate = Abzüge vergleichen
analysis-step-write = Wert für „{ $value }“ in die Kamera schreiben
analysis-step-check = Kamera aus- und einschalten, Menü ablesen (erwartet: „{ $value }“)
analysis-step-restore = Ursprünglichen Wert für „{ $value }“ zurückschreiben
analysis-step-report = Analysebericht speichern
analysis-do-prepare = VORBEREITUNG: 1. Im Kameramenü „{ $setting }“ auf „{ $value }“ stellen und das Menü mit der MENU-Taste verlassen, damit die Kamera die Einstellung speichert. 2. Kamera eingeschaltet lassen. 3. Kabel stecken und an der Kamera „Bildübertragung (MSC)“ wählen.
analysis-do-change = IHR SCHRITT: 1. Kabel abziehen. 2. Die Kamera bleibt eingeschaltet. 3. Im Kameramenü „{ $setting }“ auf „{ $value }“ stellen und das Menü mit der MENU-Taste verlassen, damit die Kamera die Einstellung speichert. 4. Kabel wieder stecken und „Bildübertragung (MSC)“ wählen.
analysis-do-check = IHR SCHRITT: 1. Kabel abziehen. 2. Kamera aus- und wieder einschalten. 3. Im Menü nachsehen, was bei „{ $setting }“ steht.
analysis-do-reconnect = Kabel wieder stecken und an der Kamera „Bildübertragung (MSC)“ wählen.
analysis-reading = Abzug { $index } von { $total }: Während des Lesens das Kabel NICHT abziehen.
analysis-cli-continue = Weiter mit Enter, abbrechen mit q:
analysis-confirm-word = ja
analysis-cli-confirm = Zum Bestätigen „{ $word }“ eingeben:
analysis-write-declined = Nicht bestätigt. Es wurde nichts geschrieben.
analysis-aborted = Abgebrochen. Es wurde nichts geschrieben.
analysis-snapshot-done = Abzug gelesen. Die Kamera hat den Servicemodus verlassen.
analysis-result = { $compared } Werte verglichen: { $count } Kandidaten, { $running } Werte haben sich von selbst geändert.
analysis-candidate-consistent = passt eindeutig zum Menü
analysis-candidate-inconsistent = passt nicht eindeutig
analysis-write-preview-heading = Das wird geschrieben
analysis-write-preview = Wert { $id } ({ $size } Byte): von { $old } („{ $old-label }“) auf { $new } („{ $new-label }“). Den neuen Wert hat die Kamera selbst gespeichert, als „{ $new-label }“ eingestellt war.
btn-analysis-write = Schreiben
btn-analysis-skip-write = Ohne Schreibtest abschließen
analysis-written = Wert geschrieben, zurückgelesen und dauerhaft gespeichert.
analysis-menu-question = Was zeigt das Menü jetzt?
analysis-menu-other = etwas anderes
btn-analysis-menu-confirm = Übernehmen
btn-analysis-restore = Ursprünglichen Wert zurückschreiben
analysis-restored = Ursprünglicher Wert zurückgeschrieben und dauerhaft gespeichert.
analysis-survived = Der geschriebene Wert war nach dem Neustart noch gespeichert.
analysis-not-survived = Der geschriebene Wert war nach dem Neustart nicht mehr gespeichert.
analysis-report-saved = Analysebericht gespeichert: { $path }
analysis-report-hint = Der Bericht enthält keine Seriennummer und kein Backup. Sie können ihn weitergeben, damit der Befund geprüft und dokumentiert werden kann.
btn-analysis-export = Bericht exportieren …
file-filter-analysis = Analysebericht
analysis-pending = Ein Schreibtest wurde nicht abgeschlossen. Der ursprüngliche Wert ist gesichert und kann zurückgeschrieben werden.
analysis-pending-none = Für diese Kamera ist kein offener Schreibtest gespeichert.
analysis-invalid-setting = Bitte die Menüeinstellung so angeben, wie die Kamera sie nennt.
analysis-invalid-values = Bitte zwei bis { $max } verschiedene Menüwerte angeben. Der erste ist der, der jetzt eingestellt ist.
analysis-already-complete = Alle Abzüge sind bereits gelesen.
analysis-other-camera = Das ist nicht dieselbe Kamera wie beim ersten Abzug. Die Analyse wurde angehalten.
analysis-write-not-allowed = Der Schreibtest ist in diesem Zustand nicht möglich.
analysis-camera-changed = Die Kamera hält inzwischen einen anderen Wert als beim ersten Abzug. Es wurde nichts geschrieben.
analysis-write-refused = Die Kamera hat den Wert abgelehnt; alles ist wie vorher. ({ $detail })
analysis-refuse-none = Schreibtest nicht möglich: Kein Wert ist dem Menü gefolgt.
analysis-refuse-ambiguous = Schreibtest nicht möglich: { $count } Werte sind dem Menü gefolgt, keiner lässt sich sicher zuordnen.
analysis-refuse-inconsistent = Schreibtest nicht möglich: Der Wert passt nicht eindeutig zu den Menüwerten.
analysis-refuse-size = Schreibtest nicht möglich: Der Wert ist { $size } Byte groß; geschrieben werden nur kleine Werte.
analysis-refuse-read-only = Schreibtest nicht möglich: Die Kamera kennzeichnet den Wert als schreibgeschützt.
analysis-refuse-protected = Schreibtest nicht möglich: Die Kamera kennzeichnet den Wert als geschützt.
settings-analysis-heading = Funktionsanalyse
settings-analysis-purpose = Die Funktionsanalyse dokumentiert, in welchem Wert ihres Einstellungsspeichers die Kamera eine ihrer eigenen Menüeinstellungen ablegt. Sie richtet sich an alle, die den Einstellungsspeicher ihres Modells verstehen und die Ergebnisse teilen möchten.
settings-analysis-why-off = Sie ist ausgeschaltet, weil sie für die normale Nutzung nicht gebraucht wird, die Kamera mehrfach in den Servicemodus schaltet und auf Wunsch auch in die Kamera schreibt. Sie ist experimentell und erst an wenigen Modellen erprobt.
settings-analysis-toggle = Funktionsanalyse einschalten (Eintrag „Analyse“ im Menü)
settings-analysis-confirm = Funktionsanalyse einschalten? Sie schaltet die Kamera mehrfach in den Servicemodus und kann auf Wunsch in die Kamera schreiben. Nur an der eigenen Kamera verwenden.
btn-analysis-enable = Einschalten
settings-analysis-on = Die Funktionsanalyse ist eingeschaltet.
settings-analysis-off = Die Funktionsanalyse ist ausgeschaltet.
analysis-needs-admin = Jeder Abzug braucht für den Servicemodus Administratorrechte. Windows fragt einmal pro Programmsitzung danach; Sie können die Treiberhilfe jetzt starten oder warten, bis der erste Abzug sie anfordert.
analysis-changed-hint = Bitte prüfen: Es wurde kein Kandidat gefunden. Die folgenden Werte haben sich geändert, sind aber nicht zum Anfangswert zurückgekehrt. Wechselt einer davon nur ein einziges Mal zwischen zwei Werten, stand die Kamera beim ersten Abzug vermutlich nicht auf dem ersten Menüwert. Dann die Analyse mit richtig eingestelltem Anfangswert wiederholen.
analysis-changed-info = Info: Werte, die sich während der Analyse von selbst geändert haben (z. B. Zähler, Uhrzeit), werden ignoriert. Sie müssen nichts tun.
elevation-admin = Administrator
elevation-standard = Standardrechte
elevation-admin-hint = Das Programm läuft mit Administratorrechten und kann den Treiber zuweisen, den der Servicemodus braucht.
elevation-standard-hint = Das Programm läuft mit normalen Rechten. Für Servicemodus, Tweaks und Funktionsanalyse startet es bei Bedarf eine Treiberhilfe mit Administratorrechten; Windows fragt dafür einmal pro Sitzung. Zum Starten klicken.
driver-helper-declined = Die Abfrage nach Administratorrechten wurde abgelehnt. Ohne sie kann der Treiber für den Servicemodus nicht zugewiesen werden; es wurde nichts verändert.
elevation-helper = Treiberhilfe aktiv
elevation-helper-hint = Eine Treiberhilfe mit Administratorrechten weist für den Servicemodus den Treiber zu. Sie läuft ohne Fenster und endet mit dem Programm; es kommt keine weitere Abfrage.
elevation-helper-confirm = Treiberhilfe jetzt starten? Windows fragt einmal nach Administratorrechten, danach bis zum Programmende nicht mehr.
analysis-cross-heading = Gegenprüfung an einer weiteren Kamera
analysis-cross-intro = Optional: Prüft, ob eine andere Kamera den gefundenen Wert ebenfalls hat und was dort eingestellt ist. Es wird nur gelesen. Gleiche Nummer bedeutet nicht sicher gleiche Funktion; erst ein Blick ins Menü der anderen Kamera bestätigt das. Das Ergebnis kommt in denselben Analysebericht.
analysis-cross-do = 1. Die analysierte Kamera abziehen. 2. Die andere Kamera einschalten, ihren USB-Modus auf „Auto“ stellen, Kabel stecken und „Bildübertragung (MSC)“ wählen.
btn-analysis-cross = Andere Kamera prüfen
analysis-cross-present = vorhanden, Wert { $value }
analysis-cross-match = entspricht „{ $label }“
analysis-cross-nomatch = entspricht keinem gemessenen Menüwert
analysis-cross-absent = nicht vorhanden
analysis-cross-same-camera = Das ist die analysierte Kamera selbst. Bitte die andere Kamera anschließen.
analysis-cross-not-now = Die Gegenprüfung ist erst möglich, wenn alle Abzüge gelesen sind und kein geschriebener Wert mehr zurückgeschrieben werden muss.
analysis-cross-none = Es gibt keinen Kandidaten, der sich gegenprüfen ließe.
analysis-wrong-camera = Angeschlossen ist nicht die analysierte Kamera. Bitte die { $model } anschließen; es wurde nichts geschrieben.
keys-fetching = Quelle wird heruntergeladen…
keys-scanning = Es wird nach Schlüsselmaterial gesucht…
keys-complete = Alles Schlüsselmaterial ist vorhanden.
keys-incomplete = Es fehlt noch Schlüsselmaterial.
keys-present = vorhanden
keys-missing = fehlt
keys-class-senser = Servicemodus-Schlüssel (Service-Shell, Tweaks, Analyse)
keys-class-cic = XPD-Prüfsummenschlüssel (App-Installation)
keys-class-spk = SPK-Schlüsselmaterial (App-Installation, apk2spk)
keys-class-fdat = FDAT-Schlüsselmaterial (Updater-Shell-Firmware)
plugin-heading = Plugins
plugin-intro = Plugins erweitern dieses Programm um Sprachen, Schalter für Kameraeinstellungen, Kompatibilitätsangaben, Werkzeuge und Diagnosen. Sie stammen nur aus Dateien, die Sie selbst auswählen – einen Plugin-Store gibt es nicht. Ihr Code läuft in einer Sandbox ohne Zugriff auf Ihre Dateien, das Netzwerk oder das Schlüsselmaterial. Installieren Sie nur Plugins, deren Autor Sie vertrauen; Änderungen wirken ab dem nächsten Start.
plugin-none = Keine Plugins installiert.
plugin-inactive = (aktiv nach einem Neustart)
plugin-author = Autor
plugin-restart = Starten Sie das Programm neu, damit die Änderung wirksam wird.
btn-plugin-install = Plugin installieren…
plugin-install-confirm = Dieses Plugin darf:
btn-confirm-plugin-install = Plugin installieren
plugin-installed = Das Plugin wurde installiert.
plugin-remove-confirm = Das Plugin „{ $name }“ entfernen?
plugin-removed = Das Plugin { $id } wurde entfernt.
plugin-perm-languages = dem Programm Sprachen hinzufügen
plugin-perm-camera-settings = Kameraeinstellungen ändern (nur zwischen den zwei Werten, die es nennt, und erst nach Ihrer Bestätigung)
plugin-perm-compatibility = der Kompatibilitätsübersicht Einträge hinzufügen
plugin-perm-code = eigenen Code in der Sandbox ausführen (Werkzeuge oder Diagnosen)
plugin-perm-host-os = abfragen, welches Betriebssystem dies ist
plugin-perm-host-cameras = sehen, welche Sony-Kameras in welchem Modus verbunden sind
plugin-invalid = Dies ist kein verwendbares Plugin: { $reason }
plugin-no-folder = Der Ordner für Benutzerdaten konnte nicht ermittelt werden; Plugins lassen sich daher nicht installieren.
plugin-not-installed = Es ist kein Plugin mit der Kennung { $id } installiert.
plugin-failed = Das Plugin ist fehlgeschlagen: { $reason }
plugin-unknown-entry = Kein geladenes Plugin stellt { $id } bereit.
plugin-run-done = Das Plugin ist fertig.
plugin-run-warning = Das Plugin ist fertig, mit Hinweisen.
plugin-run-error = Das Plugin hat einen Fehler gemeldet.
plugin-tools-heading = Plugin-Werkzeuge
plugin-tools-hint = Diese stammen aus installierten Plugins und laufen in einer Sandbox. Ein Plugin erhält nur, was Sie ihm übergeben und was es angemeldet hat, und kann die Kamera nicht verändern.
plugin-diagnostics-heading = Plugin-Diagnosen
plugin-compat-heading = Aus einem Plugin
plugin-col-note = Anmerkung
file-filter-plugin = Plugin-Datei
keys-not-set-up = Das für diesen Vorgang nötige Schlüsselmaterial ist nicht eingerichtet.
err-missing-keys = Das für diesen Vorgang nötige Schlüsselmaterial ist nicht eingerichtet.
keys-heading = Schlüsselmaterial benötigt
keys-intro = Die Servicemodus-Schlüssel, der XPD-Prüfsummenschlüssel und das SPK-Schlüsselmaterial sind nicht Teil dieses Programms. Geben Sie die Adresse einer Quelle an, die Sie haben — ein Repository, ein Archiv oder eine Datei — und das Programm findet und importiert die erkannten Teile. Nichts wird ausgeführt, und das Material bleibt nur auf diesem Rechner.
keys-law-notice = Beachten Sie die in Ihrem Land geltenden Gesetze und arbeiten Sie nur mit Kameras und Daten, die Ihnen gehören.
keys-url-label = Quelladresse (eine pro Zeile)
keys-url-required = Bitte geben Sie mindestens eine Quelladresse an.
btn-keys-import = Importieren
btn-import-keys = Schlüsselmaterial einrichten
