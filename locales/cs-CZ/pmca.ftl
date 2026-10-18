# Český překlad PMCA-RE RELOADED.
# Překládá se pouze pravá strana (text za znakem =). Identifikátory nikdy neměňte.
# Jeden záznam na řádek: <id> = <text>.

app-title = PMCA-RE RELOADED – sada nástrojů pro fotoaparáty

# Navigace / karty
nav-camera-info = Informace o fotoaparátu
nav-install-app = Aplikace
nav-tweaks = Úpravy
nav-network = Síť
nav-firmware = Firmware
nav-updater = Shell updateru
nav-service = Servisní shell
nav-tools = Souborové nástroje
nav-settings = Nastavení
nav-license = Licence

# Úvodní stránka: stránka, kterou okno otevírá.
nav-welcome = Vítejte
welcome-intro-heading = Co tento program dělá
welcome-intro-body = Tento program pomáhá zachovat funkce vašeho fotoaparátu Sony a přizpůsobit jej vašim potřebám. Poté, co Sony ukončil obchod s PlayMemories Camera Apps, neexistoval žádný způsob, jak zakoupené nebo již nainstalované aplikace zálohovat či znovu nainstalovat. Reset fotoaparátu nainstalované aplikace nenávratně smaže. Tento program umožňuje stávající aplikace zálohovat a znovu nainstalovat a navíc může nabídnout volně dostupné komunitní aplikace. Můžete také instalovat vlastní aplikace (soubory *.apk). Při potížích program načte informace o fotoaparátu a interní protokoly pro analýzu. Nastavení, která fotoaparát už má, ale v menu je nenabízí, lze přepnout a opět vrátit zpět. Program komunikuje s fotoaparátem přímo přes kabel USB. Které modely jsou podporovány, najdete v seznamu na stránce Informace o fotoaparátu.
welcome-start-heading = První krok
welcome-start-body = Nejprve fotoaparát zapněte a ještě před připojením kabelu nastavte v jeho nabídce „Připojení USB“ na Velkokapacitní úložiště nebo MTP. „Auto“ obvykle funguje také, protože si fotoaparát pak sám jednu z těch dvou možností zvolí. Na pořadí záleží: jakmile je kabel připojen, k nabídce fotoaparátu se už nedostanete. Teprve potom fotoaparát připojte přes USB, otevřete Informace o fotoaparátu a stiskněte „Načíst informace o fotoaparátu“. Jakmile se objeví model a verze firmwaru, spojení je v pořádku a všechny další funkce tohoto programu jsou k dispozici.
welcome-start-driver = Objeví-li se místo toho hlášení o ovladači, řešte nejdříve to. Ve Windows si fotoaparát přivlastňuje systémový ovladač a je třeba jej nahradit, než bude možný přímý přístup přes USB, který tato sada nástrojů potřebuje. Program to udělá sám, ale přiřazení ovladače je právo, které Windows přiznávají jen správci. V takovém případě vám program nabídne možnost restartovat jej jako správce a Windows si vyžádají potvrzení této akce. Na konci sezení se přiřazení opět zruší. V systémech Linux a macOS to není nutné.
welcome-start-driver-linux = Objeví-li se místo toho hlášení o ovladači, řešte nejdříve to. V Linuxu není třeba nic nahrazovat: k fotoaparátu se přistupuje přímo. Jednorázově je potřeba právo zařízení otevřít — pravidlo udev pro identifikátor výrobce Sony — a pro režim velkokapacitního úložiště uvolnit jaderný ovladač usb-storage. Obojí se připravuje mimo tento program s právy roota; skript scripts/linux-prepare.sh ve zdrojovém stromu udělá obojí. Toto okno samo o taková práva nikdy nežádá.
welcome-start-driver-macos = Pokud se místo toho zobrazí upozornění na ovladač, řiďte se nejprve jím. V macOS není třeba nic instalovat ani nahrazovat. Nastavte připojení USB fotoaparátu na MTP. Kvůli úpravám si toto okno vyžádá vaše heslo správce: probíhají přes připojení fotoaparátu jako velkokapacitního úložiště, které macOS předá jen takovému programu.
welcome-tweaks-heading = Úpravy a jazyky
welcome-tweaks-body = Úpravy jsou nastavení, která fotoaparát již má, ale nenabízí je ve svém menu: časový limit nahrávání, limit nahrávání ve 4K, volbu mezi PAL a NTSC, instalátor aplikací přes USB — a jazyky menu.
welcome-tweaks-languages-heading = Jazyky fotoaparátu
welcome-tweaks-languages-body = Fotoaparáty Sony zobrazují jen jazyky menu, které jsou běžné v oblasti jejich prodeje. Kdo žije v zahraničí, proto často nenajde svůj jazyk a musí si vystačit s cizím. Další jazyky však firmware už obsahuje. Tento program je přidá do menu, abyste mohli fotoaparát ovládat v jazyce, který znáte.
welcome-tweaks-ui-heading = Jazyk tohoto programu
welcome-tweaks-ui-body = Zcela odděleně od toho stojí volič nahoře. Mění pouze jazyk tohoto programu a fotoaparátu se nikdy nedotýká.
welcome-tweaks-safety = Každá úprava se zapisuje do záložní paměti fotoaparátu a každou lze týmž přepínačem vzít zpět. Přerušený zápis může fotoaparát trvale poškodit, proto během něj neodpojujte kabel ani nevyjímejte baterii.
welcome-privacy-heading = Vaše data
welcome-privacy-where = Stránka „Licence“ toto vše uvádí znovu v úplnosti, spolu s licencemi všech knihoven, z nichž je tento program sestaven.
welcome-where-heading = Kde co najít
welcome-where-info = Model, sériové číslo a verze firmwaru. Zároveň test spojení.
welcome-where-install = Aplikace ze seznamu komunity, nebo váš vlastní soubor .apk.
welcome-where-tweaks = Nastavení, na která menu fotoaparátu nedosáhne, mezi nimi jazyky.
welcome-where-network = Data asistence GPS a přístupové body i služby používané pro živé vysílání.
welcome-where-firmware = Zapsat obraz firmwaru Sony do fotoaparátu.
welcome-where-tools = Rozbalit a prohlédnout si soubory firmwaru a balíčků. Bez fotoaparátu.
welcome-where-shells = Přímý přístup k paměti a souborovému systému fotoaparátu. Nejprve zobrazit v Nastavení.
welcome-where-settings = Motiv, pokročilý režim a úprava překladu tohoto programu.
welcome-where-license = Odkud tento převod pochází, na čí práci staví a za jakých podmínek je zveřejněn.
welcome-where-hint = Žádná stránka na fotoaparát sama od sebe nesáhá. Nic se nečte ani nezapisuje, dokud nestisknete tlačítko, které to ohlašuje.

# Tlačítka
btn-get-info = Načíst informace o fotoaparátu
btn-refresh = Obnovit
btn-load-apps = Načíst dostupné aplikace
btn-list-tweaks = Zobrazit dostupné úpravy
btn-install = Nainstalovat vybranou aplikaci
btn-install-file = Nainstalovat soubor .apk…
btn-apply = Použít
btn-save = Uložit do fotoaparátu
btn-add = Přidat
btn-remove = Odebrat
btn-reveal = Zobrazit
btn-update-gps = Přenést asistenční data GPS do zařízení
btn-cancel = Zrušit
btn-browse = Procházet…
btn-switch-msc = Přepnout do režimu velkokapacitního úložiště
btn-start-driver-helper = Spustit pomocníka ovladačů

# Popisky filtrů typů souborů nabízených v dialozích pro výběr souboru
file-filter-apk = Aplikace pro Android (*.apk)
file-filter-firmware = Obraz firmwaru Sony (*.dat)
file-filter-usage-log = Protokol používání (*.bin)
file-filter-catalog = Soubor překladu (*.ftl)
file-filter-spk = Balíček aplikace Sony (*.spk)
file-filter-any = Všechny soubory
file-filter-gps = Asistenční data GPS (*.dat)

# Stav / zprávy
msg-connecting = Probíhá připojování…
msg-usage-log-written = Protokol používání byl zapsán do { $path }
msg-apps-found = Nalezeno aplikací: { $count }
msg-switch-mode = Přepněte fotoaparát do režimu MTP nebo velkokapacitního úložiště.
msg-no-tweaks = Pro tento fotoaparát nejsou k dispozici žádné úpravy.
msg-done = Hotovo.

# Přepnutí režimu USB fotoaparátu (MTP -> velkokapacitní úložiště). Nabízí se pod
# chybou „nesprávný režim“; pro opačný směr neexistuje žádný příkaz USB.
mode-switching = Fotoaparát je žádán o přepnutí do režimu velkokapacitního úložiště…
mode-switch-wait = Čeká se na opětovné připojení fotoaparátu…
camera-confirm-hint = Hlášení na fotoaparátu (např. o www.sony.net/pm) lze zavřít; postup neruší.
mode-switch-done = Fotoaparát je nyní v režimu velkokapacitního úložiště.
mode-switch-timeout = Fotoaparát se v režimu velkokapacitního úložiště neobjevil. Vypněte jej a znovu zapněte a zkuste to znovu.
mode-switch-impossible = Tento režim nelze změnit přes USB. Vypněte fotoaparát a znovu jej zapněte.
mode-switch-hint = Přes USB lze přepnout pouze fotoaparát v režimu MTP. Změna je dočasná: režim USB nastavený v nabídce fotoaparátu zůstává nezměněn, takže vypnutí a opětovné zapnutí fotoaparátu — nebo odpojení a připojení kabelu — obnoví předchozí režim. Pro cestu zpět neexistuje žádný příkaz USB.

# Informační pole fotoaparátu
info-model = Model
info-product-code = Kód produktu
info-serial = Sériové číslo
info-firmware = Verze firmwaru
info-lens = Objektiv
info-region = Oblast
info-mode = Režim
mode-mass-storage = Velkokapacitní úložiště
mode-mtp = MTP
mode-updater = Režim updateru
mode-app-install = Režim instalace aplikací
mode-service = Servisní režim
mode-unknown = Neznámý
info-android-version = Verze platformy Android
info-photographer = Fotograf
info-copyright = Copyright
info-photographer-1 = Fotograf (položka 1)
info-photographer-2 = Fotograf (položka 2)
info-copyright-1 = Copyright (položka 1)
info-copyright-2 = Copyright (položka 2)
diagnose-services-heading = Fotoaparát není dostupný?
diagnose-services-hint = Některé služby Windows mohou fotoaparát v režimu MTP blokovat, takže se k němu tato aplikace nedostane. Kontrola se Windows jen ptá, které služby běží; nic nemění.
btn-diagnose-services = Zkontrolovat služby Windows
diagnose-services-none = Neběží žádná známá rušivá služba.
diagnose-services-found = Tyto spuštěné služby mohou rušit přístup k fotoaparátu:
diagnose-services-advice = Zastavte službu v okně Služby (services.msc) nebo následujícím příkazem jako správce a poté fotoaparát znovu připojte:

# Aktualizace firmwaru / shell
updater-model = Model fotoaparátu
updater-firmware-current = Aktuální verze firmwaru
updater-guard-ok = Obraz firmwaru odpovídá tomuto fotoaparátu.
updater-no-fdat = Pro model { $model } není k dispozici žádný dodávaný firmware pro režim aktualizace.
updater-model-required = Fotoaparát je již v režimu aktualizace a v něm nemůže sdělit svůj model. Vypněte fotoaparát a znovu jej zapněte, aby se spustil v normálním režimu, a zkuste to znovu. Případně zadejte soubor payloadu (--fdat-file) nebo v konzoli model (--model <MODEL>).
updater-already-updater = Fotoaparát je již v režimu aktualizace.
updater-flash-required = Změna fotoaparátu bez potvrzení byla odmítnuta. Spusťte příkaz znovu s --flash, aby se fotoaparát přepnul do režimu aktualizace a zapsal se payload.
updater-flashing = Do fotoaparátu se zapisuje payload updateru…
updater-switch-wait = Postupujte podle pokynů na displeji fotoaparátu. Čeká se na přepnutí fotoaparátu do režimu aktualizace…
updater-switch-timeout = Vypršel časový limit čekání na fotoaparát. Jakmile se znovu připojí, spusťte příkaz znovu.
updater-shell-ready = Payload updateru běží. Příkazy zobrazíte pomocí „help“, ukončíte pomocí „exit“.
updater-shell-done = Relace shellu byla ukončena.
updater-session-panic = Relace updateru neočekávaně skončila. Než to zkusíte znovu, fotoaparát vypněte a zase zapněte.

# Co odpovídá shell updateru. Chyby při čtení řádku jsou formulovány jen jednou,
# v klíčích service-*: oba shelly sdílejí rozkladač, takže cesta v uvozovkách
# i špatná šestnáctková hodnota selhávají v obou stejně.
updater-backup-synced = Záloha byla zapsána do paměti flash.
updater-backup-locked = Chráněná nastavení zálohy jsou nyní zamčena.
updater-backup-writable = Do chráněných nastavení zálohy lze nyní zapisovat.
updater-shell-console-only = Root shell potřebuje terminál a existuje jen na příkazové řádce: spusťte pmca-console updatershell --flash a napište tam „shell“.

# Stránka shellu updateru.
updater-warning = Pozor: aby se k tomuto shellu dalo dojít, restartuje se fotoaparát do režimu updateru a zapíše se do něj payload – malý program tohoto projektu, který fotoaparát přijme jako aktualizaci firmwaru a který pak odpovídá na příkazy tohoto shellu. Ten při vypnutí fotoaparátu zase zmizí, ale příkazy, které nabízí, zapisují do trvalé paměti nastavení fotoaparátu a přerušený zápis může fotoaparát nenávratně poškodit. Navíc se přitom resetují všechna nastavení fotoaparátu – na rozdíl od servisního shellu.
updater-console-heading = Spuštění shellu updateru na příkazové řádce
updater-driver-hint = Shell updateru potřebuje fotoaparát na vlastním ovladači velkokapacitního úložiště Windows, USBSTOR, protože tímto ovladačem se přenáší payload. Je to přesný opak toho, co potřebuje servisní shell. Aplikace ovladač WinUSB sama přiřadí i sama uvolní, takže není třeba nic připravovat ručně — ovladač přiřazený jiným nástrojem je ale nutné nejdřív vrátit zpět. Fotoaparát v režimu updateru se hlásí pod vlastním ID produktu a ani ten nesmí být spřažen s jiným ovladačem.
updater-fdat-label = Soubor payloadu (volitelně)
updater-confirm = Fotoaparát se restartuje do režimu updateru a zapíše se do něj payload. Pokračovat?
updater-cancelled = Zrušeno. Fotoaparát nebyl změněn.

btn-shell-primer = Co je dobré vědět o tomto shellu
btn-shell-primer-hide = Skrýt vysvětlení
shell-primer-compare-heading = Dva shelly, dva protokoly
shell-primer-compare = Obě stránky nabízejí příkazovou řádku do fotoaparátu, ale není to tentýž shell. Servisní shell mluví vlastním tovární servisním protokolem Sony, na který fotoaparát po přihlášení odpovídá sám; nic se přitom do fotoaparátu nezapisuje. Shell aktualizátoru existuje jen proto, že tento projekt zapisuje do fotoaparátu vlastní program: fotoaparát se restartuje do režimu aktualizace, přijme payload, jako by šlo o firmware, a ten pak odpovídá na příkazy uvedené níže. Otevřený může být vždy jen jeden z nich.
shell-primer-payload = „Payload“ (anglicky užitečný náklad) označuje vlastní obsah, který nějaká přenosová cesta doručuje. U shellu aktualizátoru je to malý program tohoto projektu: fotoaparát ho přijme stejnou cestou jako aktualizaci firmwaru, ale ponechá si ho jen do vypnutí. Servisní shell žádný payload nepotřebuje, protože fotoaparát odpovídá na jeho příkazy vlastním firmwarem. U souborů firmwaru znamená tento pojem totéž: vlastní obraz firmwaru v kontejneru souboru .dat.
shell-primer-when-service = Servisní shell je pro vše, co čte nebo mění uložená nastavení fotoaparátu, zatímco běží jeho vlastní firmware: čtení bloku HASP, uložení návratové kopie zálohovací oblasti, změna jednotlivých položek, kopírování souborů z fotoaparátu. Používá jej pod povrchem i stránka Úpravy; vše odtud lze udělat i zde ručně.
shell-primer-when-updater = Shell aktualizátoru je na řadě, když se k fotoaparátu nelze dostat jinak, nebo když potřebujete shell na běžícím systému místo jednotlivých příkazů. Jen tato cesta nabízí skutečný root shell a funguje i na fotoaparátu, který odmítá servisní režim. Stojí restart, jeden zápis a nastavení fotoaparátu, je to tedy druhá volba, nikoli první.
shell-primer-driver-heading = Ovladače: každý vyžaduje opak
shell-primer-driver = Servisní shell mluví surovým USB a ve Windows vyžaduje fotoaparát navázaný na WinUSB. Aplikace tento ovladač přiřadí sama cestou do servisního režimu a po skončení relače jej zase uvolní; žádný samostatný nástroj se toho neúčastní. Shell aktualizátoru jde přes USBSTOR, vlastní ovladač velkokapacitního úložiště ve Windows, a nefunguje, dokud je fotoaparát navázán na WinUSB — uvolnění proto patří k ukončení servisní relače. Přiřazení ovladače vyžaduje práva správce. Na Linuxu není potřeba měnit ovladač ani u jednoho, stačí přístup k uzlu zařízení.
shell-primer-risk-heading = Co se může pokazit
shell-primer-risk = Níže označené příkazy zapisují do trvalé paměti nastavení fotoaparátu. Přerušený zápis může způsobit, že se fotoaparát nespustí, a nic z toho nelze vzít zpět — právě proto existuje bk d: uložte si kopii zálohovací oblasti dřív, než cokoli změníte. Samotný payload aktualizátoru ve fotoaparátu nezůstane, vypnutím zmizí. Cesta přes režim aktualizace však resetuje všechna nastavení fotoaparátu.
shell-primer-commands-heading = Příkazy tohoto shellu
shell-primer-writes = Příkazy označené (WRITES) mění fotoaparát. Všechny ostatní jen čtou.
shell-cmd-hasp = Přečte blok HASP, chráněnou identitní oblast fotoaparátu.
shell-cmd-pull = Stáhne soubor z fotoaparátu. Bez místního názvu se zachová název na fotoaparátu.
shell-cmd-push = Zkopíruje místní soubor do fotoaparátu a nahradí, co tam je. (WRITES)
shell-cmd-rm = Smaže soubor ve fotoaparátu. (WRITES)
shell-cmd-rmem = Zapíše oblast paměti zařízení do místního souboru. Posun i délka jsou šestnáctkové.
shell-cmd-bk-r = Přečte jednu položku zálohovací oblasti a zobrazí ji jako šestnáctkový výpis.
shell-cmd-bk-d = Uloží celou zálohovací oblast do souboru. Udělejte to dřív, než cokoli změníte: je to jediná cesta zpět.
shell-cmd-bk-dump-txt = Uloží celou zálohovací oblast jako <file>.bin a čitelný výpis jako <file>.txt.
shell-cmd-bk-stat = Zobrazí stavový blok zálohy fotoaparátu včetně regionu.
shell-cmd-bk-w = Zapíše jednu položku zálohovací oblasti, zadanou jako šestnáctkové bajty oddělené mezerami. (WRITES)
shell-cmd-bk-s-sub = Uloží zálohovací oblast jednoho podsystému do flash paměti, aby změna přežila restart. (WRITES)
shell-cmd-bk-s = Uloží zálohovací oblast do flash paměti, aby změna přežila restart. (WRITES)
shell-cmd-bk-lock = Nastaví nebo zruší ochranu proti zápisu chráněných položek zálohy; některá nastavení se při zapnuté ochraně změnit nedají. (WRITES)
shell-cmd-term = Zapne nebo vypne ladicí terminál fotoaparátu. (WRITES)
shell-cmd-info = Přečte stavový blok zálohy, který říká, zda je oblast vůbec zapisovatelná.
shell-cmd-shell = Otevře root shell na fotoaparátu. Potřebuje terminál, takže je dostupný jen v konzoli; shell tcp <port> jej místo toho nabídne na 127.0.0.1 jinému klientovi. (WRITES)
shell-cmd-help = Vypíše příkazy, které tento shell přijímá.
shell-cmd-exit = Ukončí relaci a opustí shell.
shell-primer-diagnostics-heading = Diagnostika a oprava
shell-cmd-bk-trace = Zaznamená do souboru, paket po paketu, jak fotoaparát posílá celou zálohovací oblast přes USB. Slouží ke zkoumání velkých záloh; může fotoaparát nechat zaseknutý uprostřed odpovědi. Pak odpojte kabel a fotoaparát vypněte a zapněte, v nouzi vyjměte baterii.
shell-cmd-bk-rewrite = Přečte zálohovací oblast, zapíše ji beze změny zpět a přečte ji znovu. Ukáže, zda fotoaparát přijme celý obraz, aniž by se změnilo jakékoli nastavení. (WRITES)
shell-cmd-bk-restore-attrs = Vrátí atributy všech položek zálohy, například ochranu proti zápisu, na stav z kopie uložené pomocí bk d nebo bk dump-txt. Hodnoty zůstanou beze změny. Používejte jen kopii ze stejného fotoaparátu. (WRITES)
shell-primer-diagnostics-env = Tři proměnné prostředí působí na konzoli i na grafické rozhraní, pokud jsou nastaveny před spuštěním, například v PowerShellu pomocí $env:PMCA_TWEAKS_DIRECT = "1". PMCA_TWEAKS_DIRECT přiměje tweaky zapisovat jednotlivé hodnoty, i když je záloha menší než 1 MB. PMCA_TWEAKS_IMAGE je přiměje zapsat celý obraz, i když je větší než 1 MB; jsou-li nastaveny obě, platí PMCA_TWEAKS_DIRECT. PMCA_TLS_DUMP=<soubor> uloží při instalaci aplikace první zprávu TLS fotoaparátu (ClientHello), aby šlo prozkoumat neúspěšné navázání spojení.

# Instalace aplikace
install-switching = Přepíná se do režimu instalace aplikací…
install-disabled = Fotoaparát odmítl přejít do režimu instalace aplikací. Obvykle je příčinou nastavení „Povolit instalátor aplikací přes USB“: musí být zapnuté v části „Úpravy“.
install-stays-app-mode = Fotoaparát zůstane v režimu instalace aplikací, ve kterém lze instalovat další aplikace. Pro všechny ostatní funkce odpojte kabel USB a fotoaparát vypněte a znovu zapněte.
err-usb-app-install-mode = Fotoaparát je stále v režimu instalace aplikací a v něm přijímá pouze instalace aplikací. Odpojte kabel USB a poté fotoaparát vypněte a znovu zapněte — při restartu s připojeným kabelem se vrátí přímo do tohoto režimu.
install-switch-wait = Čeká se na opětovné připojení fotoaparátu v režimu instalace aplikací…
install-switch-timeout = Vypršel časový limit čekání na přechod fotoaparátu do režimu instalace aplikací. Jakmile se znovu připojí, spusťte příkaz znovu.
install-bad-mode = Fotoaparát nelze v tomto režimu použít. Přepněte jej do režimu MTP nebo velkokapacitního úložiště.
install-starting = Spouští se instalace…
install-error = Chyba komunikace při instalaci
install-server-failed = Místní instalační server nepřijal připojení fotoaparátu:
install-done = Úloha byla úspěšně dokončena.

# Kontrola souboru APK
apk-package = Balíček:
apk-unreadable = Upozornění: soubor .apk se nepodařilo analyzovat.
apk-incompatible = Upozornění: tato aplikace vyžaduje minimální úroveň SDK, kterou fotoaparát nemusí poskytovat.
apk-unsigned = Upozornění: soubor .apk neobsahuje žádný podpis.

# Komunitní obchod s aplikacemi
store-loading = Načítá se seznam aplikací…
store-resolving = Zjišťuje se nejnovější vydání…
store-downloading = Stahuje se
store-unknown-app = Neznámá aplikace
store-no-release = Pro tuto aplikaci není k dispozici žádné vydání ke stažení
store-not-installable = žádné vydání
msg-no-apps = Nebyly nalezeny žádné aplikace.
store-heading = Komunitní obchod s aplikacemi
store-discontinued = Sony ukončilo provoz oficiálního obchodu PlayMemories Camera Apps; jeho servery již neodpovídají, takže z něj nelze aplikace kupovat ani stahovat.
store-use-own-apk = Vlastní soubor .apk můžete nainstalovat i nadále: vyberte jej níže nebo jej přetáhněte do tohoto okna. Komunitní obchod s aplikacemi níže rovněž stále funguje.
btn-store-sync = Ověřit fotoaparát
btn-install-selected = Instalovat vybrané
btn-store-stop = Zastavit po této aplikaci
btn-store-stopping = Zastaví se po této aplikaci…
btn-store-retry = Zopakovat neúspěšné
btn-continue = Pokračovat
btn-reinstall = Nainstalovat znovu
btn-skip = Přeskočit
btn-confirm-updater = Restartovat a zapsat
btn-confirm-tweaks = Zapsat úpravy
btn-confirm-languages = Zapnout jazyky
btn-confirm-wifi = Nahradit přístupové body
btn-confirm-stream = Přepsat nastavení
btn-confirm-show-shells = Zobrazit shelly
btn-confirm-discard = Zahodit překlad
store-select-all = Vybrat všechny aplikace
store-col-app = Aplikace
store-col-version = Verze
store-col-author = Autor
store-col-camera = Verze ve fotoaparátu
store-col-status = Stav
store-camera-absent = nenainstalováno
store-version-unknown = neznámá
store-batch-query = Načítají se nainstalované aplikace z fotoaparátu…
store-sync-done = Fotoaparát ověřen. Nainstalované aplikace ze seznamu: { $count }
store-batch-downloading = Stahuje se aplikace { $current } z { $total }…
store-batch-installing = Aplikace { $current } z { $total }: instaluje se { $name }…
store-reinstall-confirm = Aplikace { $name } je ve fotoaparátu již nainstalována. Nainstalovaná verze: { $installed }. Verze v obchodě s aplikacemi: { $store }. Chcete ji nainstalovat znovu?
store-state-queued = čeká
store-state-installed = nainstalováno
store-state-failed = selhalo
store-state-skipped = přeskočeno
store-state-not-run = neprovedeno
store-batch-summary = Nainstalováno: { $done } · selhalo: { $failed } · přeskočeno: { $skipped } · neprovedeno: { $notrun }

# Servisní režim (senser)
usb-mode-advice = Pro tweaky, zálohy a servisní shell nastavte režim připojení USB fotoaparátu na „Auto“, ne napevno na hromadné ukládání. Když se fotoaparát po relaci zeptá na režim, zvolte „Hromadné ukládání (MSC)“. Při pevně nastaveném hromadném ukládání fotoaparát další přechod do servisního režimu často odmítne.
service-switch-refused = Fotoaparát nepřešel do servisního režimu a znovu se nepřihlásil k USB; na displeji pak obvykle stojí „Zkontrolujte připojené zařízení“. Odpojte USB kabel, znovu jej připojte a zkuste to znovu.
service-switching = Přepíná se do servisního režimu…
service-switch-wait = Čeká se na opětovné připojení fotoaparátu v servisním režimu…
service-switch-timeout = Fotoaparát se v servisním režimu neobjevil. Hlásí se jako samostatné zařízení USB (ID produktu 0336), kterému je nutné přiřadit vlastní ovladač WinUSB; ve správci zařízení zkontrolujte zařízení Sony bez ovladače. Vypnutím a zapnutím fotoaparátu jej vrátíte do běžného režimu.
service-authenticating = Probíhá ověřování…
# Zobrazí se, když je fotoaparát stále v servisním režimu po předchozím běhu,
# a když se to nepodařilo vrátit zpět.
service-stranded-reset = Fotoaparát je stále v servisním režimu z předchozího běhu. Nejdříve se ukončí ona relace…
service-stranded-failed = Fotoaparát uvízl v servisním režimu z předchozího běhu a nereagoval na žádost o jeho opuštění. Vypněte fotoaparát a znovu jej zapněte, poté to zkuste znovu.
service-bad-mode = Fotoaparát nelze v tomto režimu použít. Přepněte jej do režimu velkokapacitního úložiště.
service-driver-required = Přímý přístup k USB byl odepřen a ovladač WinUSB se nepodařilo přiřadit. Přiřazení ovladače vyžaduje práva správce.
service-warning = Varování: servisní režim umožňuje přímý přístup k paměti, souborovému systému a kalibračním datům fotoaparátu. Zápisové příkazy mohou fotoaparát trvale poškodit.
service-shell-ready = Servisní shell je připraven. Příkazy zobrazíte pomocí „help“, ukončíte pomocí „exit“.
service-shell-done = Relace servisního shellu byla ukončena.
# Hlášeno při spuštění, pokud předchozí běh skončil dříve, než stačil uvolnit
# ovladač, který přiřadil.
driver-swept = Bylo odstraněno { $count } přiřazení ovladače, které zbylo po předchozím běhu.
# Hlásí se při startu, pokud týz přerušený běh navíc nechal fotoaparát
# v servisním režimu, kde by další relaci odmítl.
driver-swept-camera = Fotoaparát, který zůstal po předchozím běhu v servisním režimu, byl vrácen do normálního režimu.
driver-release-replug = Windows při uvolňování ovladače čeká na fotoaparát. Odpojte USB kabel a znovu jej připojte, aby se úklid mohl dokončit.
# Výsledky příkazu `driver`.
driver-bound = { $id } nyní používá WinUSB (balíček ovladače { $package }).
driver-released = Bylo uvolněno { $count } přiřazení ovladače; fotoaparát opět používá ovladače dodávané se systémem Windows.
driver-needs-elevation = Přiřazení ovladače vyžaduje práva správce. Spusťte tento příkaz v konzoli spuštěné jako správce.
driver-leftover-needs-elevation = Dřívější běh přiřadil fotoaparátu ovladač, který ještě nebyl uvolněn. Dokud existuje, fotoaparát žádá o kontrolu připojeného zařízení a jeho hromadné ukládání nefunguje. Uvolnění vyžaduje práva správce; potvrďte prosím dotaz systému Windows.
driver-binding = Fotoaparátu se přiřazuje ovladač WinUSB…
driver-bind-wait = Čeká se, až se fotoaparát znovu připojí s novým ovladačem…
driver-bind-timeout = Fotoaparát se po přiřazení ovladače znovu neobjevil. Odpojte jej, znovu připojte a zkuste to znovu.
driver-bind-failed = Ovladač WinUSB se nepodařilo přiřadit: { $detail }
# Odpovědi shellu. Pocházejí z jádra aplikace, aby konzole i okno používaly
# stejná slova.
service-usage = Použití: { $usage }
service-bad-hex = Není šestnáctková hodnota: { $value }
service-unknown-command = Neznámý příkaz: { $command }. Seznam zobrazíte příkazem 'help'.
service-unterminated-quote = Řádek končí uvnitř argumentu v uvozovkách.
service-help = Přehled příkazů:
service-read-bytes = Přečteno { $count } bajtů.
service-pulled = { $remote } zkopírováno z fotoaparátu do { $local }.
service-pushed = { $local } zkopírováno do { $remote } ve fotoaparátu.
service-removed = { $remote } smazáno z fotoaparátu.
service-dumped = { $size } bajtů od { $offset } uloženo do { $local }.
service-backup-saved = Obraz zálohy ({ $count } bajtů) uložen do { $local }.
service-backup-text-saved = Obraz zálohy ({ $count } bajtů) uložen do { $bin } a výpis do { $txt }.
service-backup-status = Stav zálohy, region: { $region }
service-backup-rewritten = Obraz ({ $count } bajtů) byl beze změny zapsán zpět a znovu načten; trvale uložen nebyl.
service-backup-attrs-restored = Atributy { $count } položek byly vráceny podle { $local }.
service-backup-written = Položka zálohy { $id } zapsána.
service-backup-persisted = Záloha podsystému { $id } byla zapsána do paměti flash.
service-terminal-on = Ladící terminál je nyní zapnutý.
service-terminal-off = Ladící terminál je nyní vypnutý.
btn-service-terminal-open = Otevřít sériovou konzoli
btn-service-terminal-close = Zavřít sériovou konzoli
service-terminal-heading = Sériová konzole
service-terminal-hint = U fotoaparátů s Linuxem je ladicí terminál rootovský shell přes USB. Otevřete jej a zadávejte příkazy jako v terminálu.
service-terminal-label = Konzole
service-terminal-interactive = Sériová konzole otevřena. Zadávejte příkazy; příkazem „exit“ se vrátíte do servisního shellu.

# Servisní stránka bez rozšířeného režimu: jak se k shellu dostat z konzole a
# co je třeba připravit.
service-console-heading = Spustit servisní shell v konzoli
service-console-hint = Otevřete příkazový řádek ve složce s pmca-console a spusťte tento příkaz:
service-driver-heading = Než začnete
service-driver-hint = Servisní režim vyžaduje přímý přístup k USB, který Windows přes ovladač velkokapacitního úložiště neposkytuje. Aplikace přiřadí ovladač WinUSB sama — dvakrát, jednou když je fotoaparát v režimu velkokapacitního úložiště a podruhé poté, co se znovu objeví v servisním režimu pod druhým identifikátorem produktu — a po skončení relače obě přiřazení uvolní. Přiřazení ovladače vyžaduje práva správce, spusťte tedy konzoli jako správce; žádný další nástroj není potřeba.
driver-note-linux = Zde není třeba nic instalovat: libusb se k fotoaparátu dostane přímo, aniž by vytlačoval ovladač jádra. Potřebné je jen oprávnění zařízení otevřít a prostředí, které je právě nedrží. Přidejte pravidlo udev pro identifikátor výrobce Sony 054c a před použitím režimu velkokapacitního úložiště odpojte ovladač usb-storage. Skript scripts/linux-prepare.sh udělá obojí a navíc zastaví sledování svazků gvfs.
driver-note-macos = Pro přímý přístup přes USB není třeba nic instalovat. Nastavte připojení USB fotoaparátu na MTP. V režimu MTP program ukončí službu fotoaparátů macOS ptpcamerad, pokud fotoaparát drží; systém ji podle potřeby znovu spustí. Úpravy, servisní shell a analýza funkcí probíhají přes připojení velkokapacitního úložiště, které macOS předá jen programu s právy správce: úpravy si vyžádají vaše heslo, zbylé dvě potřebují příkazovou řádku se sudo. Tato platforma je ze tří nejméně otestovaná; počítejte spíše s tím, že problém nahlásíte, než že ho najdete již popsaný.
service-advanced-hint = Shell lze zobrazit i zde, v Nastavení, Rozšířený režim.
btn-copy-command = Kopírovat příkaz
msg-command-copied = Příkaz byl zkopírován do schránky.

# Servisní stránka se zapnutým rozšířeným režimem.
btn-service-open = Připojit
btn-service-close = Odpojit
btn-service-run = Spustit
btn-service-browse = Vložit cestu k souboru…
shell-path-quote = Tuto cestu nelze vložit: obsahuje uvozovku, kterou příkazový řádek používá k vymezení začátku a konce argumentu. Přejmenujte soubor nebo jej přesuňte do cesty bez tohoto znaku.
service-command-label = Příkaz
service-not-connected = Nepřipojeno.

# Řetězce používané pouze v grafickém rozhraní
install-apk-path = Cesta k místnímu souboru .apk (volitelné)
install-drop-hint = Sem přetáhněte soubor .apk

# Popisy úprav
tweak-rec-limit = Videozáznam bez časového omezení
tweak-rec-limit-4k = Videozáznam ve 4K bez časového omezení
tweak-language = Zobrazit všechny jazyky nabídky
tweak-pal-ntsc = Povolit přepínač PAL / NTSC a upozornění
tweak-pal-ntsc-mode = Videosystém (PAL / NTSC)
tweak-pal-ntsc-pal = PAL
tweak-pal-ntsc-ntsc = NTSC
tweak-pal-ntsc-note = Po změně videosystému fotoaparát vyžaduje naformátování paměťové karty. Nejprve vše zazálohujte — fotografie, videa, LUT. Nejjednodušší je kartu před přepnutím vyjmout.
tweaks-set-malformed = Očekáváno --set ID=HODNOTA, obdrženo: { $arg }
tweaks-set-unknown-value = Neznámá hodnota „{ $value }“. Očekávána jedna z: { $options }
tweaks-set-not-a-choice = { $id } je přepínač, ne volba. Použijte --enable nebo --disable.
tweaks-needs-set = { $id } je volba, ne přepínač. Použijte --set { $id }=HODNOTA s jednou z: { $options }
tweak-usb-installer = Povolit instalátor aplikací přes USB

# Postup při úpravách
tweaks-warning = Varování: použití úprav přepisuje zálohovací paměť fotoaparátu. Přerušený zápis může fotoaparát trvale poškodit.
tweaks-reading = Načítá se nastavení fotoaparátu…
tweaks-writing = Zapisuje se nastavení fotoaparátu…
tweaks-done = Nastavení fotoaparátu bylo aktualizováno.
tweaks-write-failed = Fotoaparát neuložil nové nastavení tak, jak bylo odesláno. Část se přesto mohla zapsat: načtěte nastavení znovu a v případě potřeby je vraťte zpět. Bezpečnou cestou zpět je záloha pořízená před změnou.
tweaks-confirm-required = Zápis bez potvrzení byl odmítnut. Po přečtení výše uvedeného varování spusťte příkaz znovu s --confirm.
tweaks-state-enabled = zapnuto
tweaks-state-disabled = vypnuto
tweaks-full-read-failed = Fotoaparát nedodal úplnou zálohu ({ $detail }). Poté často přestane reagovat: vypněte ho, na několik sekund vyjměte baterii, vložte ji zpět a znovu ho připojte.
tweaks-direct-mode = Záloha tohoto fotoaparátu je větší než 1 MB, proto se nastavení čtou a zapisují jednotlivě. Před každou změnou se staré hodnoty uloží a lze je vrátit tlačítkem „Vrátit změny“.
tweaks-direct-forced = PMCA_TWEAKS_DIRECT vynucuje zápis po jednotlivých hodnotách, proto se nastavení čtou a zapisují jednotlivě. Před každou změnou se staré hodnoty uloží a lze je vrátit tlačítkem „Vrátit změny“.
tweaks-direct-refused = Fotoaparát změnu odmítl ({ $detail }). Již zapsané hodnoty byly vráceny; nic se nezměnilo. Některé modely, zejména japonské, toto nastavení povolují jen přes úplnou zálohu.
tweaks-direct-revert-failed = Fotoaparát změnu odmítl ({ $detail }) a ne všechny již zapsané hodnoty se podařilo vrátit. Staré hodnoty jsou uloženy: zkuste „Vrátit změny“.
tweaks-direct-restart = Nastavení fotoaparátu aktualizováno. Aby se změna projevila, fotoaparát vypněte a znovu zapněte.
tweaks-rollback-unsaved = Aktuální hodnoty nebylo možné uložit pro vrácení; nic se nezměnilo.
tweaks-image-saved = Obraz zálohy z doby před změnou byl uložen: { $path }
tweaks-whole-image-hint = Chcete-li změnu zapsat místo toho přes celý obraz zálohy, spusťte stejný příkaz znovu s --whole-image. Obraz z doby před změnou se nejprve uloží.
tweaks-whole-image-confirm = Zapsat změnu přes celý obraz zálohy? Fotoaparát ji jako jednotlivou hodnotu odmítl. Celý obraz se zapíše zpět i se změnou; obraz z doby před změnou se nejprve uloží v tomto počítači.
btn-whole-image = Zapsat celou zálohu
tweaks-undo-none = Pro tento fotoaparát nejsou uloženy žádné hodnoty.
tweaks-undo-confirm = Zapsat uložené hodnoty zpět do fotoaparátu? Tím se vrátí všechny změny zapsané jednotlivě od jejich uložení.
tweaks-undo-done = Uložené hodnoty byly zapsány zpět. Fotoaparát vypněte a znovu zapněte.
tweaks-undo-available = Hodnoty z doby před první změnou jsou uloženy a lze je zapsat zpět.
btn-undo-tweaks = Vrátit změny
tweaks-identity-heading = Identita fotoaparátu (jen ke čtení)
tweaks-caution-languages = Dva jazyky — pravděpodobně čínštinu (zjednodušenou) a čínštinu (tradiční) — tento fotoaparát neumí zobrazit. Při přechodu na ně se nabídka jazyků restartuje, proto zůstávají pro jistotu vypnuté.
tweaks-caution-accept = Aktivovat tyto dva jazyky i tak (na vlastní odpovědnost)
tweaks-caution-confirm = Tyto dva jazyky mohou způsobit restart nabídky jazyků ve fotoaparátu. Nic se nepoškodí a změnu lze vrátit, ale možná budete muset nastavení hledat znovu od začátku. Přesto aktivovat?

# Nastavení
settings-language = Jazyk
settings-theme = Motiv
settings-theme-system = Systémový
settings-theme-light = Světlý
settings-theme-dark = Tmavý

# Rozšířený režim. Ochrana před omylem, nikoli před útočníkem: vše, co zobrazí,
# je stejně dostupné z konzole.
settings-advanced-heading = Rozšířený režim
settings-advanced-hint = Rozšířený režim zobrazí servisní shell a shell updateru na stejnojmenných stránkách. Servisní režim dává přímý přístup k paměti fotoaparátu, k jeho souborovému systému a ke kalibračním datům; špatně napsaný příkaz může fotoaparát vyřadit z provozu. Nechte jej vypnutý, dokud nevíte, co budete psát.
settings-advanced-toggle = Zobrazit servisní shell a shell updateru
settings-advanced-on = Rozšířený režim je zapnutý. Stránky Servisní shell a Shell updateru nyní nabízejí samotné shelly.
settings-advanced-off = Rozšířený režim je vypnutý. Stránky Servisní shell a Shell updateru místo toho vysvětlují cestu přes konzoli.
settings-advanced-confirm = Servisní shell a shell updateru mohou fotoaparát trvale poškodit. Zobrazit oba shelly?

# Úprava překladu. Jazykové soubory lze stáhnout, změnit v libovolném textovém
# editoru a znovu nahrát; nahraný soubor je před použitím zkontrolován.
settings-catalog-heading = Upravit překlad
settings-catalog-hint = Stáhněte jazykový soubor, upravte jej v libovolném textovém editoru a znovu jej nahrajte. Překládá se pouze text napravo od rovnítka a vynechané záznamy si ponechají své vestavěné znění. Volba jazyka na tomto místě nemění jazyk aplikace.
settings-catalog-locale = Jazykový soubor
settings-catalog-builtin = Používá se vestavěný překlad.
settings-catalog-custom = Používá se váš vlastní překlad:
btn-catalog-export = Stáhnout…
btn-catalog-import = Nahrát…
btn-catalog-reset = Obnovit vestavěný
catalog-exported = Zapsáno do
catalog-imported = Překlad byl použit.
catalog-reset-done = Vestavěný překlad byl obnoven.
catalog-reset-none = Žádný vlastní překlad k obnovení neexistoval.
catalog-reset-confirm = Zahodit vlastní překlad pro tento jazyk?
catalog-rejected = Soubor nebyl použit. Opravte tyto řádky:
catalog-count-accepted = Použité záznamy
catalog-count-missing = Záznamy převzaté z vestavěného překladu
catalog-count-untranslated = Záznamy shodné s anglickým originálem
catalog-line = Řádek
catalog-problem-syntax = není záznam ve tvaru „id = text“
catalog-problem-continuation = pokračuje na následujícím řádku, což není podporováno
catalog-problem-duplicate = je definován více než jednou
catalog-problem-unknown-key = není záznam, který tato aplikace používá
catalog-problem-empty = nemá žádný text
catalog-problem-placeholder = zástupný symbol byl změněn, očekáváno:

# Původ, poděkování a licence. Zobrazuje se na stránce s licencí a příkazem
# `pmca-console license`. Samotný text licence se nepřekládá: je to právní
# dokument a uvádí se doslovně.
about-thanks-heading = Poděkování
about-thanks-body-1 = Tento nástroj existuje díky ma1co, který zjistil, jak fotoaparáty Sony komunikují přes USB, a v roce 2015 vydal Sony-PMCA-RE, a díky všem, kdo k němu a k projektům OpenMemories v následujících letech přispěli.
about-thanks-body-2 = Rozpoznávání fotoaparátu, protokol externích příkazů, postup aktualizace firmwaru i servisní režim jsou zcela jejich prací a tento port na ni navazuje. Děkujeme. Na tomto základě přináší PMCA-RE RELOADED mnoho nových funkcí, které doplnil masterflai. Především jde o nyní integrované automatické přepínání ovladače zařízení ve Windows: fotoaparátu již není třeba ručně přiřazovat jiný ovladač pomocí samostatného nástroje, takže je celý postup pohodlnější, rychlejší a méně riskantní.
about-contributors-heading = Přispěvatelé
about-contributors-body = Kromě ma1co přispěli do Sony-PMCA-RE tito lidé, případně ve vlastních kopiích zveřejnili změny, které byly převzaty do tohoto portu:
about-purpose-heading = Proč tento port vznikl
about-purpose-body-1 = Původní aplikace se již nevyvíjí. Byla napsaná pro Python 2, jehož podpora skončila v roce 2020, a její části se opírají o služby, které Sony mezitím ukončila, takže na současných systémech již neběží bez omezení.
about-purpose-body-2 = Účelem tohoto portu je zachování, nikoli nahrazení, a stejně tak bezproblémové používání dnes: udržet tyto fotoaparáty použitelné a zajistit, aby program bez potíží fungoval na současných počítačích i s novějšími modely fotoaparátů. Vše, co uměl originál, by měl umět i tento port.
about-origin-heading = Jak vznikl
about-origin-body-1 = Převod z Pythonu do jazyka Rust byl proveden s pomocí AI.
about-origin-body-2 = Stejně jasně stojí za to říct, co to neznamená: chování je pokryto automatizovanou sadou testů a plánem hardwarových testů. Důkladně byla ověřena především verze pro Windows, protože ovladače zařízení z ní dělají nejsložitější platformu; macOS a Linux naopak nebylo možné otestovat v plném rozsahu. Považujte to prosím za experimentální software.
about-license-heading = Licence
about-license-body = Tento port je vydán pod licencí MIT, tedy pod stejnou licencí jako původní projekt. Původní oznámení o autorských právech zůstává beze změny, jak tato licence vyžaduje.
about-license-full = Text licence
about-third-party-heading = Licence použitých knihoven
about-third-party-body = Tento program je staticky slinkovaný: kód každé použité knihovny je uvnitř spustitelného souboru a licence těchto knihoven vyžadují, aby s ním putovala i jejich autorskoprávní oznámení. Níže jsou uvedena v plném znění, jeden záznam na balíček.
about-third-party-toggle = Zobrazit všechna oznámení třetích stran
about-privacy-heading = Data
about-privacy-body-1 = Tento program běží na vašem vlastním počítači. Neodesílá žádná data o používání, není v něm žádná telemetrie ani povinný účet. Nic z toho, co zde děláte, se nikam nehlásí.
about-privacy-body-2 = Dvě funkce se však samy připojují k síti a protistrana se přitom dozví vaši IP adresu: asistenční data GPS se stahují od Sony a seznam aplikací z GitHubu. Bez těchto funkcí zůstává program zcela offline.
about-privacy-body-3 = To, co se z fotoaparátu vyčte — včetně sériového čísla a uložených hesel k Wi-Fi — váš počítač neopustí. Pokud to zapíšete do souboru, dejte pozor, komu takový soubor předáte.
about-privacy-body-4 = Za tímto programem nestojí žádná služba a k jeho autorům se nedostávají žádná data, není zde tedy nic, oč by šlo žádat nebo co nechat vymazat. Jak dvě jmenované služby naloží s adresami, které vidí, se řídí jejich vlastními zásadami ochrany soukromí, nikoli těmito.
about-legal-heading = Právní upozornění
about-legal-trademark = Toto je nezávislý projekt. Nevytvořila jej společnost Sony, neschválila jej ani s ním nijak nesouvisí. »Sony«, »PlayMemories« a názvy modelů fotoaparátů jsou ochranné známky společnosti Sony Group Corporation a jsou zde uvedeny pouze proto, aby bylo zřejmé, se kterými fotoaparáty tento nástroj komunikuje.
about-legal-own-device = Tento program je určen k použití s fotoaparáty, které vlastníte.
about-legal-warranty = To, co tento nástroj dělá, leží mimo záměr společnosti Sony pro tyto fotoaparáty a může ukončit vaši záruku. Software sám je poskytován bez jakékoli záruky a používáte jej na vlastní riziko. Zda a nakolik je zpětná analýza vlastního zařízení přípustná, se liší podle země.
about-links-heading = Projekty
about-link-original = Sony-PMCA-RE — původní projekt
about-link-fwtool = fwtool.py — nástroje pro firmware, z nichž vychází tvorba aktualizačních obrazů
about-link-tweak = OpenMemories: Tweak — aplikace doporučená pro fotoaparáty, které ji podporují
about-link-applist = OpenMemories: App list — další aplikace
about-link-platform = OpenMemories: Platform — přibalené tělo aktualizátoru
about-link-devices = Podporované modely fotoaparátů
about-link-nexhack = Nex-Hack — komunitní výzkum, na kterém tento projekt staví
about-link-zve10 = zv-e10-re — výzkum rozhraní a paměti nastavení ZV-E10
about-version = Verze
btn-copy-url = Kopírovat adresu
msg-url-copied = Adresa byla zkopírována do schránky.

# Bezpečnostní upozornění
notice-experimental = Toto je experimentální software. Používáte jej na vlastní nebezpečí.

# Nastavení sítě (asistenční data GPS, přístupové body a služby pro živé vysílání)
gps-downloading = Stahují se asistenční data GPS…
gps-writing = Asistenční data GPS se zapisují do fotoaparátu…
gps-bad-size = Asistenční data GPS mají neočekávanou velikost a nebyla zapsána.
wifi-read = Z fotoaparátu se načítají přístupové body pro živé vysílání…
wifi-written = Přístupové body pro živé vysílání byly zapsány.
stream-read = Z fotoaparátu se načítá nastavení živého vysílání…
stream-written = Nastavení živého vysílání bylo zapsáno.
net-secret-warning = Varování: tento soubor obsahuje hesla Wi-Fi a přístupové tokeny v čitelné podobě.
wifi-secret-warning = Fotoaparát tato hesla ukládá nezašifrovaná a zde se zobrazují přesně tak, jak jsou uložena. Zůstávají na tomto počítači; uložíte-li je do souboru, dejte pozor, komu jej předáte.
gps-heading = Asistenční data GPS
gps-hint = Přetáhněte sem soubor, zadejte cestu, nebo pole ponechte prázdné a aktuální data se stáhnou od Sony.
btn-gps-devices = Kompatibilní zařízení
btn-gps-devices-hide = Skrýt kompatibilní zařízení
gps-devices-note = Tyto modely mají vestavěný modul GPS. U mnoha z nich ho má jen varianta, jejíž název končí písmenem V; varianty bez V GPS nemají.
gps-devices-compact = Kompaktní fotoaparáty (DSC)
gps-devices-slt = Fotoaparáty SLT (bajonet A)
gps-devices-action = Akční kamery
gps-devices-camcorder = Videokamery (Handycam)
gps-col-variant = Varianta s GPS
gps-col-arch = Architektura
gps-col-module = Modul GPS
gps-col-apps = Podpora aplikací
gps-yes = ano
gps-no = ne
gps-unknown = neznámé
gps-devices-untested = GPS k dispozici, podpora netestována
gps-devices-untested-note = Tyto modely mají vestavěný modul GPS, ale nejsou v seznamu zařízení OpenMemories. Zda s nimi tato aplikace funguje, nebylo testováno, a proto je jejich architektura i podpora aplikací neznámá.
wifi-heading = Přístupové body pro živé vysílání
wifi-hint = Jde o přístupové body používané funkcí živého vysílání fotoaparátu, nikoli o vlastní nastavení Wi-Fi fotoaparátu. Tělo bez funkce vysílání tento požadavek odmítne.
wifi-multi = Seznam více přístupových bodů
wifi-none = Nejsou uloženy žádné přístupové body.
wifi-writing = Přístupové body pro živé vysílání se zapisují do fotoaparátu…
wifi-confirm = Tím se nahradí všechny přístupové body uložené ve fotoaparátu. Pokračovat?
wifi-empty-refused = Zápis prázdného seznamu byl odmítnut, protože by smazal všechny uložené přístupové body.
stream-heading = Živé vysílání
stream-writing = Nastavení živého vysílání se zapisuje do fotoaparátu…
stream-confirm = Tím se přepíše nastavení živého vysílání ve fotoaparátu. Pokračovat?
stream-invalid-json = Nastavení není platný formát JSON:

# Aktualizace firmwaru a oficiální obchod
firmware-warning = Varování: tímto se přepíše firmware fotoaparátu. Po spuštění aktualizaci nepřerušujte: neodpojujte kabel USB a fotoaparát nevypínejte. Používejte pouze soubor firmwaru určený přesně pro tento model fotoaparátu a dbejte na dostatečně nabitou baterii. Jinak se fotoaparát může stát trvale nepoužitelným.
firmware-untested = Neověřeno: zápis firmwaru byl přenesen, ale nikdy vyzkoušen na skutečném fotoaparátu. Nezdařený nebo přerušený zápis nelze vzít zpět, proto vše na této stránce považujte za nepotvrzené.
firmware-confirm-required = Zápis bez parametru --confirm byl odmítnut.
firmware-truncated = Soubor firmwaru je neúplný: payload FDAT přesahuje za konec souboru.
firmware-updating = Aktualizace firmwaru:
firmware-invalid = Soubor není platný obraz firmwaru Sony
firmware-dat-path = Soubor firmwaru (.dat)
firmware-drop-hint = Sem přetáhněte soubor firmwaru .dat
firmware-file-required = Nejprve vyberte soubor firmwaru.
firmware-check-ok = Fotoaparát tento obraz přijímá. Nic nebylo zapsáno.
firmware-cancelled = Zápis firmwaru byl zrušen. Fotoaparát nebyl změněn.
firmware-current-version = Nainstalovaná verze
firmware-new-version = Verze v tomto obrazu
firmware-models-hint = Fotoaparát je kompatibilní, pokud se k němu shell updateru dokáže připojit.
btn-firmware-check = Zkontrolovat obraz
btn-firmware-flash = Zapsat firmware
firmware-own-device = Potvrďte, že si jste těchto rizik vědomi a aktualizujete výhradně své vlastní zařízení.
btn-firmware-accept = Rizikům rozumím – zapsat

# Souborové nástroje: operace, které žádný fotoaparát nepotřebují.
tools-hint = Tyto operace pracují se soubory a fotoaparátu se nikdy nedotknou, takže tady nemůže nic poškodit. Jsou to tytéž operace, které pmca-console nabízí jako apk2spk, spk2apk, print-backup, print-usagelog, guess-firmware a xpd-checksum.
tools-operation = Operace
tools-apk2spk = Zabalit .apk do kontejneru .spk
tools-spk2apk = Rozbalit .spk zpět na .apk
tools-print-backup = Zobrazit obsah souboru Backup.bin
tools-print-usagelog = Zobrazit obsah souboru Usagelog.bin
tools-guess-firmware = Najít payload uvnitř firmwarového .dat
tools-xpd-checksum = Spočítat kontrolní součet CIC souboru
tools-input = Vstupní soubor
tools-file-required = Nejdřív vyberte soubory, které tato operace potřebuje.
tools-apk-invalid = Soubor se nepodařilo zabalit jako .spk
tools-spk-invalid = Soubor není platný balíček .spk
tools-backup-invalid = Soubor není platný obraz Backup.bin
tools-usagelog-invalid = Soubor není platný protokol používání
tools-reset-value = výchozí hodnota:
tools-fdat-found = Payload nalezen na offsetu { $offset }, velikost { $size }.
tools-checksum = Kontrolní součet
tools-written = Zapsáno do { $path }

# Odkazy na dokumentaci projektu
docs-compatibility = Kompatibilita fotoaparátů
docs-compatibility-hint = Původní projekt poskytl přehled ověřených fotoaparátů. Zahrnuje převážně starší modely a najdete jej na této adrese:
devices-note = Údaje z přehledu původního projektu. Číslo u podpory aplikací je verze aplikační platformy fotoaparátu.
devices-more-note = Následující fotoaparáty nebyly s tímto programem výslovně testovány. Zaškrtnutí v posledním sloupci znamená, že komunita hlásila jejich použití. Je-li navíc servisní shell označen ✖, použití bylo neúspěšné.
btn-more-models = Další zařízení
btn-more-models-hide = Skrýt další zařízení
check-updater-yes = Vaše zařízení { $model } je kompatibilní: podle seznamu zařízení se k němu shell updateru dokáže připojit.
check-updater-no = Vaše zařízení { $model } není kompatibilní: podle seznamu zařízení se k němu shell updateru nedokáže připojit.
check-updater-unknown = Kompatibilita vašeho zařízení { $model } je neznámá: není v žádném seznamu zařízení tohoto programu, takže není známo, zda se k němu shell updateru dokáže připojit.
check-apps-no = Vaše zařízení { $model } není kompatibilní: podle seznamu zařízení nepodporuje aplikace.
check-apps-untested = Kompatibilita vašeho zařízení { $model } je neznámá: je v seznamu zařízení, ale podpora aplikací není potvrzena.
check-apps-unknown = Kompatibilita vašeho zařízení { $model } je neznámá: není v žádném seznamu zařízení tohoto programu, takže není známo, zda podporuje aplikace.
check-gps-no = Vaše zařízení { $model } není kompatibilní: není v seznamu zařízení s přijímačem GPS.
check-gps-untested = Kompatibilita vašeho zařízení { $model } je neznámá: podle seznamu zařízení má přijímač GPS, ale přenos do tohoto modelu nebyl testován.
check-gps-unknown = Kompatibilita vašeho zařízení { $model } je neznámá: není v žádném seznamu zařízení tohoto programu, takže není známo, zda má přijímač GPS.
check-unidentified = Připojené zařízení se nepodařilo identifikovat, jeho kompatibilita je proto neznámá.
check-proceed = Chcete přesto pokračovat?
check-cancelled = Operace zrušena. Zařízení nebylo změněno.
devices-col-confirmed = Ověřeno
devices-group-ilc = Systémové fotoaparáty (bajonet E a A)
devices-group-remote = Dálková ovládání Live-View
btn-copy-link = Kopírovat odkaz
shutter-heading = Počítadlo závěrky
shutter-hint = Fotoaparát počítá každé spuštění závěrky a aktuální stav zapisuje do každé fotografie, kterou uloží (ARW, JPEG nebo HEIF). Vyberte naposledy pořízenou fotografii a přečtěte aktuální stav. Soubor se pouze čte, nikdy se nemění.
shutter-source = Umístění počítadla v souboru se liší podle modelu. Pozice pocházejí z veřejně zdokumentovaného výzkumu (ExifTool) a nejsou potvrzeny pro každý model.
shutter-drop-hint = Sem přetáhněte fotografii (ARW, JPEG, HEIF)
btn-shutter-read = Vyhodnotit fotografii
shutter-file-required = Nejprve vyberte fotografii.
shutter-file = Soubor
shutter-taken = Datum pořízení
shutter-count = Počet spuštění závěrky
shutter-type = Závěrka
shutter-type-mechanical = Mechanická
shutter-type-electronic = Elektronická
shutter-electronic-hint = Tato fotografie byla pořízena elektronickou závěrkou. U mnoha modelů se takové snímky nezapočítávají; hodnota pak ukazuje stav při posledním mechanickém spuštění.
shutter-not-sony = Fotografie nepochází z fotoaparátu Sony. Počítadlo závěrky z ní nelze přečíst.
shutter-unknown-model = Není známo, kam tento model fotoaparátu ukládá počítadlo závěrky v souboru, proto se nezobrazuje žádná hodnota.
shutter-not-recorded = Soubor neobsahuje čitelné počítadlo závěrky. Stává se to u fotografií uložených nebo exportovaných programem pro úpravu obrázků a u některých modelů u snímků pořízených elektronickou závěrkou. Vyberte původní soubor z paměťové karty.
shutter-model-mismatch = Fotografie pochází z jiného modelu fotoaparátu, než je připojený fotoaparát.
shutter-invalid = Soubor není čitelná fotografie
file-filter-photo = Fotografie (ARW, JPEG, HEIF)
usage-log-heading = Protokol používání
usage-log-hint = Fotoaparát vede vnitřní protokol v nezdokumentovaném formátu. Ukládá se doslovně; z fotoaparátu se pouze čte.
usage-log-written = Zapsáno do
btn-usage-log-info = O protokolu používání
btn-usage-log-info-hide = Skrýt vysvětlení
usage-log-info-what-heading = Co je tento protokol
usage-log-info-what = Sony protokol interně nazývá „KikiLog“. Fotoaparát ho zapisuje sám; tento program ho pouze čte. Žádný veřejný popis formátu není znám.
usage-log-info-read = Čtení nic nemění: několik stažení za sebou bylo bajtově shodných.
usage-log-info-flush-heading = Proč chybí nové záznamy
usage-log-info-flush = Nové záznamy se často neobjeví hned: fotoaparát je může uložit až při vypnutí, někdy rozložené do několika cyklů zapnutí a vypnutí. Fotoaparát vypněte a znovu zapněte, znovu stáhněte a opakujte, dokud nepřibývají nové záznamy.
usage-log-info-content-heading = Co z něj lze vyčíst
usage-log-info-shutter = Spuštění závěrky: některé události odpovídají vždy jednomu snímku. Jejich počet však není počítadlo závěrky: protokol má pevnou velikost a nemusí pokrývat celou dobu používání fotoaparátu. Celkový stav vede fotoaparát jinde a zapisuje ho do ukládaných fotografií; lze ho přečíst v části „Počítadlo závěrky“.
usage-log-info-clock = Časové údaje jsou jen tak přesné jako hodiny fotoaparátu. Po výměně baterie se hodiny mohou vrátit zpět a vytvářet záznamy s výchozím datem fotoaparátu.
usage-log-info-unknown = Žádný veřejný popis čísel událostí není znám. Program zobrazuje čísla bez výkladu.
usage-log-info-privacy-heading = Ochrana soukromí
usage-log-info-privacy = Soubor obsahuje sériová čísla vložených paměťových karet. Nesdílejte ani nezveřejňujte jej bezmyšlenkovitě.
btn-usage-log = Uložit protokol
usagelog-size = Oblast protokolu
usagelog-cursor = Pozice zápisu
usagelog-spare = Druhá oblast
usagelog-count = Záznamy
usagelog-events = Události podle čísla
usagelog-entries = Záznamy v pořadí
msg-link-copied = Odkaz byl zkopírován do schránky.

# Chybové zprávy z backendu, adresované stabilním kódem, který backend hlásí.
# Proměnnou část zprávy (cestu, stavový bajt, chybu zařízení) doplňuje
# rozhraní, takže tyto texty dávají smysl jako celé věty i bez ní.
# Záměrně zde není `err-operation`: tento kód označuje zprávu, kterou backend
# již přeložil a která se musí zobrazit doslovně.
err-usb-no-device = Nebyl nalezen žádný fotoaparát. Připojte fotoaparát a zapněte jej.
err-usb-camera-not-ready = Fotoaparát je připojen, ale není připraven: je vypnutý, nebo na displeji čeká na volbu režimu USB. Zapněte ho, nebo na fotoaparátu zvolte „Hromadné ukládání (MSC)“.
camera-choose-usb-mode = Fotoaparát čeká na volbu režimu USB. Na fotoaparátu zvolte „Hromadné ukládání (MSC)“.
err-usb-too-many-devices = Je připojen více než jeden fotoaparát. Odpojte všechny kromě toho, se kterým chcete pracovat.
err-usb-transfer = Přenos přes USB se nezdařil. Zkuste jiný kabel nebo port a nepoužívejte rozbočovače USB.
err-usb-permission = Program nesmí otevřít fotoaparát přes USB. V Linuxu se toto právo udělí jednorázově pravidlem udev pro identifikátor výrobce Sony; skript scripts/linux-prepare.sh ve zdrojovém stromu je nainstaluje. Poté fotoaparát odpojte a znovu připojte.
err-usb-busy = Fotoaparát používá jiný program, takže jej nešlo otevřít. Zavřete ostatní programy, které by k němu mohly přistupovat; v Linuxu jej může držet monitor médií prostředí, který zastaví skript scripts/linux-prepare.sh. V macOS přepněte připojení USB fotoaparátu na MTP: v režimu velkokapacitního úložiště jej drží diskový ovladač systému.
err-usb-needs-root = macOS předá připojení fotoaparátu jako velkokapacitního úložiště programu jen tehdy, když běží s právy správce. Spusťte program příkazové řádky přes sudo, například: sudo pmca-console tweaks. Informace o fotoaparátu lze číst i bez toho, pokud je připojení USB fotoaparátu nastaveno na MTP.
macos-admin-prompt = PMCA-RE RELOADED potřebuje práva správce, aby macOS odebral připojení fotoaparátu jako velkokapacitního úložiště. Karta mezitím zmizí z Finderu; pokud jsou na ní otevřené soubory, nejprve ji tam vysuňte.
macos-admin-declined = Práva správce nebyla udělena, fotoaparátu se nic nestalo. V macOS je úpravy potřebují.
macos-admin-failed = Program nemohl spustit svého pomocníka s právy správce.
err-usb-stall = Fotoaparát přerušil přenos přes USB.
err-usb-protocol = Fotoaparát odeslal neočekávanou odpověď.
err-usb-msc-status = Fotoaparát odmítl příkaz úložiště.
cli-error-prefix = chyba
err-usb-driver = Systém Windows přiřadil tomuto fotoaparátu ovladač, který neumožňuje přímý přístup. Tento režim vyžaduje ovladač WinUSB, který si aplikace umí přiřadit sama — ale jen s právy správce.
err-needs-elevation = Přiřazení nebo uvolnění ovladače vyžaduje práva správce a dotaz systému Windows byl odmítnut. Zkuste to znovu tlačítkem „Spustit pomocníka ovladačů“; na fotoaparátu není třeba nic měnit.
err-usb-no-mass-storage-volume = Fotoaparát je připojen v režimu velkokapacitního úložiště, ale nenabízí žádnou jednotku. Probuďte jej, potvrďte na jeho displeji režim USB, pokud se zeptá, a zkontrolujte, zda je vložena paměťová karta. Neměňte ovladač: tento režim potřebuje právě standardní ovladač systému Windows.
err-usb-msc-sense = Fotoaparát ohlásil chybu úložiště.
err-usb-ptp-response = Fotoaparát odmítl příkaz.
err-usb-unsupported = Tento model fotoaparátu tuto funkci nepodporuje.
err-usb-updater-status = Nástroj pro aktualizaci firmwaru ve fotoaparátu operaci odmítl.
err-usb-senser = Příkaz servisního režimu se nezdařil.
err-market-core = Datový soubor fotoaparátu se nepodařilo přečíst.
err-market-json = Přijatá data jsou poškozená.
err-market-missing-field = V odpovědi chybělo očekávané pole.
err-market-no-apk = K instalaci nebyla vybrána žádná aplikace.
err-market-http = Síťový požadavek se nezdařil.
err-market-io = Místní instalační server selhal.
err-market-tls = Šifrované připojení k fotoaparátu se nepodařilo navázat.
err-apk = Soubor aplikace se nepodařilo přečíst.
err-io = Soubor se nepodařilo přečíst nebo zapsat.
err-json = Fotoaparát odeslal poškozená data.
err-unknown-locale = Tento jazyk není k dispozici.
err-catalog-encoding = Soubor překladu není uložen jako text UTF-8. Uložte jej znovu v kódování UTF-8.
err-internal = Došlo k vnitřní chybě. Jde o chybu v aplikaci.

## Function analysis
nav-analysis = Analýza
analysis-title = Analýza funkcí
analysis-experimental = Experimentální. Tato funkce je nová a vyzkoušená jen na několika fotoaparátech.
analysis-own-hardware = Používejte ji jen na vlastním fotoaparátu. Už samotný servisní režim je zásah, se kterým Sony nepočítá; odpovědnost nesete vy.
analysis-explanation-toggle = Vysvětlení: účel, průběh, rizika
analysis-purpose-heading = K čemu slouží?
analysis-purpose = Analýza funkcí dokumentuje, ve které hodnotě své paměti nastavení (zálohy) fotoaparát ukládá jedno ze svých vlastních nastavení menu. Výsledek popisuje paměť nastavení tohoto fotoaparátu; lze jej porovnat s jinými fotoaparáty, ale nemění to, co fotoaparát nabízí.
analysis-how-heading = Jak probíhá?
analysis-how = Pro každou zadanou hodnotu nabídky program jednou přečte celou paměť nastavení, pokaždé ve vlastní krátké relaci servisního režimu. Mezi čteními odpojíte kabel a změníte nastavení v nabídce fotoaparátu; fotoaparát zůstává zapnutý. Nakonec se první hodnota přečte ještě jednou. Hodnoty, které sledují nabídku a na konci jsou zpět na počáteční hodnotě, jsou kandidáti. Hodnoty, které se mění samy (čítače, hodiny), se vyřadí.
analysis-requirements-heading = Předpoklady
analysis-requirements = Fotoaparát je připojen přes USB, jeho režim USB je nastaven na „Auto“ a po každé relaci na fotoaparátu zvolíte „Hromadné ukládání (MSC)“. Ve Windows potřebuje program práva správce, protože pro servisní režim krátce přiřazuje ovladač. Nabijte baterii; každá relace trvá asi minutu.
analysis-read-heading = Čtecí kroky
analysis-read = Snímky stavu pouze čtou. V nastavení fotoaparátu se nic nemění. Riziko je malé, ale ne nulové: už přepnutí do servisního režimu je zásah, se kterým Sony nepočítá.
analysis-write-heading = Zapisovací krok (test zápisu)
analysis-write = Jen pokud ho výslovně zapnete: program zapíše nalezenou hodnotu do fotoaparátu, vy po restartu v nabídce ověříte, zda působí, a poté se zapíše zpět původní hodnota. Zapisuje se jen hodnota, kterou fotoaparát sám uložil, a jen pokud byl nalezen právě jeden vhodný kandidát. Stará hodnota se předem zálohuje. Přesto tento krok fotoaparát mění; chyba ho může zanechat ve stavu, který opraví jen servis.
analysis-legal-heading = Právní informace
analysis-legal = Software je poskytován bez jakékoli záruky (licence MIT). Zásahy do fotoaparátu mohou ovlivnit záruku. Funkci používejte jen na fotoaparátech, které vám patří. Zpráva o analýze neobsahuje sériové číslo ani nic z paměti nastavení kromě nalezených kandidátů; nikdy se neodesílá automaticky.
analysis-start-heading = Nová analýza
analysis-setting-label = Nastavení nabídky, jak ho fotoaparát nazývá
analysis-values-label = Hodnoty nabídky, oddělené čárkami
analysis-values-hint = První hodnota je ta, která je nastavena teď. Dvě až osm hodnot; každá stojí jednu relaci servisního režimu.
analysis-write-option = Poté provést test zápisu (mění fotoaparát)
analysis-write-warning = Test zápisu mění fotoaparát: zapíše hodnotu do jeho paměti nastavení a poté zapíše zpět původní hodnotu. Jen na vlastním fotoaparátu, na vlastní riziko.
analysis-write-consent = Fotoaparát je můj a nesu riziko.
btn-analysis-write-enable = Zapnout test zápisu
btn-analysis-start = Spustit analýzu
btn-analysis-snapshot = Přečíst zálohu
btn-analysis-new = Nová analýza
analysis-discard-confirm = Zahodit probíhající analýzu? Dosavadní zpráva zůstane uložena.
analysis-access-read = ČTENÍ
analysis-access-write = ZÁPIS
analysis-access-user = RUČNĚ
analysis-access-local = PC
analysis-step-snapshot = Přečíst zálohu, nastavení je na „{ $value }“
analysis-step-change = V nabídce fotoaparátu přepnout na „{ $value }“
analysis-step-evaluate = Porovnat snímky stavu
analysis-step-write = Zapsat do fotoaparátu hodnotu pro „{ $value }“
analysis-step-check = Vypnout a zapnout fotoaparát, odečíst nabídku (očekáváno: „{ $value }“)
analysis-step-restore = Zapsat zpět původní hodnotu pro „{ $value }“
analysis-step-report = Uložit zprávu o analýze
analysis-do-prepare = PŘÍPRAVA: 1. V nabídce fotoaparátu nastavte „{ $setting }“ na „{ $value }“ a opusťte nabídku tlačítkem MENU, aby fotoaparát nastavení uložil. 2. Nechte fotoaparát zapnutý. 3. Připojte kabel a na fotoaparátu zvolte „Hromadné ukládání (MSC)“.
analysis-do-change = VÁŠ KROK: 1. Odpojte kabel. 2. Fotoaparát zůstává zapnutý. 3. V nabídce fotoaparátu nastavte „{ $setting }“ na „{ $value }“ a opusťte nabídku tlačítkem MENU, aby fotoaparát nastavení uložil. 4. Znovu připojte kabel a zvolte „Hromadné ukládání (MSC)“.
analysis-do-check = VÁŠ KROK: 1. Odpojte kabel. 2. Vypněte a znovu zapněte fotoaparát. 3. Podívejte se v nabídce, co je u „{ $setting }“.
analysis-do-reconnect = Znovu připojte kabel a na fotoaparátu zvolte „Hromadné ukládání (MSC)“.
analysis-reading = Snímek stavu { $index } z { $total }: během čtení kabel NEODPOJUJTE.
analysis-cli-continue = Enter pro pokračování, q pro zrušení:
analysis-confirm-word = ano
analysis-cli-confirm = Pro potvrzení napište „{ $word }“:
analysis-write-declined = Nepotvrzeno. Nic nebylo zapsáno.
analysis-aborted = Zrušeno. Nic nebylo zapsáno.
analysis-snapshot-done = Snímek stavu přečten. Fotoaparát opustil servisní režim.
analysis-result = Porovnáno { $compared } hodnot: { $count } kandidátů, { $running } hodnot se změnilo samo.
analysis-candidate-consistent = jednoznačně odpovídá nabídce
analysis-candidate-inconsistent = neodpovídá jednoznačně
analysis-write-preview-heading = Toto bude zapsáno
analysis-write-preview = Hodnota { $id } ({ $size } B): z { $old } („{ $old-label }“) na { $new } („{ $new-label }“). Novou hodnotu uložil fotoaparát sám, když bylo nastaveno „{ $new-label }“.
btn-analysis-write = Zapsat
btn-analysis-skip-write = Dokončit bez testu zápisu
analysis-written = Hodnota zapsána, zpětně přečtena a trvale uložena.
analysis-menu-question = Co teď ukazuje nabídka?
analysis-menu-other = něco jiného
btn-analysis-menu-confirm = Převzít
btn-analysis-restore = Zapsat zpět původní hodnotu
analysis-restored = Původní hodnota zapsána zpět a trvale uložena.
analysis-survived = Zapsaná hodnota byla po restartu stále uložena.
analysis-not-survived = Zapsaná hodnota po restartu již nebyla uložena.
analysis-report-saved = Zpráva o analýze uložena: { $path }
analysis-report-hint = Zpráva neobsahuje sériové číslo ani zálohu. Můžete ji předat dál, aby bylo možné zjištění ověřit a zdokumentovat.
btn-analysis-export = Exportovat zprávu…
file-filter-analysis = Zpráva o analýze
analysis-pending = Test zápisu nebyl dokončen. Původní hodnota je zálohována a lze ji zapsat zpět.
analysis-pending-none = Pro tento fotoaparát není uložen žádný nedokončený test zápisu.
analysis-invalid-setting = Zadejte nastavení nabídky tak, jak ho fotoaparát nazývá.
analysis-invalid-values = Zadejte dvě až { $max } různé hodnoty nabídky. První je ta, která je nastavena teď.
analysis-already-complete = Všechny snímky stavu jsou již přečteny.
analysis-other-camera = Toto není stejný fotoaparát jako při prvním snímku stavu. Analýza byla zastavena.
analysis-write-not-allowed = Test zápisu není v tomto stavu možný.
analysis-camera-changed = Fotoaparát má nyní jinou hodnotu než při prvním snímku stavu. Nic nebylo zapsáno.
analysis-write-refused = Fotoaparát hodnotu odmítl; vše je jako předtím. ({ $detail })
analysis-refuse-none = Test zápisu není možný: žádná hodnota nesledovala nabídku.
analysis-refuse-ambiguous = Test zápisu není možný: nabídku sledovalo { $count } hodnot, žádnou nelze s jistotou přiřadit.
analysis-refuse-inconsistent = Test zápisu není možný: hodnota jednoznačně neodpovídá hodnotám nabídky.
analysis-refuse-size = Test zápisu není možný: hodnota má { $size } B; zapisují se jen malé hodnoty.
analysis-refuse-read-only = Test zápisu není možný: fotoaparát označuje hodnotu jako jen pro čtení.
analysis-refuse-protected = Test zápisu není možný: fotoaparát označuje hodnotu jako chráněnou.
settings-analysis-heading = Analýza funkcí
settings-analysis-purpose = Analýza funkcí dokumentuje, ve které hodnotě své paměti nastavení fotoaparát ukládá jedno ze svých vlastních nastavení menu. Je určena těm, kdo chtějí porozumět paměti nastavení svého modelu a sdílet výsledky.
settings-analysis-why-off = Je vypnutá, protože běžné používání ji nepotřebuje, protože několikrát přepíná fotoaparát do servisního režimu a protože na požádání do fotoaparátu i zapisuje. Je experimentální a vyzkoušená jen na několika modelech.
settings-analysis-toggle = Zapnout analýzu funkcí (položka „Analýza“ v nabídce)
settings-analysis-confirm = Zapnout analýzu funkcí? Několikrát přepíná fotoaparát do servisního režimu a na požádání může do fotoaparátu zapisovat. Používejte ji jen na vlastním fotoaparátu.
btn-analysis-enable = Zapnout
settings-analysis-on = Analýza funkcí je zapnutá.
settings-analysis-off = Analýza funkcí je vypnutá.
analysis-needs-admin = Každý snímek stavu potřebuje pro servisní režim práva správce. Windows se na ně zeptá jednou za relaci programu; pomocníka ovladačů můžete spustit hned, nebo počkat, až o něj požádá první snímek.
analysis-changed-hint = Zkontrolujte: nebyl nalezen žádný kandidát. Následující hodnoty se změnily, ale nevrátily se na počáteční hodnotu. Pokud se některá z nich mění jen jednou mezi dvěma hodnotami, fotoaparát při prvním snímku stavu pravděpodobně nebyl nastaven na první hodnotu nabídky. Pak analýzu zopakujte se správně nastavenou počáteční hodnotou.
analysis-changed-info = Info: hodnoty, které se během analýzy změnily samy (např. čítače, hodiny), se ignorují. Nemusíte nic dělat.
elevation-admin = Správce
elevation-standard = Standardní práva
elevation-admin-hint = Program běží s právy správce a může přiřadit ovladač, který servisní režim potřebuje.
elevation-standard-hint = Program běží se standardními právy. Pro servisní režim, tweaky a analýzu funkcí v případě potřeby spustí pomocníka ovladačů s právy správce; Windows se zeptá jednou za relaci. Klepnutím ho spustíte.
driver-helper-declined = Dotaz na práva správce byl odmítnut. Bez nich nelze přiřadit ovladač pro servisní režim; nic nebylo změněno.
btn-remove-foreign-drivers = Odebrat staré ovladače
btn-not-now = Teď ne
btn-ok = OK
foreign-drivers-found = Systém Windows stále uchovává ovladače, které pro zařízení Sony nainstaloval Zadig nebo podobný nástroj: { $packages }. Windows jim přidělí fotoaparát dřív, než tento program stihne přiřadit vlastní ovladač; servisní režim proto selže a fotoaparát se nemusí zobrazit jako jednotka. Odebrat je nyní? Fotoaparát pak bude používat ovladače dodávané se systémem Windows. Odebrání vyžaduje jednou oprávnění správce.
foreign-drivers-removed = Staré ovladače byly odebrány. Odpojte fotoaparát a znovu jej připojte.
foreign-drivers-remaining = Tyto ovladače se nepodařilo odebrat: { $packages }. Restartujte Windows a zkuste to znovu, nebo zařízení i s ovladačem odinstalujte ve Správci zařízení.
foreign-drivers-hint = Stále jsou nainstalovány ovladače, které pro zařízení Sony nainstaloval Zadig nebo podobný nástroj ({ $packages }). Windows jim přidělí fotoaparát dřív, než tento program stihne přiřadit vlastní ovladač, a servisní režim pak může selhat. Odeberte je příkazem: pmca-console driver zadig --remove
driver-zadig-none = Pro zařízení Sony nejsou nainstalovány žádné ovladače ze Zadigu ani podobných nástrojů.
driver-zadig-entry = { $package } ({ $provider }): { $devices }
driver-zadig-remove-hint = Odeberte je příkazem: pmca-console driver zadig --remove
driver-zadig-removed = Odebráno starých balíčků ovladačů: { $count }. Odpojte fotoaparát a znovu jej připojte.
elevation-helper = Pomocník ovladačů aktivní
elevation-helper-hint = Pomocník s právy správce přiřazuje ovladač pro servisní režim. Nemá okno a končí s programem; žádný další dotaz nepřijde.
elevation-helper-confirm = Spustit pomocníka ovladačů nyní? Windows se jednou zeptá na práva správce a do ukončení programu už ne.
analysis-cross-heading = Kontrola na jiném fotoaparátu
analysis-cross-intro = Volitelné: ověří, zda i jiný fotoaparát má nalezenou hodnotu a co je v něm nastaveno. Pouze čte. Stejné číslo nemusí znamenat stejnou funkci; potvrdí to až pohled do nabídky druhého fotoaparátu. Výsledek se zapíše do stejné zprávy o analýze.
analysis-cross-do = 1. Odpojte analyzovaný fotoaparát. 2. Zapněte druhý fotoaparát, nastavte jeho režim USB na „Auto“, připojte kabel a zvolte „Hromadné ukládání (MSC)“.
btn-analysis-cross = Zkontrolovat jiný fotoaparát
analysis-cross-present = existuje, hodnota { $value }
analysis-cross-match = odpovídá „{ $label }“
analysis-cross-nomatch = neodpovídá žádné naměřené hodnotě nabídky
analysis-cross-absent = neexistuje
analysis-cross-same-camera = Toto je samotný analyzovaný fotoaparát. Připojte prosím druhý fotoaparát.
analysis-cross-not-now = Kontrola je možná až po přečtení všech snímků stavu, když žádná zapsaná hodnota nečeká na vrácení.
analysis-cross-none = Není žádný kandidát ke kontrole.
analysis-wrong-camera = Připojený fotoaparát není ten analyzovaný. Připojte { $model }; nic nebylo zapsáno.
keys-fetching = Stahuje se zdroj…
keys-scanning = Hledá se klíčový materiál…
keys-complete = Veškerý klíčový materiál je k dispozici.
keys-incomplete = Část klíčového materiálu stále chybí.
keys-present = k dispozici
keys-missing = chybí
keys-class-senser = Klíče servisního režimu (servisní shell, vylepšení, analýza)
keys-class-cic = Klíč kontrolního součtu XPD (instalace aplikací)
keys-class-spk = Klíčový materiál SPK (instalace aplikací, apk2spk)
keys-class-fdat = Klíčový materiál FDAT (firmware updater-shellu)
plugin-heading = Zásuvné moduly
plugin-intro = Zásuvné moduly rozšiřují tento program o jazyky, přepínače nastavení fotoaparátu, údaje o kompatibilitě, nástroje a diagnostiku. Pocházejí pouze ze souborů, které si vyberete sami – žádný obchod se zásuvnými moduly neexistuje. Jejich kód běží v izolovaném prostředí bez přístupu k vašim souborům, síti nebo klíčovému materiálu. Instalujte pouze moduly od autorů, kterým důvěřujete; změny se projeví při příštím spuštění.
plugin-none = Nejsou nainstalovány žádné zásuvné moduly.
plugin-inactive = (aktivní po restartu)
plugin-author = Autor
plugin-restart = Změna se projeví po restartu programu.
btn-plugin-install = Nainstalovat zásuvný modul…
plugin-install-confirm = Tento zásuvný modul smí:
btn-confirm-plugin-install = Nainstalovat zásuvný modul
plugin-installed = Zásuvný modul byl nainstalován.
plugin-remove-confirm = Odebrat zásuvný modul „{ $name }“?
plugin-removed = Zásuvný modul { $id } byl odebrán.
plugin-perm-languages = přidávat do programu jazyky
plugin-perm-camera-settings = měnit nastavení fotoaparátu (jen mezi dvěma hodnotami, které uvádí, a teprve po vašem potvrzení)
plugin-perm-compatibility = přidávat položky do přehledu kompatibility
plugin-perm-code = spouštět vlastní kód v izolovaném prostředí (nástroje nebo diagnostika)
plugin-perm-host-os = zjistit, o jaký operační systém jde
plugin-perm-host-cameras = vidět, které fotoaparáty Sony jsou připojeny a v jakém režimu
plugin-invalid = Nejde o použitelný zásuvný modul: { $reason }
plugin-no-folder = Složku s uživatelskými daty se nepodařilo určit, proto nelze zásuvné moduly instalovat.
plugin-not-installed = Zásuvný modul s identifikátorem { $id } není nainstalován.
plugin-failed = Zásuvný modul selhal: { $reason }
plugin-unknown-entry = Žádný načtený zásuvný modul neposkytuje { $id }.
plugin-run-done = Zásuvný modul skončil.
plugin-run-warning = Zásuvný modul skončil s poznámkami.
plugin-run-error = Zásuvný modul ohlásil chybu.
plugin-tools-heading = Nástroje zásuvných modulů
plugin-tools-hint = Pocházejí z nainstalovaných zásuvných modulů a běží v izolovaném prostředí. Modul dostane jen to, co mu předáte, a to, co ohlásil, a fotoaparát změnit nemůže.
plugin-diagnostics-heading = Diagnostika zásuvných modulů
plugin-compat-heading = Ze zásuvného modulu
plugin-col-note = Poznámka
file-filter-plugin = Soubor zásuvného modulu
keys-not-set-up = Klíčový materiál, který tato operace vyžaduje, není nastaven.
err-missing-keys = Klíčový materiál, který tato operace vyžaduje, není nastaven.
keys-heading = Vyžaduje se klíčový materiál
keys-intro = Klíče servisního režimu, klíč kontrolního součtu XPD a klíčový materiál SPK nejsou součástí tohoto programu. Zadejte adresu zdroje, který máte — úložiště, archiv nebo soubor — a program najde a naimportuje rozpoznané části. Nic se nespouští a materiál zůstává jen v tomto počítači.
keys-law-notice = Dodržujte zákony platné tam, kde se nacházíte, a pracujte jen s fotoaparáty a daty, které jsou vaše.
keys-url-label = Adresa zdroje (jedna na řádek)
keys-url-required = Zadejte alespoň jednu adresu zdroje.
btn-keys-import = Importovat
btn-import-keys = Nastavit klíčový materiál
