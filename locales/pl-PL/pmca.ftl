# Polskie tłumaczenie PMCA-RE RELOADED.
# Tłumaczyć wyłącznie prawą stronę (po znaku =). Identyfikatorów nigdy nie zmieniać.
# Jeden komunikat w jednej linii: <id> = <tekst>.

app-title = PMCA-RE RELOADED – zestaw narzędzi do aparatów

# Nawigacja / karty
nav-camera-info = Informacje o aparacie
nav-install-app = Aplikacje
nav-tweaks = Modyfikacje
nav-network = Sieć
nav-firmware = Oprogramowanie układowe
nav-updater = Powłoka aktualizatora
nav-service = Powłoka serwisowa
nav-tools = Narzędzia plikowe
nav-settings = Ustawienia
nav-license = Licencja

# Strona powitalna: strona, od której zaczyna się okno.
nav-welcome = Powitanie
welcome-intro-heading = Co robi ten program
welcome-intro-body = Ten program pomaga zachować funkcje Twojego aparatu Sony i dostosować go do Twoich potrzeb. Gdy Sony zamknęło sklep z PlayMemories Camera Apps, nie było sposobu, by wykonać kopię zapasową zakupionych lub już zainstalowanych aplikacji ani zainstalować ich ponownie. Reset aparatu bezpowrotnie usuwa zainstalowane aplikacje. Ten program umożliwia tworzenie kopii zapasowych istniejących aplikacji i ich ponowną instalację, a dodatkowo może udostępniać swobodnie dostępne aplikacje społeczności. Można też instalować własne aplikacje (pliki *.apk). W razie problemów program odczytuje informacje o aparacie i wewnętrzne dzienniki do analizy. Ustawienia, które aparat już ma, ale których nie oferuje jego menu, można przełączyć i ponownie cofnąć. Program komunikuje się z aparatem bezpośrednio przez kabel USB. Obsługiwane modele są wymienione na stronie Informacje o aparacie.
welcome-start-heading = Pierwszy krok
welcome-start-body = Najpierw włącz aparat i — dopóki kabel nie jest podłączony — ustaw w jego menu „Połączenie USB” na Pamięć masową lub MTP. „Auto” zwykle też działa, ponieważ aparat sam wybiera wtedy jedno z dwojga. Kolejność ma znaczenie: gdy kabel już jest podłączony, do menu aparatu nie da się już dostać. Dopiero potem podłącz aparat przez USB, otwórz Informacje o aparacie i naciśnij „Pobierz informacje o aparacie”. Gdy pojawi się model i wersja oprogramowania układowego, połączenie jest w porządku i wszystkie pozostałe funkcje tego programu są dostępne.
welcome-start-driver = Jeśli zamiast tego pojawi się komunikat o sterowniku, zajmij się najpierw nim. W systemie Windows sterownik systemowy zawłaszcza aparat i trzeba go zastąpić, zanim możliwy będzie bezpośredni dostęp USB, którego ten zestaw narzędzi potrzebuje. Program robi to sam, ale przypisanie sterownika to uprawnienie, które Windows przyznaje wyłącznie administratorowi. W takim przypadku program oferuje możliwość ponownego uruchomienia go jako administrator, a Windows poprosi o potwierdzenie tej operacji. Po zakończeniu sesji przypisanie zostaje cofnięte. W systemach Linux i macOS nie jest to konieczne.
welcome-start-driver-linux = Jeśli zamiast tego pojawi się komunikat o sterowniku, zajmij się najpierw nim. W systemie Linux nie trzeba niczego zastępować: aparat jest osiągalny bezpośrednio. Jednorazowo potrzebne są prawo do otwarcia urządzenia — reguła udev dla identyfikatora producenta Sony — oraz, dla trybu pamięci masowej, odłączenie sterownika jądra usb-storage. Oba przygotowuje się poza tym programem z prawami roota; skrypt scripts/linux-prepare.sh w źródłach robi jedno i drugie. To okno samo nigdy nie prosi o takie prawa.
welcome-start-driver-macos = Jeśli zamiast tego pojawi się komunikat o sterowniku, najpierw postępuj zgodnie z nim. W systemie macOS niczego nie trzeba instalować ani zastępować. Ustaw połączenie USB aparatu na MTP. Do ustawień, powłoki serwisowej, analizy funkcji, aktualizacji oprogramowania układowego i powłoki aktualizatora to okno raz poprosi o hasło administratora: działają one przez połączenie pamięci masowej aparatu, które macOS przekazuje tylko takiemu programowi.
welcome-tweaks-heading = Modyfikacje i języki
welcome-tweaks-body = Modyfikacje to ukryte opcje, które aparat już ma, ale których nie oferuje w menu: limit czasu nagrywania, limit nagrywania 4K, wybór między PAL a NTSC, instalator aplikacji przez USB — i języki menu.
welcome-tweaks-languages-heading = Języki aparatu
welcome-tweaks-languages-body = Aparaty Sony wyświetlają tylko te języki menu, które są powszechne w regionie ich sprzedaży. Kto mieszka za granicą, często nie znajduje więc swojego języka i musi radzić sobie z obcym. Tymczasem kolejne języki są już zapisane w oprogramowaniu układowym. Ten program przenosi je do menu, dzięki czemu aparat można obsługiwać w znajomym języku.
welcome-tweaks-ui-heading = Język tego programu
welcome-tweaks-ui-body = Czymś zupełnie innym jest lista wyboru u góry. Zmienia ona wyłącznie język tego programu i nigdy nie dotyka aparatu.
welcome-tweaks-safety = Każda modyfikacja jest zapisywana w pamięci zapasowej aparatu i każdą można cofnąć tym samym przełącznikiem. Przerwany zapis może trwale uszkodzić aparat, więc nie odłączaj kabla ani nie wyjmuj akumulatora w trakcie zapisu.
welcome-privacy-heading = Twoje dane
welcome-privacy-where = Strona „Licencja” przedstawia to wszystko jeszcze raz w całości, wraz z licencjami wszystkich bibliotek, z których zbudowano ten program.
welcome-where-heading = Gdzie co znaleźć
welcome-where-info = Model, numer seryjny i wersja oprogramowania układowego. Zarazem test połączenia.
welcome-where-install = Aplikacje z listy społeczności albo własny plik .apk.
welcome-where-tweaks = Ustawienia, do których menu aparatu nie sięga, w tym języki.
welcome-where-network = Dane wspomagania GPS oraz punkty dostępu i usługi używane do transmisji na żywo.
welcome-where-firmware = Zapisać obraz oprogramowania układowego Sony w aparacie.
welcome-where-tools = Rozpakować i obejrzeć pliki oprogramowania układowego i pakietów. Bez aparatu.
welcome-where-shells = Bezpośredni dostęp do pamięci i systemu plików aparatu. Najpierw trzeba go wyświetlić w Ustawieniach.
welcome-where-settings = Motyw, tryb zaawansowany i edycja tłumaczenia tego programu.
welcome-where-license = Skąd pochodzi to przeniesienie, na czyjej pracy się opiera i na jakich warunkach jest udostępnione.
welcome-where-hint = Żadna strona nie działa na aparat sama z siebie. Nic nie jest odczytywane ani zapisywane, dopóki nie naciśniesz przycisku, który to zapowiada.

# Przyciski
btn-get-info = Pobierz informacje o aparacie
btn-refresh = Odśwież
btn-load-apps = Wczytaj dostępne aplikacje
btn-list-tweaks = Pokaż dostępne modyfikacje
btn-install = Zainstaluj wybraną aplikację
btn-install-file = Zainstaluj plik .apk…
btn-apply = Zastosuj
btn-save = Zapisz w aparacie
btn-add = Dodaj
btn-remove = Usuń
btn-reveal = Pokaż
btn-update-gps = Przenieś dane wspomagające GPS do urządzenia
btn-cancel = Anuluj
btn-browse = Przeglądaj…
btn-switch-msc = Przełącz w tryb pamięci masowej
btn-start-driver-helper = Uruchom pomocnika sterowników

# Etykiety filtrów typów plików w oknach dialogowych wyboru pliku
file-filter-apk = Aplikacja Android (*.apk)
file-filter-firmware = Obraz oprogramowania układowego Sony (*.dat)
file-filter-usage-log = Dziennik użycia (*.bin)
file-filter-catalog = Plik tłumaczenia (*.ftl)
file-filter-spk = Pakiet aplikacji Sony (*.spk)
file-filter-any = Wszystkie pliki
file-filter-gps = Dane wspomagające GPS (*.dat)

# Stan / komunikaty
msg-connecting = Łączenie…
msg-usage-log-written = Dziennik użycia zapisano w { $path }
msg-apps-found = Znaleziono aplikacje: { $count }
msg-switch-mode = Przełącz aparat w tryb MTP lub pamięci masowej.
msg-no-tweaks = Brak modyfikacji dostępnych dla tego aparatu.
msg-done = Gotowe.

# Przełączanie trybu USB aparatu (MTP -> pamięć masowa). Proponowane pod
# komunikatem o błędnym trybie; nie ma polecenia USB w drugą stronę.
mode-switching = Żądanie przełączenia aparatu w tryb pamięci masowej…
mode-switch-wait = Oczekiwanie na ponowne połączenie aparatu…
camera-confirm-hint = Komunikaty na aparacie (np. o www.sony.net/pm) można zamknąć; nie zakłócają one działania.
mode-switch-done = Aparat pracuje teraz w trybie pamięci masowej.
mode-switch-timeout = Aparat nie pojawił się ponownie w trybie pamięci masowej. Wyłącz go i włącz ponownie, a następnie spróbuj jeszcze raz.
mode-switch-impossible = Tego trybu nie można zmienić przez USB. Wyłącz aparat i włącz go ponownie.
mode-switch-hint = Przez USB można przełączyć wyłącznie aparat pracujący w trybie MTP. Zmiana jest tymczasowa: tryb USB wybrany w menu aparatu pozostaje bez zmian, więc wyłączenie i ponowne włączenie aparatu — albo odłączenie i ponowne podłączenie kabla — przywraca poprzedni tryb. Nie istnieje polecenie USB pozwalające wrócić.

# Pola informacji o aparacie
info-model = Model
info-product-code = Kod produktu
info-serial = Numer seryjny
info-firmware = Wersja oprogramowania układowego
info-lens = Obiektyw
info-region = Region
info-mode = Tryb
mode-mass-storage = Pamięć masowa
mode-mtp = MTP
mode-updater = Tryb aktualizatora
mode-app-install = Tryb instalacji aplikacji
mode-service = Tryb serwisowy
mode-unknown = Nieznany
info-android-version = Wersja platformy Android
info-photographer = Fotograf
info-copyright = Prawa autorskie
info-photographer-1 = Fotograf (wpis 1)
info-photographer-2 = Fotograf (wpis 2)
info-copyright-1 = Prawa autorskie (wpis 1)
info-copyright-2 = Prawa autorskie (wpis 2)
diagnose-services-heading = Aparat jest nieosiągalny?
diagnose-services-hint = Niektóre usługi systemu Windows mogą zajmować aparat w trybie MTP, przez co ta aplikacja nie może się z nim połączyć. Sprawdzenie jedynie pyta system Windows, które usługi działają; niczego nie zmienia.
btn-diagnose-services = Sprawdź usługi Windows
diagnose-services-none = Nie działa żadna znana zakłócająca usługa.
diagnose-services-found = Te działające usługi mogą zakłócać dostęp do aparatu:
diagnose-services-advice = Zatrzymaj usługę w oknie Usługi (services.msc) lub poniższym poleceniem jako administrator, a następnie podłącz aparat ponownie:

# Aktualizator oprogramowania układowego / powłoka
updater-model = Model aparatu
updater-firmware-current = Bieżąca wersja oprogramowania układowego
updater-guard-ok = Obraz oprogramowania układowego pasuje do tego aparatu.
updater-no-fdat = Dla modelu { $model } nie ma dołączonego oprogramowania aktualizatora.
updater-model-required = Aparat pracuje już w trybie aktualizatora i nie może w nim podać swojego modelu. Wyłącz aparat i włącz go ponownie, aby uruchomił się w trybie normalnym, a następnie spróbuj jeszcze raz. Możesz też wskazać plik payloadu (--fdat-file) lub, w konsoli, model (--model <MODEL>).
updater-already-updater = Aparat pracuje już w trybie aktualizatora.
updater-flash-required = Odmowa modyfikacji aparatu bez potwierdzenia. Uruchom ponownie z --flash, aby przełączyć w tryb aktualizatora i zapisać payload.
updater-flashing = Zapisywanie payloadu aktualizatora w aparacie…
updater-switch-wait = Postępuj zgodnie z instrukcjami na ekranie aparatu. Oczekiwanie na przełączenie aparatu w tryb aktualizatora…
updater-switch-timeout = Upłynął limit czasu oczekiwania na aparat. Uruchom polecenie ponownie, gdy aparat połączy się na nowo.
updater-shell-ready = Payload aktualizatora działa. Wpisz „help”, aby zobaczyć polecenia, lub „exit”, aby zakończyć.
updater-shell-done = Sesja powłoki zakończona.
updater-session-panic = Sesja aktualizatora zakończyła się nieoczekiwanie. Przed kolejną próbą wyłącz i włącz aparat.

# Co odpowiada powłoka aktualizatora. Błędy przy czytaniu wiersza sformułowano
# tylko raz, w kluczach service-*: obie powłoki dzielą ten sam analizator, więc
# ścieżka w cudzysłowie i błędna wartość szesnastkowa zawodzą w obu tak samo.
updater-backup-synced = Kopia zapasowa zapisana w pamięci flash.
updater-backup-locked = Chronione ustawienia kopii zapasowej są teraz zablokowane.
updater-backup-writable = Do chronionych ustawień kopii zapasowej można teraz zapisywać.
updater-shell-console-only = Powłoka roota wymaga terminala i jest dostępna tylko z wiersza poleceń: uruchom pmca-console updatershell --flash i wpisz tam „shell”.

# Strona powłoki aktualizatora.
updater-warning = Uwaga: aby dotrzeć do tej powłoki, aparat uruchamia się ponownie w trybie aktualizatora i zapisywany jest w nim payload – mały program tego projektu, który aparat przyjmuje jak aktualizację oprogramowania układowego i który następnie odpowiada na polecenia tej powłoki. Payload znika, gdy tylko aparat zostanie wyłączony, ale polecenia, które oferuje, zapisują w trwałej pamięci ustawień aparatu, a przerwany zapis może trwale uszkodzić aparat. Ponadto wszystkie ustawienia aparatu zostają przy tym zresetowane – w przeciwieństwie do powłoki serwisowej.
updater-console-heading = Uruchamianie powłoki aktualizatora z wiersza poleceń
updater-driver-hint = Powłoka aktualizatora wymaga, by aparat pozostał na własnym sterowniku pamięci masowej Windows, USBSTOR, bo tędy przesyłany jest payload. To dokładne przeciwieństwo tego, czego wymaga powłoka serwisowa. Aplikacja sama przypisuje i sama zwalnia sterownik WinUSB, więc niczego nie trzeba przygotowywać ręcznie — ale sterownik przypisany innym narzędziem trzeba najpierw cofnąć. Aparat w trybie aktualizatora zgłasza się pod własnym identyfikatorem produktu i też nie wolno go przepinać.
updater-fdat-label = Plik payloadu (opcjonalnie)
updater-confirm = Aparat uruchomi się ponownie w trybie aktualizatora i zostanie w nim zapisany payload. Kontynuować?
updater-cancelled = Anulowano. Aparat nie został zmieniony.

btn-shell-primer = Warto wiedzieć o tej powłoce
btn-shell-primer-hide = Ukryj objaśnienie
shell-primer-compare-heading = Dwie powłoki, dwa protokoły
shell-primer-compare = Obie strony udostępniają wiersz poleceń do aparatu, ale to nie ta sama powłoka. Powłoka serwisowa mówi własnym fabrycznym protokołem serwisowym Sony, na który aparat po uwierzytelnieniu odpowiada sam; nic nie jest przy tym zapisywane w aparacie. Powłoka aktualizatora istnieje wyłącznie dlatego, że ten projekt zapisuje w aparacie własny program: aparat uruchamia się ponownie w trybie aktualizacji, przyjmuje payload tak, jakby był oprogramowaniem układowym, a ten payload odpowiada potem na poniższe polecenia. Jednocześnie otwarta może być tylko jedna z nich.
shell-primer-payload = „Payload” (z angielskiego: ładunek użyteczny) oznacza właściwą zawartość, którą przenosi dana droga transportu. W przypadku powłoki aktualizatora jest to mały program tego projektu: aparat przyjmuje go tą samą drogą co aktualizację oprogramowania układowego, ale zachowuje go tylko do wyłączenia. Powłoka serwisowa nie potrzebuje payloadu, ponieważ aparat odpowiada na jej polecenia własnym oprogramowaniem. W plikach oprogramowania układowego termin oznacza to samo: właściwy obraz oprogramowania w kontenerze pliku .dat.
shell-primer-when-service = Powłoka serwisowa służy do wszystkiego, co czyta lub zmienia zapisane ustawienia aparatu, gdy nadal działa jego własne oprogramowanie: odczyt bloku HASP, zapisanie kopii przywracania obszaru kopii zapasowej, zmiana pojedynczych wpisów, pobieranie plików z aparatu. Korzysta z niej pod spodem także strona Modyfikacje; wszystko stamtąd da się zrobić tutaj ręcznie.
shell-primer-when-updater = Powłoka aktualizatora jest właściwym wyborem, gdy do aparatu nie da się dotrzeć inaczej albo gdy zamiast pojedynczych poleceń potrzebna jest powłoka na działającym systemie. Tylko ta droga daje prawdziwą powłokę roota i działa nawet na aparacie, który odmawia trybu serwisowego. Kosztuje ponowne uruchomienie, jeden zapis i ustawienia aparatu, jest więc drugim, a nie pierwszym wyborem.
shell-primer-driver-heading = Sterowniki: każda wymaga czegoś przeciwnego
shell-primer-driver = Powłoka serwisowa mówi surowym USB i w systemie Windows wymaga aparatu powiązanego z WinUSB. Aplikacja sama przypisuje ten sterownik w drodze do trybu serwisowego i sama zwalnia go po zakończeniu sesji; żadne osobne narzędzie nie bierze w tym udziału. Powłoka aktualizatora idzie przez USBSTOR, własny sterownik pamięci masowej Windows, i nie działa, dopóki aparat jest powiązany z WinUSB — dlatego zwolnienie przypisania należy do zamykania sesji serwisowej. Przypisanie sterownika wymaga uprawnień administratora. W Linuksie żadna zamiana sterownika nie jest potrzebna, wystarczy dostęp do węzła urządzenia.
shell-primer-risk-heading = Co może pójść nie tak
shell-primer-risk = Oznaczone niżej polecenia zapisują w trwałej pamięci ustawień aparatu. Przerwany zapis może sprawić, że aparat się nie uruchomi, a niczego z tego nie da się cofnąć — właśnie po to istnieje bk d: zapisz kopię obszaru kopii zapasowej, zanim cokolwiek zmienisz. Sam payload aktualizatora nie zostaje w aparacie – wyłączenie aparatu go usuwa. Droga przez tryb aktualizatora resetuje jednak wszystkie ustawienia aparatu.
shell-primer-commands-heading = Polecenia tej powłoki
shell-primer-writes = Polecenia oznaczone (WRITES) zmieniają aparat. Wszystkie pozostałe tylko czytają.
shell-cmd-hasp = Odczytuje blok HASP, chroniony obszar tożsamości aparatu.
shell-cmd-pull = Pobiera plik z aparatu. Bez nazwy lokalnej zachowywana jest nazwa z aparatu.
shell-cmd-push = Kopiuje plik lokalny do aparatu, zastępując to, co tam jest. (WRITES)
shell-cmd-rm = Usuwa plik w aparacie. (WRITES)
shell-cmd-rmem = Zapisuje obszar pamięci urządzenia do pliku lokalnego. Przesunięcie i długość są szesnastkowe.
shell-cmd-bk-r = Odczytuje jeden wpis obszaru kopii zapasowej i pokazuje go jako zrzut szesnastkowy.
shell-cmd-bk-d = Zapisuje cały obszar kopii zapasowej do pliku. Zrób to przed jakąkolwiek zmianą: to jedyna droga powrotu.
shell-cmd-bk-dump-txt = Zapisuje cały obszar kopii zapasowej jako <file>.bin oraz czytelny wykaz jako <file>.txt.
shell-cmd-bk-stat = Pokazuje blok stanu kopii zapasowej aparatu wraz z regionem.
shell-cmd-bk-w = Zapisuje jeden wpis obszaru kopii zapasowej, podany jako bajty szesnastkowe rozdzielone spacjami. (WRITES)
shell-cmd-bk-s-sub = Utrwala w pamięci flash obszar kopii zapasowej jednego podsystemu, aby zmiana przetrwała ponowne uruchomienie. (WRITES)
shell-cmd-bk-s = Utrwala obszar kopii zapasowej w pamięci flash, aby zmiana przetrwała ponowne uruchomienie. (WRITES)
shell-cmd-bk-lock = Włącza lub wyłącza ochronę przed zapisem chronionych wpisów kopii zapasowej; niektóre ustawienia nie dają się zmienić, dopóki jest włączona. (WRITES)
shell-cmd-term = Włącza lub wyłącza terminal diagnostyczny aparatu. (WRITES)
shell-cmd-info = Odczytuje blok stanu kopii zapasowej, który mówi, czy obszar w ogóle da się zapisać.
shell-cmd-shell = Otwiera powłokę roota na aparacie. Wymaga terminala, więc dostępna jest tylko w konsoli; shell tcp <port> udostępnia ją zamiast tego na 127.0.0.1 innemu klientowi. (WRITES)
shell-cmd-help = Wypisuje polecenia, które przyjmuje ta powłoka.
shell-cmd-exit = Kończy sesję i opuszcza powłokę.
shell-primer-diagnostics-heading = Diagnostyka i naprawa
shell-cmd-bk-trace = Zapisuje do pliku, pakiet po pakiecie, jak aparat wysyła cały obszar kopii zapasowej przez USB. Służy do badania dużych kopii; może zostawić aparat zawieszony w połowie odpowiedzi. Wtedy odłącz kabel, wyłącz i włącz aparat, w razie potrzeby wyjmując akumulator.
shell-cmd-bk-rewrite = Odczytuje obszar kopii zapasowej, zapisuje go z powrotem bez zmian i odczytuje ponownie. Pokazuje, czy aparat przyjmuje cały obraz, bez zmiany żadnego ustawienia. (WRITES)
shell-cmd-bk-restore-attrs = Przywraca atrybuty wszystkich wpisów kopii zapasowej, np. ochronę przed zapisem, do stanu z kopii zapisanej poleceniem bk d lub bk dump-txt. Wartości pozostają bez zmian. Używaj tylko kopii z tego samego aparatu. (WRITES)
shell-primer-diagnostics-env = Trzy zmienne środowiskowe działają zarówno na konsolę, jak i na interfejs graficzny, jeśli zostaną ustawione przed uruchomieniem, np. w PowerShell poleceniem $env:PMCA_TWEAKS_DIRECT = "1". PMCA_TWEAKS_DIRECT sprawia, że tweaki zapisują pojedyncze wartości, nawet gdy kopia zapasowa ma mniej niż 1 MB. PMCA_TWEAKS_IMAGE sprawia, że zapisują cały obraz, nawet gdy ma ponad 1 MB; jeśli ustawiono obie, obowiązuje PMCA_TWEAKS_DIRECT. PMCA_TLS_DUMP=<plik> zapisuje podczas instalacji aplikacji pierwszą wiadomość TLS aparatu (ClientHello), aby można było zbadać nieudane połączenie.

# Instalacja aplikacji
install-switching = Przełączanie w tryb instalacji aplikacji…
install-disabled = Aparat odmówił przejścia w tryb instalacji aplikacji. Zwykle odpowiada za to ustawienie „Włącz instalator aplikacji USB”: trzeba je włączyć w sekcji „Modyfikacje”.
install-stays-app-mode = Aparat pozostaje w trybie instalacji aplikacji, w którym można instalować kolejne aplikacje. Aby korzystać z pozostałych funkcji, odłącz kabel USB, a następnie wyłącz i ponownie włącz aparat.
err-usb-app-install-mode = Aparat nadal jest w trybie instalacji aplikacji i przyjmuje w nim tylko instalacje aplikacji. Odłącz kabel USB, a następnie wyłącz i ponownie włącz aparat — ponowne uruchomienie przy podłączonym kablu od razu przywraca ten tryb.
install-switch-wait = Oczekiwanie na ponowne połączenie aparatu w trybie instalacji aplikacji…
install-switch-timeout = Upłynął limit czasu oczekiwania na przejście aparatu w tryb instalacji aplikacji. Uruchom polecenie ponownie, gdy aparat połączy się na nowo.
install-bad-mode = W tym trybie nie można korzystać z aparatu. Przełącz go w tryb MTP lub pamięci masowej.
install-starting = Rozpoczynanie instalacji…
install-error = Błąd komunikacji podczas instalacji
install-server-failed = Lokalny serwer instalacji nie przyjął połączenia od aparatu:
install-done = Zadanie zakończone pomyślnie.

# Analiza pliku APK
apk-package = Pakiet:
apk-unreadable = Ostrzeżenie: nie udało się przeanalizować pliku .apk.
apk-incompatible = Ostrzeżenie: ta aplikacja wymaga minimalnego poziomu SDK, którego aparat może nie zapewniać.
apk-unsigned = Ostrzeżenie: plik .apk nie zawiera podpisu.

# Społecznościowy sklep z aplikacjami
store-loading = Wczytywanie listy aplikacji…
store-resolving = Wyszukiwanie najnowszego wydania…
store-downloading = Pobieranie
store-unknown-app = Nieznana aplikacja
store-no-release = Brak wydania tej aplikacji do pobrania
store-not-installable = brak wydania
msg-no-apps = Nie znaleziono aplikacji.
store-heading = Społecznościowy sklep z aplikacjami
store-discontinued = Sony zamknęło oficjalny sklep PlayMemories Camera Apps; jego serwery nie odpowiadają, więc nie można już z niego kupować ani pobierać aplikacji.
store-use-own-apk = Nadal można zainstalować własny plik .apk: wybierz go poniżej lub przeciągnij na to okno. Społecznościowy sklep z aplikacjami niżej także nadal działa.
btn-store-sync = Sprawdź aparat
btn-install-selected = Zainstaluj wybrane
btn-store-stop = Zatrzymaj po tej aplikacji
btn-store-stopping = Zatrzymanie po tej aplikacji…
btn-store-retry = Ponów nieudane
btn-continue = Kontynuuj
btn-reinstall = Zainstaluj ponownie
btn-skip = Pomiń
btn-confirm-updater = Uruchom ponownie i zapisz
btn-confirm-tweaks = Zapisz modyfikacje
btn-confirm-languages = Włącz języki
btn-confirm-wifi = Zastąp punkty dostępowe
btn-confirm-stream = Nadpisz ustawienia
btn-confirm-show-shells = Pokaż powłoki
btn-confirm-discard = Odrzuć tłumaczenie
store-select-all = Zaznacz wszystkie aplikacje
store-col-app = Aplikacja
store-col-version = Wersja
store-col-author = Autor
store-col-camera = Wersja na aparacie
store-col-status = Stan
store-camera-absent = niezainstalowana
store-version-unknown = nieznana
store-batch-query = Odczytywanie zainstalowanych aplikacji z aparatu…
store-sync-done = Aparat sprawdzony. Zainstalowane aplikacje z listy: { $count }
store-batch-downloading = Pobieranie aplikacji { $current } z { $total }…
store-batch-installing = Aplikacja { $current } z { $total }: instalowanie { $name }…
store-reinstall-confirm = Aplikacja { $name } jest już zainstalowana na aparacie. Zainstalowana wersja: { $installed }. Wersja w sklepie z aplikacjami: { $store }. Zainstalować ją ponownie?
store-state-queued = oczekuje
store-state-installed = zainstalowana
store-state-failed = niepowodzenie
store-state-skipped = pominięta
store-state-not-run = niewykonana
store-batch-summary = Zainstalowane: { $done } · nieudane: { $failed } · pominięte: { $skipped } · niewykonane: { $notrun }

# Tryb serwisowy (senser)
usb-mode-advice = Do tweaków, kopii zapasowych i powłoki serwisowej ustaw tryb połączenia USB aparatu na „Auto”, a nie na stałe na pamięć masową. Gdy aparat po sesji zapyta o tryb, wybierz „Pamięć masowa (MSC)”. Przy stale ustawionej pamięci masowej aparat często odmawia kolejnego przejścia w tryb serwisowy.
service-switch-refused = Aparat nie zakończył przejścia w tryb serwisowy i nie połączył się ponownie przez USB; zwykle wyświetla wtedy „Sprawdź podłączone urządzenie”. Odłącz kabel USB, podłącz go ponownie i spróbuj jeszcze raz.
service-switching = Przełączanie w tryb serwisowy…
service-switch-wait = Oczekiwanie na ponowne połączenie aparatu w trybie serwisowym…
service-switch-timeout = Aparat nie pojawił się w trybie serwisowym. Zgłasza się wtedy jako osobne urządzenie USB (identyfikator produktu 0336), które wymaga własnego przypisania sterownika WinUSB; sprawdź w menedżerze urządzeń, czy jest tam urządzenie Sony bez sterownika. Wyłącz i włącz aparat, aby wrócić do trybu normalnego.
service-authenticating = Uwierzytelnianie…
# Wyświetlane, gdy aparat wciąż jest w trybie serwisowym po poprzednim
# uruchomieniu, oraz gdy nie udało się tego cofnąć.
service-stranded-reset = Aparat wciąż jest w trybie serwisowym po poprzednim uruchomieniu. Najpierw kończę tamtą sesję…
service-stranded-failed = Aparat utknął w trybie serwisowym po poprzednim uruchomieniu i nie odpowiedział na żądanie jego opuszczenia. Wyłącz i ponownie włącz aparat, a następnie spróbuj jeszcze raz.
service-bad-mode = W tym trybie nie można korzystać z aparatu. Przełącz go w tryb pamięci masowej.
service-driver-required = Odmówiono bezpośredniego dostępu do USB, a sterownika WinUSB nie udało się przypisać. Przypisanie sterownika wymaga uprawnień administratora.
service-warning = Ostrzeżenie: tryb serwisowy daje bezpośredni dostęp do pamięci, systemu plików i danych kalibracyjnych aparatu. Polecenia zapisu mogą trwale uszkodzić aparat.
service-shell-ready = Powłoka serwisowa gotowa. Wpisz „help”, aby zobaczyć polecenia, lub „exit”, aby zakończyć.
service-shell-done = Sesja powłoki serwisowej zakończona.
# Zgłaszane przy starcie, gdy poprzednie uruchomienie zakończyło się, zanim
# zdążyło zwolnić przypisany sterownik.
driver-swept = Usunięto { $count } przypisanie sterownika pozostawione przez poprzednie uruchomienie.
# Zgłaszane przy starcie, gdy to samo przerwane uruchomienie pozostawiło też
# aparat w trybie serwisowym, w którym odrzuciłby następną sesję.
driver-swept-camera = Aparat pozostawiony w trybie serwisowym przez poprzednie uruchomienie został przywrócony do normalnego trybu.
driver-release-replug = Windows czeka na aparat podczas zwalniania sterownika. Odłącz kabel USB i podłącz go ponownie, aby sprzątanie mogło się zakończyć.
# Wyniki polecenia `driver`.
driver-bound = { $id } używa teraz sterownika WinUSB (pakiet { $package }).
driver-released = Zwolniono { $count } przypisanie sterownika; aparat ponownie korzysta ze sterowników dostarczanych z systemem Windows.
driver-needs-elevation = Przypisanie sterownika wymaga uprawnień administratora. Uruchom to polecenie w konsoli otwartej jako administrator.
driver-leftover-needs-elevation = Wcześniejsze uruchomienie przypisało aparatowi sterownik, który nie został jeszcze zwolniony. Dopóki istnieje, aparat prosi o sprawdzenie podłączonego urządzenia, a jego pamięć masowa nie działa. Zwolnienie wymaga uprawnień administratora; potwierdź zapytanie systemu Windows.
driver-binding = Przypisywanie sterownika WinUSB do aparatu…
driver-bind-wait = Oczekiwanie na ponowne połączenie aparatu z nowym sterownikiem…
driver-bind-timeout = Aparat nie pojawił się ponownie po przypisaniu sterownika. Odłącz go, podłącz ponownie i spróbuj jeszcze raz.
driver-bind-failed = Nie udało się przypisać sterownika WinUSB: { $detail }
# Odpowiedzi powłoki. Pochodzą z warstwy wykonawczej, aby konsola i okno
# używały tych samych słów.
service-usage = Składnia: { $usage }
service-bad-hex = To nie jest wartość szesnastkowa: { $value }
service-unknown-command = Nieznane polecenie: { $command }. Wpisz 'help', aby zobaczyć listę.
service-unterminated-quote = Wiersz kończy się wewnątrz argumentu w cudzysłowie.
service-help = Lista poleceń:
service-read-bytes = Odczytano { $count } bajtów.
service-pulled = Skopiowano { $remote } z aparatu do { $local }.
service-pushed = Skopiowano { $local } do { $remote } w aparacie.
service-removed = Usunięto { $remote } z aparatu.
service-dumped = Zapisano { $size } bajtów od { $offset } do { $local }.
service-backup-saved = Obraz kopii zapasowej ({ $count } bajtów) zapisano w { $local }.
service-backup-text-saved = Obraz kopii zapasowej ({ $count } bajtów) zapisano w { $bin }, a wykaz w { $txt }.
service-backup-status = Stan kopii zapasowej, region: { $region }
service-backup-rewritten = Obraz ({ $count } bajtów) zapisano z powrotem bez zmian i odczytano ponownie; nie zapisano go na stałe.
service-backup-attrs-restored = Atrybuty { $count } wpisów przywrócono według { $local }.
service-backup-written = Zapisano wpis kopii zapasowej { $id }.
service-backup-persisted = Kopię zapasową podsystemu { $id } zapisano w pamięci flash.
service-terminal-on = Terminal diagnostyczny jest teraz włączony.
service-terminal-off = Terminal diagnostyczny jest teraz wyłączony.
btn-service-terminal-open = Otwórz konsolę szeregową
btn-service-terminal-close = Zamknij konsolę szeregową
service-terminal-heading = Konsola szeregowa
service-terminal-hint = W aparatach z Linuksem terminal diagnostyczny to powłoka roota przez USB. Otwórz ją i wpisuj polecenia jak w terminalu.
service-terminal-label = Konsola
service-terminal-interactive = Konsola szeregowa otwarta. Wpisuj polecenia; wpisz „exit”, aby wrócić do powłoki serwisowej.

# Strona serwisowa bez trybu zaawansowanego: jak dostać się do powłoki z
# konsoli i co trzeba wcześniej przygotować.
service-console-heading = Uruchom powłokę serwisową w konsoli
service-console-hint = Otwórz wiersz poleceń w folderze z pmca-console i wykonaj to polecenie:
service-driver-heading = Zanim zaczniesz
service-driver-hint = Tryb serwisowy wymaga bezpośredniego dostępu USB, którego system Windows nie udziela przez sterownik pamięci masowej. Aplikacja sama przypisuje sterownik WinUSB — dwa razy, raz gdy aparat jest w trybie pamięci masowej i drugi raz po tym, jak pojawi się ponownie w trybie serwisowym pod innym identyfikatorem produktu — i zwalnia oba przypisania po zakończeniu sesji. Przypisanie sterownika wymaga uprawnień administratora, więc uruchom konsolę jako administrator; żadne inne narzędzie nie jest potrzebne.
driver-note-linux = Tutaj nie trzeba niczego instalować: libusb sięga aparatu bezpośrednio, bez wypierania sterownika jądra. Potrzebne są jedynie prawo do otwarcia urządzenia oraz środowisko graficzne, które go nie zajmuje. Dodaj regułę udev dla identyfikatora producenta Sony 054c i odłącz sterownik usb-storage przed użyciem trybu pamięci masowej. Skrypt scripts/linux-prepare.sh robi jedno i drugie, a przy okazji zatrzymuje monitory woluminów gvfs.
driver-note-macos = Do bezpośredniego dostępu przez USB niczego nie trzeba instalować. Ustaw połączenie USB aparatu na MTP. W trybie MTP program zamyka usługę aparatów macOS ptpcamerad, jeśli zajmuje ona aparat; system uruchamia ją ponownie w razie potrzeby. Ustawienia, powłoka serwisowa, analiza funkcji, aktualizacje oprogramowania układowego i powłoka aktualizatora działają przez połączenie pamięci masowej, które macOS przekazuje tylko programowi z uprawnieniami administratora: okno prosi o hasło raz na uruchomienie, wiersz poleceń wymaga sudo. Ta platforma jest najsłabiej przetestowana z trzech; licz raczej na to, że zgłosisz problem, niż że znajdziesz go już opisany.
service-advanced-hint = Powłokę można też wyświetlić tutaj, w Ustawieniach, Tryb zaawansowany.
btn-copy-command = Kopiuj polecenie
msg-command-copied = Polecenie skopiowano do schowka.

# Strona serwisowa z włączonym trybem zaawansowanym.
btn-service-open = Połącz
btn-service-close = Rozłącz
btn-service-run = Wykonaj
btn-service-browse = Wstaw ścieżkę pliku…
shell-path-quote = Nie można wstawić tej ścieżki: zawiera cudzysłów prosty, czyli znak, którym wiersz poleceń wyznacza początek i koniec argumentu. Zmień nazwę pliku lub przenieś go do ścieżki bez tego znaku.
service-command-label = Polecenie
service-not-connected = Brak połączenia.

# Teksty używane tylko w interfejsie graficznym
install-apk-path = Ścieżka do lokalnego pliku .apk (opcjonalnie)
install-drop-hint = Upuść tutaj plik .apk

# Opisy modyfikacji
tweak-rec-limit = Nagrywanie wideo bez limitu czasu
tweak-rec-limit-4k = Nagrywanie wideo 4K bez limitu czasu
tweak-language = Pokaż wszystkie języki menu
tweak-language-note = Aparat pokazuje tylko języki znane jego oprogramowaniu, więc może ich być mniej, niż włączono tutaj. Na przykład DSC-RX1RM2 pokazuje 25 z 35.
tweak-pal-ntsc = Włącz przełącznik PAL / NTSC i ostrzeżenie
tweak-pal-ntsc-mode = System wideo (PAL / NTSC)
tweak-pal-ntsc-pal = PAL
tweak-pal-ntsc-ntsc = NTSC
tweak-pal-ntsc-note = Po zmianie systemu wideo aparat wymaga sformatowania karty pamięci. Najpierw wykonaj kopię zapasową wszystkiego — zdjęć, filmów, plików LUT. Najprościej jest wyjąć kartę przed przełączeniem.
tweaks-set-malformed = Oczekiwano --set ID=WARTOŚĆ, otrzymano: { $arg }
tweaks-set-unknown-value = Nieznana wartość „{ $value }”. Oczekiwano jednej z: { $options }
tweaks-set-not-a-choice = { $id } to przełącznik, a nie wybór. Użyj --enable lub --disable.
tweaks-needs-set = { $id } to wybór, a nie przełącznik. Użyj --set { $id }=WARTOŚĆ z jedną z: { $options }
tweak-usb-installer = Włącz instalator aplikacji USB

# Przebieg stosowania modyfikacji
tweaks-warning = Ostrzeżenie: stosowanie modyfikacji nadpisuje pamięć kopii zapasowej aparatu. Przerwany zapis może trwale uszkodzić aparat.
tweaks-reading = Odczyt ustawień aparatu…
tweaks-writing = Zapis ustawień aparatu…
tweaks-done = Ustawienia aparatu zaktualizowane.
tweaks-write-failed = Aparat nie zapisał nowych ustawień w takiej postaci, w jakiej zostały wysłane. Część mogła mimo to zostać zapisana: odczytaj ustawienia ponownie i w razie potrzeby cofnij zmiany. Bezpieczną drogą powrotu jest kopia zapasowa wykonana przed zmianą.
tweaks-confirm-required = Odmowa zapisu bez potwierdzenia. Po przeczytaniu powyższego ostrzeżenia uruchom ponownie z --confirm.
tweaks-state-enabled = włączone
tweaks-state-disabled = wyłączone
tweaks-full-read-failed = Aparat nie przekazał pełnej kopii zapasowej ({ $detail }). Potem często przestaje odpowiadać: wyłącz go, wyjmij akumulator na kilka sekund, włóż go z powrotem i podłącz aparat ponownie.
tweaks-direct-mode = Kopia zapasowa tego aparatu jest większa niż 1 MB, dlatego ustawienia są odczytywane i zapisywane pojedynczo. Przed każdą zmianą stare wartości są zapisywane i można je przywrócić przyciskiem „Cofnij zmiany”.
tweaks-direct-forced = PMCA_TWEAKS_DIRECT wymusza zapis pojedynczych wartości, dlatego ustawienia są odczytywane i zapisywane pojedynczo. Przed każdą zmianą stare wartości są zapisywane i można je przywrócić przyciskiem „Cofnij zmiany”.
tweaks-direct-refused = Aparat odrzucił zmianę ({ $detail }). Już zapisane wartości zostały przywrócone; nic nie zostało zmienione. Niektóre modele, zwłaszcza japońskie, pozwalają na to ustawienie tylko przez pełną kopię zapasową.
tweaks-direct-revert-failed = Aparat odrzucił zmianę ({ $detail }) i nie wszystkie już zapisane wartości udało się przywrócić. Stare wartości są zapisane: spróbuj „Cofnij zmiany”.
tweaks-direct-restart = Ustawienia aparatu zaktualizowane. Wyłącz i ponownie włącz aparat, aby zmiana zaczęła działać.
tweaks-rollback-unsaved = Nie udało się zapisać bieżących wartości do cofnięcia; nic nie zostało zmienione.
tweaks-image-saved = Obraz kopii zapasowej sprzed zmiany został zapisany: { $path }
tweaks-whole-image-hint = Aby zamiast tego zapisać zmianę przez pełny obraz kopii zapasowej, uruchom to samo polecenie ponownie z --whole-image. Obraz sprzed zmiany zostanie najpierw zapisany.
tweaks-whole-image-confirm = Zapisać zmianę przez pełny obraz kopii zapasowej? Aparat odrzucił ją jako pojedynczą wartość. Cały obraz zostanie zapisany z powrotem wraz ze zmianą; obraz sprzed zmiany zostanie najpierw zapisany na tym komputerze.
btn-whole-image = Zapisz pełną kopię
tweaks-undo-none = Dla tego aparatu nie ma zapisanych wartości.
tweaks-undo-confirm = Zapisać zachowane wartości z powrotem w aparacie? Cofnie to wszystkie zmiany zapisane pojedynczo od chwili ich zachowania.
tweaks-undo-done = Zachowane wartości zostały zapisane z powrotem. Wyłącz i ponownie włącz aparat.
tweaks-undo-available = Wartości sprzed pierwszej zmiany są zachowane i można je zapisać z powrotem.
btn-undo-tweaks = Cofnij zmiany
tweaks-identity-heading = Tożsamość aparatu (tylko do odczytu)
tweaks-caution-languages = Dwóch języków — prawdopodobnie chińskiego (uproszczonego) i chińskiego (tradycyjnego) — ten aparat nie potrafi wyświetlić. Przewinięcie do nich powoduje ponowne uruchomienie menu języków, dlatego pozostają wyłączone na wszelki wypadek.
tweaks-caution-accept = Włącz mimo to te dwa języki (na własną odpowiedzialność)
tweaks-caution-confirm = Te dwa języki mogą spowodować ponowne uruchomienie menu języków w aparacie. Nic się nie psuje, a zmianę można cofnąć, ale ustawienie trzeba będzie może odnaleźć od nowa. Włączyć mimo to?

# Ustawienia
settings-language = Język
settings-theme = Motyw
settings-theme-system = Systemowy
settings-theme-light = Jasny
settings-theme-dark = Ciemny

# Tryb zaawansowany. Zabezpieczenie przed pomyłką, nie przed napastnikiem:
# wszystko, co wyświetla, i tak jest dostępne z konsoli.
settings-advanced-heading = Tryb zaawansowany
settings-advanced-hint = Tryb zaawansowany wyświetla powłokę serwisową i powłokę aktualizatora na stronach o tych samych nazwach. Tryb serwisowy daje bezpośredni dostęp do pamięci aparatu, jego systemu plików i danych kalibracyjnych; źle wpisane polecenie może unieruchomić aparat. Pozostaw go wyłączony, dopóki nie wiesz, co zamierzasz zapisać.
settings-advanced-toggle = Pokaż powłokę serwisową i powłokę aktualizatora
settings-advanced-on = Tryb zaawansowany jest włączony. Strony Powłoka serwisowa i Powłoka aktualizatora udostępniają teraz same powłoki.
settings-advanced-off = Tryb zaawansowany jest wyłączony. Strony Powłoka serwisowa i Powłoka aktualizatora objaśniają zamiast tego drogę przez konsolę.
settings-advanced-confirm = Powłoka serwisowa i powłoka aktualizatora mogą trwale uszkodzić aparat. Pokazać obie powłoki?
settings-update-heading = Powiadomienie o aktualizacji
settings-update-purpose = Gdy jest włączone, program przy każdym uruchomieniu pyta GitHub, czy opublikowano nowszą wersję, i jeśli tak, informuje o tym u góry okna. GitHub poznaje przy tym Twój adres IP; nic więcej nie jest wysyłane. Wersje przedpremierowe nie są zgłaszane.
settings-update-toggle = Informuj przy uruchomieniu o nowej wersji
settings-update-on = Powiadomienie o aktualizacji jest włączone. Od następnego uruchomienia nowa wersja będzie zgłaszana u góry okna.
settings-update-off = Powiadomienie o aktualizacji jest wyłączone. Program nie szuka nowych wersji.
update-available = Dostępna jest nowa wersja
update-current = Zainstalowana wersja
update-latest = Nowa wersja
btn-update-dismiss = Ukryj

# Edycja tłumaczenia. Pliki językowe można pobrać, zmienić w dowolnym edytorze
# tekstu i wysłać z powrotem; wysłany plik jest sprawdzany przed użyciem.
settings-catalog-heading = Edycja tłumaczenia
settings-catalog-hint = Pobierz plik językowy, zmień go w dowolnym edytorze tekstu i wyślij z powrotem. Tłumaczony jest wyłącznie tekst po prawej stronie znaku równości, a pominięte wpisy zachowują wbudowane brzmienie. Wybór języka w tym miejscu nie zmienia języka aplikacji.
settings-catalog-locale = Plik językowy
settings-catalog-builtin = Używane jest tłumaczenie wbudowane.
settings-catalog-custom = Używane jest tłumaczenie własne:
btn-catalog-export = Pobierz…
btn-catalog-import = Wyślij…
btn-catalog-reset = Przywróć wbudowane
catalog-exported = Zapisano w
catalog-imported = Tłumaczenie zastosowane.
catalog-reset-done = Przywrócono tłumaczenie wbudowane.
catalog-reset-none = Nie było własnego tłumaczenia do przywrócenia.
catalog-reset-confirm = Odrzucić własne tłumaczenie dla tego języka?
catalog-rejected = Plik nie został zastosowany. Popraw następujące linie:
catalog-count-accepted = Zastosowane wpisy
catalog-count-missing = Wpisy zachowane z tłumaczenia wbudowanego
catalog-count-untranslated = Wpisy identyczne z angielskim oryginałem
catalog-line = Linia
catalog-problem-syntax = to nie jest wpis w postaci „id = tekst”
catalog-problem-continuation = kontynuacja w następnej linii, co nie jest obsługiwane
catalog-problem-duplicate = zdefiniowano więcej niż raz
catalog-problem-unknown-key = to nie jest wpis używany przez tę aplikację
catalog-problem-empty = nie zawiera tekstu
catalog-problem-placeholder = zmieniono symbol zastępczy, oczekiwano:

# Pochodzenie, podziękowania i licencja. Wyświetlane na stronie licencji oraz
# przez `pmca-console license`. Sam tekst licencji nie jest tłumaczony: to
# dokument prawny i jest przytaczany dosłownie.
about-thanks-heading = Podziękowania
about-thanks-body-1 = To narzędzie istnieje dzięki ma1co, który rozpracował sposób komunikacji aparatów Sony przez USB i w 2015 roku udostępnił Sony-PMCA-RE, oraz dzięki wszystkim, którzy przyczynili się do jego rozwoju i do projektów OpenMemories w kolejnych latach.
about-thanks-body-2 = Wykrywanie aparatu, protokół poleceń zewnętrznych, procedura aktualizacji oprogramowania układowego i tryb serwisowy to w całości ich praca, a ten port ją kontynuuje. Dziękujemy. Na tym fundamencie PMCA-RE RELOADED dodaje wiele nowych funkcji, które wniósł masterflai. Najważniejsza z nich to wbudowane teraz automatyczne przełączanie sterownika urządzenia w systemie Windows: aparatowi nie trzeba już ręcznie przypisywać innego sterownika osobnym narzędziem, dzięki czemu cały proces jest wygodniejszy, szybszy i mniej ryzykowny.
about-contributors-heading = Współtwórcy
about-contributors-body = Oprócz ma1co te osoby przyczyniły się do Sony-PMCA-RE albo opublikowały we własnych kopiach zmiany, które przeniesiono do tego portu:
about-purpose-heading = Dlaczego powstał ten port
about-purpose-body-1 = Pierwotna aplikacja nie jest już rozwijana. Została napisana dla Python 2, którego wsparcie zakończyło się w 2020 roku, a jej części opierają się na usługach, które Sony od tego czasu wyłączyło, więc na obecnych systemach nie działa już bez ograniczeń.
about-purpose-body-2 = Celem tego portu jest zachowanie, a nie zastąpienie, a w równym stopniu bezproblemowe korzystanie dziś: utrzymanie tych aparatów w użyciu oraz to, by program działał bez trudności zarówno na współczesnych komputerach, jak i z nowszymi modelami aparatów. Wszystko, co potrafił oryginał, powinno być nadal możliwe.
about-origin-heading = Jak powstał
about-origin-body-1 = Przeniesienie z Pythona na Rust zostało wykonane z pomocą SI.
about-origin-body-2 = Warto równie jasno powiedzieć, czego to nie oznacza: zachowanie programu jest objęte automatycznym zestawem testów oraz planem testów sprzętowych. Przede wszystkim wersja dla Windows została dokładnie sprawdzona, ponieważ sterowniki urządzeń czynią ją najbardziej złożoną platformą; macOS i Linux nie mogły natomiast zostać przetestowane w pełnym zakresie. Prosimy traktować to jako oprogramowanie eksperymentalne.
about-license-heading = Licencja
about-license-body = Ten port jest publikowany na licencji MIT, tej samej co pierwotny projekt. Oryginalna nota o prawach autorskich pozostaje niezmieniona, zgodnie z wymogami tej licencji.
about-license-full = Tekst licencji
about-third-party-heading = Licencje użytych bibliotek
about-third-party-body = Ten program jest konsolidowany statycznie: kod każdej używanej biblioteki znajduje się wewnątrz pliku wykonywalnego, a licencje tych bibliotek wymagają, by towarzyszyły mu ich noty o prawach autorskich. Poniżej przytoczono je w całości, po jednym wpisie na pakiet.
about-third-party-toggle = Pokaż wszystkie noty licencyjne stron trzecich
about-privacy-heading = Dane
about-privacy-body-1 = Ten program działa na Twoim własnym komputerze. Nie wysyła danych o użytkowaniu, nie ma w nim telemetrii ani obowiązku zakładania konta. Nic z tego, co tu robisz, nie jest nigdzie zgłaszane.
about-privacy-body-2 = Dwie funkcje same nawiązują jednak połączenia, a druga strona poznaje przy tym Twój adres IP: dane wspomagania GPS pobierane są od Sony, a lista aplikacji z GitHuba. Bez tych funkcji program pozostaje całkowicie offline. Jeśli włączysz w ustawieniach powiadomienie o aktualizacji, program będzie też przy każdym uruchomieniu pytał GitHub o najnowszą wersję.
about-privacy-body-3 = To, co zostaje odczytane z aparatu — w tym numer seryjny i zapisane hasła Wi-Fi — nie opuszcza Twojego komputera. Jeśli zapiszesz to do pliku, zwróć uwagę, komu ten plik przekazujesz.
about-privacy-body-4 = Za tym programem nie stoi żadna usługa, a do jego autorów nie trafiają żadne dane, nie ma tu więc o co pytać ani czego kazać usuwać. To, co dwa wymienione serwisy robią z adresami, które widzą, podlega ich własnym oświadczeniom o prywatności, nie temu.
about-legal-heading = Informacje prawne
about-legal-trademark = To niezależny projekt. Nie został stworzony przez firmę Sony, nie jest przez nią firmowany ani w żaden sposób z nią powiązany. »Sony«, »PlayMemories« oraz nazwy modeli aparatów są znakami towarowymi Sony Group Corporation i wymieniono je tutaj wyłącznie po to, by wskazać, z jakimi aparatami narzędzie to się porozumiewa.
about-legal-own-device = Ten program jest przeznaczony do użytku z aparatami, których jesteś właścicielem.
about-legal-warranty = To, co robi to narzędzie, wykracza poza zamierzenia firmy Sony wobec tych aparatów i może zakończyć obowiązywanie gwarancji. Samo oprogramowanie udostępniane jest bez jakiejkolwiek gwarancji, a korzystanie z niego odbywa się na własne ryzyko. To, czy i w jakim zakresie inżynieria wsteczna własnego urządzenia jest dozwolona, zależy od kraju.
about-links-heading = Projekty
about-link-original = Sony-PMCA-RE — pierwotny projekt
about-link-fwtool = fwtool.py — narzędzia do firmware’u, na których opiera się tworzenie obrazów aktualizacji
about-link-tweak = OpenMemories: Tweak — aplikacja zalecana dla aparatów, które ją obsługują
about-link-applist = OpenMemories: App list — kolejne aplikacje
about-link-platform = OpenMemories: Platform — dołączone ciało aktualizatora
about-link-devices = Obsługiwane modele aparatów
about-link-nexhack = Nex-Hack — badania społeczności, na których opiera się ten projekt
about-link-zve10 = zv-e10-re — badania interfejsów i pamięci ustawień ZV-E10
about-version = Wersja
btn-copy-url = Kopiuj adres
msg-url-copied = Adres skopiowano do schowka.

# Informacja o bezpieczeństwie
notice-experimental = To jest oprogramowanie eksperymentalne. Używasz go na własne ryzyko.

# Ustawienia sieci (dane wspomagające GPS, punkty dostępowe i usługi transmisji na żywo)
gps-downloading = Pobieranie danych wspomagających GPS…
gps-writing = Zapisywanie danych wspomagających GPS w aparacie…
gps-bad-size = Dane wspomagające GPS mają nieoczekiwany rozmiar i nie zostały zapisane.
wifi-read = Odczyt punktów dostępowych transmisji na żywo z aparatu…
wifi-written = Zapisano punkty dostępowe transmisji na żywo.
stream-read = Odczyt ustawień transmisji na żywo z aparatu…
stream-written = Zapisano ustawienia transmisji na żywo.
net-secret-warning = Ostrzeżenie: ten plik zawiera hasła Wi-Fi oraz tokeny dostępu zapisane jawnym tekstem.
wifi-secret-warning = Aparat przechowuje te hasła bez szyfrowania i są tu pokazywane dokładnie tak, jak zostały zapisane. Pozostają na tym komputerze; jeśli zapiszesz je do pliku, zwróć uwagę, komu go przekazujesz.
gps-heading = Dane wspomagające GPS
gps-hint = Przeciągnij tu plik, wpisz ścieżkę lub pozostaw pole puste, aby pobrać aktualne dane od Sony.
btn-gps-devices = Zgodne urządzenia
btn-gps-devices-hide = Ukryj zgodne urządzenia
gps-devices-note = Te modele mają wbudowany moduł GPS. W wielu z nich ma go tylko wariant, którego nazwa kończy się literą V; warianty bez V nie mają GPS.
gps-devices-compact = Aparaty kompaktowe (DSC)
gps-devices-slt = Aparaty SLT (mocowanie A)
gps-devices-action = Kamery sportowe
gps-devices-camcorder = Kamery wideo (Handycam)
gps-col-variant = Wariant z GPS
gps-col-arch = Architektura
gps-col-module = Moduł GPS
gps-col-apps = Obsługa aplikacji
gps-yes = tak
gps-no = nie
gps-unknown = nieznane
gps-devices-untested = GPS obecny, obsługa nieprzetestowana
gps-devices-untested-note = Te modele mają wbudowany moduł GPS, ale nie ma ich na liście urządzeń OpenMemories. Nie sprawdzono, czy ta aplikacja z nimi współpracuje, dlatego ich architektura i obsługa aplikacji są nieznane.
wifi-heading = Punkty dostępowe transmisji na żywo
wifi-hint = To są punkty dostępowe używane przez funkcję transmisji na żywo aparatu, a nie własne ustawienia Wi-Fi aparatu. Korpus bez funkcji transmisji odrzuca tę operację.
wifi-multi = Lista wielu punktów dostępowych
wifi-none = Nie zapisano żadnych punktów dostępowych.
wifi-writing = Zapisywanie punktów dostępowych transmisji na żywo w aparacie…
wifi-confirm = Ta operacja zastąpi wszystkie punkty dostępowe zapisane w aparacie. Kontynuować?
wifi-empty-refused = Odmowa zapisu pustej listy, która skasowałaby wszystkie zapisane punkty dostępowe.
stream-heading = Transmisja na żywo
stream-writing = Zapisywanie ustawień transmisji na żywo w aparacie…
stream-confirm = Ta operacja nadpisze ustawienia transmisji na żywo w aparacie. Kontynuować?
stream-invalid-json = Ustawienia nie są prawidłowym formatem JSON:

# Aktualizacja oprogramowania układowego i oficjalny sklep
firmware-warning = Ostrzeżenie: ta operacja nadpisuje oprogramowanie układowe aparatu. Nie przerywaj aktualizacji po jej rozpoczęciu: nie odłączaj kabla USB i nie wyłączaj aparatu. Używaj wyłącznie pliku oprogramowania układowego przeznaczonego dokładnie dla tego modelu aparatu i upewnij się, że akumulator jest wystarczająco naładowany. W przeciwnym razie aparat może stać się trwale bezużyteczny.
firmware-untested = Niesprawdzone: zapis oprogramowania układowego został przeniesiony, ale nigdy nie był wypróbowany na prawdziwym aparacie. Nieudanego lub przerwanego zapisu nie da się cofnąć, więc traktuj wszystko na tej stronie jako niepotwierdzone.
firmware-confirm-required = Odmowa zapisu bez --confirm.
firmware-truncated = Plik oprogramowania układowego jest obcięty: payload FDAT wykracza poza koniec pliku.
firmware-updating = Aktualizacja oprogramowania układowego:
firmware-invalid = Ten plik nie jest prawidłowym obrazem oprogramowania układowego Sony
firmware-dat-path = Plik oprogramowania układowego (.dat)
firmware-drop-hint = Upuść tutaj plik oprogramowania układowego .dat
firmware-file-required = Najpierw wybierz plik oprogramowania układowego.
firmware-check-ok = Aparat akceptuje ten obraz. Nic nie zostało zapisane.
firmware-cancelled = Zapis anulowany. Aparat nie został zmodyfikowany.
firmware-current-version = Zainstalowana wersja
firmware-new-version = Wersja w tym obrazie
firmware-models-hint = Aparat jest zgodny, jeśli powłoka aktualizatora może się z nim połączyć.
btn-firmware-check = Sprawdź obraz
btn-firmware-flash = Zapisz oprogramowanie układowe
firmware-own-device = Potwierdź, że znasz te zagrożenia i aktualizujesz wyłącznie własne urządzenie.
btn-firmware-accept = Rozumiem ryzyko – zapisz

# Narzędzia plikowe: operacje, które w ogóle nie potrzebują aparatu.
tools-hint = Te operacje działają na plikach i nigdy nie dotykają aparatu, więc nic tutaj nie może go uszkodzić. Są to te same operacje, które pmca-console oferuje jako apk2spk, spk2apk, print-backup, print-usagelog, guess-firmware i xpd-checksum.
tools-operation = Operacja
tools-apk2spk = Zapakuj plik .apk do kontenera .spk
tools-spk2apk = Rozpakuj plik .spk z powrotem do .apk
tools-print-backup = Pokaż zawartość pliku Backup.bin
tools-print-usagelog = Pokaż zawartość pliku Usagelog.bin
tools-guess-firmware = Znajdź payload wewnątrz pliku .dat oprogramowania układowego
tools-xpd-checksum = Oblicz sumę kontrolną CIC pliku
tools-input = Plik wejściowy
tools-file-required = Najpierw wybierz pliki, których potrzebuje ta operacja.
tools-apk-invalid = Nie udało się zapakować pliku jako .spk
tools-spk-invalid = Plik nie jest prawidłowym pakietem .spk
tools-backup-invalid = Plik nie jest prawidłowym obrazem Backup.bin
tools-usagelog-invalid = Plik nie jest prawidłowym dziennikiem użycia
tools-reset-value = wartość początkowa:
tools-fdat-found = Payload znaleziony pod przesunięciem { $offset }, rozmiar { $size }.
tools-checksum = Suma kontrolna
tools-written = Zapisano do { $path }

# Odsyłacze do dokumentacji projektu
docs-compatibility = Zgodność aparatów
docs-compatibility-hint = Pierwotny projekt udostępnił przegląd sprawdzonych aparatów. Obejmuje on głównie starsze modele i znajduje się pod tym adresem:
devices-note = Dane z przeglądu pierwotnego projektu. Liczba przy obsłudze aplikacji to wersja platformy aplikacji aparatu.
devices-more-note = Poniższe aparaty nie były wyraźnie testowane z tym programem. Znacznik w ostatniej kolumnie oznacza, że społeczność zgłosiła ich użycie. Jeśli dodatkowo powłoka serwisowa jest oznaczona ✖, użycie się nie powiodło.
btn-more-models = Inne urządzenia
btn-more-models-hide = Ukryj inne urządzenia
check-updater-yes = Twoje urządzenie { $model } jest zgodne: według listy urządzeń powłoka aktualizatora może się z nim połączyć.
check-updater-no = Twoje urządzenie { $model } nie jest zgodne: według listy urządzeń powłoka aktualizatora nie może się z nim połączyć.
check-updater-unknown = Zgodność Twojego urządzenia { $model } jest nieznana: nie ma go na żadnej liście urządzeń tego programu, więc nie wiadomo, czy powłoka aktualizatora może się z nim połączyć.
check-apps-no = Twoje urządzenie { $model } nie jest zgodne: według listy urządzeń nie obsługuje aplikacji.
check-apps-untested = Zgodność Twojego urządzenia { $model } jest nieznana: jest na liście urządzeń, ale obsługa aplikacji nie została potwierdzona.
check-apps-unknown = Zgodność Twojego urządzenia { $model } jest nieznana: nie ma go na żadnej liście urządzeń tego programu, więc nie wiadomo, czy obsługuje aplikacje.
check-gps-no = Twoje urządzenie { $model } nie jest zgodne: nie ma go na liście urządzeń z odbiornikiem GPS.
check-gps-untested = Zgodność Twojego urządzenia { $model } jest nieznana: według listy urządzeń ma odbiornik GPS, ale przesyłanie danych do tego modelu nie zostało przetestowane.
check-gps-unknown = Zgodność Twojego urządzenia { $model } jest nieznana: nie ma go na żadnej liście urządzeń tego programu, więc nie wiadomo, czy ma odbiornik GPS.
check-unidentified = Nie udało się zidentyfikować podłączonego urządzenia, więc jego zgodność jest nieznana.
check-proceed = Czy mimo to chcesz kontynuować?
check-cancelled = Operacja anulowana. Urządzenie nie zostało zmienione.
devices-col-confirmed = Potwierdzone
devices-group-ilc = Aparaty z wymienną optyką (mocowanie E i A)
devices-group-remote = Piloty Live-View
btn-copy-link = Kopiuj odnośnik
shutter-heading = Licznik migawki
shutter-hint = Aparat liczy każde wyzwolenie migawki i zapisuje bieżący stan w każdym zapisywanym zdjęciu (ARW, JPEG lub HEIF). Wybierz najnowsze zdjęcie, aby odczytać bieżący stan. Plik jest tylko odczytywany, nigdy nie jest zmieniany.
shutter-source = Położenie licznika w pliku zależy od modelu. Pozycje pochodzą z publicznie udokumentowanych badań (ExifTool) i nie zostały potwierdzone dla każdego modelu.
shutter-drop-hint = Upuść tutaj zdjęcie (ARW, JPEG, HEIF)
btn-shutter-read = Analizuj zdjęcie
shutter-file-required = Najpierw wybierz zdjęcie.
shutter-file = Plik
shutter-taken = Data wykonania
shutter-count = Wyzwolenia migawki
shutter-type = Migawka
shutter-type-mechanical = Mechaniczna
shutter-type-electronic = Elektroniczna
shutter-electronic-hint = To zdjęcie wykonano migawką elektroniczną. W wielu modelach takie zdjęcia nie są liczone; wartość pokazuje wtedy stan z ostatniego wyzwolenia mechanicznego.
shutter-not-sony = Zdjęcie nie zostało wykonane aparatem Sony. Nie można z niego odczytać licznika migawki.
shutter-unknown-model = Nie wiadomo, gdzie ten model aparatu zapisuje licznik migawki w pliku, dlatego żadna wartość nie jest wyświetlana.
shutter-not-recorded = Plik nie zawiera czytelnego licznika migawki. Zdarza się to w zdjęciach zapisanych lub wyeksportowanych przez program do edycji obrazów, a w niektórych modelach w zdjęciach wykonanych migawką elektroniczną. Wybierz oryginalny plik z karty pamięci.
shutter-model-mismatch = Zdjęcie pochodzi z innego modelu aparatu niż podłączony aparat.
shutter-invalid = Plik nie jest czytelnym zdjęciem
file-filter-photo = Zdjęcia (ARW, JPEG, HEIF)
usage-log-heading = Dziennik użycia
usage-log-hint = Aparat prowadzi wewnętrzny dziennik w nieudokumentowanym formacie. Jest on zapisywany bez zmian; z aparatu następuje wyłącznie odczyt.
usage-log-written = Zapisano w
btn-usage-log-info = O dzienniku użycia
btn-usage-log-info-hide = Ukryj objaśnienie
usage-log-info-what-heading = Czym jest ten dziennik
usage-log-info-what = Sony wewnętrznie nazywa ten dziennik „KikiLog”. Aparat zapisuje go sam; ten program go tylko odczytuje. Nie jest znany żaden publiczny opis formatu.
usage-log-info-read = Odczyt niczego nie zmienia: kilka kolejnych pobrań było identycznych bajt w bajt.
usage-log-info-flush-heading = Dlaczego brakuje nowych wpisów
usage-log-info-flush = Nowe wpisy często nie pojawiają się od razu: aparat może zapisywać je dopiero przy wyłączeniu, czasem rozłożone na kilka cykli włączenia i wyłączenia. Wyłącz i włącz aparat, pobierz ponownie i powtarzaj, aż przestaną pojawiać się nowe wpisy.
usage-log-info-content-heading = Co można z niego odczytać
usage-log-info-shutter = Wyzwolenia migawki: niektóre zdarzenia odpowiadają każde jednemu zdjęciu. Ich liczba nie jest jednak licznikiem migawki: dziennik ma stały rozmiar i nie musi obejmować całego okresu użytkowania aparatu. Aparat przechowuje łączny stan gdzie indziej i zapisuje go w zapisywanych zdjęciach; można go odczytać w sekcji „Licznik migawki”.
usage-log-info-clock = Znaczniki czasu są tak dokładne jak zegar aparatu. Po wymianie akumulatora zegar może się cofnąć i tworzyć wpisy z początkową datą aparatu.
usage-log-info-unknown = Nie jest znany żaden publiczny opis numerów zdarzeń. Program pokazuje numery bez ich interpretacji.
usage-log-info-privacy-heading = Prywatność
usage-log-info-privacy = Plik zawiera numery seryjne włożonych kart pamięci. Nie udostępniaj ani nie publikuj go bezrefleksyjnie.
btn-usage-log = Zapisz dziennik
usagelog-size = Obszar dziennika
usagelog-cursor = Pozycja zapisu
usagelog-spare = Drugi obszar
usagelog-count = Wpisy
usagelog-events = Zdarzenia według numeru
usagelog-entries = Wpisy w kolejności
msg-link-copied = Odnośnik skopiowano do schowka.

# Komunikaty o błędach zaplecza, przypisane do stałego kodu zgłaszanego przez zaplecze.
# Zmienna część komunikatu (ścieżka, bajt stanu, błąd urządzenia) jest
# dołączana przez warstwę prezentacji, więc czyta się je jak pełne zdania i bez niej.
# Celowo nie ma wpisu `err-operation`: ten kod oznacza komunikat już
# przetłumaczony przez zaplecze, który trzeba pokazać dosłownie.
err-usb-no-device = Nie znaleziono aparatu. Podłącz aparat i włącz go.
err-usb-camera-not-ready = Aparat jest podłączony, ale nie jest gotowy: jest wyłączony albo czeka na ekranie na wybór trybu USB. Włącz go lub wybierz w aparacie „Pamięć masowa (MSC)”.
camera-choose-usb-mode = Aparat czeka na wybór trybu USB. Wybierz w aparacie „Pamięć masowa (MSC)”.
err-usb-too-many-devices = Podłączono więcej niż jeden aparat. Odłącz wszystkie oprócz tego, z którym chcesz pracować.
err-usb-transfer = Transfer USB nie powiódł się. Spróbuj innego kabla lub portu i unikaj koncentratorów USB.
err-usb-permission = Program nie ma uprawnień do otwarcia aparatu przez USB. W systemie Linux to prawo nadaje się jednorazowo regułą udev dla identyfikatora producenta Sony; skrypt scripts/linux-prepare.sh w źródłach ją instaluje. Następnie odłącz aparat i podłącz go ponownie.
err-usb-busy = Z aparatu korzysta inny program, więc nie udało się go otworzyć. Zamknij inne programy, które mogą mieć do niego dostęp; w systemie Linux może go przytrzymywać monitor multimediów środowiska, który zatrzymuje skrypt scripts/linux-prepare.sh. W systemie macOS ustaw połączenie USB aparatu na MTP: w trybie pamięci masowej trzyma go systemowy sterownik dysku.
err-usb-needs-root = macOS przekazuje połączenie pamięci masowej aparatu programowi tylko wtedy, gdy działa on z uprawnieniami administratora. Uruchom program wiersza poleceń przez sudo, na przykład: sudo pmca-console tweaks. Informacje o aparacie można odczytać także bez tego, jeśli połączenie USB aparatu jest ustawione na MTP.
macos-admin-prompt = PMCA-RE RELOADED potrzebuje uprawnień administratora, aby odebrać systemowi macOS połączenie pamięci masowej aparatu. Karta znika w tym czasie z Findera; jeśli są na niej otwarte pliki, najpierw ją tam wysuń.
macos-admin-declined = Nie przyznano uprawnień administratora, więc aparatu nie zmieniono. W systemie macOS ta funkcja ich wymaga.
macos-admin-failed = Program nie mógł uruchomić swojego pomocnika z uprawnieniami administratora.
err-usb-stall = Aparat przerwał transfer USB.
err-usb-protocol = Aparat odesłał nieoczekiwaną odpowiedź.
err-usb-msc-status = Aparat odrzucił polecenie pamięci masowej.
cli-error-prefix = błąd
err-usb-driver = System Windows przypisał temu aparatowi sterownik, który nie zezwala na bezpośredni dostęp. Ten tryb wymaga sterownika WinUSB, który aplikacja może przypisać sama — ale tylko z uprawnieniami administratora.
err-needs-elevation = Przypisanie lub zwolnienie sterownika wymaga uprawnień administratora, a zapytanie systemu Windows zostało odrzucone. Spróbuj ponownie przyciskiem „Uruchom pomocnika sterowników”; w aparacie nic nie trzeba zmieniać.
err-usb-no-mass-storage-volume = Aparat jest podłączony w trybie pamięci masowej, ale nie udostępnia żadnego dysku. Wybudź go, potwierdź tryb USB na jego ekranie, jeśli o to prosi, i sprawdź, czy włożona jest karta pamięci. Nie zmieniaj sterownika: ten tryb wymaga właśnie standardowego sterownika systemu Windows.
err-usb-msc-sense = Aparat zgłosił błąd pamięci masowej.
err-usb-ptp-response = Aparat odrzucił polecenie.
err-usb-unsupported = Ten model aparatu nie obsługuje tej funkcji.
err-usb-updater-status = Aktualizator oprogramowania układowego w aparacie odrzucił tę operację.
err-usb-senser = Polecenie trybu serwisowego nie powiodło się.
err-market-core = Nie udało się odczytać pliku danych aparatu.
err-market-json = Odebrane dane są uszkodzone.
err-market-missing-field = W odpowiedzi zabrakło oczekiwanego pola.
err-market-no-apk = Nie wybrano aplikacji do instalacji.
err-market-http = Żądanie sieciowe nie powiodło się.
err-market-io = Lokalny serwer instalacji zgłosił błąd.
err-market-tls = Nie udało się nawiązać szyfrowanego połączenia z aparatem.
err-apk = Nie udało się odczytać pliku aplikacji.
err-io = Nie udało się odczytać lub zapisać pliku.
err-json = Aparat przesłał uszkodzone dane.
err-unknown-locale = Ten język jest niedostępny.
err-catalog-encoding = Plik tłumaczenia nie jest zapisany jako tekst UTF-8. Zapisz go ponownie w kodowaniu UTF-8.
err-internal = Wystąpił błąd wewnętrzny. To jest błąd w aplikacji.

## Function analysis
nav-analysis = Analiza
analysis-title = Analiza funkcji
analysis-experimental = Eksperymentalne. Ta funkcja jest nowa i wypróbowano ją tylko na kilku aparatach.
analysis-own-hardware = Używaj jej tylko na własnym aparacie. Już sam tryb serwisowy jest ingerencją, której Sony nie przewiduje; odpowiedzialność spoczywa na Tobie.
analysis-explanation-toggle = Wyjaśnienie: cel, przebieg, ryzyko
analysis-purpose-heading = Do czego służy?
analysis-purpose = Analiza funkcji dokumentuje, w której wartości pamięci ustawień (kopii zapasowej) aparat zapisuje jedno z własnych ustawień menu. Wynik opisuje pamięć ustawień tego aparatu; można go porównać z innymi aparatami, ale nie zmienia on tego, co aparat oferuje.
analysis-how-heading = Jak przebiega?
analysis-how = Dla każdej podanej wartości menu program odczytuje raz całą pamięć ustawień, za każdym razem w osobnej krótkiej sesji trybu serwisowego. Pomiędzy odczytami odłączasz kabel i zmieniasz ustawienie w menu aparatu; aparat pozostaje włączony. Na końcu pierwsza wartość jest odczytywana jeszcze raz. Wartości, które podążają za menu i na końcu wracają do wartości początkowej, są kandydatami. Wartości zmieniające się samoczynnie (liczniki, zegar) są odrzucane.
analysis-requirements-heading = Wymagania
analysis-requirements = Aparat jest podłączony przez USB, jego tryb USB jest ustawiony na „Auto”, a po każdej sesji wybierasz na aparacie „Pamięć masowa (MSC)”. W systemie Windows program potrzebuje uprawnień administratora, ponieważ na chwilę przypisuje sterownik dla trybu serwisowego. Naładuj akumulator; każda sesja trwa około minuty.
analysis-read-heading = Kroki odczytu
analysis-read = Migawki tylko odczytują. W ustawieniach aparatu nic się nie zmienia. Ryzyko jest niewielkie, ale nie zerowe: już przełączenie w tryb serwisowy jest ingerencją, której Sony nie przewiduje.
analysis-write-heading = Krok zapisu (test zapisu)
analysis-write = Tylko jeśli wyraźnie go włączysz: program zapisuje znalezioną wartość w aparacie, po ponownym uruchomieniu sprawdzasz w menu, czy działa, a potem zapisywana jest z powrotem wartość pierwotna. Zapisywana jest tylko wartość, którą aparat sam zapisał, i tylko wtedy, gdy znaleziono dokładnie jednego pasującego kandydata. Stara wartość jest wcześniej zabezpieczana. Mimo to ten krok zmienia aparat; błąd może pozostawić go w stanie, który naprawi tylko serwis.
analysis-legal-heading = Kwestie prawne
analysis-legal = Oprogramowanie jest dostarczane bez jakiejkolwiek gwarancji (licencja MIT). Ingerencje w aparat mogą wpłynąć na jego gwarancję. Używaj tej funkcji tylko na aparatach, które do Ciebie należą. Raport analizy nie zawiera numeru seryjnego ani niczego z pamięci ustawień poza znalezionymi kandydatami; nigdy nie jest wysyłany automatycznie.
analysis-start-heading = Nowa analiza
analysis-setting-label = Ustawienie menu, tak jak nazywa je aparat
analysis-values-label = Wartości menu, oddzielone przecinkami
analysis-values-hint = Pierwsza wartość to ta ustawiona teraz. Od dwóch do ośmiu wartości; każda kosztuje jedną sesję trybu serwisowego.
analysis-write-option = Następnie wykonaj test zapisu (zmienia aparat)
analysis-write-warning = Test zapisu zmienia aparat: zapisuje wartość w jego pamięci ustawień, a potem zapisuje z powrotem wartość pierwotną. Tylko na własnym aparacie, na własne ryzyko.
analysis-write-consent = Aparat należy do mnie i biorę na siebie ryzyko.
btn-analysis-write-enable = Włącz test zapisu
btn-analysis-start = Rozpocznij analizę
btn-analysis-snapshot = Odczytaj kopię zapasową
btn-analysis-new = Nowa analiza
analysis-discard-confirm = Odrzucić trwającą analizę? Dotychczasowy raport pozostaje zapisany.
analysis-access-read = ODCZYT
analysis-access-write = ZAPIS
analysis-access-user = RĘCZNIE
analysis-access-local = PC
analysis-step-snapshot = Odczytaj kopię zapasową, ustawienie ma wartość „{ $value }”
analysis-step-change = Zmień na „{ $value }” w menu aparatu
analysis-step-evaluate = Porównaj migawki
analysis-step-write = Zapisz w aparacie wartość dla „{ $value }”
analysis-step-check = Wyłącz i włącz aparat, odczytaj menu (oczekiwane: „{ $value }”)
analysis-step-restore = Zapisz z powrotem pierwotną wartość dla „{ $value }”
analysis-step-report = Zapisz raport analizy
analysis-do-prepare = PRZYGOTOWANIE: 1. W menu aparatu ustaw „{ $setting }” na „{ $value }” i wyjdź z menu przyciskiem MENU, aby aparat zapisał ustawienie. 2. Pozostaw aparat włączony. 3. Podłącz kabel i wybierz na aparacie „Pamięć masowa (MSC)”.
analysis-do-change = TWOJA KOLEJ: 1. Odłącz kabel. 2. Aparat pozostaje włączony. 3. W menu aparatu ustaw „{ $setting }” na „{ $value }” i wyjdź z menu przyciskiem MENU, aby aparat zapisał ustawienie. 4. Podłącz ponownie kabel i wybierz „Pamięć masowa (MSC)”.
analysis-do-check = TWOJA KOLEJ: 1. Odłącz kabel. 2. Wyłącz i ponownie włącz aparat. 3. Sprawdź w menu, co jest ustawione dla „{ $setting }”.
analysis-do-reconnect = Podłącz ponownie kabel i wybierz na aparacie „Pamięć masowa (MSC)”.
analysis-reading = Migawka { $index } z { $total }: NIE odłączaj kabla podczas odczytu.
analysis-cli-continue = Enter, aby kontynuować, q, aby anulować:
analysis-confirm-word = tak
analysis-cli-confirm = Wpisz „{ $word }”, aby potwierdzić:
analysis-write-declined = Nie potwierdzono. Nic nie zostało zapisane.
analysis-aborted = Anulowano. Nic nie zostało zapisane.
analysis-snapshot-done = Migawka odczytana. Aparat opuścił tryb serwisowy.
analysis-result = Porównano { $compared } wartości: { $count } kandydatów, { $running } wartości zmieniło się samoczynnie.
analysis-candidate-consistent = jednoznacznie pasuje do menu
analysis-candidate-inconsistent = nie pasuje jednoznacznie
analysis-write-preview-heading = To zostanie zapisane
analysis-write-preview = Wartość { $id } ({ $size } B): z { $old } („{ $old-label }”) na { $new } („{ $new-label }”). Aparat sam zapisał nową wartość, gdy ustawione było „{ $new-label }”.
btn-analysis-write = Zapisz
btn-analysis-skip-write = Zakończ bez testu zapisu
analysis-written = Wartość zapisana, odczytana ponownie i trwale zachowana.
analysis-menu-question = Co teraz pokazuje menu?
analysis-menu-other = coś innego
btn-analysis-menu-confirm = Zatwierdź
btn-analysis-restore = Zapisz z powrotem pierwotną wartość
analysis-restored = Pierwotna wartość zapisana z powrotem i trwale zachowana.
analysis-survived = Zapisana wartość nadal była zachowana po ponownym uruchomieniu.
analysis-not-survived = Zapisana wartość nie była już zachowana po ponownym uruchomieniu.
analysis-report-saved = Raport analizy zapisany: { $path }
analysis-report-hint = Raport nie zawiera numeru seryjnego ani kopii zapasowej. Możesz go przekazać, aby wynik mógł zostać sprawdzony i udokumentowany.
btn-analysis-export = Eksportuj raport…
file-filter-analysis = Raport analizy
analysis-pending = Test zapisu nie został ukończony. Pierwotna wartość jest zabezpieczona i można ją zapisać z powrotem.
analysis-pending-none = Dla tego aparatu nie zapisano żadnego nieukończonego testu zapisu.
analysis-invalid-setting = Podaj ustawienie menu tak, jak nazywa je aparat.
analysis-invalid-values = Podaj od dwóch do { $max } różnych wartości menu. Pierwsza to ta ustawiona teraz.
analysis-already-complete = Wszystkie migawki zostały już odczytane.
analysis-other-camera = To nie jest ten sam aparat co przy pierwszej migawce. Analizę zatrzymano.
analysis-write-not-allowed = Test zapisu nie jest możliwy w tym stanie.
analysis-camera-changed = Aparat ma teraz inną wartość niż przy pierwszej migawce. Nic nie zostało zapisane.
analysis-write-refused = Aparat odrzucił wartość; wszystko jest jak przedtem. ({ $detail })
analysis-refuse-none = Test zapisu niemożliwy: żadna wartość nie podążała za menu.
analysis-refuse-ambiguous = Test zapisu niemożliwy: { $count } wartości podążało za menu, żadnej nie da się pewnie przypisać.
analysis-refuse-inconsistent = Test zapisu niemożliwy: wartość nie pasuje jednoznacznie do wartości menu.
analysis-refuse-size = Test zapisu niemożliwy: wartość ma { $size } B; zapisywane są tylko małe wartości.
analysis-refuse-read-only = Test zapisu niemożliwy: aparat oznacza wartość jako tylko do odczytu.
analysis-refuse-protected = Test zapisu niemożliwy: aparat oznacza wartość jako chronioną.
settings-analysis-heading = Analiza funkcji
settings-analysis-purpose = Analiza funkcji dokumentuje, w której wartości pamięci ustawień aparat zapisuje jedno z własnych ustawień menu. Jest przeznaczona dla osób, które chcą zrozumieć pamięć ustawień swojego modelu i udostępnić wyniki.
settings-analysis-why-off = Jest wyłączona, ponieważ zwykłe użytkowanie jej nie wymaga, ponieważ kilka razy przełącza aparat w tryb serwisowy i ponieważ na życzenie także zapisuje w aparacie. Jest eksperymentalna i wypróbowano ją tylko na kilku modelach.
settings-analysis-toggle = Włącz analizę funkcji (pozycja „Analiza” w menu)
settings-analysis-confirm = Włączyć analizę funkcji? Kilka razy przełącza aparat w tryb serwisowy i na życzenie może zapisywać w aparacie. Używaj jej tylko na własnym aparacie.
btn-analysis-enable = Włącz
settings-analysis-on = Analiza funkcji jest włączona.
settings-analysis-off = Analiza funkcji jest wyłączona.
analysis-needs-admin = Każda migawka wymaga uprawnień administratora dla trybu serwisowego. Windows pyta o nie raz na sesję programu; możesz uruchomić pomocnika sterowników teraz albo poczekać, aż poprosi o niego pierwsza migawka.
analysis-changed-hint = Sprawdź: nie znaleziono żadnego kandydata. Poniższe wartości się zmieniły, ale nie wróciły do wartości początkowej. Jeśli któraś z nich zmienia się tylko raz między dwiema wartościami, aparat przy pierwszej migawce prawdopodobnie nie był ustawiony na pierwszą wartość menu. Powtórz wtedy analizę z poprawnie ustawioną wartością początkową.
analysis-changed-info = Info: wartości, które zmieniły się samoczynnie podczas analizy (np. liczniki, zegar), są pomijane. Nic nie musisz robić.
elevation-admin = Administrator
elevation-standard = Uprawnienia standardowe
elevation-admin-hint = Program działa z uprawnieniami administratora i może przypisać sterownik potrzebny trybowi serwisowemu.
elevation-standard-hint = Program działa ze standardowymi uprawnieniami. Dla trybu serwisowego, tweaków i analizy funkcji w razie potrzeby uruchamia pomocnika sterowników z uprawnieniami administratora; Windows pyta raz na sesję. Kliknij, aby go uruchomić.
driver-helper-declined = Odrzucono zapytanie o uprawnienia administratora. Bez nich nie można przypisać sterownika dla trybu serwisowego; nic nie zostało zmienione.
btn-remove-foreign-drivers = Usuń stare sterowniki
btn-not-now = Nie teraz
btn-ok = OK
foreign-drivers-found = W systemie Windows nadal są sterowniki, które Zadig lub podobne narzędzie zainstalowało dla urządzeń Sony: { $packages }. Windows przydziela im aparat, zanim ten program zdąży przypisać własny sterownik; przez to tryb serwisowy kończy się niepowodzeniem, a aparat może nie pojawiać się jako dysk. Usunąć je teraz? Aparat będzie potem korzystał ze sterowników dostarczanych z systemem Windows. Usunięcie wymaga jednorazowo uprawnień administratora.
foreign-drivers-removed = Stare sterowniki zostały usunięte. Odłącz aparat i podłącz go ponownie.
foreign-drivers-remaining = Nie udało się usunąć tych sterowników: { $packages }. Uruchom ponownie system Windows i spróbuj jeszcze raz albo odinstaluj urządzenie razem ze sterownikiem w Menedżerze urządzeń.
foreign-drivers-hint = Nadal są zainstalowane sterowniki, które Zadig lub podobne narzędzie zainstalowało dla urządzeń Sony ({ $packages }). Windows przydziela im aparat, zanim ten program zdąży przypisać własny sterownik, przez co tryb serwisowy może się nie udać. Usuń je poleceniem: pmca-console driver zadig --remove
driver-zadig-none = Dla urządzeń Sony nie są zainstalowane żadne sterowniki z Zadig ani podobnych narzędzi.
driver-zadig-entry = { $package } ({ $provider }): { $devices }
driver-zadig-remove-hint = Usuń je poleceniem: pmca-console driver zadig --remove
driver-zadig-removed = Usunięto stare pakiety sterowników: { $count }. Odłącz aparat i podłącz go ponownie.
elevation-helper = Pomocnik sterowników aktywny
elevation-helper-hint = Pomocnik z uprawnieniami administratora przypisuje sterownik dla trybu serwisowego. Nie ma okna i kończy działanie razem z programem; kolejnych zapytań nie będzie.
elevation-helper-confirm = Uruchomić teraz pomocnika sterowników? Windows zapyta raz o uprawnienia administratora i nie zrobi tego ponownie do zamknięcia programu.
analysis-cross-heading = Sprawdzenie na innym aparacie
analysis-cross-intro = Opcjonalnie: sprawdza, czy inny aparat również ma znalezioną wartość i co jest w nim ustawione. Tylko odczyt. Ten sam numer nie musi oznaczać tej samej funkcji; potwierdzi to dopiero spojrzenie do menu drugiego aparatu. Wynik trafia do tego samego raportu analizy.
analysis-cross-do = 1. Odłącz analizowany aparat. 2. Włącz drugi aparat, ustaw jego tryb USB na „Auto”, podłącz kabel i wybierz „Pamięć masowa (MSC)”.
btn-analysis-cross = Sprawdź inny aparat
analysis-cross-present = jest, wartość { $value }
analysis-cross-match = odpowiada „{ $label }”
analysis-cross-nomatch = nie odpowiada żadnej zmierzonej wartości menu
analysis-cross-absent = brak
analysis-cross-same-camera = To jest sam analizowany aparat. Podłącz drugi aparat.
analysis-cross-not-now = Sprawdzenie jest możliwe dopiero, gdy wszystkie migawki są odczytane i żadna zapisana wartość nie czeka na przywrócenie.
analysis-cross-none = Nie ma kandydata do sprawdzenia.
analysis-wrong-camera = Podłączony aparat nie jest analizowanym. Podłącz { $model }; nic nie zostało zapisane.
keys-fetching = Pobieranie źródła…
keys-scanning = Wyszukiwanie materiału klucza…
keys-complete = Cały materiał klucza jest obecny.
keys-incomplete = Część materiału klucza wciąż brakuje.
keys-present = obecny
keys-missing = brak
keys-class-senser = Klucze trybu serwisowego (powłoka serwisowa, poprawki, analiza)
keys-class-cic = Klucz sumy kontrolnej XPD (instalacja aplikacji)
keys-class-spk = Materiał klucza SPK (instalacja aplikacji, apk2spk)
keys-class-fdat = Materiał klucza FDAT (firmware updater-shella)
plugin-heading = Wtyczki
plugin-intro = Wtyczki rozszerzają ten program o języki, przełączniki ustawień aparatu, informacje o zgodności, narzędzia i diagnostykę. Pochodzą wyłącznie z plików wybranych przez użytkownika – sklepu z wtyczkami nie ma. Ich kod działa w piaskownicy bez dostępu do plików, sieci ani materiału kluczy. Należy instalować tylko wtyczki zaufanych autorów; zmiany obowiązują od następnego uruchomienia.
plugin-none = Brak zainstalowanych wtyczek.
plugin-inactive = (aktywna po ponownym uruchomieniu)
plugin-author = Autor
plugin-restart = Zmiana zacznie obowiązywać po ponownym uruchomieniu programu.
btn-plugin-install = Zainstaluj wtyczkę…
plugin-install-confirm = Ta wtyczka może:
btn-confirm-plugin-install = Zainstaluj wtyczkę
plugin-installed = Wtyczka została zainstalowana.
plugin-remove-confirm = Usunąć wtyczkę „{ $name }”?
plugin-removed = Wtyczka { $id } została usunięta.
plugin-perm-languages = dodawać języki do programu
plugin-perm-camera-settings = zmieniać ustawienia aparatu (tylko między dwiema wskazanymi przez nią wartościami i dopiero po potwierdzeniu)
plugin-perm-compatibility = dodawać wpisy do przeglądu zgodności
plugin-perm-code = uruchamiać własny kod w piaskownicy (narzędzia lub diagnostyka)
plugin-perm-host-os = sprawdzać, jaki to system operacyjny
plugin-perm-host-cameras = widzieć, które aparaty Sony są podłączone i w jakim trybie
plugin-invalid = To nie jest użyteczna wtyczka: { $reason }
plugin-unknown-field = Ta wtyczka korzysta z funkcji, której ta wersja programu nie zna ({ $field }). Została przygotowana dla nowszej lub innej edycji programu.
plugin-no-folder = Nie udało się ustalić folderu danych użytkownika, więc nie można instalować wtyczek.
plugin-not-installed = Nie zainstalowano wtyczki o identyfikatorze { $id }.
plugin-failed = Wtyczka zakończyła się błędem: { $reason }
plugin-unknown-entry = Żadna wczytana wtyczka nie udostępnia { $id }.
plugin-run-done = Wtyczka zakończyła pracę.
plugin-run-warning = Wtyczka zakończyła pracę z uwagami.
plugin-run-error = Wtyczka zgłosiła błąd.
plugin-tools-heading = Narzędzia wtyczek
plugin-tools-hint = Pochodzą z zainstalowanych wtyczek i działają w piaskownicy. Wtyczka otrzymuje tylko to, co zostanie jej przekazane, i to, co zadeklarowała, oraz nie może zmienić aparatu.
plugin-diagnostics-heading = Diagnostyka wtyczek
plugin-compat-heading = Z wtyczki
plugin-col-note = Uwaga
file-filter-plugin = Plik wtyczki
keys-not-set-up = Materiał klucza wymagany przez tę operację nie jest skonfigurowany.
err-missing-keys = Materiał klucza wymagany przez tę operację nie jest skonfigurowany.
keys-heading = Wymagany materiał klucza
keys-intro = Klucze trybu serwisowego, klucz sumy kontrolnej XPD i materiał klucza SPK nie są częścią tego programu. Podaj adres źródła, które masz — repozytorium, archiwum lub plik — a program znajdzie i zaimportuje rozpoznane części. Nic nie jest uruchamiane, a materiał pozostaje tylko na tym komputerze.
keys-law-notice = Przestrzegaj przepisów obowiązujących w Twoim miejscu i pracuj tylko z aparatami i danymi, które należą do Ciebie.
keys-url-label = Adres źródła (jeden na wiersz)
keys-url-required = Podaj co najmniej jeden adres źródła.
btn-keys-import = Importuj
btn-import-keys = Skonfiguruj materiał klucza
