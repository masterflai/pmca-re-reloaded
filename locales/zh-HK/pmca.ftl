# PMCA-RE RELOADED 繁體中文（香港）本地化。
# 每行一條訊息：<id> = <text>。如要加入新語言，請將此檔案複製到
# locales/<lang>/pmca.ftl，並只翻譯等號右邊的文字。

app-title = PMCA-RE RELOADED – 相機工具箱

# 導覽／分頁
nav-camera-info = 相機資料
nav-install-app = 應用程式
nav-tweaks = 調整
nav-network = 網絡
nav-firmware = 固件
nav-updater = 更新模式 Shell
nav-service = 服務模式 Shell
nav-tools = 檔案工具
nav-settings = 設定
nav-license = 授權

# 歡迎頁面：視窗開啟時顯示的頁面。它說明這套工具的用途、
# 首先要做甚麼，以及哪個頁面解答哪個問題。
#
# 「語言」在本應用程式中有兩種不同意思：本視窗的語言，
# 以及相機本身選單提供的語言。兩者各有獨立標題，
# 而非只各佔一句，因為混淆兩者正是
# 本頁面要防止的錯誤。
nav-welcome = 歡迎
welcome-intro-heading = 本程式的用途
welcome-intro-body = 本程式協助您保留 Sony 相機的功能，並按您的需要作出調整。自從 Sony 關閉 PlayMemories Camera Apps 商店後，已購買或已安裝的應用程式便無法備份或重新安裝。重設相機會永久刪除已安裝的應用程式，且無法復原。本程式可備份現有的應用程式並重新安裝，亦可提供可免費取得的社群應用程式。您亦可安裝自己的應用程式（*.apk 檔案）。出現問題時，本程式會讀取相機資料及內部記錄以供分析。相機本身具備但選單中沒有提供的設定，可以切換，亦可再次還原。本程式透過 USB 線直接與相機通訊。支援的型號列於「相機資料」頁面。
welcome-start-heading = 第一步
welcome-start-body = 請先開啟相機，並在尚未接上 USB 線時，於相機選單中將「USB 連接」設為「大容量儲存」或 MTP。「自動」通常亦可行，因為相機會自行選定兩者之一。次序很重要：一旦接上 USB 線，便無法再進入相機的選單。之後才以 USB 連接相機，開啟「相機資料」並按「取得相機資料」。當型號及固件版本顯示出來，即表示連接正常，本程式的所有其他功能亦可使用。
welcome-start-driver = 如出現的是驅動程式提示，請先按該提示操作。在 Windows 上，系統驅動程式會佔用相機，必須先將其替換，才能進行本工具所需的直接 USB 存取。本程式會自行處理，但指派驅動程式是 Windows 只授予管理員的權利。在此情況下，本程式會讓您選擇以管理員身分重新啟動，而 Windows 會要求您確認操作。工作階段結束時，該指派會再次解除。在 Linux 及 macOS 上無需此步驟。
welcome-start-driver-linux = 如出現的是驅動程式提示，請先按該提示操作。在 Linux 上無需替換任何東西：程式可直接存取相機。只需一次性準備開啟裝置的權限——即一條針對 Sony 廠商 ID 的 udev 規則——以及在大容量儲存模式下釋放核心驅動程式 usb-storage。兩者均須在本程式以外以 root 權限準備；原始碼樹中的 scripts/linux-prepare.sh 腳本會同時處理兩者。本視窗本身從不要求此類權限。
welcome-start-driver-macos = 如出現的是驅動程式提示，請先按該提示操作。在 macOS 上無需安裝或替換任何東西。請將相機的 USB 連接設為 MTP。使用調整、服務模式 Shell、功能分析、固件更新及更新模式 Shell 時，本視窗會要求您輸入一次管理員密碼：這些功能經由相機的大容量儲存連接進行，而 macOS 只會將該連接交給具備此等權限的程式。
welcome-tweaks-heading = 調整與語言
welcome-tweaks-body = 調整是指相機本身已具備、但選單中沒有提供的設定：錄影時間限制、4K 錄影限制、PAL 與 NTSC 之間的選擇、USB 應用程式安裝程式——以及選單語言。
welcome-tweaks-languages-heading = 相機的語言
welcome-tweaks-languages-body = Sony 相機只會顯示其銷售地區常用的選單語言。如您身處海外，往往找不到自己的語言，只能將就使用外語。其實固件中已內置更多語言。本程式會將這些語言加入選單，讓您能以熟悉的語言操作相機。
welcome-tweaks-ui-heading = 本程式的語言
welcome-tweaks-ui-body = 與上述完全無關的是頂部的選擇器。它只會變更本程式的語言，絕不會觸及相機。
welcome-tweaks-safety = 每項調整都會寫入相機的備份記憶體，而且每一項都可以用同一個開關還原。寫入中斷可能會永久損壞相機，因此寫入期間請勿拔除 USB 線或取出電池。
welcome-privacy-heading = 您的資料
welcome-privacy-where = 「授權」頁面會再次完整說明上述內容，並列出構成本程式的所有程式庫的授權條款。
welcome-where-heading = 功能位置一覽
welcome-where-info = 型號、序號及固件版本。這亦是連接測試。
welcome-where-install = 社群清單中的應用程式，或您自己的 .apk 檔案。
welcome-where-tweaks = 相機選單無法觸及的設定，包括語言。
welcome-where-network = GPS 輔助數據，以及直播所用的存取點和服務。
welcome-where-firmware = 將 Sony 固件映像寫入相機。
welcome-where-tools = 解開及檢查固件和套件檔案。無需連接相機。
welcome-where-shells = 直接存取相機的記憶體及檔案系統。須先在「設定」中啟用。
welcome-where-settings = 主題、進階模式，以及編輯本程式的翻譯。
welcome-where-license = 本移植版本的來源、建基於哪些人的成果，以及適用的條款。
welcome-where-hint = 沒有任何頁面會自行操作相機。在您按下註明會進行讀取或寫入的按鈕之前，本程式不會讀取或寫入任何內容。

# 按鈕
btn-get-info = 取得相機資料
btn-refresh = 重新整理
btn-load-apps = 載入可用的應用程式
btn-list-tweaks = 顯示可用的調整
btn-install = 安裝所選的應用程式
btn-install-file = 安裝 .apk 檔案…
btn-apply = 套用
btn-save = 儲存至相機
btn-add = 新增
btn-remove = 移除
btn-reveal = 顯示
btn-update-gps = 將 GPS 輔助數據傳送至裝置
btn-cancel = 取消
btn-browse = 瀏覽…
btn-switch-msc = 切換至大容量儲存模式
btn-start-driver-helper = 啟動驅動程式助手

# 檔案對話方塊所提供的檔案類型篩選器標籤
file-filter-apk = Android 應用程式（*.apk）
file-filter-firmware = Sony 固件映像（*.dat）
file-filter-usage-log = 使用記錄（*.bin）
file-filter-catalog = 翻譯檔案（*.ftl）
file-filter-spk = Sony 應用程式套件（*.spk）
file-filter-any = 任何檔案
file-filter-gps = GPS 輔助數據（*.dat）

# 狀態／訊息
msg-connecting = 正在連接…
msg-usage-log-written = 使用記錄已寫入 { $path }
msg-apps-found = 找到 { $count } 個應用程式
msg-switch-mode = 請將相機切換至 MTP 或大容量儲存模式。
msg-no-tweaks = 此相機沒有可用的調整。
msg-done = 完成。

# 相機 USB 模式切換（MTP -> 大容量儲存）。顯示於「模式錯誤」
# 錯誤訊息下方；反方向沒有對應的 USB 指令。
mode-switching = 正在要求相機切換至大容量儲存模式…
mode-switch-wait = 正在等待相機重新連接…
camera-confirm-hint = 相機上的訊息（例如有關 www.sony.net/pm 的訊息）可以關閉，不會造成干擾。
mode-switch-done = 相機現已處於大容量儲存模式。
mode-switch-timeout = 相機未有以大容量儲存模式重新出現。請將相機關機再開機，然後再試一次。
mode-switch-impossible = 此模式無法透過 USB 變更。請將相機關機再開機。
mode-switch-hint = 只有處於 MTP 模式的相機才能透過 USB 切換。此變更只屬暫時性：相機本身選單中所選的 USB 模式維持不變，因此將相機關機再開機——或拔除再重新接上 USB 線——便會回復之前的模式。反方向沒有對應的 USB 指令。

# 相機資料欄位
info-model = 型號
info-product-code = 產品代碼
info-serial = 序號
info-firmware = 固件版本
info-lens = 鏡頭
info-region = 地區
info-mode = 模式
mode-mass-storage = 大容量儲存
mode-mtp = MTP
mode-updater = 更新模式
mode-app-install = 應用程式安裝模式
mode-service = 服務模式
mode-unknown = 不明
info-android-version = Android 平台版本
info-photographer = 攝影師
info-copyright = 版權
info-photographer-1 = 攝影師（項目 1）
info-photographer-2 = 攝影師（項目 2）
info-copyright-1 = 版權（項目 1）
info-copyright-2 = 版權（項目 2）
diagnose-services-heading = 無法連接相機？
diagnose-services-hint = 部分 Windows 服務可能會佔用處於 MTP 模式的相機，令本應用程式無法存取相機。此檢查只會向 Windows 查詢正在執行哪些服務，不會作出任何變更。
btn-diagnose-services = 檢查 Windows 服務
diagnose-services-none = 沒有已知會造成干擾的服務正在執行。
diagnose-services-found = 以下正在執行的服務可能會干擾相機存取：
diagnose-services-advice = 請在「服務」（services.msc）中停止該服務，或以管理員身分執行以下指令，然後重新連接相機：

# 固件更新程式／Shell
updater-model = 相機型號
updater-firmware-current = 目前的固件版本
updater-guard-ok = 固件映像與此相機相符。
updater-no-fdat = 沒有適用於型號 { $model } 的內置更新程式固件。
updater-model-required = 相機已處於更新模式，在此模式下無法報告其型號。請將相機關機再開機，讓它以正常模式啟動，然後再試一次。或者，您可指定有效負載檔案（--fdat-file），或在主控台中指定型號（--model <MODEL>）。
updater-already-updater = 相機已處於更新模式。
updater-flash-required = 未經確認，拒絕修改相機。請加上 --flash 重新執行，以切換至更新模式並寫入有效負載。
updater-flashing = 正在將更新程式有效負載寫入相機…
updater-switch-wait = 請按照相機螢幕上的指示操作。正在等待相機切換至更新模式…
updater-switch-timeout = 等待相機逾時。請在相機重新連接後再次執行指令。
updater-shell-ready = 更新程式有效負載正在執行。輸入「help」查看指令，輸入「exit」離開。
updater-shell-done = Shell 工作階段已結束。
updater-session-panic = 更新模式工作階段意外結束。請先將相機關機再開機，然後再試。

# 更新模式 Shell 的回應。解析失敗的訊息只在 service-* 鍵中寫一次：
# 兩個 Shell 共用同一個詞法分析器，因此帶引號的路徑及無效的
# 十六進制數值在兩者中的失敗方式相同。
updater-backup-synced = 備份已同步至快閃記憶體。
updater-backup-locked = 受保護的備份設定現已鎖定。
updater-backup-writable = 受保護的備份設定現已可以寫入。
updater-shell-console-only = root Shell 需要終端機，因此只能在主控台中使用：請執行 pmca-console updatershell --flash，並在該處輸入「shell」。

# 更新模式 Shell 頁面。
updater-warning = 警告：進入此 Shell 時，相機會重新啟動至更新模式，並寫入一個有效負載——那是本項目的一個小程式，相機會像接受固件更新一樣接受它，之後由它回應此 Shell 的指令。有效負載本身會在相機關機時被捨棄，但它所提供的指令會寫入相機的永久設定記憶體，而寫入中斷可能會永久損壞您的相機。此外，此過程亦會重設相機的所有設定——這點與服務模式 Shell 不同。
updater-console-heading = 在主控台中執行更新模式 Shell
updater-driver-hint = 更新模式 Shell 需要相機使用 Windows 本身的大容量儲存驅動程式 USBSTOR，因為有效負載正是經由它傳送。這與服務模式 Shell 的需要恰恰相反。本應用程式會自行指派及解除 WinUSB 驅動程式，因此無需手動準備任何東西——但由其他工具指派的驅動程式必須先行還原。處於更新模式的相機會以其獨有的產品 ID 列舉，同樣不得重新綁定。
updater-fdat-label = 有效負載檔案（選填）
updater-confirm = 相機將重新啟動至更新模式，並寫入有效負載。要繼續嗎？
updater-cancelled = 已取消。相機未被修改。

# 兩個 Shell 頁面上可摺疊的說明。指令語法本身永不翻譯——
# 那些是必須準確輸入的字詞——因此這裏只存放
# 每條指令旁邊的說明。
btn-shell-primer = 關於此 Shell
btn-shell-primer-hide = 隱藏說明
shell-primer-compare-heading = 兩個 Shell，兩種通訊協定
shell-primer-compare = 兩個頁面都提供連接相機的指令列，但它們並非同一個 Shell。服務模式 Shell 使用 Sony 本身的工廠維修通訊協定，相機在驗證您的身分後便會自行回應；過程中不會將任何內容寫入相機。更新模式 Shell 之所以存在，純粹因為本項目將自己的程式寫入相機：相機重新啟動至更新模式，把有效負載當作固件映像接收，之後由有效負載回應下列指令。兩者同一時間只能開啟其中一個。
shell-primer-payload = 「有效負載」（payload）是指傳送途徑所承載的實際內容。就更新模式 Shell 而言，它是本項目的一個小程式：相機以接受固件更新的相同方式接受它，但只會保留至關機為止。服務模式 Shell 無需有效負載，因為相機以其本身的固件回應指令。就固件檔案而言，此詞的意思相同：即 .dat 檔案容器內的實際固件映像。
shell-primer-when-service = 凡是要在相機繼續執行其本身固件的同時，讀取或編輯相機已儲存的設定，都應使用服務模式 Shell——例如讀取 HASP 區塊、儲存備份區域的回復用副本、變更個別項目、從相機複製檔案。「調整」頁面在底層使用的亦是它，因此凡是在該頁面可以做到的，在這裏都可以手動完成。
shell-primer-when-updater = 當無法以其他方式連接相機，或您需要的是在執行中系統上的 Shell 而非個別指令時，便應使用更新模式 Shell。這是唯一提供真正 root Shell 的途徑，而且在拒絕進入服務模式的相機上仍然可用。代價是一次重新啟動、一次寫入以及相機的設定，因此它是次選而非首選。
shell-primer-driver-heading = 驅動程式：兩者的需要恰恰相反
shell-primer-driver = 服務模式 Shell 使用原始 USB 通訊，在 Windows 上需要相機綁定至 WinUSB。本應用程式在進入服務模式時會自行指派該驅動程式，並在工作階段結束時再次解除，因此無需使用其他工具。更新模式 Shell 經由 Windows 本身的大容量儲存驅動程式 USBSTOR 傳送，當相機綁定至 WinUSB 時便無法運作——因此解除指派是完成服務模式工作階段的一部分，而非事後補救。指派驅動程式需要管理員權限。在 Linux 上兩者均無需更換驅動程式，只需要裝置節點的存取權限。
shell-primer-risk-heading = 可能出現的問題
shell-primer-risk = 下列已標示的指令會寫入相機的永久設定記憶體。寫入中斷可能令相機無法啟動，而這裏的操作全部無法復原——這正是 bk d 存在的原因：在變更任何內容前，先儲存一份備份區域的副本。更新程式有效負載本身不會保留在相機上；相機關機時便會將其捨棄。不過，經由更新模式這條途徑會重設相機的所有設定。
shell-primer-commands-heading = 此 Shell 的指令
shell-primer-writes = 標示為（寫入）的指令會變更相機。其餘指令只會讀取。
shell-cmd-hasp = 讀取 HASP 區塊，即相機受保護的身分區域。
shell-cmd-pull = 從相機複製檔案。如未指定本機檔案名稱，則沿用遠端檔案名稱。
shell-cmd-push = 將本機檔案複製至相機，並取代該處的現有內容。（寫入）
shell-cmd-rm = 刪除相機上的檔案。（寫入）
shell-cmd-rmem = 將裝置記憶體的一個區域寫入本機檔案。位移及長度均為十六進制。
shell-cmd-bk-r = 讀取備份區域的一個項目，並以十六進制傾印顯示。
shell-cmd-bk-d = 將整個備份區域儲存至檔案。請在變更任何內容前執行此操作：這是唯一的回復途徑。
shell-cmd-bk-dump-txt = 將整個備份區域儲存為 <file>.bin，並將可讀的清單儲存為 <file>.txt。
shell-cmd-bk-stat = 顯示相機的備份狀態區塊，包括地區。
shell-cmd-bk-w = 寫入備份區域的一個項目，內容以空格分隔的十六進制位元組表示。（寫入）
shell-cmd-bk-s-sub = 將某個子系統的備份區域儲存至快閃記憶體，令變更在重新啟動後仍然保留。（寫入）
shell-cmd-bk-s = 將備份區域儲存至快閃記憶體，令變更在重新啟動後仍然保留。（寫入）
shell-cmd-bk-lock = 設定或清除受保護備份項目的寫入保護；部分設定在寫入保護啟用時會拒絕變更。（寫入）
shell-cmd-term = 開啟或關閉相機的偵錯終端機。（寫入）
shell-cmd-info = 讀取備份狀態區塊，該區塊會顯示該區域究竟可否寫入。
shell-cmd-shell = 在相機上開啟 root Shell。它需要終端機，因此只能在主控台中使用；shell tcp <port> 則會改為在 127.0.0.1 上提供給其他用戶端使用。（寫入）
shell-cmd-help = 列出此 Shell 接受的指令。
shell-cmd-exit = 結束工作階段並離開 Shell。
shell-primer-diagnostics-heading = 診斷與修復
shell-cmd-bk-trace = 將相機透過 USB 傳送整個備份區域的過程，逐個封包記錄至檔案。用於調查大型備份；此操作可能會令相機在回應途中停滯。屆時請拔除 USB 線，並將相機關機再開機，如有需要請取出電池。
shell-cmd-bk-rewrite = 讀取備份區域，原封不動寫回，然後再次讀取。在不變更任何設定的情況下，顯示相機是否接受完整映像。（寫入）
shell-cmd-bk-restore-attrs = 將所有備份項目的屬性（例如寫入保護）還原為以 bk d 或 bk dump-txt 儲存的副本中的屬性。數值維持不變。請只使用同一部相機的副本。（寫入）
shell-primer-diagnostics-env = 如在啟動前設定以下三個環境變數，它們會同時影響主控台及圖形介面，例如在 PowerShell 中使用 $env:PMCA_TWEAKS_DIRECT = "1"。PMCA_TWEAKS_DIRECT 令調整即使在備份小於 1 MB 時亦寫入個別數值。PMCA_TWEAKS_IMAGE 令調整即使在備份大於 1 MB 時亦寫入完整映像；兩者同時設定時，以 PMCA_TWEAKS_DIRECT 為準。PMCA_TLS_DUMP=<file> 會在安裝應用程式期間儲存相機的第一條 TLS 訊息（ClientHello），用於調查連接失敗的原因。

# 應用程式安裝
install-switching = 正在切換至應用程式安裝模式…
install-disabled = 相機拒絕進入應用程式安裝模式。這通常與「啟用 USB 應用程式安裝程式」設定有關：必須在「調整」中將其開啟。
install-stays-app-mode = 相機會保持在應用程式安裝模式，可繼續安裝其他應用程式。如要使用所有其他功能，請拔除 USB 線，然後將相機關機再開機。
err-usb-app-install-mode = 相機仍處於應用程式安裝模式，此模式下只接受應用程式安裝。請拔除 USB 線，然後將相機關機再開機——如在仍然連接時重新啟動，相機會直接返回此模式。
install-switch-wait = 正在等待相機以應用程式安裝模式重新連接…
install-switch-timeout = 等待相機進入應用程式安裝模式逾時。請在相機重新連接後再次執行指令。
install-bad-mode = 相機無法在此模式下使用。請將其切換至 MTP 或大容量儲存模式。
install-starting = 正在開始安裝…
install-error = 安裝通訊錯誤
install-server-failed = 本機安裝伺服器未有接受相機的連接：
install-done = 工作已順利完成。

# APK 檢查
apk-package = 套件：
apk-unreadable = 警告：無法分析此 .apk 檔案。
apk-incompatible = 警告：此應用程式聲明的最低 SDK 級別可能高於相機所能提供的級別。
apk-unsigned = 警告：此 .apk 檔案沒有簽署。

# 社群應用程式商店
store-loading = 正在載入應用程式清單…
store-resolving = 正在查詢最新版本…
store-downloading = 正在下載
store-unknown-app = 不明應用程式
store-no-release = 此應用程式沒有可下載的版本
store-not-installable = 沒有版本
msg-no-apps = 找不到應用程式。
store-heading = 社群應用程式商店
store-discontinued = Sony 已關閉官方的 PlayMemories Camera Apps 商店；其伺服器已不再回應，因此無法再從該商店購買或下載應用程式。
store-use-own-apk = 您仍可安裝自己的 .apk：在下方選取檔案，或將檔案拖放至此視窗。下方的社群應用程式商店亦仍然可用。
btn-store-sync = 檢查相機
btn-install-selected = 安裝所選項目
btn-store-stop = 完成此應用程式後停止
btn-store-stopping = 將於完成此應用程式後停止…
btn-store-retry = 重試失敗的應用程式
btn-continue = 繼續
btn-reinstall = 重新安裝
btn-skip = 略過
btn-confirm-updater = 重新啟動並寫入
btn-confirm-tweaks = 寫入調整
btn-confirm-languages = 啟用語言
btn-confirm-wifi = 取代存取點
btn-confirm-stream = 覆寫設定
btn-confirm-show-shells = 顯示 Shell
btn-confirm-discard = 捨棄翻譯
store-select-all = 選取所有應用程式
store-col-app = 應用程式
store-col-version = 版本
store-col-author = 作者
store-col-camera = 相機上的版本
store-col-status = 狀態
store-camera-absent = 未安裝
store-version-unknown = 不明
store-batch-query = 正在從相機讀取已安裝的應用程式…
store-sync-done = 已檢查相機。清單中已安裝的應用程式：{ $count }
store-batch-downloading = 正在下載第 { $current } 個應用程式（共 { $total } 個）…
store-batch-installing = 第 { $current } 個應用程式（共 { $total } 個）：正在安裝 { $name }…
store-reinstall-confirm = { $name } 已安裝在相機上。已安裝版本：{ $installed }。應用程式商店中的版本：{ $store }。要重新安裝嗎？
store-state-queued = 等候中
store-state-installed = 已安裝
store-state-failed = 失敗
store-state-skipped = 已略過
store-state-not-run = 未執行
store-batch-summary = 已安裝：{ $done } · 失敗：{ $failed } · 已略過：{ $skipped } · 未執行：{ $notrun }
store-sources-hint = 這些應用程式來自您在「設定」中新增的來源。本程式不會檢查它們。
app-backup-heading = 備份已安裝的應用程式
app-backup-intro = 將相機上已安裝的應用程式複製至此電腦，日後即使已無處可取得，亦可重新安裝。為此，相機會重新啟動至更新模式；其應用程式儲存區只會被讀取。
app-backup-own-camera = 請只備份您自己相機的應用程式，並且不要將檔案轉交他人：應用程式的權利屬於其作者。
app-backup-restore-hint = 如要還原應用程式，請在上方的「安裝 .apk 檔案…」中選擇已儲存的 .apk 檔案。
btn-app-backup = 備份應用程式
app-backup-confirm = 相機將重新啟動至更新模式，並寫入一個會持續執行至相機關機為止的有效負載。應用程式儲存區只會被讀取。要繼續嗎？
btn-confirm-app-backup = 重新啟動並備份
app-backup-cancelled = 已取消備份。
app-backup-confirm-required = 備份會令相機重新啟動至更新模式。請以 --confirm 確認。
app-backup-no-folder = 此電腦上沒有用作存放備份的數據資料夾。請以 --out 選擇資料夾。
app-backup-no-model = 無法讀取相機型號。請將相機關機再開機，以大容量儲存或 MTP 模式連接，然後再試一次。
app-backup-mounting = 正在尋找相機的應用程式儲存區…
app-backup-no-partition = 在此相機上找不到應用程式儲存區。它可能不支援應用程式。
app-backup-copying = 正在複製 { $name }（{ $current }/{ $total }）…
app-backup-none = 相機上沒有安裝任何應用程式。
app-backup-done = 已儲存 { $count } 個應用程式。
app-backup-power-cycle = 請將相機關機再開機，以離開更新模式。
app-backup-folder = 資料夾
app-backup-col-file = 檔案
app-backup-state-new = 已儲存
app-backup-state-kept = 先前已儲存
btn-copy-path = 複製路徑
msg-path-copied = 路徑已複製至剪貼簿。
store-no-sources = 尚未新增任何來源。請在「設定 → 應用程式及插件來源」中新增一個；原有項目的應用程式清單會在該處列為建議。
store-col-source = 來源
store-filter-source = 顯示來源
store-filter-all = 全部來源
sources-heading = 應用程式及插件來源
sources-intro = 應用程式及插件來自您在此自行新增的來源：開發者用來發佈其作品的公開 GitHub 儲存庫。本程式不會寄存、檢查或推薦任何來源或其中的任何內容；來源的責任完全由其作者承擔。新增儲存庫之前，請先查看清楚。
sources-none = 尚未新增任何來源。
sources-add-label = GitHub 儲存庫（擁有者/名稱或其地址）
btn-source-add = 新增來源
btn-confirm-source-add = 新增來源
sources-add-confirm = 要將 { $source } 新增為來源嗎？本程式未有檢查過它。安裝它所提供的內容，責任由其作者承擔。同意之前，請先查看儲存庫：{ $url }
sources-adding = 正在讀取來源…
sources-added = 已新增來源：{ $source }
sources-check-repo = 從儲存庫安裝任何內容之前，請先查看清楚：{ $url }
sources-removed = 已移除來源：{ $source }
sources-remove-confirm = 要移除來源 { $source } 嗎？為其應用程式記下的簽署者亦會一併清除。
sources-stars-title = GitHub 上的星星：表示有多少用戶標記了此儲存庫。星星並不代表質素或安全性。
sources-suggestion-heading = 建議
sources-suggestion-community = 作者已將其項目封存的相機應用程式。此清單由本程式的作者 masterflai 維護；它只指向應用程式作者自行發佈的版本，不包含任何自有檔案。這些應用程式未經檢查。
sources-suggestion-applist = 原有項目 Sony-PMCA-RE 的應用程式清單。它由其作者 ma1co 維護，而非由本程式維護，並已多年未有更新。
btn-source-activate = 啟用
sources-search-heading = 在 GitHub 上尋找來源
sources-search-hint = 在 GitHub 上搜尋主題為 pmca-source 的儲存庫。搜尋結果未經本程式檢查，亦不代表推薦。新增儲存庫之前，請先查看清楚；星星只顯示有多少 GitHub 用戶標記了它。
btn-source-search = 搜尋 GitHub
sources-searching = 正在搜尋 GitHub…
sources-search-none = GitHub 找不到主題為 pmca-source 的儲存庫。
sources-search-found = 找到 { $count } 個儲存庫。當中沒有任何一個經本程式檢查。
sources-updated = 最後更改：{ $date }
sources-archived = 已封存
sources-already = 已新增
sources-no-folder = 無法確定用戶數據資料夾，因此無法儲存來源。
source-invalid = 這不是 GitHub 儲存庫。請輸入「擁有者/名稱」，例如 ma1co/OpenMemories-AppList，或輸入儲存庫的地址。
source-exists = { $source } 已經新增過。
source-unknown = 沒有此來源：{ $source }
source-unreadable = { $source } 的預設分支中沒有可讀取的 { $file }。
source-failed = 無法讀取來源 { $source }：{ $reason }
source-signer-changed = 來自 { $source } 的 { $name } 由與之前不同的簽署者簽署，因此未有安裝。如其作者確實更換了簽署金鑰，請移除此來源後再重新新增。
source-signer-pinned = 來自 { $source } 的 { $name } 並非以其來源指定的證書簽署，因此未有安裝。
source-ambiguous = 有多個來源提供 { $package }：{ $sources }。請使用 --source 選擇其中一個。
source-plugin-mismatch = 下載的插件 ID 為 { $found }，但來源聲明的是 { $id }。插件未有安裝。
plugin-sources-heading = 來自您已新增的來源的插件
plugin-sources-hint = 任何人都可以在自己的來源中發佈插件。此清單顯示您在「應用程式及插件來源」中新增的所有來源的插件，不論發佈者是誰；本程式不會檢查這些插件。
plugin-signed = 已簽署，金鑰 { $key }
plugin-unsigned = 未簽署
plugin-signer-changed = 已安裝的 { $id } 所用的簽署金鑰與此檔案的不同，或此檔案未有簽署。插件並未被取代。如其作者確實更換了金鑰，請先移除已安裝的插件。
plugin-key-written = 簽署金鑰已寫入 { $path }。請將此檔案保密並備份：任何擁有它的人都可以用您的名義簽署插件；沒有它，您便無法發佈現有安裝會接受的更新。公開金鑰：{ $key }
btn-details = 詳細資料
details-repository = 儲存庫
details-homepage = 項目主頁
details-source = 來源
msg-url-copied-browser = 網址已複製至剪貼簿。請將其貼上至瀏覽器以查看該頁面。
store-activate-suggestion = 尚未新增任何來源。要啟用原有項目的應用程式清單（{ $source }）嗎？它由其作者 ma1co 維護，而非由本程式維護，且本程式未有檢查過它。之後清單會在「設定 → 應用程式及插件來源」中顯示。
btn-activate-list = 啟用清單
btn-plugin-sources-load = 顯示來自來源的插件
plugin-sources-none = 您的來源並無提供任何插件。
plugin-source-unchecked = 來源：{ $source }。本程式未有檢查此插件。
btn-plugin-source-install = 安裝

# 服務（senser）模式
usb-mode-advice = 使用調整、備份及服務模式 Shell 時，請將相機的 USB 連接模式設為「自動」，而非固定設為大容量儲存。工作階段結束後，如相機詢問模式，請選擇「大容量儲存（MSC）」。若固定設為大容量儲存，相機經常會拒絕下一次切換至服務模式。
service-switch-refused = 相機未有完成切換至服務模式，亦未有透過 USB 重新連接；此時相機通常會顯示「請檢查已連接的裝置」。請拔除 USB 線，重新接上後再試一次。
service-switching = 正在切換至服務模式…
service-switch-wait = 正在等待相機以服務模式重新連接…
service-switch-timeout = 相機未有以服務模式出現。相機會以獨立的 USB 裝置（產品 ID 0336）重新列舉，需要另行指派 WinUSB 驅動程式；請在裝置管理員中查看是否有未安裝驅動程式的 Sony 裝置。將相機關機再開機，即可使其返回正常模式。
service-authenticating = 正在驗證…
# 在發現相機仍停留在先前執行所留下的服務模式時，以及
# 無法解除該狀態時顯示。
service-stranded-reset = 相機仍處於先前執行所留下的服務模式。正在先結束該工作階段…
service-stranded-failed = 相機因先前的執行而停留在服務模式，且未有回應退出該模式的要求。請將相機關機再開機，然後重試。
service-bad-mode = 相機無法在此模式下使用。請將其切換至大容量儲存模式。
service-driver-required = 原始 USB 存取被拒，且無法指派 WinUSB 驅動程式。指派驅動程式需要管理員權限。
service-warning = 警告：服務模式會授予對相機記憶體、檔案系統及校準數據的原始存取權。寫入指令可能會永久損壞您的相機。
service-shell-ready = 服務模式 Shell 已就緒。輸入「help」查看指令，輸入「exit」離開。
service-shell-done = 服務模式 Shell 工作階段已結束。
# 在先前的執行於釋放其指派的驅動程式前被中斷時，
# 於啟動時報告。
driver-swept = 已移除 { $count } 個先前執行遺留的驅動程式指派。
# 在同一次中斷的執行亦令相機停留在服務模式時，於啟動時報告；
# 相機在該模式下會拒絕下一個工作階段。
driver-swept-camera = 先前執行令相機停留在服務模式，現已將其恢復至正常模式。
driver-release-replug = 在解除驅動程式期間，Windows 正在等待相機。請拔除 USB 線再重新接上，以完成清理。
# `driver` 指令的結果。
driver-bound = { $id } 現正使用 WinUSB（驅動程式套件 { $package }）。
driver-released = 已解除 { $count } 個驅動程式指派；相機已恢復使用 Windows 隨附的驅動程式。
driver-needs-elevation = 指派驅動程式需要管理員權限。請在以管理員身分啟動的主控台中執行此指令。
driver-leftover-needs-elevation = 先前的一次執行為相機指派了驅動程式，而且尚未解除。只要該驅動程式仍然存在，相機便會要求您檢查已連接的裝置，其大容量儲存功能亦無法運作。解除該驅動程式需要管理員權限；請確認 Windows 的提示。
# 服務模式途徑自行進行的驅動程式指派，因此無需
# 任何外部工具。
driver-binding = 正在為相機指派 WinUSB 驅動程式…
driver-bind-wait = 正在等待相機以新的驅動程式重新連接…
driver-bind-timeout = 指派驅動程式後，相機未有重新出現。請拔除相機，重新連接後再試一次。
driver-bind-failed = 無法指派 WinUSB 驅動程式：{ $detail }

# Shell 的回應。這些訊息由後端產生，因此主控台和
# 視窗的措辭相同。
service-usage = 用法：{ $usage }
service-bad-hex = 不是十六進制數值：{ $value }
service-unknown-command = 不明指令：{ $command }。輸入「help」查看清單。
service-unterminated-quote = 此行在帶引號的參數中途結束。
service-help = 指令參考：
service-read-bytes = 已讀取 { $count } 位元組。
service-pulled = 已將 { $remote } 從相機複製至 { $local }。
service-pushed = 已將 { $local } 複製至相機上的 { $remote }。
service-removed = 已從相機刪除 { $remote }。
service-dumped = 已將從 { $offset } 開始的 { $size } 位元組傾印至 { $local }。
service-backup-saved = 已將備份映像（{ $count } 位元組）儲存至 { $local }。
service-backup-text-saved = 已將備份映像（{ $count } 位元組）儲存至 { $bin }，並將清單儲存至 { $txt }。
service-backup-status = 備份狀態，地區：{ $region }
service-backup-rewritten = 映像（{ $count } 位元組）已原封不動寫回並再次讀取；未有永久儲存。
service-backup-attrs-restored = 已將 { $count } 個項目的屬性還原為 { $local } 中的屬性。
service-backup-written = 已寫入備份項目 { $id }。
service-backup-persisted = 已將子系統 { $id } 的備份儲存至快閃記憶體。
service-terminal-on = 偵錯終端機現已開啟。
service-terminal-off = 偵錯終端機現已關閉。
btn-service-terminal-open = 開啟序列主控台
btn-service-terminal-close = 關閉序列主控台
service-terminal-heading = 序列主控台
service-terminal-hint = 在 Linux 相機上，偵錯終端機是一個經由 USB 連接的 root Shell。開啟後，即可像在終端機中一樣輸入指令。
service-terminal-label = 主控台
service-terminal-interactive = 序列主控台已開啟。請輸入指令；輸入「exit」可返回服務模式 Shell。

# 未啟用進階模式時的服務頁面：如何從主控台進入 Shell，
# 以及事前須準備甚麼。
service-console-heading = 在主控台中執行服務模式 Shell
service-console-hint = 在存放 pmca-console 的資料夾中開啟終端機，然後執行以下指令：
service-driver-heading = 開始之前
service-driver-hint = 服務模式需要原始 USB 存取權，而 Windows 不會透過大容量儲存驅動程式授予此權限。本應用程式會自行指派 WinUSB 驅動程式——共兩次，一次在相機處於大容量儲存模式時，另一次在相機以第二個產品 ID 重新以服務模式出現之後——並在工作階段結束時解除這兩項指派。指派驅動程式需要管理員權限，因此請以管理員身分啟動主控台；無需任何其他工具。
driver-note-linux = 這裏無需安裝任何東西：libusb 可直接存取相機，無需取代核心驅動程式。所需的只是開啟裝置的權限，以及一個並未佔用該裝置的桌面環境。請為 Sony 的廠商 ID 054c 安裝一條 udev 規則，並在使用大容量儲存模式前卸離 usb-storage 驅動程式。原始碼樹中的 scripts/linux-prepare.sh 腳本會處理這兩項工作，並同時停止 gvfs 磁碟區監察程式。
driver-note-macos = 原始 USB 存取無需安裝任何東西。請將相機的 USB 連接設為 MTP。在 MTP 模式下，如 macOS 的相機服務 ptpcamerad 佔用相機，本程式會將其結束；系統會在需要時重新啟動該服務。調整、服務模式 Shell、功能分析、固件更新及更新模式 Shell 均經由大容量儲存連接進行，而 macOS 只會將該連接交給具備管理員權限的程式：視窗每次執行時會要求輸入一次密碼，指令列則需要 sudo。此平台是三者中測試最少的，因此遇到問題時，較可能需要由您回報，而非發現已有相關說明。
service-advanced-hint = Shell 亦可在此顯示，請前往「設定」中的「進階模式」。
btn-copy-command = 複製指令
msg-command-copied = 指令已複製至剪貼簿。

# 已啟用進階模式時的服務頁面。
btn-service-open = 連接
btn-service-close = 中斷連接
btn-service-run = 執行
btn-service-browse = 插入檔案路徑…
shell-path-quote = 無法插入此路徑：路徑中含有雙引號，而指令列正是以此字元標示參數的開始和結束位置。請將檔案重新命名，或將其移至不含此字元的路徑。
service-command-label = 指令
service-not-connected = 未連接。

# 僅限圖形介面的字串
install-apk-path = 本機 .apk 檔案的路徑（選填）
install-drop-hint = 將 .apk 檔案拖放至此

# 調整說明
tweak-rec-limit = 錄影時不設時間限制
tweak-rec-limit-4k = 錄製 4K 影片時不設時間限制
tweak-language = 顯示所有選單語言
tweak-language-note = 相機只會顯示其固件認識的語言，因此顯示的語言可能少於此處啟用的數目。例如 DSC-RX1R 會顯示 35 種中的 25 種。
tweak-pal-ntsc = 啟用 PAL / NTSC 選擇器及警告
tweak-pal-ntsc-mode = 視訊制式（PAL / NTSC）
tweak-pal-ntsc-pal = PAL
tweak-pal-ntsc-ntsc = NTSC
tweak-pal-ntsc-note = 變更視訊制式後，相機會要求將記憶卡格式化。請先備份卡上的所有內容——相片、影片、LUT。最簡單的做法是在切換前取出記憶卡。
tweaks-set-malformed = 預期格式為 --set ID=VALUE，但收到：{ $arg }
tweaks-set-unknown-value = 不明數值「{ $value }」。預期為以下其中一項：{ $options }
tweaks-set-not-a-choice = { $id } 是開關，而非選項。請使用 --enable 或 --disable。
tweaks-needs-set = { $id } 是選項，而非開關。請使用 --set { $id }=VALUE，數值為以下其中一項：{ $options }
tweak-usb-installer = 啟用 USB 應用程式安裝程式

# 調整流程
tweaks-warning = 警告：套用調整會重寫相機的備份記憶體。寫入中斷可能會永久損壞您的相機。
tweaks-info-toggle = 調整有甚麼作用？
tweaks-info-what-heading = 調整是甚麼
tweaks-info-what = 調整是指相機本身已具備、但選單中沒有提供的設定。本程式讓您可以使用這些設定：不設時間限制的錄影（包括 4K）、固件中包含的所有選單語言、PAL 與 NTSC 之間的選擇，以及 USB 應用程式安裝程式。插件可加入更多調整。會出現哪些調整視乎相機型號而定。
tweaks-info-how-heading = 運作方式
tweaks-info-how = 「顯示可用的調整」會讓相機短暫進入服務模式，並讀取其備份記憶體。此過程不會變更任何內容。之後您會看到哪些調整已開啟或關閉，以及相機身分（只供顯示）。「套用」只會寫入您已變更的項目。其後本程式會再次讀取記憶體，並檢查相機是否已全部接納。
tweaks-info-undo-heading = 如何還原
tweaks-info-undo = 每次寫入前，本程式都會將先前的狀態儲存在此電腦上。每項調整都可以用同一個開關再次關閉。對於逐項寫入數值的相機，亦會提供「還原變更」。
tweaks-info-needs-heading = 所需條件
tweaks-info-needs = 需要金鑰資料（請參閱「歡迎」頁面），並按下文所述將相機的 USB 連接設為「自動」。Windows 及 macOS 會要求管理員權限，讓本程式可在服務模式下存取相機。寫入期間請勿拔除 USB 線或取出電池。
app-backup-target = 目標資料夾
btn-choose-folder = 選擇資料夾…
btn-default-folder = 使用預設資料夾
tweaks-reading = 正在讀取相機設定…
tweaks-writing = 正在寫入相機設定…
tweaks-done = 相機設定已更新。
tweaks-write-failed = 相機未有按傳送的內容儲存新設定。部分設定可能仍已寫入：請再次讀取設定，如有需要請將其還原。在變更前製作的備份副本是安全的回復途徑。
tweaks-confirm-required = 未經確認，拒絕寫入。請在閱讀上述警告後，加上 --confirm 重新執行。
tweaks-state-enabled = 已啟用
tweaks-state-disabled = 已停用
tweaks-full-read-failed = 相機未有傳送完整的備份（{ $detail }）。相機之後經常會停止回應：請將相機關機，取出電池數秒，再放回電池並重新連接。
tweaks-direct-mode = 此相機的備份大於 1 MB，因此其設定會逐項讀取及寫入。每次變更前都會儲存舊數值，並可使用「還原變更」寫回。
tweaks-direct-forced = PMCA_TWEAKS_DIRECT 強制使用逐項屬性途徑，因此此相機的設定會逐項讀取及寫入。每次變更前都會儲存舊數值，並可使用「還原變更」寫回。
tweaks-direct-refused = 相機拒絕了變更（{ $detail }）。已寫入的數值已還原；沒有作出任何變更。部分型號（尤其是日本型號）只容許透過完整備份映像變更此設定。
tweaks-direct-revert-failed = 相機拒絕了變更（{ $detail }），而且並非所有已寫入的數值都能還原。舊數值已儲存：請嘗試「還原變更」。
tweaks-direct-restart = 相機設定已更新。請將相機關機再開機，令變更生效。
tweaks-rollback-unsaved = 無法儲存目前的數值以供還原，因此沒有作出任何變更。
tweaks-image-saved = 已儲存變更前的備份映像：{ $path }
tweaks-whole-image-hint = 如要改為透過完整備份映像寫入變更，請加上 --whole-image 再次執行同一指令。系統會先儲存變更前的映像。
tweaks-whole-image-confirm = 要透過完整備份映像寫入變更嗎？相機拒絕以單一數值寫入此變更。整個映像會連同變更一併寫回；變更前的映像會先儲存在此電腦上。
btn-whole-image = 寫入完整備份
tweaks-undo-none = 此相機沒有已儲存的數值。
tweaks-undo-confirm = 要將已儲存的數值寫回相機嗎？這會還原自儲存以來逐項寫入的所有變更。
tweaks-undo-done = 已將儲存的數值寫回。請將相機關機再開機。
tweaks-undo-available = 首次變更前的數值已儲存，可以寫回。
btn-undo-tweaks = 還原變更
tweaks-identity-heading = 相機身分（唯讀）
tweaks-caution-languages = 此相機無法顯示其中兩種語言——可能是中文（簡體）及中文（繁體）。捲動至這兩種語言時，語言選單會重新啟動，因此為審慎起見，這兩種語言會保持關閉。
tweaks-caution-accept = 仍然啟用這兩種語言（風險自負）
tweaks-caution-confirm = 這兩種語言可能令相機的語言選單重新啟動。這不會造成任何損壞，變更亦可還原，但您可能需要從頭再次尋找該設定。仍要啟用嗎？

# 設定
settings-language = 語言
settings-theme = 主題
settings-theme-system = 系統
settings-theme-light = 淺色
settings-theme-dark = 深色

# 進階模式。這是防止誤操作的保護，而非防範攻擊者：
# 它所顯示的一切本來就可以從主控台使用。
settings-advanced-heading = 進階模式
settings-advanced-hint = 進階模式會在同名頁面上顯示服務模式 Shell 及更新模式 Shell。服務模式會授予對相機記憶體、檔案系統及校準數據的原始存取權；一條輸入錯誤的指令便可能令相機無法使用。除非您清楚知道將要寫入甚麼，否則請保持關閉。
settings-advanced-toggle = 顯示服務模式及更新模式 Shell
settings-advanced-on = 進階模式已開啟。「服務模式 Shell」及「更新模式 Shell」頁面現在會提供 Shell 本身。
settings-advanced-off = 進階模式已關閉。「服務模式 Shell」及「更新模式 Shell」頁面會改為說明如何使用主控台。
settings-advanced-confirm = 服務模式及更新模式 Shell 可能會永久損壞您的相機。要顯示這兩個 Shell 嗎？
settings-update-heading = 更新通知
settings-update-purpose = 開啟此選項後，本程式每次啟動時都會向 GitHub 查詢是否已發佈較新的版本，如有便會在視窗頂部顯示通知。在此過程中，GitHub 會得知您的 IP 位址；除此之外不會傳送任何資料。預發佈版本不會發出通知。
settings-update-toggle = 啟動時通知我有新版本可用
settings-update-on = 更新通知已開啟。從下次啟動起，新版本會在視窗頂部顯示通知。
settings-update-off = 更新通知已關閉。只有在您點按導覽列下方的版本號碼時，本程式才會檢查新版本。
update-check-hint = 立即檢查是否有新版本
update-check-heading = 檢查更新
update-checking = 正在尋找新版本…
update-none = 您正在使用最新版本（{ $version }）。
update-available = 有新版本可用
update-current = 已安裝版本
update-latest = 新版本
btn-update-dismiss = 隱藏

# 編輯翻譯。語言檔案可以下載，在任何文字編輯器中修改，
# 然後再次上載；上載的檔案在使用前會先經過檢查。
settings-catalog-heading = 編輯翻譯
settings-catalog-hint = 下載語言檔案，在任何文字編輯器中修改，然後再次上載。只需翻譯等號右邊的文字，您略去的項目會保留內置的字眼。在此選擇語言並不會變更本應用程式的語言。
settings-catalog-locale = 語言檔案
settings-catalog-builtin = 正在使用內置翻譯。
settings-catalog-custom = 正在使用您自己的翻譯：
btn-catalog-export = 下載…
btn-catalog-import = 上載…
btn-catalog-reset = 還原內置翻譯
catalog-exported = 已寫入
catalog-imported = 已套用翻譯。
catalog-reset-done = 已還原內置翻譯。
catalog-reset-none = 沒有可還原的自訂翻譯。
catalog-reset-confirm = 要捨棄您為此語言自行製作的翻譯嗎？
catalog-rejected = 檔案未獲套用。請更正以下各行：
catalog-count-accepted = 已套用的項目
catalog-count-missing = 沿用內置翻譯的項目
catalog-count-untranslated = 與英文原文相同的項目
catalog-line = 行
catalog-problem-syntax = 不是「id = 文字」格式的項目
catalog-problem-continuation = 延續至下一行，此寫法不受支援
catalog-problem-duplicate = 定義了多於一次
catalog-problem-unknown-key = 不是本應用程式使用的項目
catalog-problem-empty = 沒有文字
catalog-problem-placeholder = 預留位置已變更，預期為：

# 來源、鳴謝及授權。顯示於「授權」頁面，亦會由
# `pmca-console license` 顯示。授權文字本身不作翻譯：它是一份
# 法律文件，按原文逐字轉載。
about-thanks-heading = 鳴謝
about-thanks-body-1 = 本工具得以存在，全賴 ma1co 研究出 Sony 相機如何透過 USB 通訊，並於 2015 年發佈 Sony-PMCA-RE；亦有賴其後多年來為該項目及 OpenMemories 各項目作出貢獻的每一位人士。
about-thanks-body-2 = 相機偵測、外部指令通訊協定、固件更新程序及服務模式全部都是他們的成果，而本移植版本將這些成果延續下去。謹此致謝。在此基礎上，PMCA-RE RELOADED 加入了許多由 masterflai 貢獻的新功能，其中最重要的是現已內置於程式中的 Windows 裝置驅動程式自動切換功能：再也無需使用另一個工具為相機手動指派其他驅動程式，令整個過程更方便、更快捷，風險亦更低。
about-contributors-heading = 貢獻者
about-contributors-body = 除了 ma1co 之外，以下人士亦曾為 Sony-PMCA-RE 作出貢獻，或在其各自的副本中發佈了已納入本移植版本的變更：
about-purpose-heading = 本移植版本的由來
about-purpose-body-1 = 原有的應用程式已停止開發。它是為 Python 2 編寫的，而 Python 2 已於 2020 年終止支援；其部分功能亦依賴 Sony 其後已關閉的服務，因此在現今的系統上已無法不受限制地執行。
about-purpose-body-2 = 本移植版本的目的在於保存而非取代，同時亦讓用戶今天能順暢使用：讓這些相機繼續可用，並在現今的電腦及較新的相機型號上都能順利運作。原有程式能做到的一切，本程式都應該仍能做到。
about-origin-heading = 開發方式
about-origin-body-1 = 由 Python 移植至 Rust 的工作是在 AI 的協助下完成的。
about-origin-body-2 = 同樣應坦白說明的是，這並不代表程式未經把關：程式的行為由一套自動化測試及一份硬件測試計劃涵蓋。Windows 版本尤其經過全面測試，因為裝置驅動程式令其成為最複雜的平台；macOS 及 Linux 則未能進行同樣全面的測試。請將本程式視為實驗性軟件。
about-license-heading = 授權
about-license-body = 本移植版本以 MIT 授權條款發佈，與原有項目所用的授權條款相同。原有的版權聲明按該授權條款的要求原封不動保留。
about-license-full = 授權條款全文
about-third-party-heading = 第三方授權條款
about-third-party-body = 本程式採用靜態連結：其使用的每個程式庫的程式碼都包含在執行檔內，而這些程式庫的授權條款要求其版權聲明須隨之一併提供。以下完整轉載這些聲明，每個套件一項。
about-third-party-toggle = 顯示所有第三方聲明
about-privacy-heading = 資料
about-privacy-body-1 = 本程式在您自己的電腦上執行。它不會傳送任何使用數據，沒有遙測功能，亦無需帳戶。您在此進行的任何操作都不會向任何地方報告。
about-privacy-body-2 = 有兩項功能會自行連接網絡，而對方會在此過程中得知您的 IP 位址：GPS 輔助數據從 Sony 下載；您新增的來源及其星星則從 GitHub 讀取，如您要求，亦會在 GitHub 上進行搜尋；應用程式及插件從這些來源所指明的網址下載。不使用這些功能時，本程式會完全離線運作。如您在設定中開啟更新通知，本程式亦會在每次啟動時向 GitHub 查詢最新版本。點按導覽列下方的版本號碼亦會查詢一次。
about-privacy-body-3 = 從相機讀取的內容——包括其序號及任何已儲存的 Wi-Fi 密碼——不會離開您的電腦。如您將這些內容寫入檔案，請留意將該檔案交給何人。
about-privacy-body-4 = 本程式背後並無運作任何服務，亦沒有任何數據傳送至其作者，因此這裏沒有可供查閱或要求刪除的資料。上述兩項服務如何處理它們所見的位址，受其各自的私隱聲明規管，而非本聲明。
about-legal-heading = 法律聲明
about-legal-trademark = 這是一個獨立項目，並非由 Sony 製作或認可，亦與 Sony 沒有任何關連。「Sony」、「PlayMemories」及相機型號名稱均為 Sony Group Corporation 的商標，在此使用僅為說明本工具可與哪些相機通訊。
about-legal-own-device = 本程式只供用於您擁有的相機。
about-legal-warranty = 本工具所做的事情超出 Sony 對這些相機的預期用途，這樣做可能令您的保養失效。本軟件本身不附帶任何保證，使用風險由您自行承擔。對自己擁有的裝置進行逆向工程是否獲准許，以及准許至何種程度，視乎您身處的地方而定。
about-links-heading = 項目
about-link-original = Sony-PMCA-RE — 原有項目
about-link-fwtool = fwtool.py — 更新映像產生功能所依據的固件工具
about-link-tweak = OpenMemories: Tweak — 建議支援此應用程式的相機使用
about-link-applist = OpenMemories: App list — 更多應用程式
about-link-platform = OpenMemories: Platform — 本工具隨附的更新程式主體
about-link-devices = 支援的相機型號
about-link-nexhack = Nex-Hack — 本項目所依據的社群研究
about-link-zve10 = zv-e10-re — 有關 ZV-E10 介面及設定記憶體的研究
about-version = 版本
btn-copy-url = 複製網址
msg-url-copied = 網址已複製至剪貼簿。

# 安全提示
notice-experimental = 這是實驗性軟件。使用風險自負。

# 網絡設定（GPS 輔助、直播存取點及服務）
gps-downloading = 正在下載 GPS 輔助數據…
gps-writing = 正在將 GPS 輔助數據寫入相機…
gps-bad-size = GPS 輔助數據的大小異常，因此未有寫入。
wifi-read = 正在從相機讀取直播存取點…
wifi-written = 已寫入直播存取點。
stream-read = 正在從相機讀取直播設定…
stream-written = 已寫入直播設定。
net-secret-warning = 警告：此檔案以明文載有 Wi-Fi 密碼及存取權杖。
wifi-secret-warning = 相機以未加密的方式儲存這些密碼，此處按其儲存時的原樣顯示。這些密碼只會保留在此電腦上；如您將其儲存至檔案，請留意將該檔案交給何人。
gps-heading = GPS 輔助數據
gps-hint = 將檔案拖放至此、輸入路徑，或將欄位留空以從 Sony 取得最新數據。
btn-gps-devices = 相容裝置
btn-gps-devices-hide = 隱藏相容裝置
gps-devices-note = 以下型號內置 GPS 模組。其中許多型號只有名稱以 V 結尾的版本才配備 GPS；不帶 V 的版本沒有 GPS。
gps-devices-compact = 小型相機（DSC）
gps-devices-slt = SLT 相機（A 接環）
gps-devices-action = 運動相機
gps-devices-camcorder = 攝錄機（Handycam）
gps-col-variant = GPS 版本
gps-col-arch = 架構
gps-col-module = GPS 模組
gps-col-apps = 應用程式支援
gps-yes = 是
gps-no = 否
gps-unknown = 不明
gps-devices-untested = 配備 GPS，支援情況未經測試
gps-devices-untested-note = 以下型號內置 GPS 模組，但並未列於 OpenMemories 的裝置清單中。本應用程式能否與這些型號通訊尚未經過測試，因此其架構及應用程式支援情況不明。
wifi-heading = 直播存取點
wifi-hint = 這些是相機直播功能所使用的存取點，而非相機本身的 Wi-Fi 設定。不具備直播功能的機身會拒絕此操作。
wifi-multi = 多個存取點清單
wifi-none = 沒有已儲存的存取點。
wifi-writing = 正在將直播存取點寫入相機…
wifi-confirm = 此操作會取代相機上儲存的所有存取點。要繼續嗎？
wifi-empty-refused = 拒絕寫入空白清單，因為這會清除所有已儲存的存取點。
stream-heading = 直播
stream-writing = 正在將直播設定寫入相機…
stream-confirm = 此操作會覆寫相機的直播設定。要繼續嗎？
stream-invalid-json = 設定不是有效的 JSON：

# 固件更新
firmware-warning = 警告：此操作會覆寫相機的固件。更新開始後請勿中斷：請勿拔除 USB 線，亦不要關閉相機。請只使用專為此相機型號而設的固件檔案，並確保電池電量充足。否則相機可能會永久無法使用。
firmware-untested = 未經測試：固件燒錄功能已完成移植，但從未在真實相機上試用。寫入失敗或中斷均無法復原，因此請將本頁的所有功能視為未經驗證。
firmware-confirm-required = 未加上 --confirm，拒絕燒錄。
firmware-truncated = 固件檔案不完整：FDAT 有效負載超出了檔案結尾。
firmware-updating = 正在更新固件：
firmware-invalid = 此檔案不是有效的 Sony 固件映像
firmware-dat-path = 固件檔案（.dat）
firmware-drop-hint = 將 .dat 固件檔案拖放至此
firmware-file-required = 請先選擇固件檔案。
firmware-check-ok = 相機接受此映像。尚未寫入任何內容。
firmware-cancelled = 已取消燒錄。相機未被修改。
firmware-current-version = 已安裝版本
firmware-new-version = 此映像中的版本
firmware-models-hint = 如更新模式 Shell 能夠連接相機，該相機即屬相容。
btn-firmware-check = 檢查映像
btn-firmware-flash = 燒錄固件
firmware-own-device = 請確認您知悉這些風險，並只會更新您自己的裝置。
btn-firmware-accept = 已了解風險 – 燒錄

# 檔案工具：完全無需相機的操作。
tools-hint = 這些操作只處理檔案，絕不會觸及相機，因此這裏的任何操作都不會損壞相機。它們與 pmca-console 以 apk2spk、spk2apk、print-backup、print-usagelog、guess-firmware 及 xpd-checksum 提供的操作相同。
tools-operation = 操作
tools-apk2spk = 將 .apk 封裝至 .spk 容器
tools-spk2apk = 將 .spk 解封裝為 .apk
tools-print-backup = 顯示 Backup.bin 的內容
tools-print-usagelog = 顯示 Usagelog.bin 的內容
tools-guess-firmware = 在固件 .dat 中尋找有效負載
tools-xpd-checksum = 計算檔案的 CIC 總和檢查碼
tools-input = 輸入檔案
tools-file-required = 請先選擇此操作所需的檔案。
tools-apk-invalid = 無法將此檔案封裝為 .spk
tools-spk-invalid = 此檔案不是有效的 .spk 套件
tools-backup-invalid = 此檔案不是有效的 Backup.bin 映像
tools-usagelog-invalid = 此檔案不是有效的使用記錄
tools-reset-value = 重設值：
tools-fdat-found = 在位移 { $offset } 處找到有效負載，大小為 { $size }。
tools-checksum = 總和檢查碼
tools-written = 已寫入 { $path }

# 項目文件的連結
docs-compatibility = 相機相容性
docs-compatibility-hint = 原有項目提供了一份已測試相機的概覽。內容主要涵蓋較舊的相機型號，可於以下網址查閱：
devices-note = 取自原有項目的概覽。應用程式支援旁邊的數字是相機應用程式平台的版本。
devices-more-note = 以下相機未曾以本程式進行明確測試。最後一欄中的剔號表示社群曾報告使用過這些相機。如服務模式 Shell 一欄同時標示為 ✖，則表示該次使用失敗。
btn-more-models = 更多裝置
btn-more-models-hide = 隱藏更多裝置
check-updater-yes = 您的裝置 { $model } 相容：根據裝置清單，更新模式 Shell 能夠連接此裝置。
check-updater-no = 您的裝置 { $model } 不相容：根據裝置清單，更新模式 Shell 無法連接此裝置。
check-updater-unknown = 您的裝置 { $model } 的相容性不明：此裝置不在本程式的任何裝置清單中，因此無法得知更新模式 Shell 能否連接此裝置。
check-apps-no = 您的裝置 { $model } 不相容：根據裝置清單，此裝置不支援應用程式。
check-apps-untested = 您的裝置 { $model } 的相容性不明：此裝置在裝置清單中，但尚未確認是否支援應用程式。
check-apps-unknown = 您的裝置 { $model } 的相容性不明：此裝置不在本程式的任何裝置清單中，因此無法得知是否支援應用程式。
check-gps-no = 您的裝置 { $model } 不相容：此裝置不在配備 GPS 接收器的裝置清單中。
check-gps-untested = 您的裝置 { $model } 的相容性不明：根據裝置清單，此裝置配備 GPS 接收器，但傳送至此型號的功能未經測試。
check-gps-unknown = 您的裝置 { $model } 的相容性不明：此裝置不在本程式的任何裝置清單中，因此無法得知是否配備 GPS 接收器。
check-unidentified = 無法識別已連接的裝置，因此其相容性不明。
check-proceed = 仍要繼續嗎？
check-cancelled = 已取消。裝置未被修改。
devices-col-confirmed = 已確認
devices-group-ilc = 可換鏡頭相機（E 接環及 A 接環）
devices-group-remote = 即時取景遙控器
btn-copy-link = 複製連結
shutter-heading = 快門次數
shutter-hint = 相機會計算每一次快門釋放，並將目前的數字寫入它儲存的每張相片（ARW、JPEG 或 HEIF）。請選擇最近拍攝的相片以讀取目前的次數。檔案只會被讀取，絕不會被修改。
shutter-source = 次數儲存在檔案中的位置因型號而異。這些位置來自公開記錄的研究（ExifTool），並未就每個型號獲得確認。
shutter-drop-hint = 將相片（ARW、JPEG、HEIF）拖放至此
btn-shutter-read = 分析相片
shutter-file-required = 請先選擇相片。
shutter-file = 檔案
shutter-taken = 拍攝日期
shutter-count = 快門釋放次數
shutter-type = 快門
shutter-type-mechanical = 機械
shutter-type-electronic = 電子
shutter-electronic-hint = 此相片是以電子快門拍攝的。在許多型號上，這類拍攝不會計算在內；此時顯示的數值為最後一次機械快門釋放時的次數。
shutter-not-sony = 此相片並非以 Sony 相機拍攝，因此無法從中讀取快門次數。
shutter-unknown-model = 目前不知道此相機型號將快門次數儲存在檔案中的哪個位置，因此不會顯示數值。
shutter-not-recorded = 此檔案不含可讀取的快門次數。經影像編輯軟件儲存或匯出的相片，以及部分型號以電子快門拍攝的相片，都會出現這種情況。請選擇記憶卡上的原始檔案。
shutter-model-mismatch = 此相片是以與已連接相機不同的相機型號拍攝的。
shutter-invalid = 此檔案不是可讀取的相片
file-filter-photo = 相片（ARW、JPEG、HEIF）
usage-log-heading = 使用記錄
usage-log-hint = 相機以未公開的格式保存內部記錄。記錄會按原樣儲存；本程式只會從相機讀取。
usage-log-written = 已寫入
btn-usage-log-info = 關於使用記錄
btn-usage-log-info-hide = 隱藏說明
usage-log-info-what-heading = 此記錄是甚麼
usage-log-info-what = Sony 在內部將此記錄稱為「KikiLog」。記錄由相機自行寫入；本程式只會讀取。目前沒有已知的公開格式說明。
usage-log-info-read = 讀取不會變更任何內容：連續多次下載的結果逐位元組完全相同。
usage-log-info-flush-heading = 為何缺少最近的記錄項目
usage-log-info-flush = 新的記錄項目往往不會即時出現：相機可能只在關機時才儲存，有時更會分散於多次開關機週期。請將相機關機再開機，再次下載，並重複此步驟，直至不再出現新的記錄項目為止。
usage-log-info-content-heading = 記錄能告訴您甚麼
usage-log-info-shutter = 快門釋放：某些事件各自代表一次曝光。不過，計算這些事件並不能得出快門次數：記錄的大小固定，不一定涵蓋相機的整個使用期。相機會在別處保存總數，並將其寫入所儲存的相片；可在「快門次數」部分讀取。
usage-log-info-clock = 時間戳記的準確度取決於相機時鐘。更換電池後，時鐘可能會倒退，並產生帶有相機初始日期的記錄項目。
usage-log-info-unknown = 目前沒有已知的事件編號公開說明。本程式會顯示編號，但不作解釋。
usage-log-info-privacy-heading = 私隱
usage-log-info-privacy = 此檔案包含曾插入的記憶卡序號。請勿隨意分享或發佈。
btn-usage-log = 儲存記錄
usagelog-size = 記錄區域
usagelog-cursor = 寫入位置
usagelog-spare = 第二區域
usagelog-count = 項目
usagelog-events = 按編號分類的事件
usagelog-entries = 按次序排列的項目
msg-link-copied = 連結已複製至剪貼簿。

# 後端失敗訊息，以後端報告的固定代碼作為鍵。
# 訊息中的可變部分（路徑、狀態位元組、裝置錯誤）由前端
# 附加，因此即使沒有該部分，這些訊息仍是完整的句子。
# 這裏刻意沒有 `err-operation`：該代碼表示後端已翻譯的訊息，
# 必須按原文顯示。
err-usb-no-device = 找不到相機。請連接相機並將其開啟。如相機螢幕一直顯示正在連接，即表示電腦未能接收到相機：請使用數據線而非僅供充電的線，並直接連接相機，不要經過集線器或轉接器，然後將相機關機再開機；如仍未能解決，請暫時取出電池。使用數據線時，即使相機已關機亦會被偵測到，並顯示為未就緒；如相機關機時亦出現此訊息，該線很可能無法傳輸數據。
err-usb-camera-not-ready = 相機已連接但未就緒：相機已關機，或正在螢幕上等待選擇 USB 模式。請開啟相機，或在相機上選擇「大容量儲存（MSC）」。
camera-choose-usb-mode = 相機正在等待選擇 USB 模式。請在相機上選擇「大容量儲存（MSC）」。
err-usb-too-many-devices = 已連接多於一部相機。請中斷連接其他相機，只保留您要使用的那一部。
err-usb-transfer = USB 傳輸失敗。請嘗試使用其他線或連接埠，並避免使用 USB 集線器。
err-usb-permission = 本程式無權透過 USB 開啟相機。在 Linux 上，此權限透過一條針對 Sony 廠商 ID 的 udev 規則一次性授予；原始碼樹中的 scripts/linux-prepare.sh 腳本會安裝此規則。然後請拔除相機再重新接上。
err-usb-busy = 另一個程式正在使用相機，因此無法開啟相機。請關閉其他可能存取相機的程式；在 Linux 上，桌面的媒體監察程式可能會佔用相機，而 scripts/linux-prepare.sh 會將其停止。在 macOS 上，請將相機的 USB 連接設為 MTP：在大容量儲存模式下，系統本身的磁碟驅動程式會佔用相機。
err-usb-needs-root = macOS 只會在程式以管理員權限執行時，才將相機的大容量儲存連接交給該程式。請以 sudo 啟動指令列程式，例如：sudo pmca-console tweaks。如將相機的 USB 連接設為 MTP，則無需 sudo 亦可讀取相機資料。
macos-admin-prompt = PMCA-RE RELOADED 需要管理員權限，才能從 macOS 接管相機的大容量儲存連接。在操作期間，相機的記憶卡會從 Finder 中消失；如卡上有已開啟的檔案，請先在 Finder 中將其退出。
macos-admin-declined = 未獲授予管理員權限，因此未有觸及相機。在 macOS 上，此功能需要管理員權限。
macos-admin-failed = 本程式無法以管理員權限啟動其輔助程式。
err-usb-stall = 相機中止了 USB 傳輸。
err-usb-protocol = 相機傳回了非預期的回應。
err-usb-msc-status = 相機拒絕了一條儲存指令。
cli-error-prefix = 錯誤
cli-hint-mass-storage = 切換至大容量儲存模式：pmca-console mass-storage
err-usb-driver = Windows 已將此相機指派給不容許直接存取的驅動程式。此模式需要 WinUSB 驅動程式，本應用程式可自行指派——但只限於以管理員權限執行時。
err-needs-elevation = 指派或解除驅動程式需要管理員權限，而 Windows 的提示已被拒絕。請使用「啟動驅動程式助手」再試一次；相機上無需作出任何變更。
err-usb-no-mass-storage-volume = 相機已以大容量儲存模式連接，但未有提供任何磁碟機。請喚醒相機，如螢幕上詢問 USB 模式請予以確認，並檢查是否已插入記憶卡。請勿更換驅動程式：此模式需要 Windows 的標準驅動程式。
err-usb-msc-sense = 相機報告了儲存錯誤。
err-usb-ptp-response = 相機拒絕了指令。
err-usb-unsupported = 此相機型號不支援該功能。
err-usb-updater-status = 相機上的固件更新程式拒絕了該操作。
err-usb-senser = 服務模式指令失敗。
err-market-core = 無法讀取相機數據檔案。
err-market-json = 收到的數據格式不正確。
err-market-missing-field = 回應中缺少預期的欄位。
err-market-no-apk = 尚未選擇要安裝的應用程式。
err-market-http = 網絡要求失敗。
err-market-io = 本機安裝伺服器發生故障。
err-market-tls = 無法與相機建立加密連接。
err-apk = 無法讀取應用程式檔案。
err-io = 無法讀取或寫入檔案。
err-json = 相機傳送了格式不正確的數據。
err-unknown-locale = 此語言無法使用。
err-catalog-encoding = 翻譯檔案並非以 UTF-8 文字格式儲存。請以 UTF-8 編碼重新儲存。
err-internal = 發生內部錯誤。這是本應用程式的程式錯誤。

## 功能分析
nav-analysis = 分析
analysis-title = 功能分析
analysis-experimental = 實驗性功能。此功能是新推出的，只曾在少數相機上試用。
analysis-own-hardware = 請只在您自己的相機上使用。服務模式本身已是 Sony 未有預期的介入操作；責任由您承擔。
analysis-explanation-toggle = 說明：用途、程序、風險
analysis-purpose-heading = 有何用途？
analysis-purpose = 功能分析會記錄相機將其某項選單設定儲存在設定記憶體（備份）中的哪一個數值。結果描述的是此相機的設定記憶體；可以與其他相機比較，但不會改變相機所提供的功能。
analysis-how-heading = 如何運作？
analysis-how = 對於您輸入的每個選單數值，本程式都會完整讀取一次設定記憶體，每次都在一個獨立的短暫服務模式工作階段中進行。在兩次讀取之間，您需拔除 USB 線並在相機選單中變更設定；相機保持開啟。最後會再次讀取第一個數值。跟隨選單變化、並在最後回復至起始值的數值即為候選數值。會自行變化的數值（計數器、時鐘）會被剔除。
analysis-requirements-heading = 要求
analysis-requirements = 相機須以 USB 連接，其 USB 模式設為「自動」，並在每個工作階段後於相機上選擇「大容量儲存（MSC）」。在 Windows 上，本程式需要管理員權限，因為它會為服務模式短暫指派驅動程式。請為電池充電；每個工作階段約需一分鐘。
analysis-read-heading = 讀取步驟
analysis-read = 快照只會讀取。相機的設定不會有任何變更。風險雖低但並非為零：切換至服務模式本身已是 Sony 未有預期的介入操作。
analysis-write-heading = 寫入步驟（寫入測試）
analysis-write = 只會在您明確啟用時進行：本程式會將找到的數值寫入相機，您在重新啟動後於選單中檢查是否生效，然後再寫回原來的數值。只會寫入相機本身曾儲存的數值，而且只會在剛好找到一個相符的候選數值時才寫入。舊數值會事先儲存。儘管如此，此步驟仍會變更相機；一旦出現故障，相機可能會處於只有維修中心才能修復的狀態。
analysis-legal-heading = 法律資訊
analysis-legal = 本軟件不提供任何保證（MIT 授權條款）。對相機的介入操作可能會影響其保養。請只在您擁有的相機上使用此功能。分析報告不含序號，除找到的候選數值外，亦不含設定記憶體的任何內容；報告絕不會自動傳送至任何地方。
analysis-start-heading = 新分析
analysis-setting-label = 選單設定（按相機上的名稱）
analysis-values-label = 選單數值，以半形逗號「,」分隔
analysis-values-hint = 第一個數值是目前所設定的數值。可輸入二至八個數值；每個數值需要一個服務模式工作階段。
analysis-write-option = 然後執行寫入測試（會變更相機）
analysis-write-warning = 寫入測試會變更相機：它會將一個數值寫入設定記憶體，之後再寫回原來的數值。只限用於您自己的相機，風險自負。
analysis-write-consent = 這部相機屬於我，我承擔有關風險。
btn-analysis-write-enable = 啟用寫入測試
btn-analysis-start = 開始分析
btn-analysis-snapshot = 讀取備份
btn-analysis-new = 新分析
analysis-discard-confirm = 要捨棄進行中的分析嗎？至今為止的報告仍會保留。
analysis-access-read = 讀取
analysis-access-write = 寫入
analysis-access-user = 手動
analysis-access-local = 電腦
analysis-step-snapshot = 讀取備份，設定為「{ $value }」
analysis-step-change = 在相機選單中變更為「{ $value }」
analysis-step-evaluate = 比較快照
analysis-step-write = 將「{ $value }」的數值寫入相機
analysis-step-check = 將相機關機再開機，查看選單（預期為：「{ $value }」）
analysis-step-restore = 寫回「{ $value }」的原來數值
analysis-step-report = 儲存分析報告
analysis-do-prepare = 準備： 1. 在相機選單中將「{ $setting }」設為「{ $value }」，然後按 MENU 按鈕離開選單，讓相機儲存該設定。 2. 保持相機開啟。 3. 接上 USB 線，並在相機上選擇「大容量儲存（MSC）」。
analysis-do-change = 輪到您： 1. 拔除 USB 線。 2. 相機保持開啟。 3. 在相機選單中將「{ $setting }」設為「{ $value }」，然後按 MENU 按鈕離開選單，讓相機儲存該設定。 4. 重新接上 USB 線，並選擇「大容量儲存（MSC）」。
analysis-do-check = 輪到您： 1. 拔除 USB 線。 2. 將相機關機再開機。 3. 查看選單中「{ $setting }」顯示的內容。
analysis-do-reconnect = 重新接上 USB 線，並在相機上選擇「大容量儲存（MSC）」。
analysis-reading = 第 { $index } 個快照（共 { $total } 個）：讀取期間切勿拔除 USB 線。
analysis-cli-continue = 按 Enter 繼續，按 q 取消：
analysis-confirm-word = 是
analysis-cli-confirm = 輸入「{ $word }」以確認：
analysis-write-declined = 未獲確認。未有寫入任何內容。
analysis-aborted = 已取消。未有寫入任何內容。
analysis-snapshot-done = 已讀取快照。相機已離開服務模式。
analysis-result = 已比較 { $compared } 個數值：{ $count } 個候選數值，{ $running } 個數值自行變化。
analysis-candidate-consistent = 與選單明確對應
analysis-candidate-inconsistent = 未能明確對應
analysis-write-preview-heading = 將會寫入以下內容
analysis-write-preview = 數值 { $id }（{ $size } 位元組）：由 { $old }（「{ $old-label }」）改為 { $new }（「{ $new-label }」）。新數值是相機在設定為「{ $new-label }」時自行儲存的。
btn-analysis-write = 寫入
btn-analysis-skip-write = 不進行寫入測試並完成
analysis-written = 數值已寫入、讀回並永久儲存。
analysis-menu-question = 選單現在顯示甚麼？
analysis-menu-other = 其他
btn-analysis-menu-confirm = 確認
btn-analysis-restore = 寫回原來的數值
analysis-restored = 原來的數值已寫回並永久儲存。
analysis-survived = 重新啟動後，寫入的數值仍然保存。
analysis-not-survived = 重新啟動後，寫入的數值已不再保存。
analysis-report-saved = 已儲存分析報告：{ $path }
analysis-report-hint = 報告不含序號，亦不含備份。您可將報告轉交他人，以便核實及記錄有關發現。
btn-analysis-export = 匯出報告…
file-filter-analysis = 分析報告
analysis-pending = 有一項寫入測試尚未完成。原來的數值已儲存，可以寫回。
analysis-pending-none = 此相機沒有已儲存的未完成寫入測試。
analysis-invalid-setting = 請按相機上的名稱輸入選單設定。
analysis-invalid-values = 請輸入二至 { $max } 個不同的選單數值。第一個數值是目前所設定的數值。
analysis-already-complete = 所有快照均已讀取。
analysis-other-camera = 這部相機與第一個快照中的相機不同。分析已停止。
analysis-write-not-allowed = 在目前狀態下無法進行寫入測試。
analysis-camera-changed = 相機現時的數值與第一個快照中的不同。未有寫入任何內容。
analysis-write-refused = 相機拒絕了該數值；一切維持原狀。（{ $detail }）
analysis-refuse-none = 無法進行寫入測試：沒有數值跟隨選單變化。
analysis-refuse-ambiguous = 無法進行寫入測試：有 { $count } 個數值跟隨選單變化，無法確定對應哪一個。
analysis-refuse-inconsistent = 無法進行寫入測試：該數值與選單數值未能明確對應。
analysis-refuse-size = 無法進行寫入測試：該數值為 { $size } 位元組；只會寫入較小的數值。
analysis-refuse-read-only = 無法進行寫入測試：相機將該數值標示為唯讀。
analysis-refuse-protected = 無法進行寫入測試：相機將該數值標示為受保護。
settings-analysis-heading = 功能分析
settings-analysis-purpose = 功能分析會記錄相機將其某項選單設定儲存在設定記憶體中的哪一個數值。此功能適合想了解自己型號的設定記憶體並分享結果的人士使用。
settings-analysis-why-off = 此功能預設關閉，因為一般使用並不需要它，它會多次將相機切換至服務模式，而且在您要求時亦會寫入相機。此功能屬實驗性質，只曾在少數型號上試用。
settings-analysis-toggle = 開啟功能分析（選單中的「分析」項目）
settings-analysis-confirm = 要開啟功能分析嗎？它會多次將相機切換至服務模式，並可在您要求時寫入相機。請只在您自己的相機上使用。
btn-analysis-enable = 開啟
settings-analysis-on = 功能分析已開啟。
settings-analysis-off = 功能分析已關閉。
analysis-needs-admin = 每個快照都需要管理員權限以進入服務模式。Windows 在每個程式工作階段只會詢問一次；您可以現在啟動驅動程式助手，或等待第一個快照要求時才啟動。
analysis-changed-hint = 請檢查：找不到候選數值。以下數值有所變化，但未有回復至起始值。如其中某個數值只在兩個數值之間變化了一次，則相機在第一個快照時很可能並未處於第一個選單數值。此時請正確設定起始值，然後重新進行分析。
analysis-changed-info = 資訊：分析期間自行變化的數值（例如計數器、時鐘）會被忽略。您無需採取任何行動。
elevation-admin = 管理員
elevation-standard = 標準權限
elevation-admin-hint = 本程式正以管理員權限執行，可以指派服務模式所需的驅動程式。
elevation-standard-hint = 本程式正以標準權限執行。使用服務模式、調整及功能分析時，本程式會在需要時啟動具備管理員權限的驅動程式助手；Windows 在每個工作階段只會詢問一次。按一下即可啟動。
driver-helper-declined = 管理員權限要求已被拒絕。沒有此權限便無法為服務模式指派驅動程式；未有作出任何變更。
btn-remove-foreign-drivers = 移除舊驅動程式
btn-not-now = 暫時不要
btn-ok = 確定
foreign-drivers-found = Windows 仍保留 Zadig 或類似工具為 Sony 裝置安裝的驅動程式：{ $packages }。在本程式能指派其本身的驅動程式之前，Windows 便會將相機交給這些驅動程式，因此服務模式會失敗，相機亦可能不會顯示為磁碟機。要立即移除嗎？移除後相機會使用 Windows 隨附的驅動程式。移除需要一次管理員權限。
foreign-drivers-removed = 已移除舊驅動程式。請拔除相機再重新接上。
foreign-drivers-remaining = 無法移除以下驅動程式：{ $packages }。請重新啟動 Windows 後再試，或在裝置管理員中連同驅動程式一併解除安裝該裝置。
foreign-drivers-hint = 仍存在 Zadig 或類似工具為 Sony 裝置安裝的驅動程式（{ $packages }）。在本程式能指派其本身的驅動程式之前，Windows 便會將相機交給這些驅動程式，可能導致服務模式失敗。請使用以下指令移除：pmca-console driver zadig --remove
driver-zadig-none = 沒有為 Sony 裝置安裝來自 Zadig 或類似工具的驅動程式。
driver-zadig-entry = { $package }（{ $provider }）：{ $devices }
driver-zadig-remove-hint = 請使用以下指令移除：pmca-console driver zadig --remove
driver-zadig-removed = 已移除 { $count } 個舊驅動程式套件。請拔除相機再重新接上。
elevation-helper = 驅動程式助手已啟用
elevation-helper-hint = 具備管理員權限的驅動程式助手會為服務模式指派驅動程式。它沒有視窗，並會隨本程式結束；之後不會再有提示。
elevation-helper-confirm = 要立即啟動驅動程式助手嗎？Windows 會詢問一次管理員權限，在本程式結束前不會再詢問。
analysis-cross-heading = 在另一部相機上交叉核對
analysis-cross-intro = 選用：檢查另一部相機是否同樣具有已找到的數值，以及該相機上的設定為何。此操作只會讀取。相同的編號不一定代表相同的功能；只有查看另一部相機的選單才能確認。結果會寫入同一份分析報告。
analysis-cross-do = 1. 拔除已分析的相機。 2. 開啟另一部相機，將其 USB 模式設為「自動」，接上 USB 線並選擇「大容量儲存（MSC）」。
btn-analysis-cross = 檢查另一部相機
analysis-cross-present = 存在，數值為 { $value }
analysis-cross-match = 等同「{ $label }」
analysis-cross-nomatch = 不等同任何已量度的選單數值
analysis-cross-absent = 不存在
analysis-cross-same-camera = 這就是已分析的相機本身。請連接另一部相機。
analysis-cross-not-now = 只有在所有快照均已讀取，而且沒有寫入的數值有待還原時，才能進行交叉核對。
analysis-cross-none = 沒有可供交叉核對的候選數值。
analysis-wrong-camera = 已連接的相機並非已分析的相機。請連接 { $model }；未有寫入任何內容。
keys-fetching = 正在下載來源…
keys-scanning = 正在搜尋金鑰資料…
keys-complete = 所有金鑰資料均已齊備。
keys-incomplete = 仍欠缺部分金鑰資料。
keys-present = 已具備
keys-missing = 欠缺
keys-class-senser = 服務模式金鑰（服務模式 Shell、調整、分析）
keys-class-cic = XPD 總和檢查碼金鑰（應用程式安裝）
keys-class-spk = SPK 金鑰資料（應用程式安裝、apk2spk）
keys-class-fdat = FDAT 金鑰資料（更新模式 Shell 固件）
plugin-heading = 插件
plugin-intro = 插件可為本程式加入語言、相機設定開關、相容性備註、工具及診斷功能。插件來自您自行選擇的檔案，或來自您已新增的來源；本程式不會檢查插件。插件的程式碼在沙盒中執行，無法存取您的檔案、網絡或金鑰資料。請只安裝您信任其作者的插件；變更會在下次啟動時生效。
plugin-none = 未有安裝插件。
plugin-inactive = （重新啟動後生效）
plugin-author = 作者
plugin-restart = 請重新啟動本程式，令變更生效。
btn-plugin-install = 安裝插件…
plugin-install-confirm = 此插件可以：
btn-confirm-plugin-install = 安裝插件
plugin-installed = 插件已安裝。
plugin-remove-confirm = 要移除插件「{ $name }」嗎？
plugin-removed = 已移除插件 { $id }。
plugin-perm-languages = 為本程式加入語言
plugin-perm-camera-settings = 變更相機設定（只限於其指明的兩個數值之間，並須經您確認）
plugin-perm-compatibility = 在相容性概覽中加入項目
plugin-perm-code = 在沙盒中執行其本身的程式碼（工具或診斷）
plugin-perm-host-os = 查詢目前使用的作業系統
plugin-perm-host-cameras = 查看已連接哪些 Sony 相機及其所處模式
plugin-invalid = 這不是可用的插件：{ $reason }
plugin-unknown-field = 此插件使用了此版本程式無法識別的功能（{ $field }）。它是為較新版本或其他發行版本的程式而製作的。
plugin-no-folder = 無法確定用戶數據資料夾，因此無法安裝插件。
plugin-not-installed = 未有安裝 ID 為 { $id } 的插件。
plugin-failed = 插件執行失敗：{ $reason }
plugin-unknown-entry = 沒有已載入的插件提供 { $id }。
plugin-run-done = 插件已完成執行。
plugin-run-warning = 插件已完成執行，但有備註。
plugin-run-error = 插件報告了錯誤。
plugin-tools-heading = 插件工具
plugin-tools-hint = 這些工具來自已安裝的插件，並在沙盒中執行。插件只會收到您交給它的內容及其已聲明的內容，而且無法變更相機。
plugin-diagnostics-heading = 插件診斷
plugin-compat-heading = 來自插件
plugin-col-note = 備註
file-filter-plugin = 插件檔案
keys-not-set-up = 尚未設定此操作所需的金鑰資料。
err-missing-keys = 尚未設定此操作所需的金鑰資料。
keys-heading = 需要金鑰資料
keys-intro = 服務模式金鑰、XPD 總和檢查碼金鑰及 SPK 金鑰資料並不包含在本程式中。請輸入您所擁有的來源地址——儲存庫、壓縮檔或檔案——本程式便會找出並匯入其能識別的部分。過程中不會執行任何內容，而有關資料只會保存在此電腦上。
keys-needs-toggle = 哪些功能需要哪些金鑰資料？
keys-needs-info = 相機資料（MTP、大容量儲存、更新模式）
keys-needs-info-keys = 不需要。
keys-needs-install = 應用程式安裝及「檢查相機」
keys-needs-install-keys = XPD 總和檢查碼金鑰；安裝應用程式另需 SPK 金鑰資料。
keys-needs-service = 服務模式：服務模式 Shell、調整、分析
keys-needs-service-keys = 服務模式金鑰。
keys-needs-updater = 更新模式：更新模式 Shell 及應用程式備份
keys-needs-updater-keys = FDAT 金鑰資料。
keys-law-notice = 請遵守您所在地適用的法律，並只處理屬於您自己的相機及數據。
keys-url-label = 來源地址（每行一個）
keys-url-required = 請輸入最少一個來源地址。
btn-keys-import = 匯入
btn-import-keys = 設定金鑰資料
