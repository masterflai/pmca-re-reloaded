# Dansk oversættelse af PMCA-RE RELOADED.
# Oversæt kun højresiden (teksten efter =). Ret aldrig id'erne.

app-title = PMCA-RE RELOADED – Kameraværktøjskasse

# Navigation / faneblade
nav-camera-info = Kameraoplysninger
nav-install-app = Apps
nav-tweaks = Tilpasninger
nav-network = Netværk
nav-firmware = Firmware
nav-updater = Updater-skal
nav-service = Serviceshell
nav-tools = Filværktøjer
nav-settings = Indstillinger
nav-license = Licens

# Velkomstside: den side, vinduet åbner på.
nav-welcome = Velkommen
welcome-intro-heading = Hvad dette program kan
welcome-intro-body = Dette program hjælper med at bevare dit Sony-kameras funktioner og tilpasse det til dine behov. Efter at Sony havde lukket butikken for PlayMemories Camera Apps, var der ingen mulighed for at sikkerhedskopiere eller geninstallere købte eller allerede installerede apps. En nulstilling af kameraet sletter de installerede apps uigenkaldeligt. Dette program gør det muligt at sikkerhedskopiere eksisterende apps og installere dem igen, og det kan desuden stille frit tilgængelige fællesskabsapps til rådighed. Du kan også installere dine egne apps (*.apk-filer). Ved problemer læser programmet kameraoplysninger og interne logfiler til analyse. Indstillinger, som kameraet allerede har, men ikke tilbyder i sin menu, kan slås til og fra og fortrydes igen. Programmet taler direkte med kameraet gennem USB-kablet. Hvilke modeller der understøttes, fremgår af siden Kameraoplysninger.
welcome-start-heading = Det første skridt
welcome-start-body = Tænd først kameraet, og sæt „USB-forbindelse“ i dets menu til Masselager eller MTP, mens kablet endnu ikke sidder i. „Auto“ virker som regel også, fordi kameraet så selv vælger en af de to. Rækkefølgen betyder noget: når først kablet sidder i, kan man ikke længere nå kameraets menu. Først derefter tilsluttes kameraet med USB; åbn Kameraoplysninger, og tryk på „Hent kameraoplysninger“. Så snart modellen og firmwareversionen vises, er forbindelsen i orden, og alle programmets øvrige funktioner er tilgængelige.
welcome-start-driver = Vises der i stedet en driverbesked, så følg den først. På Windows lægger en systemdriver beslag på kameraet, og den skal erstattes, før den direkte USB-adgang, som denne værktøjskasse har brug for, er mulig. Programmet klarer det selv, men at tildele en driver er en ret, som Windows kun giver en administrator. I så fald tilbyder programmet dig at genstarte det som administrator, og Windows beder dig bekræfte handlingen. Tildelingen ophæves igen, når sessionen slutter. Under Linux og macOS er det ikke nødvendigt.
welcome-start-driver-linux = Vises der i stedet en driverbesked, så følg den først. På Linux skal intet erstattes: kameraet nås direkte. Det, der skal til én gang, er retten til at åbne enheden — en udev-regel for Sonys producentkendetegn — og, til masselagringstilstand, at frigøre kernedriveren usb-storage. Begge dele sættes op uden for dette program med root-rettigheder; scriptet scripts/linux-prepare.sh i kildeteksten gør begge dele. Dette vindue beder aldrig selv om sådanne rettigheder.
welcome-start-driver-macos = Vises der i stedet en driverbesked, så følg den først. På macOS skal intet installeres eller erstattes, og dette vindue beder aldrig om administratorrettigheder. Sæt kameraets USB-forbindelse til MTP: i masselagringstilstand holder systemets diskdriver kameraet, så det ikke kan nås.
welcome-tweaks-heading = Tilpasninger og sprog
welcome-tweaks-body = Tilpasninger er indstillinger, kameraet allerede har, men ikke tilbyder i sin menu: optagetidsgrænsen, 4K-optagegrænsen, valget mellem PAL og NTSC, USB-app-installationen — og menusprogene.
welcome-tweaks-languages-heading = Kameraets sprog
welcome-tweaks-languages-body = Sony-kameraer viser kun de menusprog, der er almindelige i deres salgsregion. Bor man i udlandet, finder man derfor ofte ikke sit eget sprog og må klare sig med et fremmed. Flere sprog ligger dog allerede i firmwaren. Dette program henter dem ind i menuen, så du kan betjene dit kamera på et velkendt sprog.
welcome-tweaks-ui-heading = Sproget i dette program
welcome-tweaks-ui-body = Adskilt fra alt det er vælgeren øverst. Den ændrer kun sproget i dette program og rører aldrig ved kameraet.
welcome-tweaks-safety = Hver tilpasning skrives ind i kameraets sikkerhedshukommelse, og hvert enkelt kan tages tilbage med den samme kontakt. En afbrudt skrivning kan beskadige kameraet permanent, så træk hverken kablet ud eller batteriet ud, mens en er i gang.
welcome-privacy-heading = Dine data
welcome-privacy-where = Siden »Licens« gennemgår det hele igen i fuld længde sammen med licenserne for alle de biblioteker, dette program er bygget af.
welcome-where-heading = Hvor finder jeg hvad
welcome-where-info = Model, serienummer og firmwareversion. Samtidig forbindelsestesten.
welcome-where-install = Apps fra fællesskabets liste, eller din egen .apk-fil.
welcome-where-tweaks = De indstillinger, kameraets menu ikke når, sprogene iblandt dem.
welcome-where-network = GPS-assistancedata samt adgangspunkter og tjenester til livestreaming.
welcome-where-firmware = Skrive et Sony-firmwarebillede til kameraet.
welcome-where-tools = Pakke firmware- og pakkefiler ud og se dem efter. Uden kamera.
welcome-where-shells = Direkte adgang til kameraets hukommelse og filsystem. Vises først, når de er slået til under Indstillinger.
welcome-where-settings = Tema, avanceret tilstand og redigering af dette programs oversættelse.
welcome-where-license = Hvor denne portering kommer fra, hvis arbejde den bygger på, og på hvilke vilkår den står.
welcome-where-hint = Ingen side rører kameraet af sig selv. Der bliver hverken læst eller skrevet noget, før du trykker på en knap, der siger det.

# Knapper
btn-get-info = Hent kameraoplysninger
btn-refresh = Opdater
btn-load-apps = Indlæs tilgængelige apps
btn-list-tweaks = Vis tilgængelige tilpasninger
btn-install = Installer valgt app
btn-install-file = Installer .apk-fil…
btn-apply = Anvend
btn-save = Gem på kameraet
btn-add = Tilføj
btn-remove = Fjern
btn-reveal = Vis
btn-update-gps = Overfør GPS-assistancedata til enheden
btn-cancel = Annuller
btn-browse = Gennemse…
btn-switch-msc = Skift til masselagringstilstand
btn-start-driver-helper = Start driverhjælper

# Etiketter til de filtypefiltre, som fildialogerne tilbyder
file-filter-apk = Android-program (*.apk)
file-filter-firmware = Sony-firmwareimage (*.dat)
file-filter-usage-log = Brugslog (*.bin)
file-filter-catalog = Oversættelsesfil (*.ftl)
file-filter-spk = Sony-apppakke (*.spk)
file-filter-any = Alle filer
file-filter-gps = GPS-assistancedata (*.dat)

# Status / meddelelser
msg-connecting = Opretter forbindelse…
msg-usage-log-written = Brugslog skrevet til { $path }
msg-apps-found = Fandt { $count } apps
msg-switch-mode = Skift kameraet til MTP- eller masselagringstilstand.
msg-no-tweaks = Der findes ingen tilpasninger til dette kamera.
msg-done = Færdig.

# Skift af kameraets USB-tilstand (MTP -> masselagring). Tilbydes under en
# "forkert tilstand"-fejl; der findes ingen USB-kommando den modsatte vej.
mode-switching = Beder kameraet om at skifte til masselagringstilstand…
mode-switch-wait = Venter på, at kameraet tilslutter sig igen…
camera-confirm-hint = Meddelelser på kameraet (f.eks. om www.sony.net/pm) kan lukkes; de forstyrrer ikke processen.
mode-switch-done = Kameraet er nu i masselagringstilstand.
mode-switch-timeout = Kameraet dukkede ikke op i masselagringstilstand. Sluk og tænd det igen, og prøv en gang til.
mode-switch-impossible = Denne tilstand kan ikke ændres via USB. Sluk og tænd kameraet igen.
mode-switch-hint = Kun et kamera i MTP-tilstand kan skiftes via USB. Ændringen er midlertidig: den USB-tilstand, der er valgt i kameraets egen menu, forbliver uændret, så når kameraet slukkes og tændes igen — eller kablet tages ud og sættes i igen — gendannes den forrige tilstand. Der findes ingen USB-kommando den modsatte vej.

# Felter med kameraoplysninger
info-model = Model
info-product-code = Produktkode
info-serial = Serienummer
info-firmware = Firmwareversion
info-lens = Objektiv
info-region = Region
info-mode = Tilstand
mode-mass-storage = Masselager
mode-mtp = MTP
mode-updater = Updater-tilstand
mode-app-install = App-installationstilstand
mode-service = Servicetilstand
mode-unknown = Ukendt
info-android-version = Android-platformsversion
info-photographer = Fotograf
info-copyright = Copyright
info-photographer-1 = Fotograf (post 1)
info-photographer-2 = Fotograf (post 2)
info-copyright-1 = Copyright (post 1)
info-copyright-2 = Copyright (post 2)
diagnose-services-heading = Kan kameraet ikke nås?
diagnose-services-hint = Nogle Windows-tjenester kan optage et kamera i MTP-tilstand, så dette program ikke kan nå det. Kontrollen spørger kun Windows, hvilke tjenester der kører; den ændrer intet.
btn-diagnose-services = Kontrollér Windows-tjenester
diagnose-services-none = Ingen kendt forstyrrende tjeneste kører.
diagnose-services-found = Disse kørende tjenester kan forstyrre adgangen til kameraet:
diagnose-services-advice = Stop tjenesten under Tjenester (services.msc) eller med følgende kommando som administrator, og tilslut derefter kameraet igen:

# Firmware-updater / shell
updater-model = Kameramodel
updater-firmware-current = Nuværende firmwareversion
updater-guard-ok = Firmwareimaget passer til dette kamera.
updater-no-fdat = Der følger ingen updater-firmware med til modellen { $model }.
updater-model-required = Kameraet er allerede i updater-tilstand og kan ikke oplyse sin model der. Sluk kameraet, og tænd det igen, så det starter i normal tilstand, og prøv så igen. Alternativt kan du angive en payload-fil (--fdat-file) eller, i konsollen, modellen (--model <MODEL>).
updater-already-updater = Kameraet er allerede i updater-tilstand.
updater-flash-required = Kameraet ændres ikke uden bekræftelse. Kør igen med --flash for at skifte til updater-tilstand og skrive payloaden.
updater-flashing = Skriver updater-payloaden til kameraet…
updater-switch-wait = Følg vejledningen på kameraets skærm. Venter på, at kameraet skifter til updater-tilstand…
updater-switch-timeout = Tidsfristen for at vente på kameraet udløb. Kør kommandoen igen, når kameraet har tilsluttet sig på ny.
updater-shell-ready = Updater-payloaden kører. Skriv 'help' for kommandoer og 'exit' for at afslutte.
updater-shell-done = Shell-sessionen er afsluttet.
updater-session-panic = Updater-sessionen blev afsluttet uventet. Sluk og tænd kameraet, før du prøver igen.

# Det, updater-skallen svarer. Fejlene ved at læse en linje er kun formuleret
# én gang, i service-*-nøglerne: de to skaller deler tolken, så en sti i
# anførselstegn og en forkert hexværdi fejler ens i dem begge.
updater-backup-synced = Backuppen er skrevet til flashhukommelsen.
updater-backup-locked = De beskyttede backupindstillinger er nu låst.
updater-backup-writable = Der kan nu skrives til de beskyttede backupindstillinger.
updater-shell-console-only = Root-skallen kræver en terminal og findes kun på kommandolinjen: kør pmca-console updatershell --flash, og skriv „shell“ der.

# Siden med updater-skallen.
updater-warning = Advarsel: for at nå denne skal genstarter kameraet i updater-tilstand, og der skrives en payload til det – et lille program fra dette projekt, som kameraet tager imod som en firmwareopdatering, og som derefter besvarer denne skals kommandoer. Payloaden forsvinder, så snart kameraet slukkes, men de kommandoer, den tilbyder, skriver i kameraets permanente indstillingshukommelse, og en afbrudt skrivning kan ødelægge kameraet for altid. Desuden nulstilles alle kameraets indstillinger i den forbindelse – modsat serviceskallen.
updater-console-heading = Kør updater-skallen på kommandolinjen
updater-driver-hint = Updater-skallen kræver, at kameraet bliver på Windows' egen masselagerdriver, USBSTOR, for det er den vej, payloaden sendes. Det er det stik modsatte af, hvad serviceskallen kræver. Programmet tildeler og frigiver selv WinUSB-driveren, så der er intet at forberede i hånden — men en driver, som et andet værktøj har tildelt, skal fortrydes først. Et kamera i updater-tilstand melder sig med sit eget produkt-id og må heller ikke bindes om.
updater-fdat-label = Payload-fil (valgfri)
updater-confirm = Kameraet genstarter i updater-tilstand, og der skrives en payload til det. Vil du fortsætte?
updater-cancelled = Annulleret. Kameraet blev ikke ændret.

btn-shell-primer = Værd at vide om denne shell
btn-shell-primer-hide = Skjul forklaringen
shell-primer-compare-heading = To shells, to protokoller
shell-primer-compare = Begge sider tilbyder en kommandolinje til kameraet, men det er ikke den samme shell. Service-shellen taler Sonys egen fabriksserviceprotokol, som kameraet selv besvarer, når du er godkendt; der skrives intet til kameraet for at komme dertil. Updater-shellen findes kun, fordi dette projekt skriver sit eget program ind i kameraet: kameraet genstarter i updater-tilstand, tager imod payloaden som var det en firmware, og den payload besvarer så kommandoerne nedenfor. Kun én af de to kan være åben ad gangen.
shell-primer-payload = „Payload“ (engelsk for nyttelast) betegner det egentlige indhold, som en transportvej bringer frem. For updater-shellen er det et lille program fra dette projekt: kameraet tager imod det på samme måde som en firmwareopdatering, men beholder det kun, indtil det slukkes. Service-shellen har ikke brug for nogen payload, fordi kameraet besvarer dens kommandoer med sin egen firmware. For firmwarefiler betyder begrebet det samme: selve firmwarebilledet i .dat-filens container.
shell-primer-when-service = Service-shellen bruges til alt, der læser eller ændrer kameraets gemte indstillinger, mens det fortsat kører sin egen firmware: læse HASP-blokken, gemme en tilbagerulningskopi af backupområdet, ændre enkelte poster, hente filer ud af kameraet. Det er også det, siden Tilpasninger bruger nedenunder; alt derfra kan gøres her i hånden.
shell-primer-when-updater = Updater-shellen er valget, når kameraet ikke kan nås på anden måde, eller når du har brug for en shell på det kørende system frem for enkelte kommandoer. Kun denne vej giver en rigtig root-shell, og den virker stadig på et kamera, der nægter servicetilstand. Den koster en genstart, en skrivning og kameraets indstillinger, så den er andetvalget, ikke førstevalget.
shell-primer-driver-heading = Drivere: de to kræver hver sit modsatte
shell-primer-driver = Service-shellen taler rå USB og kræver under Windows et kamera bundet til WinUSB. Programmet tildeler selv den driver på vej ind i servicetilstand og frigiver den igen, når sessionen slutter; intet separat værktøj er involveret. Updater-shellen kommer gennem USBSTOR, Windows' egen driver til masselagring, og virker ikke, så længe kameraet er bundet til WinUSB — derfor hører frigivelsen med til at afslutte en servicesession. Tildeling af en driver kræver administratorrettigheder. På Linux er der ikke brug for noget driverskift, kun adgang til enhedsknuden.
shell-primer-risk-heading = Hvad der kan gå galt
shell-primer-risk = De markerede kommandoer nedenfor skriver i kameraets permanente indstillingshukommelse. En afbrudt skrivning kan efterlade kameraet ude af stand til at starte, og intet af det kan fortrydes — netop derfor findes bk d: gem en kopi af backupområdet, før du ændrer noget. Selve updater-payloaden bliver ikke på kameraet; den forsvinder, når kameraet slukkes. Vejen gennem updater-tilstand nulstiller dog alle kameraets indstillinger.
shell-primer-commands-heading = Kommandoer i denne shell
shell-primer-writes = Kommandoer mærket (WRITES) ændrer kameraet. Alt andet læser kun.
shell-cmd-hasp = Læser HASP-blokken, kameraets beskyttede identitetsområde.
shell-cmd-pull = Henter en fil fra kameraet. Uden et lokalt navn beholdes navnet på kameraet.
shell-cmd-push = Kopierer en lokal fil til kameraet og erstatter det, der ligger der. (WRITES)
shell-cmd-rm = Sletter en fil på kameraet. (WRITES)
shell-cmd-rmem = Skriver et område af enhedens hukommelse til en lokal fil. Forskydning og længde er hexadecimale.
shell-cmd-bk-r = Læser én post i backupområdet og viser den som hexdump.
shell-cmd-bk-d = Gemmer hele backupområdet i en fil. Gør det, før du ændrer noget: det er den eneste vej tilbage.
shell-cmd-bk-dump-txt = Gemmer hele backupområdet som <file>.bin og en læsbar oversigt som <file>.txt.
shell-cmd-bk-stat = Viser kameraets backupstatusblok, inklusive regionen.
shell-cmd-bk-w = Skriver én post i backupområdet, angivet som hexadecimale bytes adskilt af mellemrum. (WRITES)
shell-cmd-bk-s-sub = Fastholder ét undersystems backupområde i flash, så ændringen overlever en genstart. (WRITES)
shell-cmd-bk-s = Fastholder backupområdet i flash, så ændringen overlever en genstart. (WRITES)
shell-cmd-bk-lock = Slår skrivebeskyttelsen af de beskyttede backupposter til eller fra; nogle indstillinger nægter at ændre sig, mens den er slået til. (WRITES)
shell-cmd-term = Slår kameraets fejlfindingsterminal til eller fra. (WRITES)
shell-cmd-info = Læser backupstatusblokken, der fortæller, om området overhovedet kan skrives til.
shell-cmd-shell = Åbner en root-shell på kameraet. Den kræver en terminal og findes derfor kun i konsollen; shell tcp <port> tilbyder den i stedet på 127.0.0.1 til en anden klient. (WRITES)
shell-cmd-help = Viser de kommandoer, denne shell tager imod.
shell-cmd-exit = Afslutter sessionen og forlader shellen.
shell-primer-diagnostics-heading = Diagnose og reparation
shell-cmd-bk-trace = Optager pakke for pakke i en fil, hvordan kameraet sender hele backupområdet over USB. Til undersøgelse af store backups; kan efterlade kameraet hængende midt i svaret. Tag så kablet ud, og sluk og tænd kameraet, om nødvendigt ved at tage batteriet ud.
shell-cmd-bk-rewrite = Læser backupområdet, skriver det uændret tilbage og læser det igen. Viser, om kameraet accepterer et helt image, uden at ændre en indstilling. (WRITES)
shell-cmd-bk-restore-attrs = Sætter attributterne for alle backupposter, f.eks. skrivebeskyttelsen, tilbage til dem i en kopi gemt med bk d eller bk dump-txt. Værdierne forbliver uændrede. Brug kun en kopi fra det samme kamera. (WRITES)
shell-primer-diagnostics-env = Tre miljøvariabler virker på både konsollen og den grafiske brugerflade, hvis de er sat før start, f.eks. i PowerShell med $env:PMCA_TWEAKS_DIRECT = "1". PMCA_TWEAKS_DIRECT får tweaks til at skrive enkelte værdier, også når backuppen er mindre end 1 MB. PMCA_TWEAKS_IMAGE får dem til at skrive hele imaget, også når det er større end 1 MB; er begge sat, gælder PMCA_TWEAKS_DIRECT. PMCA_TLS_DUMP=<fil> gemmer kameraets første TLS-besked (ClientHello) under en app-installation, så en mislykket forbindelse kan undersøges.

# Installation af apps
install-switching = Skifter til app-installationstilstand…
install-disabled = Kameraet nægtede at gå i app-installationstilstand. Det skyldes som regel indstillingen "Aktivér USB-app-installation": den skal slås til under "Tilpasninger".
install-stays-app-mode = Kameraet bliver i app-installationstilstand, hvor flere apps kan installeres. Til alle andre funktioner skal du tage USB-kablet ud og slukke og tænde kameraet igen.
err-usb-app-install-mode = Kameraet er stadig i app-installationstilstand og accepterer i den kun app-installationer. Tag USB-kablet ud, og sluk og tænd så kameraet igen — genstartes det, mens det stadig er tilsluttet, går det direkte tilbage i denne tilstand.
install-switch-wait = Venter på, at kameraet tilslutter sig igen i app-installationstilstand…
install-switch-timeout = Tidsfristen for at vente på app-installationstilstanden udløb. Kør kommandoen igen, når kameraet har tilsluttet sig på ny.
install-bad-mode = Kameraet kan ikke bruges i denne tilstand. Skift det til MTP- eller masselagringstilstand.
install-starting = Starter installationen…
install-error = Kommunikationsfejl under installationen
install-server-failed = Den lokale installationsserver accepterede ikke kameraets forbindelse:
install-done = Opgaven blev fuldført.

# Gennemgang af APK-filen
apk-package = Pakke:
apk-unreadable = Advarsel: .apk-filen kunne ikke analyseres.
apk-incompatible = Advarsel: denne app kræver et SDK-niveau, som kameraet måske ikke tilbyder.
apk-unsigned = Advarsel: .apk-filen har ingen signatur.

# Community-appbutik
store-loading = Indlæser applisten…
store-resolving = Finder den nyeste udgivelse…
store-downloading = Downloader
store-unknown-app = Ukendt app
store-no-release = Ingen udgivelse kan downloades til denne app
store-not-installable = ingen udgivelse
msg-no-apps = Ingen apps fundet.
store-heading = Community-appbutik
store-discontinued = Sony har lukket den officielle PlayMemories-appbutik; serverne svarer ikke længere, så apps kan ikke længere købes eller downloades derfra.
store-use-own-apk = Du kan stadig installere din egen .apk: vælg filen nedenfor, eller træk den ind i dette vindue. Community-appbutikken længere nede virker også fortsat.
btn-store-sync = Tjek kameraet
btn-install-selected = Installer valgte
btn-store-stop = Stop efter denne app
btn-store-stopping = Stopper efter denne app…
btn-store-retry = Prøv mislykkede igen
btn-continue = Fortsæt
btn-reinstall = Installér igen
btn-skip = Spring over
btn-confirm-updater = Genstart og skriv
btn-confirm-tweaks = Skriv tilpasninger
btn-confirm-languages = Aktivér sprogene
btn-confirm-wifi = Erstat adgangspunkter
btn-confirm-stream = Overskriv indstillinger
btn-confirm-show-shells = Vis skallerne
btn-confirm-discard = Kassér oversættelsen
store-select-all = Vælg alle apps
store-col-app = App
store-col-version = Version
store-col-author = Forfatter
store-col-camera = Version på kameraet
store-col-status = Status
store-camera-absent = ikke installeret
store-version-unknown = ukendt
store-batch-query = Læser de installerede apps fra kameraet…
store-sync-done = Kameraet er tjekket. Installerede apps fra listen: { $count }
store-batch-downloading = Henter app { $current } af { $total }…
store-batch-installing = App { $current } af { $total }: installerer { $name }…
store-reinstall-confirm = { $name } er allerede installeret på kameraet. Installeret version: { $installed }. Version i appbutikken: { $store }. Vil du installere appen igen?
store-state-queued = venter
store-state-installed = installeret
store-state-failed = mislykkedes
store-state-skipped = sprunget over
store-state-not-run = ikke udført
store-batch-summary = Installeret: { $done } · mislykkedes: { $failed } · sprunget over: { $skipped } · ikke udført: { $notrun }

# Servicetilstand (senser)
usb-mode-advice = Til tweaks, backups og serviceshellen skal kameraets USB-forbindelsestilstand stå på „Auto“, ikke fast på masselager. Når kameraet spørger om tilstanden efter en session, så vælg „Masselager (MSC)“. Med fast masselager afviser kameraet ofte det næste skift til servicetilstand.
service-switch-refused = Kameraet skiftede ikke til servicetilstand og meldte sig ikke på USB igen; kameraet viser så som regel „Kontroller den tilsluttede enhed“. Tag USB-kablet ud, sæt det i igen, og prøv igen.
service-switching = Skifter til servicetilstand…
service-switch-wait = Venter på, at kameraet tilslutter sig igen i servicetilstand…
service-switch-timeout = Kameraet dukkede ikke op i servicetilstand. Det melder sig som en selvstændig USB-enhed (produkt-id 0336), der skal have tildelt sin egen WinUSB-driver; se efter en Sony-enhed uden driver i Enhedshåndtering. Sluk og tænd kameraet for at bringe det tilbage i normal tilstand.
service-authenticating = Godkender…
# Vises, når kameraet stadig er i servicetilstand fra en tidligere kørsel, og
# når det ikke kunne fortrydes.
service-stranded-reset = Kameraet er stadig i servicetilstand fra en tidligere kørsel. Den session afsluttes først…
service-stranded-failed = Kameraet sidder fast i servicetilstand fra en tidligere kørsel og svarede ikke på anmodningen om at forlade den. Sluk kameraet, tænd det igen, og prøv forfra.
service-bad-mode = Kameraet kan ikke bruges i denne tilstand. Skift det til masselagringstilstand.
service-driver-required = Direkte USB-adgang blev nægtet, og WinUSB-driveren kunne ikke tildeles. Tildeling af en driver kræver administratorrettigheder.
service-warning = Advarsel: servicetilstand giver direkte adgang til kameraets hukommelse, filsystem og kalibreringsdata. Skrivekommandoer kan beskadige kameraet permanent.
service-shell-ready = Serviceshellen er klar. Skriv 'help' for kommandoer og 'exit' for at afslutte.
service-shell-done = Serviceshell-sessionen er afsluttet.
# Rapporteres ved opstart, når en tidligere kørsel blev afbrudt, før den kunne
# frigive den driver, den havde tildelt.
driver-swept = Fjernede { $count } drivertildeling(er), som en tidligere kørsel efterlod.
# Rapporteres ved opstart, når den samme afbrudte kørsel også efterlod kameraet
# i servicetilstand, hvor det ville afvise den næste session.
driver-swept-camera = Et kamera, som en tidligere kørsel efterlod i servicetilstand, er sat tilbage til normal tilstand.
driver-release-replug = Windows venter på kameraet, mens driveren frigives. Tag USB-kablet ud og sæt det i igen, så oprydningen kan blive færdig.
# Resultater af kommandoen `driver`.
driver-bound = { $id } bruger nu WinUSB (driverpakken { $package }).
driver-released = Frigav { $count } drivertildeling(er); kameraet bruger igen de drivere, der følger med Windows.
driver-needs-elevation = Tildeling af en driver kræver administratorrettigheder. Kør denne kommando i en konsol, der er startet som administrator.
driver-leftover-needs-elevation = En tidligere kørsel har tildelt kameraet en driver, som endnu ikke er frigivet. Så længe den findes, beder kameraet om at kontrollere den tilsluttede enhed, og dets masselager virker ikke. Frigivelsen kræver administratorrettigheder; bekræft Windows-forespørgslen.
driver-binding = WinUSB-driveren tildeles kameraet…
driver-bind-wait = Venter på, at kameraet tilsluttes igen med den nye driver…
driver-bind-timeout = Kameraet dukkede ikke op igen efter tildelingen af driveren. Tag det ud, tilslut det igen, og prøv forfra.
driver-bind-failed = WinUSB-driveren kunne ikke tildeles: { $detail }
# Skallens svar. De kommer fra backenden, så konsollen og vinduet bruger de
# samme ord.
service-usage = Brug: { $usage }
service-bad-hex = Ikke en hexadecimal værdi: { $value }
service-unknown-command = Ukendt kommando: { $command }. Skriv 'help' for listen.
service-unterminated-quote = Linjen slutter inde i et argument i anførselstegn.
service-help = Kommandooversigt:
service-read-bytes = { $count } byte læst.
service-pulled = { $remote } kopieret fra kameraet til { $local }.
service-pushed = { $local } kopieret til { $remote } på kameraet.
service-removed = { $remote } slettet fra kameraet.
service-dumped = { $size } byte fra { $offset } gemt i { $local }.
service-backup-saved = Sikkerhedskopien ({ $count } byte) er gemt i { $local }.
service-backup-text-saved = Sikkerhedskopien ({ $count } byte) er gemt i { $bin } og oversigten i { $txt }.
service-backup-status = Backupstatus, region: { $region }
service-backup-rewritten = Aftrykket ({ $count } byte) blev skrevet tilbage uændret og læst igen; det er ikke gemt permanent.
service-backup-attrs-restored = Attributterne for { $count } poster er sat tilbage efter { $local }.
service-backup-written = Sikkerhedskopipost { $id } skrevet.
service-backup-persisted = Sikkerhedskopien af undersystem { $id } er skrevet til flash.
service-terminal-on = Fejlsøgningsterminalen er nu slået til.
service-terminal-off = Fejlsøgningsterminalen er nu slået fra.
btn-service-terminal-open = Åbn seriel konsol
btn-service-terminal-close = Luk seriel konsol
service-terminal-heading = Seriel konsol
service-terminal-hint = På Linux-kameraer er fejlfindingsterminalen en root-shell over USB. Åbn den, og skriv kommandoer som i en terminal.
service-terminal-label = Konsol
service-terminal-interactive = Seriel konsol åben. Skriv kommandoer; skriv 'exit' for at vende tilbage til service-shellen.

# Servicesiden uden avanceret tilstand: hvordan skallen nås fra konsollen, og
# hvad der skal være på plads først.
service-console-heading = Kør serviceskallen i konsollen
service-console-hint = Åbn en terminal i mappen med pmca-console, og kør denne kommando:
service-driver-heading = Før du begynder
service-driver-hint = Servicetilstand kræver direkte USB-adgang, som Windows ikke giver gennem masselagringsdriveren. Programmet tildeler selv WinUSB-driveren — to gange, én gang mens kameraet er i masselagringstilstand, og én gang mere når det dukker op igen i servicetilstand under et andet produkt-id — og frigiver begge tildelinger, når sessionen slutter. Tildeling af en driver kræver administratorrettigheder, så start konsollen som administrator; der er ikke brug for andre værktøjer.
driver-note-linux = Her skal der ikke installeres noget: libusb når kameraet direkte uden at fortrænge en kernedriver. Der kræves kun ret til at åbne enheden og et skrivebord, der ikke allerede holder på den. Tilføj en udev-regel for Sonys producent-id 054c, og frigør driveren usb-storage, før du bruger masselagertilstand. Scriptet scripts/linux-prepare.sh gør begge dele og standser desuden gvfs-diskovervågningen.
driver-note-macos = Til direkte USB-adgang skal der ikke installeres noget. Sæt kameraets USB-forbindelse til MTP: i masselagringstilstand holder systemets diskdriver kameraet, og programmet kan ikke nå det. I MTP-tilstand afslutter programmet macOS-kameratjenesten ptpcamerad, hvis den optager kameraet; systemet genstarter den efter behov. Denne platform er den mindst afprøvede af de tre; regn med at melde et problem frem for at finde det allerede beskrevet.
service-advanced-hint = Skallen kan også vises her under Indstillinger, Avanceret tilstand.
btn-copy-command = Kopiér kommando
msg-command-copied = Kommandoen er kopieret til udklipsholderen.

# Servicesiden med avanceret tilstand slået til.
btn-service-open = Forbind
btn-service-close = Afbryd
btn-service-run = Kør
btn-service-browse = Indsæt filsti…
shell-path-quote = Denne sti kan ikke indsættes: den indeholder et anførselstegn, som kommandolinjen bruger til at markere begyndelsen og slutningen af et argument. Omdøb filen, eller flyt den til en sti uden dette tegn.
service-command-label = Kommando
service-not-connected = Ikke forbundet.

# Tekster, der kun bruges i den grafiske brugerflade
install-apk-path = Sti til en lokal .apk-fil (valgfrit)
install-drop-hint = Slip en .apk-fil her

# Beskrivelser af tilpasningerne
tweak-rec-limit = Videooptagelse uden tidsgrænse
tweak-rec-limit-4k = 4K-videooptagelse uden tidsgrænse
tweak-language = Vis alle menusprog
tweak-pal-ntsc = Aktivér PAL/NTSC-vælgeren og advarslen
tweak-pal-ntsc-mode = Videosystem (PAL / NTSC)
tweak-pal-ntsc-pal = PAL
tweak-pal-ntsc-ntsc = NTSC
tweak-pal-ntsc-note = Efter et skift af videosystem kræver kameraet, at hukommelseskortet formateres. Tag først en sikkerhedskopi af alt — billeder, videoer, LUT-filer. Nemmest er det at tage kortet ud inden skiftet.
tweaks-set-malformed = Forventede --set ID=VÆRDI, fik: { $arg }
tweaks-set-unknown-value = Ukendt værdi "{ $value }". Forventede en af: { $options }
tweaks-set-not-a-choice = { $id } er en kontakt, ikke et valg. Brug --enable eller --disable.
tweaks-needs-set = { $id } er et valg, ikke en kontakt. Brug --set { $id }=VÆRDI med en af: { $options }
tweak-usb-installer = Aktivér USB-app-installation

# Forløbet ved tilpasninger
tweaks-warning = Advarsel: når tilpasninger anvendes, overskrives kameraets backup-hukommelse. En afbrudt skrivning kan beskadige kameraet permanent.
tweaks-reading = Læser kameraets indstillinger…
tweaks-writing = Skriver kameraets indstillinger…
tweaks-done = Kameraets indstillinger er opdateret.
tweaks-write-failed = Kameraet gemte ikke de nye indstillinger, som de blev sendt. En del kan alligevel være skrevet: læs indstillingerne igen, og fortryd dem om nødvendigt. En backup taget før ændringen er den sikre vej tilbage.
tweaks-confirm-required = Der skrives ikke uden bekræftelse. Kør igen med --confirm, når du har læst advarslen ovenfor.
tweaks-state-enabled = aktiveret
tweaks-state-disabled = deaktiveret
tweaks-full-read-failed = Kameraet leverede ikke sin komplette backup ({ $detail }). Bagefter reagerer det ofte ikke længere: sluk kameraet, tag batteriet ud i nogle sekunder, sæt det i igen og tilslut på ny.
tweaks-direct-mode = Dette kameras backup er større end 1 MB, så indstillingerne læses og skrives én ad gangen. De gamle værdier gemmes før hver ændring og kan skrives tilbage med „Fortryd ændringer“.
tweaks-direct-forced = PMCA_TWEAKS_DIRECT gennemtvinger vejen med enkeltværdier, så indstillingerne læses og skrives én ad gangen. De gamle værdier gemmes før hver ændring og kan skrives tilbage med „Fortryd ændringer“.
tweaks-direct-refused = Kameraet afviste ændringen ({ $detail }). Værdier, der allerede var skrevet, er sat tilbage; intet er ændret. Nogle modeller, især japanske, tillader kun denne indstilling via den komplette backup.
tweaks-direct-revert-failed = Kameraet afviste ændringen ({ $detail }), og ikke alle allerede skrevne værdier kunne sættes tilbage. De gamle værdier er gemt: prøv „Fortryd ændringer“.
tweaks-direct-restart = Kameraindstillingerne er opdateret. Sluk og tænd kameraet, for at ændringen træder i kraft.
tweaks-rollback-unsaved = De aktuelle værdier kunne ikke gemmes til fortrydelse; intet er ændret.
tweaks-image-saved = Backup-billedet fra før ændringen er gemt: { $path }
tweaks-whole-image-hint = Kør den samme kommando igen med --whole-image for i stedet at skrive ændringen via hele backup-billedet. Billedet fra før ændringen gemmes først.
tweaks-whole-image-confirm = Skriv ændringen via hele backup-billedet? Kameraet afviste den som enkelt værdi. Hele billedet skrives tilbage med ændringen; billedet fra før ændringen gemmes først på denne computer.
btn-whole-image = Skriv hele backuppen
tweaks-undo-none = Der er ingen gemte værdier for dette kamera.
tweaks-undo-confirm = Skriv de gemte værdier tilbage til kameraet? Det fortryder alle ændringer, der er skrevet én ad gangen, siden de blev gemt.
tweaks-undo-done = De gemte værdier er skrevet tilbage. Sluk og tænd kameraet.
tweaks-undo-available = Værdierne fra før den første ændring er gemt og kan skrives tilbage.
btn-undo-tweaks = Fortryd ændringer
tweaks-identity-heading = Kameraets identitet (skrivebeskyttet)
tweaks-caution-languages = To sprog — sandsynligvis kinesisk (forenklet) og kinesisk (traditionelt) — kan dette kamera ikke vise. Sprogmenuen starter forfra, når du ruller hen til dem, så de forbliver slået fra for en sikkerheds skyld.
tweaks-caution-accept = Aktivér disse to sprog alligevel (på eget ansvar)
tweaks-caution-confirm = Disse to sprog kan få kameraets sprogmenu til at starte forfra. Der sker ingen skade, og ændringen kan fortrydes, men du skal måske finde indstillingen forfra. Aktivér alligevel?

# Indstillinger
settings-language = Sprog
settings-theme = Tema
settings-theme-system = System
settings-theme-light = Lyst
settings-theme-dark = Mørkt

# Avanceret tilstand. En beskyttelse mod et fejlklik, ikke mod en angriber:
# alt, hvad den viser, kan alligevel nås fra konsollen.
settings-advanced-heading = Avanceret tilstand
settings-advanced-hint = Avanceret tilstand viser serviceskallen og updater-skallen på siderne af samme navn. Servicetilstand giver direkte adgang til kameraets hukommelse, filsystem og kalibreringsdata; en forkert indtastet kommando kan gøre kameraet ubrugeligt. Lad den være slået fra, indtil du ved, hvad du vil skrive.
settings-advanced-toggle = Vis service- og updater-skallen
settings-advanced-on = Avanceret tilstand er slået til. Siderne Serviceshell og Updater-skal tilbyder nu selve skallerne.
settings-advanced-off = Avanceret tilstand er slået fra. Siderne Serviceskal og Updater-skal forklarer i stedet vejen gennem konsollen.
settings-advanced-confirm = Service- og updater-skallen kan beskadige dit kamera permanent. Vil du vise begge skaller?

# Redigering af en oversættelse. Sprogfiler kan downloades, ændres i en
# vilkårlig teksteditor og uploades igen; en uploadet fil kontrolleres,
# før den tages i brug.
settings-catalog-heading = Rediger oversættelse
settings-catalog-hint = Download en sprogfil, ret den i en vilkårlig teksteditor, og upload den igen. Kun teksten til højre for lighedstegnet oversættes, og poster, du udelader, beholder deres indbyggede ordlyd. Det sprog, du vælger her, ændrer ikke programmets sprog.
settings-catalog-locale = Sprogfil
settings-catalog-builtin = Den indbyggede oversættelse er i brug.
settings-catalog-custom = Din egen oversættelse er i brug:
btn-catalog-export = Download…
btn-catalog-import = Upload…
btn-catalog-reset = Gendan indbygget
catalog-exported = Skrevet til
catalog-imported = Oversættelsen er taget i brug.
catalog-reset-done = Den indbyggede oversættelse er gendannet.
catalog-reset-none = Der var ingen egen oversættelse at gendanne.
catalog-reset-confirm = Vil du kassere din egen oversættelse til dette sprog?
catalog-rejected = Filen blev ikke taget i brug. Ret disse linjer:
catalog-count-accepted = Anvendte poster
catalog-count-missing = Poster beholdt fra den indbyggede oversættelse
catalog-count-untranslated = Poster der er identiske med den engelske original
catalog-line = Linje
catalog-problem-syntax = ikke en post på formen "id = tekst"
catalog-problem-continuation = fortsat på den følgende linje, hvilket ikke understøttes
catalog-problem-duplicate = defineret mere end én gang
catalog-problem-unknown-key = ikke en post, som dette program bruger
catalog-problem-empty = har ingen tekst
catalog-problem-placeholder = pladsholder ændret, forventede:

# Oprindelse, anerkendelser og licens. Vises på licenssiden og af
# `pmca-console license`. Selve licensteksten oversættes ikke: den er et
# juridisk dokument og gengives ordret.
about-thanks-heading = Tak
about-thanks-body-1 = Dette værktøj findes takket være ma1co, som fandt ud af, hvordan Sonys kameraer kommunikerer over USB, og udgav Sony-PMCA-RE i 2015, og takket være alle, der har bidraget til det og til OpenMemories-projekterne i årene derefter.
about-thanks-body-2 = Kameraregistreringen, protokollen til eksterne kommandoer, proceduren for firmwareopdatering og servicetilstanden er alt sammen deres arbejde, og denne port fører det arbejde videre. Tak. Oven på det fundament tilføjer PMCA-RE RELOADED mange nye funktioner, som masterflai har bidraget med. Vigtigst er det nu indbyggede automatiske skift af enhedsdriveren under Windows: kameraet skal ikke længere have tildelt en anden driver manuelt med et separat værktøj, hvilket gør forløbet mere bekvemt, hurtigere og mindre risikabelt.
about-contributors-heading = Bidragydere
about-contributors-body = Ud over ma1co har disse personer bidraget til Sony-PMCA-RE eller udgivet ændringer i deres egne kopier af det, som er overtaget i denne port:
about-purpose-heading = Hvorfor denne port findes
about-purpose-body-1 = Det oprindelige program udvikles ikke længere. Det blev skrevet til Python 2, som nåede sin slutdato i 2020, og dele af det bygger på tjenester, som Sony siden har lukket, så det kører ikke længere uden begrænsninger på nutidige systemer.
about-purpose-body-2 = Formålet med denne port er bevaring frem for erstatning og i lige så høj grad problemfri brug i dag: at holde disse kameraer brugbare og få programmet til at fungere uden besvær på nutidens computere såvel som med nyere kameramodeller. Alt, hvad originalen kunne, bør denne stadig kunne.
about-origin-heading = Hvordan den blev til
about-origin-body-1 = Porteringen fra Python til Rust blev udført med hjælp fra AI.
about-origin-body-2 = Hvad det ikke betyder, er værd at sige lige så tydeligt: adfærden er dækket af en automatiseret testsuite og af en hardwaretestplan. Især Windows-versionen er afprøvet grundigt, fordi enhedsdriverne gør den til den mest komplekse platform; macOS og Linux har derimod ikke kunnet testes fuldt ud. Betragt venligst dette som eksperimentel software.
about-license-heading = Licens
about-license-body = Denne port udgives under MIT-licensen, den samme licens som det oprindelige projekt. Den oprindelige copyright-meddelelse bevares uændret, sådan som licensen kræver.
about-license-full = Licenstekst
about-third-party-heading = Licenser for de anvendte biblioteker
about-third-party-body = Dette program er statisk linket: koden fra hvert bibliotek, det bruger, ligger inde i den kørbare fil, og bibliotekernes licenser kræver, at ophavsretsmeddelelserne følger med. De er gengivet fuldt ud nedenfor, én post pr. pakke.
about-third-party-toggle = Vis alle tredjepartsmeddelelser
about-privacy-heading = Data
about-privacy-body-1 = Dette program kører på din egen computer. Det sender ingen brugsdata, der er ingen telemetri, og der kræves ingen konto. Intet af det, du gør her, indberettes nogen steder.
about-privacy-body-2 = To funktioner opretter dog forbindelse af sig selv, og modparten får derved din IP-adresse at vide: GPS-assistancedata hentes hos Sony og applisten hos GitHub. Uden disse funktioner forbliver programmet helt offline.
about-privacy-body-3 = Det, der læses ud af kameraet — herunder serienummeret og gemte Wi-Fi-adgangskoder — forlader ikke din computer. Skriver du det til en fil, så vær opmærksom på, hvem du giver den fil videre til.
about-privacy-body-4 = Der drives ingen tjeneste bag dette program, og ingen data når frem til dets ophavsmænd, så der er intet her at bede om indsigt i eller få slettet. Hvad de to nævnte tjenester gør med de adresser, de ser, følger deres egne privatlivspolitikker, ikke denne.
about-legal-heading = Juridiske oplysninger
about-legal-trademark = Dette er et uafhængigt projekt. Det er hverken lavet af, godkendt af eller på nogen måde forbundet med Sony. »Sony«, »PlayMemories« og kameramodellernes navne er varemærker tilhørende Sony Group Corporation og nævnes her udelukkende for at angive, hvilke kameraer dette værktøj taler med.
about-legal-own-device = Dette program er beregnet til brug med kameraer, du selv ejer.
about-legal-warranty = Det, dette værktøj gør, ligger uden for det, Sony har tiltænkt disse kameraer, og kan bringe din garanti til ophør. Softwaren gives uden nogen garanti i sig selv, og du bruger den på eget ansvar. Om og i hvilket omfang reverse engineering af eget udstyr er tilladt, afhænger af, hvor du befinder dig.
about-links-heading = Projekter
about-link-original = Sony-PMCA-RE — det oprindelige projekt
about-link-fwtool = fwtool.py — firmwareværktøjerne, som opbygningen af opdateringsbilleder bygger på
about-link-tweak = OpenMemories: Tweak — den app, der anbefales til kameraer, som understøtter den
about-link-applist = OpenMemories: App list — flere apps
about-link-platform = OpenMemories: Platform — den medfølgende updater-body
about-link-devices = Understøttede kameramodeller
about-link-nexhack = Nex-Hack — fællesskabets research, som dette projekt bygger på
about-link-zve10 = zv-e10-re — undersøgelser af ZV-E10's grænseflader og indstillingshukommelse
about-version = Version
btn-copy-url = Kopiér adresse
msg-url-copied = Adressen er kopieret til udklipsholderen.

# Sikkerhedsbemærkning
notice-experimental = Dette er eksperimentel software. Brug den på eget ansvar.

# Netværksindstillinger (GPS-assistance, adgangspunkter og tjenester til livestreaming)
gps-downloading = Downloader GPS-assistancedata…
gps-writing = Skriver GPS-assistancedata til kameraet…
gps-bad-size = GPS-assistancedataene har en uventet størrelse og blev ikke skrevet.
wifi-read = Læser adgangspunkterne til livestreaming fra kameraet…
wifi-written = Adgangspunkterne til livestreaming er skrevet.
stream-read = Læser indstillingerne for livestreaming fra kameraet…
stream-written = Indstillingerne for livestreaming er skrevet.
net-secret-warning = Advarsel: denne fil indeholder Wi-Fi-adgangskoder og adgangstokens i klartekst.
wifi-secret-warning = Kameraet gemmer disse adgangskoder ukrypteret, og de vises her nøjagtig som de er gemt. De bliver på denne computer; gemmer du dem i en fil, så vær opmærksom på, hvem du giver den videre til.
gps-heading = GPS-assistancedata
gps-hint = Træk en fil hertil, angiv en sti, eller lad feltet stå tomt for at hente de aktuelle data fra Sony.
btn-gps-devices = Kompatible enheder
btn-gps-devices-hide = Skjul kompatible enheder
gps-devices-note = Disse modeller har et indbygget GPS-modul. For mange af dem er det kun varianten, hvis navn ender på V, der har det; varianter uden V har ingen GPS.
gps-devices-compact = Kompaktkameraer (DSC)
gps-devices-slt = SLT-kameraer (A-fatning)
gps-devices-action = Actionkameraer
gps-devices-camcorder = Videokameraer (Handycam)
gps-col-variant = GPS-variant
gps-col-arch = Arkitektur
gps-col-module = GPS-modul
gps-col-apps = App-understøttelse
gps-yes = ja
gps-no = nej
gps-unknown = ukendt
gps-devices-untested = GPS til stede, understøttelse ikke testet
gps-devices-untested-note = Disse modeller har et indbygget GPS-modul, men står ikke på OpenMemories' enhedsliste. Det er ikke testet, om dette program virker med dem, så deres arkitektur og app-understøttelse er ukendt.
wifi-heading = Adgangspunkter til livestreaming
wifi-hint = Dette er de adgangspunkter, som kameraets livestreamingfunktion bruger, ikke kameraets egne Wi-Fi-indstillinger. Et kamerahus uden streamingfunktion afviser dette.
wifi-multi = Liste med flere adgangspunkter
wifi-none = Der er ikke gemt nogen adgangspunkter.
wifi-writing = Skriver adgangspunkterne til livestreaming til kameraet…
wifi-confirm = Dette erstatter alle adgangspunkter, der er gemt på kameraet. Vil du fortsætte?
wifi-empty-refused = En tom liste skrives ikke, da den ville slette alle gemte adgangspunkter.
stream-heading = Livestreaming
stream-writing = Skriver indstillingerne for livestreaming til kameraet…
stream-confirm = Dette overskriver kameraets indstillinger for livestreaming. Vil du fortsætte?
stream-invalid-json = Indstillingerne er ikke gyldig JSON:

# Firmwareopdatering og den officielle butik
firmware-warning = Advarsel: dette overskriver kameraets firmware. Afbryd ikke opdateringen, når den er startet: tag ikke USB-kablet ud, og sluk ikke kameraet. Brug kun en firmwarefil, der er beregnet til præcis denne kameramodel, og sørg for, at batteriet er tilstrækkeligt opladet. Ellers kan kameraet blive permanent ubrugeligt.
firmware-untested = Ikke testet: flashning af firmware er overført, men aldrig prøvet på et rigtigt kamera. En mislykket eller afbrudt skrivning kan ikke gøres om, så betragt alt på denne side som ubekræftet.
firmware-confirm-required = Der flashes ikke uden --confirm.
firmware-truncated = Firmwarefilen er afkortet: FDAT-payloaden rækker ud over filens slutning.
firmware-updating = Opdaterer firmware:
firmware-invalid = Filen er ikke et gyldigt Sony-firmwareimage
firmware-dat-path = Firmwarefil (.dat)
firmware-drop-hint = Slip en .dat-firmwarefil her
firmware-file-required = Vælg først en firmwarefil.
firmware-check-ok = Kameraet accepterer dette image. Der er ikke skrevet noget.
firmware-cancelled = Flashningen blev annulleret. Kameraet blev ikke ændret.
firmware-current-version = Installeret version
firmware-new-version = Version i dette image
firmware-models-hint = Et kamera er kompatibelt, hvis updater-skallen kan nå det.
btn-firmware-check = Kontrollér image
btn-firmware-flash = Flash firmware
firmware-own-device = Bekræft, at du er klar over disse risici og kun opdaterer din egen enhed.
btn-firmware-accept = Risici forstået – flash

# Filværktøjer: de handlinger, der slet ikke kræver et kamera.
tools-hint = Disse handlinger arbejder på filer og rører aldrig kameraet, så intet her kan ødelægge det. Det er de samme handlinger, som pmca-console tilbyder som apk2spk, spk2apk, print-backup, print-usagelog, guess-firmware og xpd-checksum.
tools-operation = Handling
tools-apk2spk = Pak en .apk ind i en .spk-container
tools-spk2apk = Pak en .spk ud til en .apk igen
tools-print-backup = Vis indholdet af en Backup.bin
tools-print-usagelog = Vis indholdet af en Usagelog.bin
tools-guess-firmware = Find payloaden i en firmwarefil (.dat)
tools-xpd-checksum = Beregn en fils CIC-kontrolsum
tools-input = Inputfil
tools-file-required = Vælg først de filer, denne handling har brug for.
tools-apk-invalid = Filen kunne ikke pakkes som .spk
tools-spk-invalid = Filen er ikke en gyldig .spk-pakke
tools-backup-invalid = Filen er ikke et gyldigt Backup.bin-aftryk
tools-usagelog-invalid = Filen er ikke en gyldig brugslog
tools-reset-value = nulstillingsværdi:
tools-fdat-found = Payload fundet ved offset { $offset }, størrelse { $size }.
tools-checksum = Kontrolsum
tools-written = Skrevet til { $path }

# Henvisninger til projektets dokumentation
docs-compatibility = Kamerakompatibilitet
docs-compatibility-hint = Det oprindelige projekt stillede en oversigt over testede kameraer til rådighed. Den omfatter hovedsageligt ældre kameramodeller og findes på denne adresse:
devices-note = Oplysninger fra det oprindelige projekts oversigt. Tallet ved app-understøttelse er versionen af kameraets app-platform.
devices-more-note = Følgende kameraer er ikke udtrykkeligt testet med dette program. Et flueben i den sidste kolonne betyder, at fællesskabet har meldt om brug af dem. Er serviceshellen desuden markeret med ✖, mislykkedes brugen.
btn-more-models = Flere enheder
btn-more-models-hide = Skjul flere enheder
check-updater-yes = Din enhed { $model } er kompatibel: ifølge enhedslisten kan updater-skallen nå den.
check-updater-no = Din enhed { $model } er ikke kompatibel: ifølge enhedslisten kan updater-skallen ikke nå den.
check-updater-unknown = Kompatibiliteten af din enhed { $model } er ukendt: den står ikke på nogen af programmets enhedslister, så det vides ikke, om updater-skallen kan nå den.
check-apps-no = Din enhed { $model } er ikke kompatibel: ifølge enhedslisten understøtter den ikke apps.
check-apps-untested = Kompatibiliteten af din enhed { $model } er ukendt: den står på enhedslisten, men app-understøttelse er ikke bekræftet.
check-apps-unknown = Kompatibiliteten af din enhed { $model } er ukendt: den står ikke på nogen af programmets enhedslister, så det vides ikke, om den understøtter apps.
check-gps-no = Din enhed { $model } er ikke kompatibel: den står ikke på listen over enheder med GPS-modtager.
check-gps-untested = Kompatibiliteten af din enhed { $model } er ukendt: ifølge enhedslisten har den en GPS-modtager, men overførslen til denne model er ikke testet.
check-gps-unknown = Kompatibiliteten af din enhed { $model } er ukendt: den står ikke på nogen af programmets enhedslister, så det vides ikke, om den har en GPS-modtager.
check-unidentified = Den tilsluttede enhed kunne ikke identificeres, så dens kompatibilitet er ukendt.
check-proceed = Vil du fortsætte alligevel?
check-cancelled = Handlingen blev annulleret. Enheden blev ikke ændret.
devices-col-confirmed = Bekræftet
devices-group-ilc = Systemkameraer (E- og A-fatning)
devices-group-remote = Live-View-fjernbetjeninger
btn-copy-link = Kopiér link
shutter-heading = Lukkertæller
shutter-hint = Kameraet tæller hver udløsning af lukkeren og skriver den aktuelle stand i hvert foto, det gemmer (ARW, JPEG eller HEIF). Vælg det senest optagne foto for at aflæse den aktuelle stand. Filen bliver kun læst, aldrig ændret.
shutter-source = Hvor tælleren står i filen, varierer fra model til model. Positionerne stammer fra offentligt dokumenterede undersøgelser (ExifTool) og er ikke bekræftet for alle modeller.
shutter-drop-hint = Slip et foto (ARW, JPEG, HEIF) her
btn-shutter-read = Analysér foto
shutter-file-required = Vælg først et foto.
shutter-file = Fil
shutter-taken = Optagelsesdato
shutter-count = Udløsninger
shutter-type = Lukker
shutter-type-mechanical = Mekanisk
shutter-type-electronic = Elektronisk
shutter-electronic-hint = Dette foto blev taget med den elektroniske lukker. På mange modeller tælles sådanne optagelser ikke med; værdien viser så standen ved den seneste mekaniske udløsning.
shutter-not-sony = Fotoet er ikke taget med et Sony-kamera. Der kan ikke aflæses en lukkertæller fra det.
shutter-unknown-model = Det vides ikke, hvor denne kameramodel gemmer lukkertælleren i filen, så der vises ingen værdi.
shutter-not-recorded = Filen indeholder ingen læsbar lukkertæller. Det sker med fotos, der er gemt eller eksporteret af et billedbehandlingsprogram, og på nogle modeller med optagelser med elektronisk lukker. Vælg den originale fil fra hukommelseskortet.
shutter-model-mismatch = Fotoet stammer fra en anden kameramodel end det tilsluttede kamera.
shutter-invalid = Filen er ikke et læsbart foto
file-filter-photo = Fotos (ARW, JPEG, HEIF)
usage-log-heading = Brugslog
usage-log-hint = Kameraet fører en intern log i et udokumenteret format. Den gemmes uændret; der læses kun fra kameraet.
usage-log-written = Skrevet til
btn-usage-log-info = Om brugsloggen
btn-usage-log-info-hide = Skjul forklaringen
usage-log-info-what-heading = Hvad loggen er
usage-log-info-what = Sony kalder internt loggen »KikiLog«. Kameraet skriver den selv; dette program læser den kun. Der kendes ingen offentlig beskrivelse af formatet.
usage-log-info-read = Læsning ændrer intet: flere downloads i træk var byte for byte ens.
usage-log-info-flush-heading = Hvorfor nye poster mangler
usage-log-info-flush = Nye poster vises ofte ikke med det samme: kameraet gemmer dem muligvis først, når det slukkes, nogle gange fordelt over flere tænd/sluk-cyklusser. Sluk og tænd kameraet, hent igen, og gentag, indtil der ikke kommer flere nye poster.
usage-log-info-content-heading = Hvad man kan aflæse
usage-log-info-shutter = Udløsninger: nogle hændelser svarer hver til én optagelse. Antallet af dem er dog ikke lukkertælleren: loggen har en fast størrelse og dækker ikke nødvendigvis kameraets samlede levetid. Kameraet fører totalen et andet sted og skriver den i de fotos, det gemmer; den kan aflæses i afsnittet »Lukkertæller«.
usage-log-info-clock = Tidsstempler er kun så gode som kameraets ur. Efter et batteriskift kan uret falde tilbage og danne poster med kameraets startdato.
usage-log-info-unknown = Der kendes ingen offentlig beskrivelse af hændelsesnumrene. Programmet viser numrene uden at fortolke dem.
usage-log-info-privacy-heading = Privatliv
usage-log-info-privacy = Filen indeholder serienumrene på isatte hukommelseskort. Del eller offentliggør den ikke ukritisk.
btn-usage-log = Gem log
usagelog-size = Logområde
usagelog-cursor = Skriveposition
usagelog-spare = Andet område
usagelog-count = Poster
usagelog-events = Hændelser efter nummer
usagelog-entries = Poster i rækkefølge
msg-link-copied = Linket er kopieret til udklipsholderen.

# Backendens fejlmeddelelser, slået op via den stabile kode, backenden melder.
# Den variable del af en meddelelse (en sti, en statusbyte, en enhedsfejl)
# tilføjes af brugerfladen, så teksterne er fuldstændige sætninger uden den.
# Der findes bevidst ingen `err-operation`: den kode markerer en meddelelse,
# som backenden allerede har oversat, og som skal vises uændret.
err-usb-no-device = Intet kamera fundet. Tilslut et kamera, og tænd det.
err-usb-camera-not-ready = Kameraet er tilsluttet, men ikke klar: Det er slukket eller venter på skærmen på, at USB-tilstanden vælges. Tænd det, eller vælg „Masselager (MSC)“ på kameraet.
camera-choose-usb-mode = Kameraet venter på, at USB-tilstanden vælges. Vælg „Masselager (MSC)“ på kameraet.
err-usb-too-many-devices = Der er tilsluttet mere end ét kamera. Frakobl alle undtagen det, du vil arbejde med.
err-usb-transfer = USB-overførslen mislykkedes. Prøv et andet kabel eller en anden port, og undgå USB-hubs.
err-usb-permission = Programmet har ikke lov til at åbne kameraet over USB. På Linux gives denne ret én gang med en udev-regel for Sonys producentkendetegn; scriptet scripts/linux-prepare.sh i kildeteksten installerer den. Tag derefter kameraet ud, og tilslut det igen.
err-usb-busy = Et andet program bruger kameraet, så det kunne ikke åbnes. Luk andre programmer, der kan tilgå det; på Linux kan skrivebordets mediemonitor holde det, hvilket scripts/linux-prepare.sh standser. På macOS skal kameraets USB-forbindelse sættes til MTP: i masselagringstilstand holder systemets egen diskdriver det.
err-usb-needs-root = macOS overlader kun kameraets masselagerforbindelse til et program, der kører med administratorrettigheder. Start kommandolinjeprogrammet med sudo, for eksempel: sudo pmca-console tweaks. Kameraoplysningerne kan også læses uden, når kameraets USB-forbindelse er sat til MTP.
err-usb-stall = Kameraet afbrød USB-overførslen.
err-usb-protocol = Kameraet sendte et uventet svar.
err-usb-msc-status = Kameraet afviste en lagerkommando.
cli-error-prefix = fejl
err-usb-driver = Windows har tildelt dette kamera en driver, der ikke tillader direkte adgang. Denne tilstand kræver WinUSB-driveren, som programmet selv kan tildele — men kun med administratorrettigheder.
err-needs-elevation = Tildeling eller frigivelse af en driver kræver administratorrettigheder, og Windows-forespørgslen blev afvist. Prøv igen med „Start driverhjælper“; intet på kameraet skal ændres.
err-usb-no-mass-storage-volume = Kameraet er tilsluttet i masselagringstilstand, men stiller ikke noget drev til rådighed. Væk det, bekræft USB-tilstanden på dets skærm, hvis det spørger, og kontrollér, at der sidder et hukommelseskort i. Skift ikke driver: denne tilstand kræver netop Windows' standarddriver.
err-usb-msc-sense = Kameraet meldte en lagerfejl.
err-usb-ptp-response = Kameraet afviste kommandoen.
err-usb-unsupported = Denne kameramodel understøtter ikke den funktion.
err-usb-updater-status = Firmware-updateren på kameraet afviste handlingen.
err-usb-senser = En kommando i servicetilstand mislykkedes.
err-market-core = En datafil fra kameraet kunne ikke læses.
err-market-json = De modtagne data var fejlbehæftede.
err-market-missing-field = Der manglede et forventet felt i svaret.
err-market-no-apk = Der er ikke valgt nogen app til installation.
err-market-http = Netværksanmodningen mislykkedes.
err-market-io = Den lokale installationsserver mislykkedes.
err-market-tls = Den krypterede forbindelse til kameraet kunne ikke oprettes.
err-apk = App-filen kunne ikke læses.
err-io = En fil kunne ikke læses eller skrives.
err-json = Kameraet sendte fejlbehæftede data.
err-unknown-locale = Dette sprog er ikke tilgængeligt.
err-catalog-encoding = Oversættelsesfilen er ikke gemt som UTF-8-tekst. Gem den igen med UTF-8-kodning.
err-internal = Der opstod en intern fejl. Dette er en fejl i programmet.

## Function analysis
nav-analysis = Analyse
analysis-title = Funktionsanalyse
analysis-experimental = Eksperimentel. Funktionen er ny og kun afprøvet på få kameraer.
analysis-own-hardware = Brug den kun på dit eget kamera. Allerede servicetilstanden er et indgreb, som Sony ikke har tiltænkt; ansvaret er dit.
analysis-explanation-toggle = Forklaring: formål, forløb, risici
analysis-purpose-heading = Hvad er den til?
analysis-purpose = Funktionsanalysen dokumenterer, i hvilken værdi i sin indstillingshukommelse (backup) kameraet gemmer en af sine egne menuindstillinger. Resultatet beskriver dette kameras indstillingshukommelse; det kan sammenlignes med andre kameraer, men ændrer ikke, hvad et kamera tilbyder.
analysis-how-heading = Hvordan foregår den?
analysis-how = For hver menuværdi, du angiver, læser programmet hele indstillingshukommelsen én gang, hver gang i sin egen korte session i servicetilstand. Ind imellem trækker du kablet ud og ændrer indstillingen i kameraets menu; kameraet forbliver tændt. Til sidst læses den første værdi igen. Værdier, der følger menuen og til sidst er tilbage på startværdien, er kandidater. Værdier, der ændrer sig af sig selv (tællere, ur), sorteres fra.
analysis-requirements-heading = Forudsætninger
analysis-requirements = Kameraet er tilsluttet via USB, dets USB-tilstand står på „Auto“, og efter hver session vælger du „Masselager (MSC)“ på kameraet. Under Windows kræver programmet administratorrettigheder, fordi det kortvarigt tildeler en driver til servicetilstanden. Oplad batteriet; hver session tager omkring et minut.
analysis-read-heading = Læsende trin
analysis-read = Øjebliksbillederne læser kun. Intet i kameraets indstillinger ændres. Risikoen er lille, men ikke nul: allerede skiftet til servicetilstand er et indgreb, som Sony ikke har tiltænkt.
analysis-write-heading = Skrivende trin (skrivetest)
analysis-write = Kun hvis du udtrykkeligt slår den til: programmet skriver den fundne værdi til kameraet, du kontrollerer efter en genstart i menuen, om den virker, og derefter skrives den oprindelige værdi tilbage. Der skrives kun en værdi, som kameraet selv har gemt, og kun hvis der er fundet præcis én passende kandidat. Den gamle værdi gemmes først. Alligevel ændrer dette trin kameraet; en fejl kan efterlade det i en tilstand, som kun service kan rette.
analysis-legal-heading = Juridisk
analysis-legal = Softwaren leveres uden nogen garanti (MIT-licens). Indgreb i kameraet kan påvirke garantien. Brug kun funktionen på kameraer, du ejer. Analyserapporten indeholder intet serienummer og intet fra indstillingshukommelsen ud over de fundne kandidater; den sendes aldrig automatisk.
analysis-start-heading = Ny analyse
analysis-setting-label = Menuindstilling, som kameraet kalder den
analysis-values-label = Menuværdier, adskilt med komma
analysis-values-hint = Den første værdi er den, der er indstillet nu. To til otte værdier; hver koster én session i servicetilstand.
analysis-write-option = Udfør derefter skrivetest (ændrer kameraet)
analysis-write-warning = Skrivetesten ændrer kameraet: den skriver en værdi i dets indstillingshukommelse og skriver derefter den oprindelige værdi tilbage. Kun på dit eget kamera, på egen risiko.
analysis-write-consent = Kameraet er mit, og jeg bærer risikoen.
btn-analysis-write-enable = Slå skrivetest til
btn-analysis-start = Start analyse
btn-analysis-snapshot = Læs backup
btn-analysis-new = Ny analyse
analysis-discard-confirm = Kassér den igangværende analyse? Rapporten indtil nu forbliver gemt.
analysis-access-read = LÆSER
analysis-access-write = SKRIVER
analysis-access-user = MANUELT
analysis-access-local = PC
analysis-step-snapshot = Læs backup, indstillingen står på „{ $value }“
analysis-step-change = Skift til „{ $value }“ i kameraets menu
analysis-step-evaluate = Sammenlign øjebliksbillederne
analysis-step-write = Skriv værdien for „{ $value }“ til kameraet
analysis-step-check = Sluk og tænd kameraet, aflæs menuen (forventet: „{ $value }“)
analysis-step-restore = Skriv den oprindelige værdi for „{ $value }“ tilbage
analysis-step-report = Gem analyserapporten
analysis-do-prepare = FORBEREDELSE: 1. Sæt „{ $setting }“ til „{ $value }“ i kameraets menu, og forlad menuen med MENU-knappen, så kameraet gemmer indstillingen. 2. Lad kameraet være tændt. 3. Sæt kablet i, og vælg „Masselager (MSC)“ på kameraet.
analysis-do-change = DIN TUR: 1. Træk kablet ud. 2. Kameraet forbliver tændt. 3. Sæt „{ $setting }“ til „{ $value }“ i kameraets menu, og forlad menuen med MENU-knappen, så kameraet gemmer indstillingen. 4. Sæt kablet i igen, og vælg „Masselager (MSC)“.
analysis-do-check = DIN TUR: 1. Træk kablet ud. 2. Sluk og tænd kameraet igen. 3. Se i menuen, hvad der står ved „{ $setting }“.
analysis-do-reconnect = Sæt kablet i igen, og vælg „Masselager (MSC)“ på kameraet.
analysis-reading = Øjebliksbillede { $index } af { $total }: træk IKKE kablet ud under læsningen.
analysis-cli-continue = Enter for at fortsætte, q for at annullere:
analysis-confirm-word = ja
analysis-cli-confirm = Skriv „{ $word }“ for at bekræfte:
analysis-write-declined = Ikke bekræftet. Intet blev skrevet.
analysis-aborted = Annulleret. Intet blev skrevet.
analysis-snapshot-done = Øjebliksbillede læst. Kameraet har forladt servicetilstanden.
analysis-result = { $compared } værdier sammenlignet: { $count } kandidater, { $running } værdier ændrede sig af sig selv.
analysis-candidate-consistent = passer entydigt til menuen
analysis-candidate-inconsistent = passer ikke entydigt
analysis-write-preview-heading = Dette bliver skrevet
analysis-write-preview = Værdi { $id } ({ $size } byte): fra { $old } („{ $old-label }“) til { $new } („{ $new-label }“). Kameraet gemte selv den nye værdi, mens „{ $new-label }“ var indstillet.
btn-analysis-write = Skriv
btn-analysis-skip-write = Afslut uden skrivetest
analysis-written = Værdi skrevet, læst tilbage og gemt permanent.
analysis-menu-question = Hvad viser menuen nu?
analysis-menu-other = noget andet
btn-analysis-menu-confirm = Overtag
btn-analysis-restore = Skriv den oprindelige værdi tilbage
analysis-restored = Den oprindelige værdi er skrevet tilbage og gemt permanent.
analysis-survived = Den skrevne værdi var stadig gemt efter genstarten.
analysis-not-survived = Den skrevne værdi var ikke længere gemt efter genstarten.
analysis-report-saved = Analyserapport gemt: { $path }
analysis-report-hint = Rapporten indeholder hverken serienummer eller backup. Du kan give den videre, så fundet kan kontrolleres og dokumenteres.
btn-analysis-export = Eksportér rapport…
file-filter-analysis = Analyserapport
analysis-pending = En skrivetest blev ikke afsluttet. Den oprindelige værdi er gemt og kan skrives tilbage.
analysis-pending-none = Der er ingen uafsluttet skrivetest gemt for dette kamera.
analysis-invalid-setting = Angiv menuindstillingen, som kameraet kalder den.
analysis-invalid-values = Angiv to til { $max } forskellige menuværdier. Den første er den, der er indstillet nu.
analysis-already-complete = Alle øjebliksbilleder er allerede læst.
analysis-other-camera = Det er ikke det samme kamera som ved det første øjebliksbillede. Analysen er stoppet.
analysis-write-not-allowed = Skrivetesten er ikke mulig i denne tilstand.
analysis-camera-changed = Kameraet har nu en anden værdi end ved det første øjebliksbillede. Intet blev skrevet.
analysis-write-refused = Kameraet afviste værdien; alt er som før. ({ $detail })
analysis-refuse-none = Skrivetest ikke mulig: ingen værdi fulgte menuen.
analysis-refuse-ambiguous = Skrivetest ikke mulig: { $count } værdier fulgte menuen, ingen kan tildeles med sikkerhed.
analysis-refuse-inconsistent = Skrivetest ikke mulig: værdien passer ikke entydigt til menuværdierne.
analysis-refuse-size = Skrivetest ikke mulig: værdien er { $size } byte; kun små værdier skrives.
analysis-refuse-read-only = Skrivetest ikke mulig: kameraet markerer værdien som skrivebeskyttet.
analysis-refuse-protected = Skrivetest ikke mulig: kameraet markerer værdien som beskyttet.
settings-analysis-heading = Funktionsanalyse
settings-analysis-purpose = Funktionsanalysen dokumenterer, i hvilken værdi i sin indstillingshukommelse kameraet gemmer en af sine egne menuindstillinger. Den er beregnet til dem, der vil forstå deres models indstillingshukommelse og dele resultaterne.
settings-analysis-why-off = Den er slået fra, fordi almindelig brug ikke kræver den, fordi den skifter kameraet til servicetilstand flere gange, og fordi den på anmodning også skriver til kameraet. Den er eksperimentel og kun afprøvet på få modeller.
settings-analysis-toggle = Slå funktionsanalysen til (punktet „Analyse“ i menuen)
settings-analysis-confirm = Slå funktionsanalysen til? Den skifter kameraet til servicetilstand flere gange og kan på anmodning skrive til kameraet. Brug den kun på dit eget kamera.
btn-analysis-enable = Slå til
settings-analysis-on = Funktionsanalysen er slået til.
settings-analysis-off = Funktionsanalysen er slået fra.
analysis-needs-admin = Hvert øjebliksbillede kræver administratorrettigheder til servicetilstand. Windows spørger én gang pr. programsession; du kan starte driverhjælperen nu eller vente, til det første øjebliksbillede beder om den.
analysis-changed-hint = Kontrollér: der blev ikke fundet nogen kandidat. Følgende værdier ændrede sig, men vendte ikke tilbage til startværdien. Hvis en af dem kun skifter én gang mellem to værdier, stod kameraet sandsynligvis ikke på den første menuværdi ved det første øjebliksbillede. Gentag så analysen med korrekt indstillet startværdi.
analysis-changed-info = Info: værdier, der ændrede sig af sig selv under analysen (f.eks. tællere, ur), ignoreres. Du skal ikke gøre noget.
elevation-admin = Administrator
elevation-standard = Standardrettigheder
elevation-admin-hint = Programmet kører med administratorrettigheder og kan tildele den driver, som servicetilstanden kræver.
elevation-standard-hint = Programmet kører med standardrettigheder. Til servicetilstand, tweaks og funktionsanalysen starter det efter behov en driverhjælper med administratorrettigheder; Windows spørger én gang pr. session. Klik for at starte den.
driver-helper-declined = Forespørgslen om administratorrettigheder blev afvist. Uden dem kan driveren til servicetilstand ikke tildeles; intet blev ændret.
btn-remove-foreign-drivers = Fjern gamle drivere
btn-not-now = Ikke nu
btn-ok = OK
foreign-drivers-found = Windows har stadig drivere, som Zadig eller et lignende værktøj har installeret til Sony-enheder: { $packages }. Windows giver kameraet til dem, før dette program kan tildele sin egen driver; derfor mislykkes servicetilstanden, og kameraet vises måske ikke som drev. Vil du fjerne dem nu? Kameraet bruger derefter de drivere, der følger med Windows. Fjernelsen kræver administratorrettigheder én gang.
foreign-drivers-removed = De gamle drivere er fjernet. Tag kameraet ud, og sæt det i igen.
foreign-drivers-remaining = Disse drivere kunne ikke fjernes: { $packages }. Genstart Windows, og prøv igen, eller afinstaller enheden sammen med driveren i Enhedshåndtering.
foreign-drivers-hint = Der findes stadig drivere, som Zadig eller et lignende værktøj har installeret til Sony-enheder ({ $packages }). Windows giver kameraet til dem, før dette program kan tildele sin egen driver, hvilket kan få servicetilstanden til at mislykkes. Fjern dem med: pmca-console driver zadig --remove
driver-zadig-none = Der er ingen drivere fra Zadig eller lignende værktøjer installeret til Sony-enheder.
driver-zadig-entry = { $package } ({ $provider }): { $devices }
driver-zadig-remove-hint = Fjern dem med: pmca-console driver zadig --remove
driver-zadig-removed = Fjernede { $count } gammel(t/le) driverpakke(r). Tag kameraet ud, og sæt det i igen.
elevation-helper = Driverhjælper aktiv
elevation-helper-hint = En hjælper med administratorrettigheder tildeler driveren til servicetilstand. Den har intet vindue og slutter med programmet; der kommer ingen yderligere forespørgsel.
elevation-helper-confirm = Start driverhjælperen nu? Windows beder om administratorrettigheder én gang og derefter ikke igen, før programmet lukkes.
analysis-cross-heading = Kontrol på et andet kamera
analysis-cross-intro = Valgfrit: kontrollerer, om et andet kamera også har den fundne værdi, og hvad der er indstillet dér. Der læses kun. Samme nummer betyder ikke nødvendigvis samme funktion; kun et blik i det andet kameras menu bekræfter det. Resultatet kommer i samme analyserapport.
analysis-cross-do = 1. Træk det analyserede kamera ud. 2. Tænd det andet kamera, sæt dets USB-tilstand til „Auto“, sæt kablet i, og vælg „Masselager (MSC)“.
btn-analysis-cross = Kontrollér andet kamera
analysis-cross-present = findes, værdi { $value }
analysis-cross-match = svarer til „{ $label }“
analysis-cross-nomatch = svarer ikke til nogen målt menuværdi
analysis-cross-absent = findes ikke
analysis-cross-same-camera = Det er selve det analyserede kamera. Tilslut det andet kamera.
analysis-cross-not-now = Kontrollen er først mulig, når alle øjebliksbilleder er læst, og ingen skrevet værdi venter på at blive skrevet tilbage.
analysis-cross-none = Der er ingen kandidat at kontrollere.
analysis-wrong-camera = Det tilsluttede kamera er ikke det analyserede. Tilslut { $model }; intet blev skrevet.
keys-fetching = Henter kilden…
keys-scanning = Søger efter nøglemateriale…
keys-complete = Alt nøglemateriale er til stede.
keys-incomplete = Noget nøglemateriale mangler stadig.
keys-present = til stede
keys-missing = mangler
keys-class-senser = Servicetilstandsnøgler (serviceskal, tweaks, analyse)
keys-class-cic = XPD-kontrolsumsnøgle (app-installation)
keys-class-spk = SPK-nøglemateriale (app-installation, apk2spk)
keys-class-fdat = FDAT-nøglemateriale (updater-shell-firmware)
plugin-heading = Plugins
plugin-intro = Plugins udvider programmet med sprog, kontakter til kameraindstillinger, kompatibilitetsoplysninger, værktøjer og diagnoser. De kommer kun fra filer, du selv vælger – der findes ingen plugin-butik. Deres kode kører i en sandkasse uden adgang til dine filer, netværket eller nøglematerialet. Installer kun plugins fra forfattere, du stoler på; ændringer træder i kraft ved næste start.
plugin-none = Ingen plugins installeret.
plugin-inactive = (aktiv efter en genstart)
plugin-author = Forfatter
plugin-restart = Genstart programmet, så ændringen træder i kraft.
btn-plugin-install = Installer plugin…
plugin-install-confirm = Dette plugin må:
btn-confirm-plugin-install = Installer plugin
plugin-installed = Pluginet er installeret.
plugin-remove-confirm = Fjern pluginet “{ $name }”?
plugin-removed = Pluginet { $id } er fjernet.
plugin-perm-languages = tilføje sprog til programmet
plugin-perm-camera-settings = ændre kameraindstillinger (kun mellem de to værdier, det angiver, og først efter din bekræftelse)
plugin-perm-compatibility = tilføje poster til kompatibilitetsoversigten
plugin-perm-code = køre sin egen kode i sandkassen (værktøjer eller diagnoser)
plugin-perm-host-os = spørge, hvilket styresystem dette er
plugin-perm-host-cameras = se, hvilke Sony-kameraer der er tilsluttet og i hvilken tilstand
plugin-invalid = Dette er ikke et brugbart plugin: { $reason }
plugin-no-folder = Mappen til brugerdata kunne ikke bestemmes, så plugins kan ikke installeres.
plugin-not-installed = Der er ikke installeret noget plugin med id'et { $id }.
plugin-failed = Pluginet mislykkedes: { $reason }
plugin-unknown-entry = Intet indlæst plugin leverer { $id }.
plugin-run-done = Pluginet er færdigt.
plugin-run-warning = Pluginet er færdigt med bemærkninger.
plugin-run-error = Pluginet har meldt en fejl.
plugin-tools-heading = Plugin-værktøjer
plugin-tools-hint = De kommer fra installerede plugins og kører i en sandkasse. Et plugin får kun det, du giver det, og det, det har anmeldt, og kan ikke ændre kameraet.
plugin-diagnostics-heading = Plugin-diagnoser
plugin-compat-heading = Fra et plugin
plugin-col-note = Bemærkning
file-filter-plugin = Plugin-fil
keys-not-set-up = Nøglematerialet, som denne handling kræver, er ikke sat op.
err-missing-keys = Nøglematerialet, som denne handling kræver, er ikke sat op.
keys-heading = Nøglemateriale kræves
keys-intro = Servicetilstandsnøglerne, XPD-kontrolsumsnøglen og SPK-nøglematerialet er ikke en del af dette program. Angiv adressen på en kilde, du har — et repository, et arkiv eller en fil — og programmet finder og importerer de dele, det genkender. Intet køres, og materialet opbevares kun på denne computer.
keys-law-notice = Overhold de love, der gælder, hvor du er, og arbejd kun med kameraer og data, der er dine egne.
keys-url-label = Kildeadresse (én pr. linje)
keys-url-required = Angiv mindst én kildeadresse.
btn-keys-import = Importer
btn-import-keys = Konﬁgurer nøglemateriale
