# PMCA-RE RELOADED 繁體中文（台灣）本地化。
# 每行一則訊息：<id> = <text>。若要新增語言，請將此檔案複製到
# locales/<lang>/pmca.ftl，並且只翻譯等號右側的文字。

app-title = PMCA-RE RELOADED – 相機工具箱

# 導覽 / 索引標籤
nav-camera-info = 相機資訊
nav-install-app = 應用程式
nav-tweaks = 調整項目
nav-network = 網路
nav-firmware = 韌體
nav-updater = 更新模式終端機
nav-service = 服務模式終端機
nav-tools = 檔案工具
nav-settings = 設定
nav-license = 授權

# 歡迎頁：視窗開啟時顯示的頁面。說明本工具箱的用途、
# 第一步該做什麼，以及哪個頁面回答哪個問題。
#
# 「語言」在本應用程式中有兩種不同的意思：
# 一是本視窗的語言，二是相機本身選單所提供的語言。
# 兩者各自有一個標題，而不只是各占一個子句，
# 因為混淆兩者正是本頁面要防止的錯誤。
nav-welcome = 歡迎
welcome-intro-heading = 本程式的用途
welcome-intro-body = 本程式可協助您保留 Sony 相機的功能，並依您的需求加以調整。Sony 關閉 PlayMemories Camera Apps 商店後，已購買或已安裝的應用程式便無從備份，也無法重新安裝。重設相機會永久刪除已安裝的應用程式，且無法復原。本程式可以備份現有的應用程式並重新安裝，也能提供可自由取得的社群應用程式。您也可以安裝自己的應用程式（*.apk 檔案）。發生問題時，本程式會讀取相機資訊與內部記錄以供分析。相機本身具備、但選單中未提供的設定可以切換，也可以再次復原。本程式透過 USB 傳輸線直接與相機通訊。支援的機型列於「相機相容性」頁面。
welcome-start-heading = 第一步
welcome-start-body = 請先開啟相機電源，並在尚未插上傳輸線時，於相機選單中將「USB 連接」設為大量儲存或 MTP。「自動」通常也可以，因為相機會自行選定其中一種。順序很重要：一旦插上傳輸線，就無法再進入相機的選單。接著才用 USB 連接相機，開啟「相機資訊」，然後按下「取得相機資訊」。只要型號和韌體版本出現，就表示連線正常，本程式的其他所有功能也都可以使用。
welcome-start-driver = 如果出現的是驅動程式提示，請先依照該提示操作。在 Windows 上，系統驅動程式會占用相機，必須先將其替換，才能進行本工具箱所需的直接 USB 存取。本程式會自行完成這項工作，但指派驅動程式是 Windows 只授予系統管理員的權限。在這種情況下，本程式會提供以系統管理員身分重新啟動的選項，Windows 會要求您確認此操作。工作階段結束時，此指派會再次解除。在 Linux 和 macOS 上不需要這麼做。
welcome-start-driver-linux = 如果出現的是驅動程式提示，請先依照該提示操作。在 Linux 上不需要替換任何東西：可以直接存取相機。只需一次性準備開啟裝置的權限——一條針對 Sony 廠商 ID 的 udev 規則——以及在大量儲存模式下釋放核心驅動程式 usb-storage。這兩件事都需在本程式之外以 root 權限完成；原始碼樹中的指令碼 scripts/linux-prepare.sh 會同時處理兩者。本視窗本身從不要求此類權限。
welcome-start-driver-macos = 如果出現的是驅動程式提示，請先依照該提示操作。在 macOS 上不需要安裝或替換任何東西。請將相機的 USB 連接設為 MTP。使用調整項目、服務模式終端機、功能分析、韌體更新和更新模式終端機時，本視窗會要求您輸入一次系統管理員密碼：這些功能需透過相機的大量儲存連線進行，而 macOS 只會將該連線交給具備這種權限的程式。
welcome-tweaks-heading = 調整項目與語言
welcome-tweaks-body = 調整項目是相機本身已具備、但未在選單中提供的設定：錄影時間限制、4K 錄影限制、PAL 與 NTSC 的選擇、USB 應用程式安裝程式——以及選單語言。
welcome-tweaks-languages-heading = 相機的語言
welcome-tweaks-languages-body = Sony 相機只會顯示其銷售地區常用的選單語言。如果您住在國外，常常找不到自己的語言，只能將就使用陌生的語言。然而，韌體中其實已內建更多語言。本程式會將它們加入選單，讓您能以熟悉的語言操作相機。
welcome-tweaks-ui-heading = 本程式的語言
welcome-tweaks-ui-body = 與上述這一切無關的是頂端的選擇器。它只會變更本程式的語言，絕不會動到相機。
welcome-tweaks-safety = 每個調整項目都會寫入相機的備份記憶體，而且每一項都能以同一個開關復原。寫入中斷可能會永久損壞相機，因此寫入進行期間請勿拔除傳輸線或取出電池。
welcome-privacy-heading = 您的資料
welcome-privacy-where = 「授權」頁面會再次完整說明這一切，並列出構成本程式的每個程式庫的授權條款。
welcome-where-heading = 功能位置一覽
welcome-where-info = 型號、序號與韌體版本。這同時也是連線測試。
welcome-where-compat = 哪些相機型號支援應用程式、更新程式 Shell 與服務 Shell。不需要相機。
welcome-where-install = 社群清單中的應用程式，或您自己的 .apk 檔案。
welcome-where-tweaks = 相機選單無法觸及的設定，其中也包括語言。
welcome-where-network = GPS 輔助資料，以及直播所用的存取點與服務。
welcome-where-firmware = 將 Sony 韌體映像檔寫入相機。
welcome-where-tools = 解開並檢視韌體與套件檔案。不需要相機。
welcome-where-shells = 直接存取相機的記憶體與檔案系統。須先在「設定」中開啟。
welcome-where-settings = 佈景主題、進階模式，以及編輯本程式的翻譯。
welcome-where-license = 這個移植版本的來源、建立在哪些人的成果之上，以及適用的條款。
welcome-where-hint = 任何頁面都不會自行對相機執行動作。在您按下明確標示會進行讀取或寫入的按鈕之前，不會讀取或寫入任何內容。

# 按鈕
btn-get-info = 取得相機資訊
btn-refresh = 重新整理
btn-load-apps = 載入可用的應用程式
btn-list-tweaks = 顯示可用的調整項目
btn-install = 安裝選取的應用程式
btn-install-file = 安裝 .apk 檔案…
btn-apply = 套用
btn-save = 儲存到相機
btn-add = 新增
btn-remove = 移除
btn-reveal = 顯示
btn-update-gps = 將 GPS 輔助資料傳送到裝置
btn-cancel = 取消
btn-browse = 瀏覽…
btn-switch-msc = 切換到大量儲存模式
btn-start-driver-helper = 啟動驅動程式協助工具

# 檔案對話方塊所提供的檔案類型篩選器標籤
file-filter-apk = Android 應用程式（*.apk）
file-filter-firmware = Sony 韌體映像檔（*.dat）
file-filter-usage-log = 使用記錄（*.bin）
file-filter-catalog = 翻譯檔案（*.ftl）
file-filter-spk = Sony 應用程式套件（*.spk）
file-filter-any = 所有檔案
file-filter-gps = GPS 輔助資料（*.dat）

# 狀態 / 訊息
msg-connecting = 正在連線…
msg-usage-log-written = 使用記錄已寫入 { $path }
msg-apps-found = 找到 { $count } 個應用程式
msg-switch-mode = 請將相機切換到 MTP 或大量儲存模式。
msg-no-tweaks = 這台相機沒有可用的調整項目。
msg-done = 完成。

# 相機 USB 模式切換（MTP -> 大量儲存）。顯示在「模式錯誤」
# 錯誤訊息下方；反方向沒有對應的 USB 命令。
mode-switching = 正在要求相機切換到大量儲存模式…
mode-switch-wait = 正在等待相機重新連線…
camera-confirm-hint = 相機上顯示的訊息（例如關於 www.sony.net/pm 的訊息）可以關閉，不會造成干擾。
mode-switch-done = 相機現在處於大量儲存模式。
mode-switch-timeout = 相機未以大量儲存模式重新出現。請將相機關機再開機後重試。
mode-switch-impossible = 此模式無法透過 USB 變更。請將相機關機再開機。
mode-switch-hint = 只有處於 MTP 模式的相機才能透過 USB 切換。這項變更是暫時的：相機選單中所選的 USB 模式不會改變，因此將相機關機再開機——或拔下傳輸線再重新連接——就會恢復先前的模式。反方向沒有對應的 USB 命令。

# 相機資訊欄位
info-model = 型號
info-product-code = 產品代碼
info-serial = 序號
info-firmware = 韌體版本
info-lens = 鏡頭
info-region = 地區
info-mode = 模式
mode-mass-storage = 大量儲存
mode-mtp = MTP
mode-updater = 更新模式
mode-app-install = 應用程式安裝模式
mode-service = 服務模式
mode-unknown = 未知
info-android-version = Android 平台版本
info-photographer = 攝影師
info-copyright = 版權
info-photographer-1 = 攝影師（項目 1）
info-photographer-2 = 攝影師（項目 2）
info-copyright-1 = 版權（項目 1）
info-copyright-2 = 版權（項目 2）
diagnose-services-heading = 無法存取相機？
diagnose-services-hint = 某些 Windows 服務可能會占用處於 MTP 模式的相機，導致本應用程式無法存取。此檢查只會詢問 Windows 哪些服務正在執行，不會變更任何內容。
btn-diagnose-services = 檢查 Windows 服務
diagnose-services-none = 沒有已知的干擾服務正在執行。
diagnose-services-found = 下列正在執行的服務可能會干擾相機的存取：
diagnose-services-advice = 請在「服務」（services.msc）中停止該服務，或以系統管理員身分執行下列命令，然後重新連接相機：

# 韌體更新程式 / 終端機
updater-model = 相機型號
updater-firmware-current = 目前的韌體版本
updater-guard-ok = 韌體映像檔與這台相機相符。
updater-no-fdat = 沒有適用於型號 { $model } 的內建更新程式韌體。
updater-model-required = 相機已處於更新模式，在此模式下無法回報其型號。請將相機關機再開機，使其以一般模式啟動，然後重試。或者，指定酬載檔案（--fdat-file），或在主控台中指定型號（--model <MODEL>）。
updater-already-updater = 相機已處於更新模式。
updater-flash-required = 未經確認，拒絕修改相機。請加上 --flash 重新執行，以切換到更新模式並寫入酬載。
updater-flashing = 正在將更新程式酬載寫入相機…
updater-switch-wait = 請依照相機螢幕上的指示操作。正在等待相機切換到更新模式…
updater-switch-timeout = 等待相機逾時。請在相機重新連線後再次執行該命令。
updater-shell-ready = 更新程式酬載正在執行。輸入「help」查看命令，輸入「exit」結束。
updater-shell-done = 終端機工作階段已結束。
updater-session-panic = 更新模式工作階段意外結束。重試前請先將相機關機再開機。

# 更新模式終端機的回應內容。剖析失敗的訊息只在 service-* 鍵中撰寫一次：
# 兩個終端機共用同一個語彙基元分析器，因此加了引號的路徑
# 與錯誤的十六進位值在兩者中會以相同方式失敗。
updater-backup-synced = 備份已同步至快閃記憶體。
updater-backup-locked = 受保護的備份設定現已鎖定。
updater-backup-writable = 受保護的備份設定現已可寫入。
updater-shell-console-only = root 殼層需要終端機，因此只能在主控台中使用：請執行 pmca-console updatershell --flash，然後在其中輸入「shell」。

# 更新模式終端機頁面。
updater-warning = 警告：要進入此終端機，相機會重新開機進入更新模式，並寫入一個酬載——這是本專案的一個小程式，相機會像接受韌體更新一樣接受它，之後由它回應此終端機的命令。酬載本身會在相機關機時被捨棄，但它所提供的命令會寫入相機的永久設定記憶體，寫入中斷可能會永久損壞您的相機。此外，此過程還會重設相機的所有設定——這點與服務模式終端機不同。
updater-console-heading = 在主控台中執行更新模式終端機
updater-driver-hint = 更新模式終端機需要相機使用 Windows 本身的大量儲存驅動程式 USBSTOR，因為酬載是透過它傳送的。這與服務模式終端機的需求正好相反。本應用程式會自行指派及解除 WinUSB 驅動程式，因此不需要手動準備任何東西——但由其他工具指派的驅動程式必須先復原。處於更新模式的相機會以自己的產品 ID 列舉，同樣不得重新繫結。
updater-fdat-label = 酬載檔案（選用）
updater-confirm = 相機將重新開機進入更新模式，並寫入酬載。要繼續嗎？
updater-cancelled = 已取消。相機未被修改。

# 兩個終端機頁面上都會顯示的可摺疊說明。命令語法本身
# 絕不翻譯——那些是必須一字不差輸入的文字——
# 因此這裡只放每個命令旁的說明。
btn-shell-primer = 關於此終端機
btn-shell-primer-hide = 隱藏說明
shell-primer-compare-heading = 兩個終端機，兩種通訊協定
shell-primer-compare = 兩個頁面都提供通往相機的命令列，但它們並不是同一個終端機。服務模式終端機使用 Sony 自有的原廠維修通訊協定，相機在驗證您的身分後會自行回應；過程中不會將任何東西寫入相機。更新模式終端機之所以存在，只是因為本專案將自己的程式寫入相機：相機重新開機進入更新模式，將酬載當作韌體映像檔接受，然後由酬載回應下列命令。兩者同時只能開啟一個。
shell-primer-payload = 「酬載」是指傳送途徑所承載的實際內容。對更新模式終端機而言，它是本專案的一個小程式：相機接受它的方式與接受韌體更新相同，但只保留到關機為止。服務模式終端機不需要酬載，因為相機會以自身的韌體回應其命令。對韌體檔案而言，這個詞的意思相同：指 .dat 檔案容器內的實際韌體映像。
shell-primer-when-service = 若要在相機繼續執行自身韌體的情況下讀取或編輯其儲存的設定，請使用服務模式終端機——例如讀取 HASP 區塊、儲存備份區域的復原副本、變更單一項目、從相機複製檔案。「調整項目」頁面底層使用的也是它，因此您在那裡能做的事，在這裡都能手動完成。
shell-primer-when-updater = 當相機無法以其他任何方式存取，或您需要在執行中的系統上取得殼層而不只是執行單一命令時，請使用更新模式終端機。這是唯一提供真正 root 殼層的途徑，而且在拒絕進入服務模式的相機上仍然可用。它的代價是一次重新開機、一次寫入以及相機的設定，因此它是次要選擇，而非首選。
shell-primer-driver-heading = 驅動程式：兩者的需求正好相反
shell-primer-driver = 服務模式終端機使用原始 USB 通訊，在 Windows 上需要將相機繫結到 WinUSB。本應用程式會在進入服務模式時自行指派該驅動程式，並在工作階段結束時再次解除，因此不需要另外的工具。更新模式終端機是透過 Windows 本身的大量儲存驅動程式 USBSTOR 傳送的，相機繫結到 WinUSB 時就無法運作——因此解除指派是結束服務工作階段的一部分，而不是事後才補做的工作。指派驅動程式需要系統管理員權限。在 Linux 上，兩者都不需要更換驅動程式，只需要裝置節點的存取權。
shell-primer-risk-heading = 可能發生的問題
shell-primer-risk = 下方標示的命令會寫入相機的永久設定記憶體。寫入中斷可能導致相機無法啟動，而且這裡的任何操作都無法復原——這正是 bk d 存在的原因：在變更任何內容之前，先儲存備份區域的副本。更新程式酬載本身不會留在相機上；相機關機時就會被捨棄。不過，經由更新模式的途徑會重設相機的所有設定。
shell-primer-commands-heading = 此終端機的命令
shell-primer-writes = 標有 (WRITES) 的命令會變更相機。其他命令都只會讀取。
shell-cmd-hasp = 讀取 HASP 區塊，即相機受保護的識別資訊區域。
shell-cmd-pull = 從相機複製檔案。若未指定本機名稱，會沿用遠端檔案名稱。
shell-cmd-push = 將本機檔案複製到相機上，取代該處原有的內容。(WRITES)
shell-cmd-rm = 刪除相機上的檔案。(WRITES)
shell-cmd-rmem = 將裝置記憶體的某個區域寫入本機檔案。位移與長度均為十六進位。
shell-cmd-bk-r = 讀取備份區域中的一個項目，並以十六進位傾印顯示。
shell-cmd-bk-d = 將整個備份區域儲存到檔案。請在變更任何內容之前執行：這是唯一的回復途徑。
shell-cmd-bk-dump-txt = 將整個備份區域儲存為 <file>.bin，並將可讀的清單儲存為 <file>.txt。
shell-cmd-bk-stat = 顯示相機的備份狀態區塊，包括地區。
shell-cmd-bk-w = 寫入備份區域中的一個項目，以空格分隔的十六進位位元組指定。(WRITES)
shell-cmd-bk-s-sub = 將單一子系統的備份區域保存到快閃記憶體，使變更在重新開機後仍然有效。(WRITES)
shell-cmd-bk-s = 將備份區域保存到快閃記憶體，使變更在重新開機後仍然有效。(WRITES)
shell-cmd-bk-lock = 設定或清除受保護備份項目的防寫保護；設定此保護時，部分設定會拒絕變更。(WRITES)
shell-cmd-term = 開啟或關閉相機的偵錯終端機。(WRITES)
shell-cmd-info = 讀取備份狀態區塊，它會指出該區域究竟能否寫入。
shell-cmd-shell = 在相機上開啟 root 殼層。它需要終端機，因此只能在主控台中使用；shell tcp <port> 則改為在 127.0.0.1 上提供給其他用戶端。(WRITES)
shell-cmd-help = 列出此終端機接受的命令。
shell-cmd-exit = 結束工作階段並離開終端機。
shell-primer-diagnostics-heading = 診斷與修復
shell-cmd-bk-trace = 將相機透過 USB 傳送整個備份區域的過程，逐個封包記錄到檔案中。用於調查大型備份；可能會使相機在回應途中卡住。此時請拔下傳輸線，將相機關機再開機，必要時取出電池。
shell-cmd-bk-rewrite = 讀取備份區域，原封不動地寫回，再讀取一次。可顯示相機是否接受完整映像，而不會變更任何設定。(WRITES)
shell-cmd-bk-restore-attrs = 將所有備份項目的屬性（例如防寫保護）還原為以 bk d 或 bk dump-txt 儲存之副本中的屬性。值會保持不變。只能使用來自同一台相機的副本。(WRITES)
shell-primer-diagnostics-env = 有三個環境變數若在啟動前設定，會同時影響主控台與圖形介面，例如在 PowerShell 中使用 $env:PMCA_TWEAKS_DIRECT = "1"。PMCA_TWEAKS_DIRECT 會讓調整項目即使在備份小於 1 MB 時也寫入單一值。PMCA_TWEAKS_IMAGE 會讓調整項目即使在映像大於 1 MB 時也寫入完整映像；兩者都設定時，以 PMCA_TWEAKS_DIRECT 為準。PMCA_TLS_DUMP=<file> 會在安裝應用程式期間儲存相機的第一則 TLS 訊息（ClientHello），以便調查連線失敗的原因。

# 應用程式安裝
install-switching = 正在切換到應用程式安裝模式…
install-disabled = 相機拒絕進入應用程式安裝模式。這通常與「啟用 USB 應用程式安裝程式」設定有關：必須在「調整項目」中將其開啟。
install-stays-app-mode = 相機會停留在應用程式安裝模式，可繼續安裝其他應用程式。若要使用其他所有功能，請拔下 USB 傳輸線，然後將相機關機再開機。
err-usb-app-install-mode = 相機仍處於應用程式安裝模式，在此模式下只接受應用程式安裝。請拔下 USB 傳輸線，然後將相機關機再開機——若在仍連接時重新開機，相機會直接回到此模式。
install-switch-wait = 正在等待相機以應用程式安裝模式重新連線…
install-switch-timeout = 等待相機進入應用程式安裝模式逾時。請在相機重新連線後再次執行該命令。
install-bad-mode = 無法在此模式下使用相機。請將其切換到 MTP 或大量儲存模式。
install-starting = 正在開始安裝…
install-error = 安裝通訊錯誤
install-server-failed = 本機安裝伺服器未接受相機的連線：
install-done = 工作已成功完成。

# APK 檢查
apk-package = 套件：
apk-unreadable = 警告：無法分析此 .apk 檔案。
apk-incompatible = 警告：此應用程式宣告的最低 SDK 等級，相機可能無法提供。
apk-unsigned = 警告：此 .apk 檔案沒有簽章。

# 社群應用程式商店
store-loading = 正在載入應用程式清單…
store-resolving = 正在查詢最新版本…
store-downloading = 正在下載
store-unknown-app = 未知的應用程式
store-no-release = 此應用程式沒有可下載的版本
store-not-installable = 無可用版本
msg-no-apps = 找不到應用程式。
store-heading = 社群應用程式商店
store-discontinued = Sony 已關閉官方的 PlayMemories Camera Apps 商店；其伺服器已不再回應，因此無法再從該商店購買或下載應用程式。
store-use-own-apk = 您仍然可以安裝自己的 .apk：在下方選取檔案，或將檔案拖曳到此視窗上。下方的社群應用程式商店也仍可使用。
btn-store-sync = 檢查相機
btn-install-selected = 安裝選取項目
btn-store-stop = 完成此應用程式後停止
btn-store-stopping = 將在完成此應用程式後停止…
btn-store-retry = 重試失敗的應用程式
btn-continue = 繼續
btn-reinstall = 重新安裝
btn-skip = 略過
btn-confirm-updater = 重新開機並寫入
btn-confirm-tweaks = 寫入調整項目
btn-confirm-languages = 啟用語言
btn-confirm-wifi = 取代存取點
btn-confirm-stream = 覆寫設定
btn-confirm-show-shells = 顯示終端機
btn-confirm-discard = 捨棄翻譯
store-select-all = 選取所有應用程式
store-col-app = 應用程式
store-col-version = 版本
store-col-author = 作者
store-col-camera = 相機上的版本
store-col-status = 狀態
store-camera-absent = 未安裝
store-version-unknown = 未知
store-batch-query = 正在從相機讀取已安裝的應用程式…
store-sync-done = 相機檢查完畢。清單中已安裝的應用程式：{ $count }
store-batch-downloading = 正在下載第 { $current } 個應用程式（共 { $total } 個）…
store-batch-installing = 第 { $current } 個應用程式（共 { $total } 個）：正在安裝 { $name }…
store-reinstall-confirm = { $name } 已安裝在相機上。已安裝的版本：{ $installed }。應用程式商店中的版本：{ $store }。要重新安裝嗎？
store-state-queued = 等待中
store-state-installed = 已安裝
store-state-failed = 失敗
store-state-skipped = 已略過
store-state-not-run = 未執行
store-batch-summary = 已安裝：{ $done } · 失敗：{ $failed } · 已略過：{ $skipped } · 未執行：{ $notrun }
store-sources-hint = 這些應用程式來自您在「設定」中新增的來源。本程式不會檢查它們。
app-backup-heading = 備份已安裝的應用程式
app-backup-intro = 將相機上已安裝的應用程式複製到這部電腦，日後即使在任何地方都已無法取得，仍可重新安裝。為此，相機會重新開機進入更新模式；其應用程式儲存區只會被讀取。
app-backup-own-camera = 請只備份您自己相機的應用程式，且不要將檔案轉交他人：應用程式的權利屬於其作者。
app-backup-restore-hint = 若要還原應用程式，請在上方的「安裝 .apk 檔案…」中選擇已儲存的 .apk 檔案。
btn-app-backup = 備份應用程式
app-backup-confirm = 相機將重新開機進入更新模式，並寫入一個會持續執行到相機關機為止的酬載。應用程式儲存區只會被讀取。要繼續嗎？
btn-confirm-app-backup = 重新開機並備份
app-backup-cancelled = 已取消備份。
app-backup-confirm-required = 備份會讓相機重新開機進入更新模式。請以 --confirm 確認。
app-backup-no-folder = 這部電腦上沒有用來存放備份的資料資料夾。請以 --out 選擇資料夾。
app-backup-no-model = 無法讀取相機型號。請將相機關機再開機，以大量儲存或 MTP 模式連接後重試。
app-backup-mounting = 正在尋找相機的應用程式儲存區…
app-backup-no-partition = 在這台相機上找不到應用程式儲存區。它可能不支援應用程式。
app-backup-copying = 正在複製 { $name }（{ $current }/{ $total }）…
app-backup-none = 相機上未安裝任何應用程式。
app-backup-done = 已儲存 { $count } 個應用程式。
app-backup-power-cycle = 請將相機關機再開機，以離開更新模式。
app-backup-folder = 資料夾
app-backup-col-file = 檔案
app-backup-state-new = 已儲存
app-backup-state-kept = 先前已儲存
btn-copy-path = 複製路徑
msg-path-copied = 路徑已複製到剪貼簿。
store-no-sources = 尚未新增任何來源。請在「設定 → 應用程式與外掛程式來源」中新增一個；本程式的社群清單與原始專案的應用程式清單會在那裡列為建議。
store-col-source = 來源
store-filter-source = 顯示來源
store-filter-all = 全部來源
sources-heading = 應用程式與外掛程式來源
sources-intro = 應用程式與外掛程式來自您在這裡自行新增的來源：開發者用來發布其作品的公開 GitHub 存放庫。本程式不託管、不檢查也不推薦任何來源或其中的任何內容；來源的責任完全由其作者承擔。新增存放庫之前，請先查看它。
sources-none = 尚未新增任何來源。
sources-add-label = GitHub 存放庫（擁有者/名稱或其位址）
btn-source-add = 新增來源
btn-confirm-source-add = 新增來源
sources-add-confirm = 要將 { $source } 新增為來源嗎？本程式未檢查過它。安裝它所提供的內容，責任由其作者承擔。同意之前，請先查看存放庫：{ $url }
sources-adding = 正在讀取來源…
sources-added = 已新增來源：{ $source }
sources-check-repo = 從存放庫安裝任何內容之前，請先查看它：{ $url }
sources-removed = 已移除來源：{ $source }
sources-remove-confirm = 要移除來源 { $source } 嗎？為其應用程式記住的簽署者也會一併清除。
sources-stars-title = GitHub 上的星星：表示有多少使用者標記了此存放庫。星星並不代表品質或安全性。
sources-suggestion-heading = 建議
sources-suggestion-community = 作者已將其專案封存的相機應用程式，以及作者要求列入的應用程式。此清單由本程式的作者 masterflai 維護；它只指向應用程式作者自行發布的版本，不包含任何自有檔案。這些應用程式未經檢查。
sources-suggestion-applist = 原始專案 Sony-PMCA-RE 的應用程式清單。它由其作者 ma1co 維護，而不是由本程式維護，且已多年未更新。
btn-source-activate = 啟用
sources-search-heading = 在 GitHub 上尋找來源
sources-search-hint = 在 GitHub 上搜尋主題為 pmca-source 的存放庫。搜尋結果未經本程式檢查，也不代表推薦。新增存放庫之前，請先查看它；星星只表示有多少 GitHub 使用者標記了它。
btn-source-search = 搜尋 GitHub
sources-searching = 正在搜尋 GitHub…
sources-search-none = GitHub 找不到主題為 pmca-source 的存放庫。
sources-search-found = 找到 { $count } 個存放庫。其中沒有任何一個經過本程式檢查。
sources-updated = 最後變更：{ $date }
sources-archived = 已封存
app-archived-note = 開發者已在 GitHub 上將此專案封存，不再積極開發。此應用程式可能無法在每台相機上完整運作，問題也不太可能再獲修正。
sources-already = 已新增
sources-no-folder = 無法判斷使用者資料資料夾，因此無法儲存來源。
source-invalid = 這不是 GitHub 存放庫。請輸入「擁有者/名稱」，例如 ma1co/OpenMemories-AppList，或輸入存放庫的位址。
source-exists = { $source } 已經新增過了。
source-unknown = 沒有這個來源：{ $source }
source-unreadable = { $source } 的預設分支中沒有可讀取的 { $file }。
source-failed = 無法讀取來源 { $source }：{ $reason }
source-signer-changed = 來自 { $source } 的 { $name } 由與先前不同的簽署者簽署，因此未安裝。如果其作者確實更換了簽署金鑰，請移除此來源後再重新新增。
source-signer-pinned = 來自 { $source } 的 { $name } 並非以其來源指定的憑證簽署，因此未安裝。
source-signature-invalid = 來自 { $source } 的 { $name } 與其簽章不符，因此未安裝；檔案可能在傳輸途中遭到竄改。（{ $reason }）
source-ambiguous = 有多個來源提供 { $package }：{ $sources }。請使用 --source 選擇其中一個。
source-plugin-mismatch = 下載的外掛程式 ID 為 { $found }，但來源宣告的是 { $id }。此外掛程式未安裝。
plugin-sources-heading = 來自您已新增來源的外掛程式
plugin-sources-hint = 任何人都可以在自己的來源中發布外掛程式。此清單會顯示您在「應用程式與外掛程式來源」中新增的所有來源的外掛程式，無論發布者是誰；本程式不會檢查它們。
plugin-signed = 已簽署，金鑰 { $key }
plugin-unsigned = 未簽署
plugin-signer-changed = 已安裝的 { $id } 所使用的簽署金鑰與此檔案的不同，或此檔案未經簽署。外掛程式未被取代。如果其作者確實更換了金鑰，請先移除已安裝的外掛程式。
plugin-key-written = 簽署金鑰已寫入 { $path }。請將此檔案保密並妥善備份：任何擁有它的人都能以您的名義簽署外掛程式；沒有它，您就無法發布現有安裝會接受的更新。公開金鑰：{ $key }
btn-details = 詳細資料
details-repository = 存放庫
details-homepage = 專案頁面
details-source = 來源
msg-url-copied-browser = 網址已複製到剪貼簿。請將其貼到瀏覽器中以檢視該頁面。
store-activate-suggestion = 尚未新增任何來源。要啟用原始專案的應用程式清單（{ $source }）嗎？它由其作者 ma1co 維護，而不是由本程式維護，且本程式未檢查過它。之後清單會出現在「設定 → 應用程式與外掛程式來源」中。
store-activate-community = 尚未新增任何來源。要啟用社群清單（{ $source }）嗎？它由本程式的作者 masterflai 維護，只指向應用程式作者自行發布的版本；這些應用程式未經檢查。之後清單會出現在「設定 → 應用程式與外掛程式來源」中。
btn-activate-list = 啟用清單
btn-plugin-sources-load = 顯示來自來源的外掛程式
plugin-sources-none = 您的來源未提供任何外掛程式。
plugin-source-unchecked = 來源：{ $source }。本程式未檢查此外掛程式。
btn-plugin-source-install = 安裝

# 服務（senser）模式
usb-mode-advice = 若要使用調整項目、備份與服務模式終端機，請將相機的 USB 連接模式設為「自動」，而不是固定設為大量儲存。工作階段結束後若相機詢問模式，請選擇「大量儲存（MSC）」。若固定設為大量儲存，相機經常會拒絕下一次切換到服務模式。
service-switch-refused = 相機未完成切換到服務模式，也未透過 USB 重新連線；此時相機通常會顯示「檢查連接的裝置」。請拔下 USB 傳輸線，重新插上後再試一次。
service-switching = 正在切換到服務模式…
service-switch-wait = 正在等待相機以服務模式重新連線…
service-switch-timeout = 相機未以服務模式出現。相機會以獨立的 USB 裝置（產品 ID 0336）重新列舉，需要另外指派 WinUSB 驅動程式；請在裝置管理員中尋找沒有驅動程式的 Sony 裝置。將相機關機再開機，即可使其回到一般模式。
service-authenticating = 正在驗證…
# 在發現相機仍處於先前執行所留下的服務模式時顯示，
# 以及在無法復原該狀態時顯示。
service-stranded-reset = 相機仍處於先前執行時的服務模式。正在先結束該工作階段…
service-stranded-failed = 相機因先前的執行而卡在服務模式，且未回應離開該模式的要求。請將相機關機再開機，然後重試。
service-bad-mode = 無法在此模式下使用相機。請將其切換到大量儲存模式。
service-driver-required = 原始 USB 存取遭拒，且無法指派 WinUSB 驅動程式。指派驅動程式需要系統管理員權限。
service-warning = 警告：服務模式可讓您直接存取相機的記憶體、檔案系統與校正資料。寫入命令可能會永久損壞您的相機。
service-shell-ready = 服務模式終端機已就緒。輸入「help」查看命令，輸入「exit」結束。
service-shell-done = 服務模式終端機工作階段已結束。
# 在啟動時回報：先前的執行在解除其指派的驅動程式之前
# 就已中斷。
driver-swept = 已移除先前執行遺留的 { $count } 個驅動程式指派。
# 在啟動時回報：同一個被中斷的執行還讓相機停留在服務模式，
# 相機會因此拒絕下一個工作階段。
driver-swept-camera = 先前的執行讓一台相機停留在服務模式，現已將其恢復為一般模式。
driver-release-replug = 解除驅動程式時，Windows 正在等待相機。請拔下 USB 傳輸線再重新插上，以便完成清理。
# `driver` 命令的結果。
driver-bound = { $id } 現在使用 WinUSB（驅動程式套件 { $package }）。
driver-released = 已解除 { $count } 個驅動程式指派；相機已改回使用 Windows 內建的驅動程式。
driver-needs-elevation = 指派驅動程式需要系統管理員權限。請從以系統管理員身分啟動的主控台執行此命令。
driver-leftover-needs-elevation = 先前的執行在相機上留下了一個已指派的驅動程式。該驅動程式存在期間，相機會要求您檢查連接的裝置，且其大量儲存功能無法運作。解除該驅動程式需要系統管理員權限；請確認 Windows 的提示。
# 服務模式途徑自行進行的驅動程式指派，因此不需要
# 外部工具。
driver-binding = 正在將 WinUSB 驅動程式指派給相機…
driver-bind-wait = 正在等待相機以新的驅動程式重新連線…
driver-bind-timeout = 指派驅動程式後，相機未重新出現。請拔下相機、重新連接後再試一次。
driver-bind-failed = 無法指派 WinUSB 驅動程式：{ $detail }

# 終端機的回應內容。這些內容由後端產生，因此主控台與
# 視窗中的措辭完全相同。
service-usage = 用法：{ $usage }
service-bad-hex = 不是十六進位值：{ $value }
service-unknown-command = 未知的命令：{ $command }。輸入「help」查看清單。
service-unterminated-quote = 此行在加上引號的引數內就結束了。
service-help = 命令參考：
service-read-bytes = 已讀取 { $count } 個位元組。
service-pulled = 已將 { $remote } 從相機複製到 { $local }。
service-pushed = 已將 { $local } 複製到相機上的 { $remote }。
service-removed = 已從相機刪除 { $remote }。
service-dumped = 已將從 { $offset } 開始的 { $size } 個位元組傾印到 { $local }。
service-backup-saved = 已將備份映像（{ $count } 個位元組）儲存到 { $local }。
service-backup-text-saved = 已將備份映像（{ $count } 個位元組）儲存到 { $bin }，並將清單儲存到 { $txt }。
service-backup-status = 備份狀態，地區：{ $region }
service-backup-rewritten = 映像（{ $count } 個位元組）已原封不動寫回並再次讀取；未永久保存。
service-backup-attrs-restored = 已將 { $count } 個項目的屬性還原為 { $local } 中的屬性。
service-backup-written = 已寫入備份項目 { $id }。
service-backup-persisted = 已將子系統 { $id } 的備份保存到快閃記憶體。
service-terminal-on = 偵錯終端機現已開啟。
service-terminal-off = 偵錯終端機現已關閉。
btn-service-terminal-open = 開啟序列主控台
btn-service-terminal-close = 關閉序列主控台
service-terminal-heading = 序列主控台
service-terminal-hint = 在 Linux 架構的相機上，偵錯終端機是透過 USB 提供的 root 殼層。開啟後，即可像在終端機中一樣輸入命令。
service-terminal-label = 主控台
service-terminal-interactive = 序列主控台已開啟。請輸入命令；輸入「exit」可返回服務模式終端機。

# 未開啟進階模式時的服務模式頁面：如何從主控台進入終端機，
# 以及事先需要準備什麼。
service-console-heading = 在主控台中執行服務模式終端機
service-console-hint = 在 pmca-console 所在的資料夾中開啟終端機，然後執行下列命令：
service-driver-heading = 開始之前
service-driver-hint = 服務模式需要原始 USB 存取，而 Windows 不會透過大量儲存驅動程式授予此存取權。本應用程式會自行指派 WinUSB 驅動程式——共兩次，一次是在相機處於大量儲存模式時，另一次是在相機以第二個產品 ID 重新出現於服務模式之後——並在工作階段結束時解除這兩個指派。指派驅動程式需要系統管理員權限，因此請以系統管理員身分啟動主控台；不需要其他工具。
driver-note-linux = 這裡不需要安裝任何東西：libusb 會直接存取相機，不需要取代核心驅動程式。需要的是開啟裝置的權限，以及桌面環境尚未占用該裝置。請為 Sony 的廠商 ID 054c 安裝一條 udev 規則，並在使用大量儲存模式前卸離 usb-storage 驅動程式。原始碼樹中的指令碼 scripts/linux-prepare.sh 會同時處理這兩件事，並一併停止 gvfs 磁碟區監視器。
driver-note-macos = 原始 USB 存取不需要安裝任何東西。請將相機的 USB 連接設為 MTP。在 MTP 模式下，若 macOS 的相機服務 ptpcamerad 占用了相機，本程式會將其結束；系統會在需要時重新啟動該服務。調整項目、服務模式終端機、功能分析、韌體更新和更新模式終端機都需透過大量儲存連線進行，而 macOS 只會將該連線交給具備系統管理員權限的程式：視窗每次執行時會要求輸入一次密碼，命令列則需要 sudo。此平台是三者中測試最少的，因此遇到問題時，與其期待已有相關說明，不如做好回報問題的準備。
service-advanced-hint = 也可以在「設定」的「進階模式」中，讓終端機顯示在這裡。
btn-copy-command = 複製命令
msg-command-copied = 命令已複製到剪貼簿。

# 已開啟進階模式時的服務模式頁面。
btn-service-open = 連線
btn-service-close = 中斷連線
btn-service-run = 執行
btn-service-browse = 插入檔案路徑…
shell-path-quote = 無法插入此路徑：它包含雙引號，而命令列正是以這個字元標示引數的開頭與結尾。請重新命名檔案，或將其移到不含此字元的路徑。
service-command-label = 命令
service-not-connected = 未連線。

# 僅限 GUI 的字串
install-apk-path = 本機 .apk 檔案的路徑（選用）
install-drop-hint = 將 .apk 檔案拖放到這裡

# 調整項目說明
tweak-rec-limit = 錄製影片時不受時間限制
tweak-rec-limit-4k = 錄製 4K 影片時不受時間限制
tweak-language = 顯示所有選單語言
tweak-language-note = 相機只會顯示其韌體認得的語言，因此出現的語言可能比這裡啟用的少。例如 DSC-RX1R 會顯示 35 種中的 25 種。
tweak-pal-ntsc = 啟用 PAL / NTSC 選擇器與警告
tweak-pal-ntsc-mode = 視訊系統（PAL / NTSC）
tweak-pal-ntsc-pal = PAL
tweak-pal-ntsc-ntsc = NTSC
tweak-pal-ntsc-note = 變更視訊系統後，相機會要求格式化記憶卡。請先備份卡上的所有內容——相片、影片、LUT。最簡單的做法是在切換前取出記憶卡。
tweaks-set-malformed = 預期格式為 --set ID=VALUE，但收到：{ $arg }
tweaks-set-unknown-value = 未知的值「{ $value }」。預期為下列其中之一：{ $options }
tweaks-set-not-a-choice = { $id } 是開關，而不是選項。請使用 --enable 或 --disable。
tweaks-needs-set = { $id } 是選項，而不是開關。請使用 --set { $id }=VALUE，並搭配下列其中一個值：{ $options }
tweak-usb-installer = 啟用 USB 應用程式安裝程式

# 調整項目流程
tweaks-warning = 警告：套用調整項目會改寫相機的備份記憶體。寫入中斷可能會永久損壞您的相機。
tweaks-info-toggle = 調整項目有什麼作用？
tweaks-info-what-heading = 調整項目是什麼
tweaks-info-what = 調整項目是相機本身已具備、但未在選單中提供的設定。本程式讓它們可以使用：不受時間限制的錄影（包含 4K）、韌體中包含的所有選單語言、PAL 與 NTSC 的選擇，以及 USB 應用程式安裝程式。外掛程式還能新增更多調整項目。會出現哪些調整項目取決於相機型號。
tweaks-info-how-heading = 運作方式
tweaks-info-how = 「顯示可用的調整項目」會讓相機短暫進入服務模式，並讀取其備份記憶體。此過程不會變更任何內容。接著您會看到哪些調整項目已開啟或關閉，以及相機識別資訊（僅供顯示）。「套用」只會寫入您變更過的項目。之後本程式會再次讀取記憶體，並檢查相機是否已全部採用。
tweaks-info-undo-heading = 如何復原
tweaks-info-undo = 每次寫入前，本程式都會將先前的狀態儲存在這部電腦上。每個調整項目都能以同一個開關再次關閉。對於逐一寫入數值的相機，另外也會提供「復原變更」。
tweaks-info-needs-heading = 所需條件
tweaks-info-needs = 需要金鑰資料（請參閱「歡迎」頁面），並依下方說明將相機的 USB 連接設為「自動」。Windows 與 macOS 會要求系統管理員權限，讓本程式能在服務模式下存取相機。寫入期間請勿拔除傳輸線或取出電池。
app-backup-target = 目標資料夾
btn-choose-folder = 選擇資料夾…
btn-default-folder = 使用預設資料夾
tweaks-reading = 正在讀取相機設定…
tweaks-writing = 正在寫入相機設定…
tweaks-done = 相機設定已更新。
tweaks-write-failed = 相機並未依照傳送的內容儲存新設定。部分設定可能仍已寫入：請重新讀取設定，必要時加以復原。變更前所取得的備份副本是安全的回復途徑。
tweaks-confirm-required = 未經確認，拒絕寫入。閱讀上方警告後，請加上 --confirm 重新執行。
tweaks-state-enabled = 已啟用
tweaks-state-disabled = 已停用
tweaks-full-read-failed = 相機未傳送完整的備份（{ $detail }）。相機之後經常會停止回應：請將其關機，取出電池數秒後再裝回，然後重新連接。
tweaks-direct-mode = 這台相機的備份大於 1 MB，因此其設定會逐一讀取和寫入。每次變更前都會儲存舊值，並可透過「復原變更」寫回。
tweaks-direct-forced = PMCA_TWEAKS_DIRECT 會強制使用逐一屬性的途徑，因此這台相機的設定會逐一讀取和寫入。每次變更前都會儲存舊值，並可透過「復原變更」寫回。
tweaks-direct-refused = 相機拒絕了此變更（{ $detail }）。已寫入的值均已還原；未變更任何內容。某些機型（特別是日本機型）只允許透過完整的備份映像變更此設定。
tweaks-direct-revert-failed = 相機拒絕了此變更（{ $detail }），且並非所有已寫入的值都能還原。舊值已儲存：請嘗試「復原變更」。
tweaks-direct-restart = 相機設定已更新。請將相機關機再開機，變更才會生效。
tweaks-rollback-unsaved = 無法儲存目前的值以供復原，因此未變更任何內容。
tweaks-image-saved = 變更前的備份映像已儲存：{ $path }
tweaks-whole-image-hint = 若要改為透過完整的備份映像寫入變更，請加上 --whole-image 再次執行相同的命令。系統會先儲存變更前的映像。
tweaks-whole-image-confirm = 要透過完整的備份映像寫入變更嗎？相機拒絕以單一值寫入。整個映像會連同變更一起寫回；變更前的映像會先儲存在這部電腦上。
btn-whole-image = 寫入完整備份
tweaks-undo-none = 這台相機沒有已儲存的值。
tweaks-undo-confirm = 要將已儲存的值寫回相機嗎？這會復原自儲存以來逐一寫入的每項變更。
tweaks-undo-done = 已儲存的值已寫回。請將相機關機再開機。
tweaks-undo-available = 第一次變更前的值已儲存，可以寫回。
btn-undo-tweaks = 復原變更
tweaks-identity-heading = 相機識別資訊（唯讀）
tweaks-caution-languages = 有兩種語言——可能是中文（簡體）和中文（繁體）——這台相機無法顯示。捲動到這兩種語言時，語言選單會重新啟動，因此為了安全起見，它們會保持關閉。
tweaks-caution-accept = 仍然啟用這兩種語言（風險自負）
tweaks-caution-confirm = 這兩種語言可能會使相機的語言選單重新啟動。這不會造成任何損壞，而且此變更可以復原，但您可能必須從頭重新找到該設定。仍要啟用嗎？

# 設定
settings-language = 語言
settings-theme = 佈景主題
settings-theme-system = 系統
settings-theme-light = 淺色
settings-theme-dark = 深色

# 進階模式。這是防止手誤的保護措施，而非防範攻擊者：
# 它顯示的所有內容無論如何都能從主控台取得。
settings-advanced-heading = 進階模式
settings-advanced-hint = 進階模式會在同名頁面上顯示服務模式終端機與更新模式終端機。服務模式可直接存取相機的記憶體、檔案系統與校正資料；輸入錯誤的命令可能會使相機無法使用。除非您清楚自己即將寫入什麼，否則請保持關閉。
settings-advanced-toggle = 顯示服務模式終端機與更新模式終端機
settings-advanced-on = 進階模式已開啟。「服務模式終端機」與「更新模式終端機」頁面現在會直接提供終端機。
settings-advanced-off = 進階模式已關閉。「服務模式終端機」與「更新模式終端機」頁面會改為說明如何使用主控台。
settings-advanced-confirm = 服務模式終端機與更新模式終端機可能會永久損壞您的相機。要顯示這兩個終端機嗎？
settings-update-heading = 更新通知
settings-update-purpose = 開啟此選項後，本程式每次啟動時都會向 GitHub 查詢是否已發行新版本，若有，便會在視窗頂端顯示通知。在此過程中，GitHub 會得知您的 IP 位址；不會傳送其他任何資料。預先發行版本不會通知。
settings-update-toggle = 啟動時若有新版本可用，請通知我
settings-update-on = 更新通知已開啟。從下次啟動起，新版本會在視窗頂端通知。
settings-update-off = 更新通知已關閉。只有在您點選導覽列下方的版本號碼時，本程式才會檢查新版本。
update-check-hint = 立即檢查是否有新版本
update-check-heading = 檢查更新
update-checking = 正在尋找新版本…
update-none = 您正在使用最新版本（{ $version }）。
update-available = 有新版本可用
update-current = 已安裝的版本
update-latest = 新版本
btn-update-dismiss = 隱藏

# 編輯翻譯。語言檔案可以下載、在任何文字編輯器中修改，
# 然後再上傳；上傳的檔案在使用前會先經過檢查。
settings-catalog-heading = 編輯翻譯
settings-catalog-hint = 下載語言檔案，在任何文字編輯器中修改後再上傳。只翻譯等號右側的文字，您省略的項目會保留內建的文字。在這裡選擇語言並不會變更應用程式的語言。
settings-catalog-locale = 語言檔案
settings-catalog-builtin = 目前使用內建的翻譯。
settings-catalog-custom = 目前使用您自己的翻譯：
btn-catalog-export = 下載…
btn-catalog-import = 上傳…
btn-catalog-reset = 還原內建翻譯
catalog-exported = 已寫入
catalog-imported = 已套用翻譯。
catalog-reset-done = 已還原內建的翻譯。
catalog-reset-none = 沒有需要還原的自訂翻譯。
catalog-reset-confirm = 要捨棄您為此語言所做的翻譯嗎？
catalog-rejected = 未套用此檔案。請修正下列各行：
catalog-count-accepted = 已套用的項目
catalog-count-missing = 沿用內建翻譯的項目
catalog-count-untranslated = 與英文原文相同的項目
catalog-line = 行
catalog-problem-syntax = 不是「id = text」格式的項目
catalog-problem-continuation = 內容延續到下一行，不支援此寫法
catalog-problem-duplicate = 重複定義了不只一次
catalog-problem-unknown-key = 不是本應用程式使用的項目
catalog-problem-empty = 沒有文字
catalog-problem-placeholder = 預留位置已變更，預期為：

# 來源、致謝與授權。顯示在「授權」頁面，並由
# `pmca-console license` 輸出。授權條款本文不翻譯：它是
# 法律文件，會逐字照錄。
about-thanks-heading = 致謝
about-thanks-body-1 = 本工具之所以存在，要感謝 ma1co——他研究出 Sony 相機如何透過 USB 通訊，並於 2015 年發布了 Sony-PMCA-RE——也要感謝在其後數年間為該專案及 OpenMemories 各專案做出貢獻的每一個人。
about-thanks-body-2 = 相機偵測、外部命令通訊協定、韌體更新程式程序與服務模式，全都是他們的成果，而這個移植版本延續了這些成果。謝謝你們。在此基礎上，PMCA-RE RELOADED 加入了許多由 masterflai 貢獻的新功能。其中最重要的是 Windows 上的裝置驅動程式自動切換功能，現已內建於本程式中：不再需要以另外的工具手動為相機指派其他驅動程式，讓整個流程更方便、更快速，風險也更低。
about-contributors-heading = 貢獻者
about-contributors-body = 除了 ma1co 之外，下列人士也為 Sony-PMCA-RE 做出了貢獻，或在自己的副本中發布了已納入此移植版本的變更：
about-purpose-heading = 此移植版本的存在理由
about-purpose-body-1 = 原始應用程式已不再開發。它是為 Python 2 撰寫的，而 Python 2 已於 2020 年終止支援；其部分功能仰賴 Sony 後來已關閉的服務，因此在現今的系統上已無法不受限制地執行。
about-purpose-body-2 = 此移植版本的目的在於保存而非取代，同時也同樣重視在今日能順暢使用：讓這些相機保持可用，並能在現今的電腦上以及搭配較新的相機機型時順利運作。原始版本能做到的一切，這個版本也應該都能做到。
about-origin-heading = 製作方式
about-origin-body-1 = 從 Python 到 Rust 的移植是在 AI 的協助下完成的。
about-origin-body-2 = 同樣值得清楚說明的是，這並不代表未經驗證：程式行為由自動化測試套件與硬體測試計畫加以涵蓋。尤其是 Windows 版本已經過徹底測試，因為裝置驅動程式使其成為最複雜的平台；macOS 和 Linux 則無法進行同樣全面的測試。請將本軟體視為實驗性軟體。
about-license-heading = 授權
about-license-body = 此移植版本依 MIT 授權條款發布，與原始專案的授權條款相同。依該授權條款的要求，原始的版權聲明維持不變。
about-license-full = 授權條款本文
about-third-party-heading = 第三方授權
about-third-party-body = 本程式採靜態連結：所使用的每個程式庫的程式碼都位於執行檔內，而這些程式庫的授權條款要求其版權聲明必須隨附在內。以下完整列出這些聲明，每個套件一則。
about-third-party-toggle = 顯示所有第三方聲明
about-privacy-heading = 資料
about-privacy-body-1 = 本程式在您自己的電腦上執行。它不會傳送任何使用資料、沒有遙測功能，也不需要帳戶。您在這裡所做的任何事都不會被回報到任何地方。
about-privacy-body-2 = 有兩項功能會自行連上網路，對方在此過程中會得知您的 IP 位址：GPS 輔助資料從 Sony 下載；您新增的來源及其星星則從 GitHub 讀取，在您要求時也會在 GitHub 上進行搜尋；應用程式與外掛程式從這些來源所指明的網址下載。若不使用這些功能，本程式會完全離線運作。如果您在設定中開啟更新通知，本程式也會在每次啟動時向 GitHub 查詢最新版本。點選導覽列下方的版本號碼也會查詢一次。
about-privacy-body-3 = 從相機讀出的資料——其中包括序號，以及所儲存的任何 Wi-Fi 密碼——不會離開您的電腦。如果您將這些資料寫入檔案，請留意要將該檔案交給誰。
about-privacy-body-4 = 本程式背後沒有營運任何服務，也沒有任何資料會傳送給其作者，因此這裡沒有可供查詢或要求刪除的資料。上述兩項服務如何處理它們看到的位址，由其各自的隱私權聲明規範，而不受本聲明約束。
about-legal-heading = 法律聲明
about-legal-trademark = 這是一個獨立專案。它並非由 Sony 製作或背書，與 Sony 也沒有任何關聯。「Sony」、「PlayMemories」及相機機型名稱均為 Sony Group Corporation 的商標，在此僅用於說明本工具可與哪些相機通訊。
about-legal-own-device = 本程式僅供搭配您自己擁有的相機使用。
about-legal-warranty = 本工具所做的事超出 Sony 對這些相機的預期用途，這麼做可能會使您的保固失效。本軟體本身不提供任何保固，使用風險由您自行承擔。對自己擁有的裝置進行逆向工程是否獲得允許、允許到何種程度，取決於您所在的地區。
about-links-heading = 專案
about-link-original = Sony-PMCA-RE — 原始專案
about-link-fwtool = fwtool.py — 更新映像檔產生功能所依據的韌體工具
about-link-tweak = OpenMemories: Tweak — 建議支援的相機使用的應用程式
about-link-applist = OpenMemories: App list — 更多應用程式
about-link-platform = OpenMemories: Platform — 本工具隨附的更新程式主體
about-link-devices = 支援的相機機型
about-link-nexhack = Nex-Hack — 本專案所依據的社群研究
about-link-zve10 = zv-e10-re — 關於 ZV-E10 介面與設定記憶體的研究
about-version = 版本
btn-copy-url = 複製網址
msg-url-copied = 網址已複製到剪貼簿。

# 安全注意事項
notice-experimental = 這是實驗性軟體。使用風險請自行承擔。

# 網路設定（GPS 輔助、直播存取點與服務）
gps-downloading = 正在下載 GPS 輔助資料…
gps-writing = 正在將 GPS 輔助資料寫入相機…
gps-bad-size = GPS 輔助資料的大小不符預期，因此未寫入。
wifi-read = 正在從相機讀取直播存取點…
wifi-written = 直播存取點已寫入。
stream-read = 正在從相機讀取直播設定…
stream-written = 直播設定已寫入。
net-secret-warning = 警告：此檔案包含明文的 Wi-Fi 密碼與存取權杖。
wifi-secret-warning = 相機以未加密的方式儲存這些密碼，此處會依儲存的原樣顯示。它們會留在這部電腦上；如果您將其儲存到檔案，請留意要將該檔案交給誰。
gps-heading = GPS 輔助資料
gps-hint = 將檔案拖曳到這裡、輸入路徑，或將欄位留空以從 Sony 取得目前的資料。
btn-gps-devices = 相容裝置
btn-gps-devices-hide = 隱藏相容裝置
gps-devices-note = 這些機型內建 GPS 模組。其中許多機型只有名稱以 V 結尾的版本才配備 GPS；名稱沒有 V 的版本則不具備 GPS。
gps-devices-compact = 輕便型相機（DSC）
gps-devices-slt = SLT 相機（A 接環）
gps-devices-action = 運動攝影機
gps-devices-camcorder = 攝錄影機（Handycam）
gps-col-variant = GPS 版本
gps-col-arch = 架構
gps-col-module = GPS 模組
gps-col-apps = 應用程式支援
gps-yes = 是
gps-no = 否
gps-unknown = 未知
gps-devices-untested = 具備 GPS，支援情況未經測試
gps-devices-untested-note = 這些機型內建 GPS 模組，但不在 OpenMemories 的裝置清單中。本應用程式能否與它們通訊尚未經過測試，因此其架構與應用程式支援情況不明。
wifi-heading = 直播用存取點
wifi-hint = 這些是相機直播功能所使用的存取點，而不是相機本身的 Wi-Fi 設定。不具備直播功能的機身會拒絕此操作。
wifi-multi = 多重存取點清單
wifi-none = 未儲存任何存取點。
wifi-writing = 正在將直播存取點寫入相機…
wifi-confirm = 這會取代相機上儲存的所有存取點。要繼續嗎？
wifi-empty-refused = 拒絕寫入空白清單，因為這會清除所有已儲存的存取點。
stream-heading = 直播
stream-writing = 正在將直播設定寫入相機…
stream-confirm = 這會覆寫相機的直播設定。要繼續嗎？
stream-invalid-json = 這些設定不是有效的 JSON：

# 韌體更新
firmware-warning = 警告：這會覆寫相機的韌體。更新開始後請勿中斷：請勿拔除 USB 傳輸線，也不要關閉相機電源。請只使用專為這款相機機型設計的韌體檔案，並確認電池電量充足。否則相機可能會永久無法使用。
firmware-untested = 未經測試：韌體燒錄功能雖已移植，但從未在實際相機上試過。寫入失敗或中斷後無法復原，因此請將此頁面上的所有內容視為未經驗證。
firmware-confirm-required = 未加上 --confirm，拒絕燒錄。
firmware-truncated = 韌體檔案不完整：FDAT 酬載超出了檔案結尾。
firmware-updating = 正在更新韌體：
firmware-invalid = 此檔案不是有效的 Sony 韌體映像檔
firmware-dat-path = 韌體檔案（.dat）
firmware-drop-hint = 將 .dat 韌體檔案拖放到這裡
firmware-file-required = 請先選擇韌體檔案。
firmware-check-ok = 相機接受此映像檔。尚未寫入任何內容。
firmware-cancelled = 已取消燒錄。相機未被修改。
firmware-current-version = 已安裝的版本
firmware-new-version = 此映像檔中的版本
firmware-models-hint = 只要更新模式終端機能存取到相機，該相機即為相容。
btn-firmware-check = 檢查映像檔
btn-firmware-flash = 燒錄韌體
firmware-own-device = 請確認您了解這些風險，且只是在更新您自己的裝置。
btn-firmware-accept = 已了解風險 – 燒錄

# 檔案工具：完全不需要相機的操作。
tools-hint = 這些操作只處理檔案，絕不會動到相機，因此這裡的任何操作都不會損壞相機。它們與 pmca-console 以 apk2spk、spk2apk、print-backup、print-usagelog、guess-firmware 和 xpd-checksum 提供的操作完全相同。
tools-operation = 操作
tools-apk2spk = 將 .apk 封裝到 .spk 容器中
tools-spk2apk = 將 .spk 解開還原為 .apk
tools-print-backup = 顯示 Backup.bin 的內容
tools-print-usagelog = 顯示 Usagelog.bin 的內容
tools-guess-firmware = 在韌體 .dat 中尋找酬載
tools-xpd-checksum = 計算檔案的 CIC 總和檢查碼
tools-input = 輸入檔案
tools-file-required = 請先選擇此操作所需的檔案。
tools-apk-invalid = 無法將此檔案封裝為 .spk
tools-spk-invalid = 此檔案不是有效的 .spk 套件
tools-backup-invalid = 此檔案不是有效的 Backup.bin 映像
tools-usagelog-invalid = 此檔案不是有效的使用記錄
tools-reset-value = 重設值：
tools-fdat-found = 在位移 { $offset } 找到酬載，大小為 { $size }。
tools-checksum = 總和檢查碼
tools-written = 已寫入 { $path }

# 專案文件的連結
docs-compatibility = 相機相容性
docs-compatibility-hint = 原始專案提供了一份已測試相機的概覽。它主要涵蓋較舊的相機機型，可在下列網址找到：
devices-note = 取自原始專案的概覽。應用程式支援旁的數字是相機應用程式平台的版本。
devices-more-note = 下列相機尚未以本程式明確測試過。最後一欄中的勾號表示社群曾回報使用過。若服務模式終端機欄也標示 ✖，則表示該次使用失敗。
btn-more-models = 更多裝置
btn-more-models-hide = 隱藏更多裝置
check-updater-yes = 您的裝置 { $model } 相容：根據裝置清單，更新模式終端機可以存取該裝置。
check-updater-no = 您的裝置 { $model } 不相容：根據裝置清單，更新模式終端機無法存取該裝置。
check-updater-unknown = 您的裝置 { $model } 的相容性不明：該裝置不在本程式的任何裝置清單中，因此無法得知更新模式終端機能否存取它。
check-apps-no = 您的裝置 { $model } 不相容：根據裝置清單，該裝置不支援應用程式。
check-apps-untested = 您的裝置 { $model } 的相容性不明：該裝置在裝置清單中，但尚未確認是否支援應用程式。
check-apps-unknown = 您的裝置 { $model } 的相容性不明：該裝置不在本程式的任何裝置清單中，因此無法得知它是否支援應用程式。
check-gps-no = 您的裝置 { $model } 不相容：該裝置不在具備 GPS 接收器的裝置清單中。
check-gps-untested = 您的裝置 { $model } 的相容性不明：根據裝置清單，該裝置具備 GPS 接收器，但尚未測試對此機型的傳輸。
check-gps-unknown = 您的裝置 { $model } 的相容性不明：該裝置不在本程式的任何裝置清單中，因此無法得知它是否具備 GPS 接收器。
check-unidentified = 無法識別已連接的裝置，因此其相容性不明。
check-proceed = 仍要繼續嗎？
check-cancelled = 已取消。裝置未被修改。
devices-col-confirmed = 已確認
devices-group-ilc = 可交換鏡頭相機（E 接環與 A 接環）
devices-group-remote = 即時預覽遙控器
btn-copy-link = 複製連結
shutter-heading = 快門次數
shutter-hint = 相機會計算每一次快門釋放，並將目前的數值寫入其儲存的每張相片（ARW、JPEG 或 HEIF）中。請選取最近拍攝的相片以讀取目前的次數。檔案只會被讀取，絕不會被修改。
shutter-source = 次數在檔案中的儲存位置因機型而異。這些位置來自公開記載的研究（ExifTool），並未在每個機型上都經過確認。
shutter-drop-hint = 將相片（ARW、JPEG、HEIF）拖放到這裡
btn-shutter-read = 分析相片
shutter-file-required = 請先選取相片。
shutter-file = 檔案
shutter-taken = 拍攝日期
shutter-count = 快門釋放次數
shutter-type = 快門
shutter-type-mechanical = 機械式
shutter-type-electronic = 電子式
shutter-electronic-hint = 這張相片是以電子快門拍攝的。在許多機型上，這類拍攝不會被計入；此時顯示的數值為最後一次機械快門釋放時的次數。
shutter-not-sony = 這張相片並非以 Sony 相機拍攝。無法從中讀取快門次數。
shutter-unknown-model = 無法得知此相機機型將快門次數儲存在檔案中的哪個位置，因此不會顯示數值。
shutter-not-recorded = 此檔案不含可讀取的快門次數。以影像編輯軟體儲存或匯出的相片，以及部分機型以電子快門拍攝的相片，都會發生這種情況。請從記憶卡中選取原始檔案。
shutter-model-mismatch = 這張相片的拍攝機型與已連接的相機不同。
shutter-invalid = 此檔案不是可讀取的相片
file-filter-photo = 相片（ARW、JPEG、HEIF）
usage-log-heading = 使用記錄
usage-log-hint = 相機會以未公開的格式保存一份內部記錄。記錄會逐字儲存；只會從相機讀取資料。
usage-log-written = 已寫入
btn-usage-log-info = 關於使用記錄
btn-usage-log-info-hide = 隱藏說明
usage-log-info-what-heading = 這份記錄是什麼
usage-log-info-what = Sony 內部將這份記錄稱為「KikiLog」。相機會自行寫入記錄；本程式只會讀取。目前沒有已知的公開格式說明。
usage-log-info-read = 讀取不會變更任何內容：連續多次下載的結果逐位元組完全相同。
usage-log-info-flush-heading = 為什麼缺少最近的項目
usage-log-info-flush = 新的項目通常不會立即出現：相機可能只在關機時才儲存這些項目，有時還會分散在數次開關機之間。請將相機關機再開機，重新下載，並重複此步驟，直到不再出現新項目為止。
usage-log-info-content-heading = 從中可以得知什麼
usage-log-info-shutter = 快門釋放：某些事件各代表一次曝光。不過，計算這些事件並不能得出快門次數：記錄的大小是固定的，不一定涵蓋相機的整個使用壽命。相機會將總數保存在別處，並寫入其儲存的相片中；可在「快門次數」區段中讀取。
usage-log-info-clock = 時間戳記的準確度取決於相機的時鐘。更換電池後，時鐘可能會倒退，並產生帶有相機初始日期的項目。
usage-log-info-unknown = 目前沒有已知的事件編號公開說明。本程式只會顯示這些編號，不加以解讀。
usage-log-info-privacy-heading = 隱私權
usage-log-info-privacy = 此檔案包含曾插入之記憶卡的序號。請勿隨意分享或張貼。
btn-usage-log = 儲存記錄
usagelog-size = 記錄區域
usagelog-cursor = 寫入位置
usagelog-spare = 第二區域
usagelog-count = 項目
usagelog-events = 依編號分類的事件
usagelog-entries = 依順序排列的項目
msg-link-copied = 連結已複製到剪貼簿。

# 後端失敗訊息，以後端回報的穩定代碼作為鍵。
# 訊息中的可變部分（路徑、狀態位元組、裝置錯誤）由前端附加，
# 因此即使沒有這些部分，這些訊息讀起來也是完整的句子。
# 刻意不提供 `err-operation`：該代碼標示的是後端
# 已翻譯好的訊息，必須原樣顯示。
err-usb-no-device = 找不到相機。請連接相機並開啟電源。如果相機的螢幕一直顯示正在連接，表示電腦沒有接收到相機：請使用資料傳輸線而非僅供充電的線材，不透過集線器或轉接器直接連接相機，並將相機關機再開機；如果仍然無效，請暫時取出電池。透過資料傳輸線連接時，即使相機已關機也會被偵測到，並回報為尚未就緒；如果相機關機時也出現此訊息，則該線材可能無法傳輸資料。
err-usb-camera-not-ready = 相機已連接，但尚未就緒：相機已關機，或正在螢幕上等待選擇 USB 模式。請開啟相機電源，或在相機上選擇「大量儲存（MSC）」。
camera-choose-usb-mode = 相機正在等待選擇 USB 模式。請在相機上選擇「大量儲存（MSC）」。
err-usb-too-many-devices = 連接了不只一台相機。請只保留要操作的那一台，中斷其他所有相機的連接。
err-usb-transfer = USB 傳輸失敗。請嘗試其他傳輸線或連接埠，並避免使用 USB 集線器。
err-usb-permission = 本程式無權透過 USB 開啟相機。在 Linux 上，此權限是透過針對 Sony 廠商 ID 的 udev 規則一次性授予的；原始碼樹中的指令碼 scripts/linux-prepare.sh 會安裝該規則。然後請拔下相機再重新插上。
err-usb-busy = 另一個程式正在使用相機，因此無法開啟。請關閉其他可能存取相機的程式；在 Linux 上，桌面環境的媒體監視器可能會占用相機，scripts/linux-prepare.sh 會將其停止。在 macOS 上，請將相機的 USB 連接設為 MTP：在大量儲存模式下，系統本身的磁碟驅動程式會占用相機。
err-usb-needs-root = macOS 只會在程式以系統管理員權限執行時，才將相機的大量儲存連線交給該程式。請使用 sudo 啟動命令列程式，例如：sudo pmca-console tweaks。若將相機的 USB 連接設為 MTP，即使沒有系統管理員權限也能讀取相機資訊。
macos-admin-prompt = PMCA-RE RELOADED 需要系統管理員權限，才能從 macOS 取得相機的大量儲存連線。作業期間，相機的記憶卡會從 Finder 中消失；如果卡上有已開啟的檔案，請先在 Finder 中將其退出。
macos-admin-declined = 未授予系統管理員權限，因此未對相機進行任何操作。在 macOS 上，此功能需要這些權限。
macos-admin-failed = 本程式無法以系統管理員權限啟動其協助程式。
err-usb-stall = 相機中止了 USB 傳輸。
err-usb-protocol = 相機傳送了非預期的回應。
err-usb-msc-status = 相機拒絕了儲存命令。
cli-error-prefix = 錯誤
cli-hint-mass-storage = 切換到大量儲存模式：pmca-console mass-storage
err-usb-driver = Windows 已將這台相機指派給不允許直接存取的驅動程式。此模式需要 WinUSB 驅動程式，本應用程式可以自行指派——但只有在以系統管理員權限執行時才能這麼做。
err-needs-elevation = 指派或解除驅動程式需要系統管理員權限，而 Windows 提示已遭拒絕。請透過「啟動驅動程式協助工具」再試一次；不需要變更相機上的任何設定。
err-usb-no-mass-storage-volume = 相機已以大量儲存模式連接，但沒有提供磁碟機。請喚醒相機，若螢幕上詢問 USB 模式請加以確認，並檢查是否已插入記憶卡。請勿變更驅動程式：此模式需要 Windows 的標準驅動程式。
err-usb-msc-sense = 相機回報了儲存錯誤。
err-usb-ptp-response = 相機拒絕了此命令。
err-usb-unsupported = 此相機機型不支援該功能。
err-usb-updater-status = 相機上的韌體更新程式拒絕了此操作。
err-usb-senser = 服務模式命令執行失敗。
err-market-core = 無法讀取相機資料檔案。
err-market-json = 收到的資料格式不正確。
err-market-missing-field = 回應中缺少預期的欄位。
err-market-no-apk = 尚未選取要安裝的應用程式。
err-market-http = 網路要求失敗。
err-market-io = 本機安裝伺服器發生錯誤。
err-market-tls = 無法建立與相機之間的加密連線。
err-apk = 無法讀取應用程式檔案。
err-io = 無法讀取或寫入檔案。
err-json = 相機傳送了格式不正確的資料。
err-unknown-locale = 無法使用此語言。
err-catalog-encoding = 翻譯檔案未以 UTF-8 文字格式儲存。請使用 UTF-8 編碼重新儲存。
err-internal = 發生內部錯誤。這是應用程式的程式錯誤。

## 功能分析
nav-analysis = 分析
analysis-title = 功能分析
analysis-experimental = 實驗性功能。此功能是新推出的，僅在少數相機上試用過。
analysis-own-hardware = 請只在您自己的相機上使用。服務模式本身就是 Sony 預期之外的介入操作；責任由您自行承擔。
analysis-explanation-toggle = 說明：用途、步驟、風險
analysis-purpose-heading = 這有什麼用途？
analysis-purpose = 功能分析會記錄相機將其某項選單設定儲存在設定記憶體（備份）中的哪個值。分析結果描述的是這台相機的設定記憶體；可以與其他相機比較，但不會改變相機所提供的功能。
analysis-how-heading = 運作方式為何？
analysis-how = 對於您輸入的每個選單值，本程式都會讀取一次完整的設定記憶體，每次都在各自的短暫服務模式工作階段中進行。在兩次讀取之間，您需拔下傳輸線並在相機選單中變更設定；相機保持開機狀態。最後會再讀取一次第一個值。隨選單變化、且在最後回到起始值的值即為候選值。會自行變化的值（計數器、時鐘）則會被排除。
analysis-requirements-heading = 需求條件
analysis-requirements = 相機已透過 USB 連接、其 USB 模式設為「自動」，且每次工作階段結束後，您都在相機上選擇「大量儲存（MSC）」。在 Windows 上，本程式需要系統管理員權限，因為它會為服務模式暫時指派驅動程式。請為電池充電；每個工作階段約需一分鐘。
analysis-read-heading = 讀取步驟
analysis-read = 快照只會讀取。相機的設定不會有任何變更。風險雖低但並非為零：切換到服務模式本身就是 Sony 預期之外的介入操作。
analysis-write-heading = 寫入步驟（寫入測試）
analysis-write = 只有在您明確開啟時才會執行：本程式會將找到的值寫入相機，您在重新開機後檢查選單中是否生效，接著再寫回原始值。只會寫入相機自行儲存過的值，而且只在恰好找到一個相符的候選值時才會寫入。舊值會事先儲存。即便如此，此步驟仍會變更相機；一旦發生故障，相機可能會進入只有維修中心才能修復的狀態。
analysis-legal-heading = 法律事項
analysis-legal = 本軟體不提供任何保固（MIT 授權）。對相機的介入操作可能會影響其保固。請只在您擁有的相機上使用此功能。分析報告不含序號，除了找到的候選值之外，也不含設定記憶體中的任何內容；它絕不會自動傳送到任何地方。
analysis-start-heading = 新的分析
analysis-setting-label = 選單設定（依相機上的名稱）
analysis-values-label = 選單值，以逗號分隔
analysis-values-hint = 第一個值是目前設定的值。可輸入二到八個值；每個值需要一個服務模式工作階段。
analysis-write-option = 接著執行寫入測試（會變更相機）
analysis-write-warning = 寫入測試會變更相機：它會將一個值寫入相機的設定記憶體，之後再寫回原始值。僅限在您自己的相機上進行，風險自負。
analysis-write-consent = 這台相機是我的，我願意承擔風險。
btn-analysis-write-enable = 啟用寫入測試
btn-analysis-start = 開始分析
btn-analysis-snapshot = 讀取備份
btn-analysis-new = 新的分析
analysis-discard-confirm = 要捨棄進行中的分析嗎？目前為止的報告會保持儲存。
analysis-access-read = 讀取
analysis-access-write = 寫入
analysis-access-user = 手動
analysis-access-local = 電腦
analysis-step-snapshot = 讀取備份，設定目前為「{ $value }」
analysis-step-change = 在相機選單中變更為「{ $value }」
analysis-step-evaluate = 比較快照
analysis-step-write = 將「{ $value }」的值寫入相機
analysis-step-check = 將相機關機再開機，讀取選單（預期：「{ $value }」）
analysis-step-restore = 寫回「{ $value }」的原始值
analysis-step-report = 儲存分析報告
analysis-do-prepare = 準備：1. 在相機選單中將「{ $setting }」設為「{ $value }」，然後按 MENU 按鈕離開選單，讓相機儲存該設定。2. 讓相機保持開機。3. 插上傳輸線，並在相機上選擇「大量儲存（MSC）」。
analysis-do-change = 換您操作：1. 拔下傳輸線。2. 相機保持開機。3. 在相機選單中將「{ $setting }」設為「{ $value }」，然後按 MENU 按鈕離開選單，讓相機儲存該設定。4. 重新插上傳輸線，並選擇「大量儲存（MSC）」。
analysis-do-check = 換您操作：1. 拔下傳輸線。2. 將相機關機再開機。3. 查看選單中「{ $setting }」顯示的內容。
analysis-do-reconnect = 重新插上傳輸線，並在相機上選擇「大量儲存（MSC）」。
analysis-reading = 第 { $index } 個快照（共 { $total } 個）：讀取期間切勿拔下傳輸線。
analysis-cli-continue = 按 Enter 繼續，按 q 取消：
analysis-confirm-word = 是
analysis-cli-confirm = 請輸入「{ $word }」以確認：
analysis-write-declined = 未確認。未寫入任何內容。
analysis-aborted = 已取消。未寫入任何內容。
analysis-snapshot-done = 快照已讀取。相機已離開服務模式。
analysis-result = 已比較 { $compared } 個值：{ $count } 個候選值，{ $running } 個值自行發生變化。
analysis-candidate-consistent = 與選單明確相符
analysis-candidate-inconsistent = 與選單並非明確相符
analysis-write-preview-heading = 將寫入下列內容
analysis-write-preview = 值 { $id }（{ $size } 個位元組）：從 { $old }（「{ $old-label }」）改為 { $new }（「{ $new-label }」）。設定為「{ $new-label }」時，相機自行儲存了這個新值。
btn-analysis-write = 寫入
btn-analysis-skip-write = 完成，不執行寫入測試
analysis-written = 值已寫入、讀回並永久保存。
analysis-menu-question = 選單現在顯示什麼？
analysis-menu-other = 其他內容
btn-analysis-menu-confirm = 確認
btn-analysis-restore = 寫回原始值
analysis-restored = 原始值已寫回並永久保存。
analysis-survived = 重新開機後，寫入的值仍然保存著。
analysis-not-survived = 重新開機後，寫入的值已不再保存。
analysis-report-saved = 分析報告已儲存：{ $path }
analysis-report-hint = 報告不含序號，也不含備份。您可以將其轉交他人，以便查核並記錄此發現。
btn-analysis-export = 匯出報告…
file-filter-analysis = 分析報告
analysis-pending = 有一項寫入測試尚未完成。原始值已儲存，可以寫回。
analysis-pending-none = 這台相機沒有儲存未完成的寫入測試。
analysis-invalid-setting = 請依相機上的名稱輸入選單設定。
analysis-invalid-values = 請輸入二到 { $max } 個不同的選單值。第一個值是目前設定的值。
analysis-already-complete = 所有快照均已讀取。
analysis-other-camera = 這台相機與第一個快照中的相機不同。分析已停止。
analysis-write-not-allowed = 在此狀態下無法執行寫入測試。
analysis-camera-changed = 相機目前保存的值與第一個快照中的值不同。未寫入任何內容。
analysis-write-refused = 相機拒絕了該值；一切維持原狀。（{ $detail }）
analysis-refuse-none = 無法執行寫入測試：沒有任何值隨選單變化。
analysis-refuse-ambiguous = 無法執行寫入測試：有 { $count } 個值隨選單變化，無法確定對應的是哪一個。
analysis-refuse-inconsistent = 無法執行寫入測試：該值與選單值並非明確相符。
analysis-refuse-size = 無法執行寫入測試：該值為 { $size } 個位元組；只會寫入較小的值。
analysis-refuse-read-only = 無法執行寫入測試：相機將該值標示為唯讀。
analysis-refuse-protected = 無法執行寫入測試：相機將該值標示為受保護。
settings-analysis-heading = 功能分析
settings-analysis-purpose = 功能分析會記錄相機將其某項選單設定儲存在設定記憶體中的哪個值。此功能適合想要了解自己機型的設定記憶體並分享結果的人。
settings-analysis-why-off = 此功能處於關閉狀態，因為一般使用並不需要它、它會多次將相機切換到服務模式，而且在您要求時還會寫入相機。此功能為實驗性質，僅在少數機型上試用過。
settings-analysis-toggle = 開啟功能分析（選單中的「分析」項目）
settings-analysis-confirm = 要開啟功能分析嗎？它會多次將相機切換到服務模式，並可在您要求時寫入相機。請只在您自己的相機上使用。
btn-analysis-enable = 開啟
settings-analysis-on = 功能分析已開啟。
settings-analysis-off = 功能分析已關閉。
analysis-needs-admin = 每個快照都需要系統管理員權限才能進入服務模式。Windows 在每個程式工作階段中只會詢問一次；您可以現在啟動驅動程式協助工具，或等到第一個快照要求時再啟動。
analysis-changed-hint = 請檢查：未找到任何候選值。下列值有所變化，但未回到起始值。如果其中某個值只在兩個值之間變化了一次，則相機在第一個快照時可能並未處於第一個選單值。此時請正確設定起始值後重新進行分析。
analysis-changed-info = 資訊：分析期間自行變化的值（例如計數器、時鐘）會被忽略。無需任何操作。
elevation-admin = 系統管理員
elevation-standard = 標準權限
elevation-admin-hint = 本程式正以系統管理員權限執行，可以指派服務模式所需的驅動程式。
elevation-standard-hint = 本程式正以標準權限執行。進行服務模式、調整項目與功能分析時，本程式會視需要以系統管理員權限啟動驅動程式協助工具；Windows 在每個工作階段中只會詢問一次。按一下即可啟動。
driver-helper-declined = 系統管理員權限的提示已遭拒絕。沒有這些權限，就無法指派服務模式所需的驅動程式；未變更任何內容。
btn-remove-foreign-drivers = 移除舊驅動程式
btn-not-now = 現在不要
btn-ok = 確定
foreign-drivers-found = Windows 仍保留由 Zadig 或類似工具為 Sony 裝置安裝的驅動程式：{ $packages }。在本程式指派自己的驅動程式之前，Windows 會先將相機交給這些驅動程式，因此服務模式會失敗，相機也可能不會顯示為磁碟機。要立即移除它們嗎？之後相機會使用 Windows 內建的驅動程式。移除這些驅動程式需要一次系統管理員權限。
foreign-drivers-removed = 舊驅動程式已移除。請拔下相機再重新插上。
foreign-drivers-remaining = 無法移除下列驅動程式：{ $packages }。請重新啟動 Windows 後再試一次，或在裝置管理員中將裝置連同其驅動程式一起解除安裝。
foreign-drivers-hint = 由 Zadig 或類似工具為 Sony 裝置安裝的驅動程式仍然存在（{ $packages }）。在本程式指派自己的驅動程式之前，Windows 會先將相機交給這些驅動程式，這可能導致服務模式失敗。請使用下列命令移除：pmca-console driver zadig --remove
driver-zadig-none = 沒有為 Sony 裝置安裝任何來自 Zadig 或類似工具的驅動程式。
driver-zadig-entry = { $package }（{ $provider }）：{ $devices }
driver-zadig-remove-hint = 請使用下列命令移除：pmca-console driver zadig --remove
driver-zadig-removed = 已移除 { $count } 個舊驅動程式套件。請拔下相機再重新插上。
elevation-helper = 驅動程式協助工具已啟用
elevation-helper-hint = 具備系統管理員權限的驅動程式協助工具會指派服務模式所需的驅動程式。它沒有視窗，會隨本程式一起結束；之後不會再出現提示。
elevation-helper-confirm = 要立即啟動驅動程式協助工具嗎？Windows 會詢問一次系統管理員權限，在本程式結束前不會再詢問。
analysis-cross-heading = 在另一台相機上交叉檢查
analysis-cross-intro = 選用：檢查另一台相機是否也有找到的值，以及其設定為何。此操作只會讀取。相同的編號不一定代表相同的功能；只有查看另一台相機的選單才能確認。結果會寫入同一份分析報告。
analysis-cross-do = 1. 拔下已分析的相機。2. 開啟另一台相機，將其 USB 模式設為「自動」，插上傳輸線並選擇「大量儲存（MSC）」。
btn-analysis-cross = 檢查另一台相機
analysis-cross-present = 存在，值為 { $value }
analysis-cross-match = 等於「{ $label }」
analysis-cross-nomatch = 不等於任何已測量的選單值
analysis-cross-absent = 不存在
analysis-cross-same-camera = 這就是已分析的相機本身。請連接另一台相機。
analysis-cross-not-now = 只有在所有快照都已讀取，且沒有等待還原的已寫入值時，才能進行交叉檢查。
analysis-cross-none = 沒有可交叉檢查的候選值。
analysis-wrong-camera = 已連接的相機不是已分析的那一台。請連接 { $model }；未寫入任何內容。
keys-fetching = 正在下載來源…
keys-scanning = 正在搜尋金鑰資料…
keys-complete = 所有金鑰資料皆已齊備。
keys-incomplete = 仍缺少部分金鑰資料。
keys-present = 已存在
keys-missing = 缺少
keys-class-senser = 服務模式金鑰（服務模式終端機、調整項目、分析）
keys-class-cic = XPD 總和檢查碼金鑰（應用程式安裝）
keys-class-spk = SPK 金鑰資料（應用程式安裝、apk2spk）
keys-class-fdat = FDAT 金鑰資料（更新模式終端機韌體）
plugin-heading = 外掛程式
plugin-intro = 外掛程式可為本程式擴充語言、相機設定開關、相容性附註、工具與診斷功能。外掛程式來自您自行選擇的檔案，或來自您已新增的來源；本程式不會檢查它們。其程式碼在沙箱中執行，無法存取您的檔案、網路或金鑰資料。請只安裝您信任其作者的外掛程式；變更會在下次啟動時生效。
plugin-none = 未安裝任何外掛程式。
plugin-inactive = （重新啟動後生效）
plugin-author = 作者
plugin-restart = 請重新啟動本程式，變更才會生效。
btn-plugin-install = 安裝外掛程式…
plugin-install-confirm = 此外掛程式可以：
btn-confirm-plugin-install = 安裝外掛程式
plugin-installed = 外掛程式已安裝。
plugin-remove-confirm = 要移除外掛程式「{ $name }」嗎？
plugin-removed = 外掛程式 { $id } 已移除。
plugin-perm-languages = 為本程式新增語言
plugin-perm-camera-settings = 變更相機設定（僅限在其指定的兩個值之間，且須經您確認）
plugin-perm-compatibility = 在相容性概覽中新增項目
plugin-perm-code = 在沙箱中執行自己的程式碼（工具或診斷）
plugin-perm-host-os = 查詢目前使用的作業系統
plugin-perm-host-cameras = 查看連接了哪些 Sony 相機，以及它們處於何種模式
plugin-invalid = 這不是可用的外掛程式：{ $reason }
plugin-unknown-field = 此外掛程式使用了本程式此版本不認得的功能（{ $field }）。它是為較新版本或其他版本的本程式所製作的。
plugin-no-folder = 無法判斷使用者資料資料夾，因此無法安裝外掛程式。
plugin-not-installed = 未安裝 ID 為 { $id } 的外掛程式。
plugin-failed = 外掛程式執行失敗：{ $reason }
plugin-unknown-entry = 沒有任何已載入的外掛程式提供 { $id }。
plugin-run-done = 外掛程式已執行完畢。
plugin-run-warning = 外掛程式已執行完畢，但有附註。
plugin-run-error = 外掛程式回報了錯誤。
plugin-tools-heading = 外掛程式工具
plugin-tools-hint = 這些工具來自已安裝的外掛程式，並在沙箱中執行。外掛程式只會收到您交給它的內容及其所宣告的內容，且無法變更相機。
plugin-diagnostics-heading = 外掛程式診斷
plugin-compat-heading = 來自外掛程式
plugin-col-note = 附註
file-filter-plugin = 外掛程式檔案
keys-not-set-up = 尚未設定此操作所需的金鑰資料。
err-missing-keys = 尚未設定此操作所需的金鑰資料。
keys-heading = 需要金鑰資料
keys-intro = 服務模式金鑰、XPD 總和檢查碼金鑰與 SPK 金鑰資料並不包含在本程式中。請輸入您擁有的來源位址——存放庫、封存檔或檔案——本程式會找出並匯入它所認得的部分。不會執行任何內容，而且這些資料只會保存在這部電腦上。
keys-needs-toggle = 哪些功能需要哪些金鑰資料？
keys-needs-info = 相機資訊（MTP、大量儲存、更新模式）
keys-needs-info-keys = 不需要。
keys-needs-install = 應用程式安裝與「檢查相機」
keys-needs-install-keys = XPD 總和檢查碼金鑰；安裝應用程式另需 SPK 金鑰資料。
keys-needs-service = 服務模式：服務模式終端機、調整項目、分析
keys-needs-service-keys = 服務模式金鑰。
keys-needs-updater = 更新模式：更新模式終端機與應用程式備份
keys-needs-updater-keys = FDAT 金鑰資料。
keys-law-notice = 請遵守您所在地區適用的法律，並且只使用您自己的相機與資料。
keys-url-label = 來源位址（每行一個）
keys-url-required = 請至少輸入一個來源位址。
btn-keys-import = 匯入
btn-import-keys = 設定金鑰資料
