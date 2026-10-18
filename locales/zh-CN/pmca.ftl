# PMCA-RE RELOADED 简体中文本地化。
# 只翻译等号右侧的文本。切勿修改消息 ID。

app-title = PMCA-RE RELOADED – 相机工具箱

# 导航 / 标签页
nav-camera-info = 相机信息
nav-install-app = 应用
nav-tweaks = 调整设置
nav-network = 网络
nav-firmware = 固件
nav-updater = 升级模式终端
nav-service = 服务模式终端
nav-tools = 文件工具
nav-settings = 设置
nav-license = 许可证

# 欢迎页：窗口启动时首先显示的页面。
nav-welcome = 欢迎
welcome-intro-heading = 这个程序能做什么
welcome-intro-body = 本程序帮助您保留索尼相机的功能，并按自己的需要进行调整。索尼关闭 PlayMemories Camera Apps 商店后，已购买或已安装的应用便无法备份，也无法重新安装。重置相机会永久删除已安装的应用，且无法恢复。本程序可以备份现有应用并重新安装，还可以提供可自由获取的社区应用。您也可以安装自己的应用（*.apk 文件）。出现问题时，本程序会读取相机信息和内部日志以供分析。相机本身具备、但菜单中并未提供的设置可以切换，也可以再次撤销。它通过 USB 线缆直接与相机通信。支持哪些机型，可在“相机信息”页面查看。
welcome-start-heading = 第一步
welcome-start-body = 请先打开相机电源，并在插上线缆之前，将菜单中的“USB 连接”设为大容量存储或 MTP。“自动”通常也可以，因为相机会自行在二者中选一。顺序很重要：一旦插上线缆，就再也进不去相机的菜单了。之后再用 USB 连接相机，打开“相机信息”，按下“获取相机信息”。一旦型号和固件版本显示出来，连接就是正常的，本程序的其余功能也都可以使用。
welcome-start-driver = 若出现的是驱动程序提示，请先按那里的说明处理。在 Windows 上，系统驱动会占用相机，必须先将其替换，本工具所需的直接 USB 访问才能实现。这一步由程序自行完成，但分配驱动是 Windows 只授予管理员的权限。在这种情况下，程序会让你选择以管理员身份重新启动它，Windows 会请你确认此操作。会话结束时该分配会被撤销。在 Linux 和 macOS 上无需此操作。
welcome-start-driver-linux = 若出现的是驱动程序提示，请先按那里的说明处理。在 Linux 上无需替换任何东西，可直接访问相机。只需一次性准备两件事：打开设备的权限——一条针对索尼厂商 ID 的 udev 规则——以及为大容量存储模式解除内核驱动 usb-storage。二者都在本程序之外以 root 权限完成；源码树中的 scripts/linux-prepare.sh 会一并处理。本窗口自身从不索取此类权限。
welcome-start-driver-macos = 如果出现的是驱动提示，请先按其操作。在 macOS 上无需安装或替换任何东西。请将相机的 USB 连接设为 MTP。使用调整功能、服务 Shell 和功能分析时，此窗口会要求输入一次管理员密码：它们通过相机的大容量存储连接进行，而 macOS 只会把该连接交给具有管理员权限的程序。
welcome-tweaks-heading = 调整项与语言
welcome-tweaks-body = 调整项是相机本就具备、却不在菜单中提供的设置：录像时长限制、4K 录像限制、PAL 与 NTSC 的选择、USB 应用安装器——以及菜单语言。
welcome-tweaks-languages-heading = 相机的语言
welcome-tweaks-languages-body = 索尼相机只显示其销售地区常用的菜单语言。因此，身在国外的人常常找不到自己的语言，只能将就使用一种陌生的语言。其实，固件中早已包含更多语言。本程序会把它们加入菜单，让您能用熟悉的语言操作相机。
welcome-tweaks-ui-heading = 本程序的语言
welcome-tweaks-ui-body = 与上述完全无关的是顶部的选择框。它只改变本程序的显示语言，从不触及相机。
welcome-tweaks-safety = 每一项调整都会写入相机的备份存储区，也都能用同一个开关恢复。写入过程被中断可能永久损坏相机，因此写入期间请勿拔线，也勿取出电池。
welcome-privacy-heading = 您的数据
welcome-privacy-where = “许可证”页面会再次完整说明这些内容，并列出构成本程序的所有库的许可证。
welcome-where-heading = 在哪里找什么
welcome-where-info = 型号、序列号和固件版本。同时也是连接测试。
welcome-where-install = 社区列表中的应用，或您自己的 .apk 文件。
welcome-where-tweaks = 相机菜单触及不到的设置，语言也在其中。
welcome-where-network = GPS 辅助数据，以及直播所用的接入点和服务。
welcome-where-firmware = 将索尼固件镜像写入相机。
welcome-where-tools = 解包并查看固件和安装包文件。无需相机。
welcome-where-shells = 直接访问相机的内存和文件系统。需先在“设置”中显示。
welcome-where-settings = 主题、高级模式，以及编辑本程序的翻译。
welcome-where-license = 这个移植版本的来历、建立在谁的工作之上，以及遵循什么条款。
welcome-where-hint = 任何页面都不会自行操作相机。在您按下明确声明用途的按钮之前，既不会读取也不会写入。

# 按钮
btn-get-info = 获取相机信息
btn-refresh = 刷新
btn-load-apps = 加载可用应用
btn-list-tweaks = 显示可用调整项
btn-install = 安装所选应用
btn-install-file = 安装 .apk 文件…
btn-apply = 应用
btn-save = 保存到相机
btn-add = 添加
btn-remove = 删除
btn-reveal = 显示
btn-update-gps = 将 GPS 辅助数据传输到设备
btn-cancel = 取消
btn-browse = 浏览…
btn-switch-msc = 切换到大容量存储模式
btn-start-driver-helper = 启动驱动助手

# 文件对话框中的文件类型筛选器
file-filter-apk = 安卓应用 (*.apk)
file-filter-firmware = 索尼固件映像 (*.dat)
file-filter-usage-log = 使用日志 (*.bin)
file-filter-catalog = 翻译文件 (*.ftl)
file-filter-spk = 索尼应用包 (*.spk)
file-filter-any = 所有文件
file-filter-gps = GPS 辅助数据 (*.dat)

# 状态 / 消息
msg-connecting = 正在连接…
msg-usage-log-written = 使用日志已写入 { $path }
msg-apps-found = 找到 { $count } 个应用
msg-switch-mode = 请将相机切换到 MTP 或大容量存储模式。
msg-no-tweaks = 此相机没有可用的调整项。
msg-done = 完成。

# 相机 USB 模式切换（MTP → 大容量存储）。在“模式错误”提示下方提供；
# 反方向没有对应的 USB 命令。
mode-switching = 正在请求相机切换到大容量存储模式…
mode-switch-wait = 正在等待相机重新连接…
camera-confirm-hint = 相机上显示的提示（例如关于 www.sony.net/pm 的提示）可以关闭，不会影响操作。
mode-switch-done = 相机现在处于大容量存储模式。
mode-switch-timeout = 相机未以大容量存储模式重新出现。请重启相机后重试。
mode-switch-impossible = 此模式无法通过 USB 切换。请重启相机。
mode-switch-hint = 只有处于 MTP 模式的相机才能通过 USB 切换。此更改是临时的：相机菜单中所选的 USB 模式保持不变，因此关闭并重新打开相机（或拔下并重新插入数据线）即可恢复之前的模式。反方向没有对应的 USB 命令。

# 相机信息字段
info-model = 型号
info-product-code = 产品代码
info-serial = 序列号
info-firmware = 固件版本
info-lens = 镜头
info-region = 地区
info-mode = 模式
mode-mass-storage = 大容量存储
mode-mtp = MTP
mode-updater = 升级模式
mode-app-install = 应用安装模式
mode-service = 服务模式
mode-unknown = 未知
info-android-version = Android 平台版本
info-photographer = 摄影师
info-copyright = 版权
info-photographer-1 = 摄影师（条目 1）
info-photographer-2 = 摄影师（条目 2）
info-copyright-1 = 版权（条目 1）
info-copyright-2 = 版权（条目 2）
diagnose-services-heading = 无法连接相机？
diagnose-services-hint = 某些 Windows 服务可能会占用 MTP 模式下的相机，导致本应用无法访问它。此检查只向 Windows 询问哪些服务正在运行，不会做任何更改。
btn-diagnose-services = 检查 Windows 服务
diagnose-services-none = 没有已知的干扰服务在运行。
diagnose-services-found = 以下正在运行的服务可能会干扰对相机的访问：
diagnose-services-advice = 请在“服务”(services.msc) 中停止该服务，或以管理员身份运行以下命令，然后重新连接相机：

# 固件更新器 / 终端
updater-model = 相机型号
updater-firmware-current = 当前固件版本
updater-guard-ok = 固件映像与此相机匹配。
updater-no-fdat = 没有适用于型号 { $model } 的内置更新器固件。
updater-model-required = 相机已处于更新器模式，在该模式下无法报告其型号。请关闭相机再重新开机，使其以正常模式启动，然后重试。或者，指定一个有效载荷文件（--fdat-file），或在控制台中指定型号（--model <MODEL>）。
updater-already-updater = 相机已处于更新器模式。
updater-flash-required = 未经确认，拒绝修改相机。请使用 --flash 重新运行，以切换到更新器模式并写入有效载荷。
updater-flashing = 正在向相机写入更新器有效载荷…
updater-switch-wait = 请按照相机屏幕上的说明操作。正在等待相机切换到更新器模式…
updater-switch-timeout = 等待相机超时。请在相机重新连接后再次运行该命令。
updater-shell-ready = 更新器有效载荷正在运行。输入 “help” 查看命令，输入 “exit” 退出。
updater-shell-done = 终端会话已结束。
updater-session-panic = 升级会话意外结束。重试前请先将相机关机再开机。

# 升级模式终端的回复。解析命令行时的错误只在 service-* 键中写一次：
# 两个终端共用同一个解析器，因此引号包围的路径和错误的十六进制值
# 在两边会以同样的方式失败。
updater-backup-synced = 备份已写入闪存。
updater-backup-locked = 受保护的备份设置现已锁定。
updater-backup-writable = 受保护的备份设置现已可写入。
updater-shell-console-only = root 终端需要一个命令行窗口，只能在命令行下使用：运行 pmca-console updatershell --flash，并在那里输入“shell”。

# 升级模式终端页面。
updater-warning = 警告：要进入这个终端，相机会重启到升级模式，并向其中写入有效载荷——这是本项目的一个小程序，相机会像接受固件更新一样接受它，随后由它响应此终端的命令。该有效载荷本身在相机关机后就会消失，但它提供的命令会写入相机的永久设置存储区，写入过程中断可能永久损坏相机。此外，这一过程还会重置相机的所有设置——服务模式终端则不会。
updater-console-heading = 在命令行中运行升级模式终端
updater-driver-hint = 升级模式终端要求相机保留在 Windows 自带的大容量存储驱动 USBSTOR 上，因为有效载荷正是经由它传送的。这与服务模式终端的要求恰好相反。本程序会自行分配并自行释放 WinUSB 驱动，无需手动准备任何东西——但由其他工具分配的驱动必须先撤销。处于升级模式的相机会以单独的产品 ID 枚举，同样不得重新绑定驱动。
updater-fdat-label = 有效载荷文件（可选）
updater-confirm = 相机将重启到升级模式，并向其中写入有效载荷。是否继续？
updater-cancelled = 已取消。相机未被修改。

btn-shell-primer = 关于本 Shell 的须知
btn-shell-primer-hide = 隐藏说明
shell-primer-compare-heading = 两个 Shell，两种协议
shell-primer-compare = 两个页面都提供通往相机的命令行，但它们不是同一个 Shell。服务 Shell 使用索尼自家的工厂服务协议，通过验证之后由相机本身作答；为此不会向相机写入任何内容。更新器 Shell 之所以存在，只是因为本项目把自己的程序写入了相机：相机重启进入更新器模式，把有效载荷当作固件接收，随后由该有效载荷回答下面列出的命令。两者同一时间只能打开其中一个。
shell-primer-payload = “有效载荷”（英文 payload）指某条传输途径所承载的实际内容。对于更新器 Shell，它是本项目的一个小程序：相机通过与固件更新相同的途径接收它，但只保留到关机为止。服务 Shell 不需要有效载荷，因为相机用自身的固件响应其命令。在固件文件中，这个词的含义相同：指 .dat 文件容器中的实际固件映像。
shell-primer-when-service = 凡是在相机继续运行自身固件的同时读取或修改其已保存设置的事情，都用服务 Shell：读取 HASP 块、保存备份区的回滚副本、修改单条条目、把文件从相机取出。“调整设置”页面在底层用的也是它，那里能做的事在这里都可以手动完成。
shell-primer-when-updater = 当相机无法通过其他方式访问，或者你需要的是在运行中的系统上的一个 Shell 而不是单条命令时，就该用更新器 Shell。只有这条路径能提供真正的 root Shell，而且在拒绝进入服务模式的相机上仍然可用。它的代价是一次重启、一次写入以及相机的设置，因此是第二选择而非首选。
shell-primer-driver-heading = 驱动：两者要求恰好相反
shell-primer-driver = 服务 Shell 使用原始 USB，在 Windows 上要求相机绑定到 WinUSB。本程序在进入服务模式的途中会自行分配该驱动，并在会话结束时自行释放，无需任何单独的工具。更新器 Shell 走的是 Windows 自带的大容量存储驱动 USBSTOR，只要相机还绑定在 WinUSB 上就无法工作——因此释放绑定是结束服务会话的一部分。分配驱动需要管理员权限。在 Linux 上两者都不需要更换驱动，只需要对设备节点的访问权限。
shell-primer-risk-heading = 可能出什么问题
shell-primer-risk = 下面标记的命令会写入相机的永久设置存储器。写入中断可能导致相机无法启动，而且这些操作都无法撤销——bk d 存在的意义正在于此：在修改任何内容之前先保存一份备份区的副本。更新器有效载荷本身不会留在相机上，关闭相机它就消失了。但经由升级模式这条路会重置相机的所有设置。
shell-primer-commands-heading = 本 Shell 的命令
shell-primer-writes = 标有 (WRITES) 的命令会改动相机。其余命令只做读取。
shell-cmd-hasp = 读取 HASP 块，即相机受保护的身份区域。
shell-cmd-pull = 从相机取出一个文件。不给本地文件名时沿用相机上的名称。
shell-cmd-push = 把本地文件复制到相机，覆盖原有内容。(WRITES)
shell-cmd-rm = 删除相机上的一个文件。(WRITES)
shell-cmd-rmem = 把设备内存的一段写入本地文件。偏移量和长度均为十六进制。
shell-cmd-bk-r = 读取备份区的一条条目并以十六进制转储显示。
shell-cmd-bk-d = 把整个备份区保存到文件。修改任何内容之前先做这件事：这是唯一的退路。
shell-cmd-bk-dump-txt = 把整个备份区保存为 <file>.bin，并把可读的列表保存为 <file>.txt。
shell-cmd-bk-stat = 显示相机的备份状态块，包括区域。
shell-cmd-bk-w = 写入备份区的一条条目，以空格分隔的十六进制字节给出。(WRITES)
shell-cmd-bk-s-sub = 把某个子系统的备份区固化到闪存，使改动能在重启后保留。(WRITES)
shell-cmd-bk-s = 把备份区固化到闪存，使改动能在重启后保留。(WRITES)
shell-cmd-bk-lock = 设置或解除受保护备份条目的写保护；部分设置在写保护状态下拒绝更改。(WRITES)
shell-cmd-term = 打开或关闭相机的调试终端。(WRITES)
shell-cmd-info = 读取备份状态块，它说明该区域是否可写。
shell-cmd-shell = 在相机上打开一个 root Shell。它需要终端，因此只在控制台中可用；shell tcp <port> 则改为在 127.0.0.1 上提供给另一个客户端。(WRITES)
shell-cmd-help = 列出本 Shell 接受的命令。
shell-cmd-exit = 结束会话并退出 Shell。
shell-primer-diagnostics-heading = 诊断与修复
shell-cmd-bk-trace = 将相机通过 USB 发送整个备份区的过程逐包记录到文件中。用于排查较大的备份；可能使相机卡在应答中途。此时请拔下数据线，关闭再打开相机，必要时取出电池。
shell-cmd-bk-rewrite = 读取备份区，原样写回后再读取一次。在不改变任何设置的情况下，显示相机是否接受完整映像。(WRITES)
shell-cmd-bk-restore-attrs = 将所有备份条目的属性（如写保护）恢复为用 bk d 或 bk dump-txt 保存的副本中的状态。数值保持不变。只能使用同一台相机的副本。(WRITES)
shell-primer-diagnostics-env = 以下三个环境变量如在启动前设置，会同时作用于控制台和图形界面，例如在 PowerShell 中执行 $env:PMCA_TWEAKS_DIRECT = "1"。PMCA_TWEAKS_DIRECT 让调整项逐个写入值，即使备份小于 1 MB。PMCA_TWEAKS_IMAGE 让其写入完整映像，即使大于 1 MB；两者都设置时以 PMCA_TWEAKS_DIRECT 为准。PMCA_TLS_DUMP=<文件> 会在安装应用时保存相机的第一条 TLS 消息（ClientHello），以便排查连接失败。

# 应用安装
install-switching = 正在切换到应用安装模式…
install-disabled = 相机拒绝进入应用安装模式。通常是“启用 USB 应用安装器”这项设置导致的：需要在“调整设置”中将其打开。
install-stays-app-mode = 相机将保持在应用安装模式，在此模式下可继续安装其他应用。如需使用其他功能，请拔下 USB 线，然后关闭并重新打开相机。
err-usb-app-install-mode = 相机仍处于应用安装模式，在此模式下只接受应用安装。请先拔下 USB 线，再关闭并重新打开相机——若在连接状态下重启，相机会直接回到该模式。
install-switch-wait = 正在等待相机以应用安装模式重新连接…
install-switch-timeout = 等待相机进入应用安装模式超时。请在相机重新连接后再次运行该命令。
install-bad-mode = 相机在此模式下无法使用。请将其切换到 MTP 或大容量存储模式。
install-starting = 正在开始安装…
install-error = 安装通信错误
install-server-failed = 本地安装服务器未接受相机的连接：
install-done = 任务已成功完成。

# APK 检查
apk-package = 软件包：
apk-unreadable = 警告：无法分析该 .apk 文件。
apk-incompatible = 警告：此应用声明的最低 SDK 版本可能高于相机所提供的版本。
apk-unsigned = 警告：该 .apk 文件没有签名。

# 社区应用商店
store-loading = 正在加载应用列表…
store-resolving = 正在查找最新版本…
store-downloading = 正在下载
store-unknown-app = 未知应用
store-no-release = 此应用没有可下载的版本
store-not-installable = 无可用版本
msg-no-apps = 未找到应用。
store-heading = 社区应用商店
store-discontinued = 索尼已关闭官方 PlayMemories Camera Apps 商店，其服务器不再响应，因此无法再从中购买或下载应用。
store-use-own-apk = 您仍然可以安装自己的 .apk：在下方选择文件，或将其拖到此窗口。下方的社区应用商店同样仍可使用。
btn-store-sync = 与相机核对
btn-install-selected = 安装所选应用
btn-store-stop = 在此应用后停止
btn-store-stopping = 将在此应用后停止…
btn-store-retry = 重试失败的应用
btn-continue = 继续
btn-reinstall = 重新安装
btn-skip = 跳过
btn-confirm-updater = 重启并写入
btn-confirm-tweaks = 写入调整设置
btn-confirm-languages = 启用这些语言
btn-confirm-wifi = 替换接入点
btn-confirm-stream = 覆盖设置
btn-confirm-show-shells = 显示终端
btn-confirm-discard = 放弃翻译
store-select-all = 选择所有应用
store-col-app = 应用
store-col-version = 版本
store-col-author = 作者
store-col-camera = 相机上的版本
store-col-status = 状态
store-camera-absent = 未安装
store-version-unknown = 未知
store-batch-query = 正在从相机读取已安装的应用…
store-sync-done = 已与相机核对。列表中已安装的应用：{ $count }
store-batch-downloading = 正在下载第 { $current }/{ $total } 个应用…
store-batch-installing = 第 { $current }/{ $total } 个应用：正在安装 { $name }…
store-reinstall-confirm = { $name } 已安装在相机上。已安装版本：{ $installed }。应用商店中的版本：{ $store }。是否重新安装？
store-state-queued = 等待中
store-state-installed = 已安装
store-state-failed = 失败
store-state-skipped = 已跳过
store-state-not-run = 未执行
store-batch-summary = 已安装：{ $done } · 失败：{ $failed } · 已跳过：{ $skipped } · 未执行：{ $notrun }

# 服务（senser）模式
usb-mode-advice = 使用调整、备份和服务 Shell 时，请将相机的 USB 连接模式设为“自动”，不要固定为大容量存储。会话结束后相机询问模式时，请选择“大容量存储 (MSC)”。固定为大容量存储时，相机经常拒绝下一次切换到服务模式。
service-switch-refused = 相机没有完成到服务模式的切换，也没有通过 USB 重新连接；此时相机通常显示“请检查所连接的设备”。请拔下 USB 线缆，重新连接后再试一次。
service-switching = 正在切换到服务模式…
service-switch-wait = 正在等待相机以服务模式重新连接…
service-switch-timeout = 相机未以服务模式出现。它会作为独立的 USB 设备（产品 ID 0336）重新枚举，需要单独分配 WinUSB 驱动；请在设备管理器中查找没有驱动的索尼设备。重启相机可使其返回正常模式。
service-authenticating = 正在验证…
# 当相机仍停留在上次运行的维修模式时，以及无法退出该模式时显示。
service-stranded-reset = 相机仍停留在上次运行的维修模式。正在先结束该会话…
service-stranded-failed = 相机因上次运行而卡在维修模式，且未响应退出请求。请将相机关机后重新开机，然后重试。
service-bad-mode = 相机在此模式下无法使用。请将其切换到大容量存储模式。
service-driver-required = 原始 USB 访问被拒绝，且无法分配 WinUSB 驱动。分配驱动需要管理员权限。
service-warning = 警告：服务模式会授予对相机内存、文件系统和校准数据的原始访问权限。写入命令可能永久损坏相机。
service-shell-ready = 服务终端已就绪。输入 “help” 查看命令，输入 “exit” 退出。
service-shell-done = 服务终端会话已结束。
# 当上一次运行在释放其分配的驱动程序之前结束时，于启动时报告。
driver-swept = 已清除上次运行残留的 { $count } 个驱动程序分配。
# 当同一次中断的运行还把相机留在维修模式时，在启动时报告；
# 相机在该模式下会拒绝下一次会话。
driver-swept-camera = 上次运行残留在维修模式的相机已恢复为正常模式。
driver-release-replug = 释放驱动程序时 Windows 正在等待相机。请拔下 USB 线缆并重新连接，以便完成清理。
# `driver` 命令的结果。
driver-bound = { $id } 现在使用 WinUSB（驱动程序包 { $package }）。
driver-released = 已解除 { $count } 个驱动程序分配；相机已恢复使用 Windows 自带的驱动程序。
driver-needs-elevation = 分配驱动程序需要管理员权限。请在以管理员身份启动的控制台中运行此命令。
driver-leftover-needs-elevation = 之前的一次运行给相机分配了驱动程序，且尚未释放。只要它存在，相机就会要求检查已连接的设备，大容量存储也无法使用。释放它需要管理员权限；请确认 Windows 的提示。
driver-binding = 正在为相机分配 WinUSB 驱动程序…
driver-bind-wait = 正在等待相机以新驱动程序重新连接…
driver-bind-timeout = 分配驱动程序后相机未重新出现。请拔下并重新连接相机，然后重试。
driver-bind-failed = 无法分配 WinUSB 驱动程序：{ $detail }
# 终端的回复。它们来自后端，因此控制台和窗口使用相同的措辞。
service-usage = 用法：{ $usage }
service-bad-hex = 不是十六进制值：{ $value }
service-unknown-command = 未知命令：{ $command }。输入 'help' 查看列表。
service-unterminated-quote = 该行在引号包围的参数中间结束。
service-help = 命令列表：
service-read-bytes = 已读取 { $count } 字节。
service-pulled = 已将 { $remote } 从相机复制到 { $local }。
service-pushed = 已将 { $local } 复制到相机上的 { $remote }。
service-removed = 已从相机删除 { $remote }。
service-dumped = 已将从 { $offset } 开始的 { $size } 字节保存到 { $local }。
service-backup-saved = 已将备份镜像（{ $count } 字节）保存到 { $local }。
service-backup-text-saved = 已将备份镜像（{ $count } 字节）保存到 { $bin }，列表保存到 { $txt }。
service-backup-status = 备份状态，区域：{ $region }
service-backup-rewritten = 镜像（{ $count } 字节）已原样写回并重新读取；未永久保存。
service-backup-attrs-restored = 已按 { $local } 恢复 { $count } 个条目的属性。
service-backup-written = 已写入备份条目 { $id }。
service-backup-persisted = 已将子系统 { $id } 的备份写入闪存。
service-terminal-on = 调试终端现已开启。
service-terminal-off = 调试终端现已关闭。
btn-service-terminal-open = 打开串行控制台
btn-service-terminal-close = 关闭串行控制台
service-terminal-heading = 串行控制台
service-terminal-hint = 在 Linux 相机上，调试终端是通过 USB 的 root shell。打开它，然后像在终端中一样输入命令。
service-terminal-label = 控制台
service-terminal-interactive = 串行控制台已打开。请输入命令；输入“exit”返回服务 shell。

# 未启用高级模式时的服务页面：如何从控制台进入终端，以及需要提前准备什么。
service-console-heading = 在控制台中启动服务终端
service-console-hint = 在包含 pmca-console 的文件夹中打开命令提示符，并运行以下命令：
service-driver-heading = 开始之前
service-driver-hint = 服务模式需要直接的 USB 访问权限，Windows 的大容量存储驱动程序并不提供。本程序会自行分配 WinUSB 驱动——共两次，一次在相机处于大容量存储模式时，另一次在它以第二个产品 ID 重新以服务模式出现之后——并在会话结束时释放这两次分配。分配驱动需要管理员权限，因此请以管理员身份启动控制台；不需要其他任何工具。
driver-note-linux = 这里无需安装任何东西：libusb 直接访问相机，不必顶替内核驱动。所需要的只是打开设备的权限，以及一个没有占用该设备的桌面环境。请为索尼的厂商编号 054c 添加一条 udev 规则，并在使用大容量存储模式前解除 usb-storage 驱动的绑定。源码树中的 scripts/linux-prepare.sh 会完成这两件事，并同时停止 gvfs 卷监视服务。
driver-note-macos = 直接访问 USB 无需安装任何东西。请将相机的 USB 连接设为 MTP。在 MTP 模式下，如果 macOS 的相机服务 ptpcamerad 占用了相机，程序会将其结束；系统会在需要时重新启动它。调整功能、服务 Shell 和功能分析都通过大容量存储连接进行，macOS 只会把它交给具有管理员权限的程序：窗口每次运行只要求输入一次密码，命令行需要 sudo。此平台是三者中测试最少的，遇到问题时更可能需要你来报告，而不是已有记录。
service-advanced-hint = 终端也可以在此处显示：设置中的高级模式。
btn-copy-command = 复制命令
msg-command-copied = 命令已复制到剪贴板。

# 启用高级模式后的服务页面。
btn-service-open = 连接
btn-service-close = 断开
btn-service-run = 执行
btn-service-browse = 插入文件路径…
shell-path-quote = 无法插入此路径：其中含有双引号，而命令行正是以该字符判定一个参数的起止位置。请重命名该文件，或将其移至不含该字符的路径。
service-command-label = 命令
service-not-connected = 尚未连接。

# 仅用于图形界面的字符串
install-apk-path = 本地 .apk 文件的路径（可选）
install-drop-hint = 将 .apk 文件拖放到此处

# 调整项说明
tweak-rec-limit = 无时长限制录制视频
tweak-rec-limit-4k = 无时长限制录制 4K 视频
tweak-language = 显示所有菜单语言
tweak-pal-ntsc = 启用 PAL / NTSC 选择器与警告
tweak-pal-ntsc-mode = 视频制式（PAL / NTSC）
tweak-pal-ntsc-pal = PAL
tweak-pal-ntsc-ntsc = NTSC
tweak-pal-ntsc-note = 切换视频制式后，相机会要求格式化存储卡。请先备份卡上的全部内容——照片、视频、LUT。最简单的做法是在切换前把卡取出。
tweaks-set-malformed = 需要 --set ID=VALUE，收到：{ $arg }
tweaks-set-unknown-value = 未知的值“{ $value }”。应为下列之一：{ $options }
tweaks-set-not-a-choice = { $id } 是开关，不是选项。请使用 --enable 或 --disable。
tweaks-needs-set = { $id } 是选项，不是开关。请使用 --set { $id }=VALUE，取值为下列之一：{ $options }
tweak-usb-installer = 启用 USB 应用安装器

# 调整流程
tweaks-warning = 警告：应用调整会重写相机的备份存储区。写入过程中断可能会永久损坏相机。
tweaks-reading = 正在读取相机设置…
tweaks-writing = 正在写入相机设置…
tweaks-done = 相机设置已更新。
tweaks-write-failed = 相机没有按发送的内容保存新设置。其中一部分仍可能已被写入：请重新读取设置，必要时撤销更改。更改前制作的备份是安全的恢复途径。
tweaks-confirm-required = 未经确认，拒绝写入。阅读上述警告后，请使用 --confirm 重新运行。
tweaks-state-enabled = 已启用
tweaks-state-disabled = 已禁用
tweaks-full-read-failed = 相机没有传送完整的备份（{ $detail }）。之后它常常不再响应：请关闭相机，取出电池几秒钟，重新装入后再连接。
tweaks-direct-mode = 这台相机的备份大于 1 MB，因此逐项读取和写入设置。每次更改前都会保存旧值，可用“撤销更改”写回。
tweaks-direct-forced = PMCA_TWEAKS_DIRECT 强制使用逐项方式，因此逐项读取和写入设置。每次更改前都会保存旧值，可用“撤销更改”写回。
tweaks-direct-refused = 相机拒绝了更改（{ $detail }）。已写入的值已恢复，没有任何更改。部分机型（尤其是日版）只允许通过完整备份更改此设置。
tweaks-direct-revert-failed = 相机拒绝了更改（{ $detail }），并非所有已写入的值都能恢复。旧值已保存：请尝试“撤销更改”。
tweaks-direct-restart = 相机设置已更新。请关闭并重新打开相机以使更改生效。
tweaks-rollback-unsaved = 无法保存当前值以供撤销，因此没有任何更改。
tweaks-image-saved = 已保存更改前的备份映像：{ $path }
tweaks-whole-image-hint = 若要改为通过完整备份映像写入更改，请加上 --whole-image 再次运行同一命令。更改前的映像会先被保存。
tweaks-whole-image-confirm = 要通过完整备份映像写入更改吗？相机拒绝了以单个值写入。整个映像将连同更改一起写回；更改前的映像会先保存在这台电脑上。
btn-whole-image = 写入完整备份
tweaks-undo-none = 这台相机没有已保存的值。
tweaks-undo-confirm = 要把保存的值写回相机吗？这会撤销自保存以来逐项写入的所有更改。
tweaks-undo-done = 已写回保存的值。请关闭并重新打开相机。
tweaks-undo-available = 第一次更改前的值已保存，可以写回。
btn-undo-tweaks = 撤销更改
tweaks-identity-heading = 相机标识（只读）
tweaks-caution-languages = 本相机无法显示两种语言（可能是简体中文和繁体中文）。滚动到它们时语言菜单会重新开始，因此出于谨慎将其保持关闭。
tweaks-caution-accept = 仍然启用这两种语言（后果自负）
tweaks-caution-confirm = 这两种语言可能导致相机的语言菜单重新启动。不会造成损坏，更改也可以撤销，但您可能需要从头开始查找该设置。仍要启用吗？

# 设置
settings-language = 语言
settings-theme = 主题
settings-theme-system = 跟随系统
settings-theme-light = 浅色
settings-theme-dark = 深色

# 高级模式。它防的是误点，而不是攻击者：它所显示的一切本来就能从控制台使用。
settings-advanced-heading = 高级模式
settings-advanced-hint = 高级模式会在“服务模式终端”和“升级模式终端”页面中显示相应终端。服务模式可直接访问相机的内存、文件系统和校准数据；一条输错的命令就可能使相机无法使用。在您确定要写入什么之前，请保持关闭。
settings-advanced-toggle = 显示服务终端与升级终端
settings-advanced-on = 高级模式已开启。“服务模式终端”和“升级模式终端”页面现在提供终端本身。
settings-advanced-off = 高级模式已关闭。“服务模式终端”和“升级模式终端”页面改为说明如何通过控制台使用。
settings-advanced-confirm = 服务终端与升级终端可能会永久损坏您的相机。确定要显示这两个终端吗？

# 编辑翻译。语言文件可以下载后在任意文本编辑器中修改，再上传；
# 上传的文件在使用前会先经过检查。
settings-catalog-heading = 编辑翻译
settings-catalog-hint = 下载语言文件，在您喜欢的文本编辑器中修改，然后重新上传。只需翻译等号右侧的文字，省略的条目将保留内置的文字。在此处选择语言不会改变应用程序的显示语言。
settings-catalog-locale = 语言文件
settings-catalog-builtin = 正在使用内置翻译。
settings-catalog-custom = 正在使用您自己的翻译：
btn-catalog-export = 下载…
btn-catalog-import = 上传…
btn-catalog-reset = 恢复内置翻译
catalog-exported = 已写入
catalog-imported = 翻译已应用。
catalog-reset-done = 已恢复内置翻译。
catalog-reset-none = 没有可恢复的自定义翻译。
catalog-reset-confirm = 要放弃该语言的自定义翻译吗？
catalog-rejected = 文件未被应用。请修正以下行：
catalog-count-accepted = 已应用的条目
catalog-count-missing = 从内置翻译保留的条目
catalog-count-untranslated = 与英文原文相同的条目
catalog-line = 行
catalog-problem-syntax = 不是形如“id = 文字”的条目
catalog-problem-continuation = 延续到下一行，不支持这种写法
catalog-problem-duplicate = 定义了多次
catalog-problem-unknown-key = 不是本应用程序使用的条目
catalog-problem-empty = 没有文字
catalog-problem-placeholder = 占位符被改动，应为：

# 来源、致谢与许可证。显示在“许可证”页面以及
# `pmca-console license` 中。许可证正文本身不翻译：它是法律文件，
# 按原文逐字转载。
about-thanks-heading = 致谢
about-thanks-body-1 = 这个工具之所以存在，是因为 ma1co 弄清了索尼相机通过 USB 通信的方式，并在 2015 年发布了 Sony-PMCA-RE；也因为在此后的岁月里为它以及 OpenMemories 各项目作出贡献的每一个人。
about-thanks-body-2 = 相机识别、外部命令协议、固件更新程序流程和服务模式，全都是他们的成果，本移植版本延续了这些工作。谢谢你们。在此基础上，PMCA-RE RELOADED 新增了许多由 masterflai 提供的功能。其中最重要的是现已内置的 Windows 设备驱动程序自动切换：不再需要借助单独的工具手动为相机分配其他驱动程序，整个过程因此更方便、更快捷，风险也更低。
about-contributors-heading = 贡献者
about-contributors-body = 除 ma1co 之外，以下人员也为 Sony-PMCA-RE 作出了贡献，或在各自的副本中发布了已被纳入本移植版本的修改：
about-purpose-heading = 本移植版本存在的原因
about-purpose-body-1 = 原始程序已不再继续开发。它是为 Python 2 编写的，而 Python 2 已于 2020 年终止支持；其中一部分还依赖 Sony 此后关闭的服务，因此在当前系统上已无法不受限制地运行。
about-purpose-body-2 = 本移植版本的目的是保存而非取代，同样也是为了今天能够顺畅使用：让这些相机继续可用，并让程序在当今的电脑上以及较新的相机型号上都能顺利运行。原始程序能做到的一切，这里都应当仍然能够做到。
about-origin-heading = 它是如何做成的
about-origin-body-1 = 从 Python 到 Rust 的移植是在 AI 的协助下完成的。
about-origin-body-2 = 这并不意味着什么，同样值得直说：其行为由一套自动化测试和一份硬件测试计划覆盖。尤其是 Windows 版本经过了详尽的检验，因为设备驱动程序使其成为最复杂的平台；相比之下，macOS 和 Linux 未能进行全面测试。请将它视为实验性软件。
about-license-heading = 许可证
about-license-body = 本移植版本以 MIT 许可证发布，与原始项目所用的许可证相同。原始版权声明按该许可证的要求原样保留。
about-license-full = 许可证全文
about-third-party-heading = 所用库的许可证
about-third-party-body = 本程序为静态链接：所用每个库的代码都在可执行文件之内，这些库的许可证要求版权声明随之一同分发。以下按每个软件包一条的方式完整列出。
about-third-party-toggle = 显示全部第三方声明
about-privacy-heading = 数据
about-privacy-body-1 = 本程序在您自己的计算机上运行。它不发送使用数据，没有遥测，也不需要账户。您在此进行的任何操作都不会上报到任何地方。
about-privacy-body-2 = 但有两项功能会自行联网，对方会因此得知您的 IP 地址：GPS 辅助数据从索尼下载，应用列表从 GitHub 下载。不使用这些功能时，本程序完全离线。
about-privacy-body-3 = 从相机读出的内容——包括序列号和已保存的 Wi-Fi 密码——不会离开您的计算机。若您将其写入文件，请注意把该文件交给了谁。
about-privacy-body-4 = 本程序背后没有运营任何服务，也没有任何数据到达其作者，因此这里没有可供查询或要求删除的内容。上述两项服务如何处理其所见的地址，取决于它们各自的隐私声明，而非本说明。
about-legal-heading = 法律声明
about-legal-trademark = 这是一个独立项目，并非由索尼制作，未获其认可，也与其没有任何关联。"Sony"、"PlayMemories" 以及各相机型号名称均为 Sony Group Corporation 的商标，此处提及仅为说明本工具与哪些相机通信。
about-legal-own-device = 本程序旨在用于您自己拥有的相机。
about-legal-warranty = 本工具所做的事情超出了索尼为这些相机设定的范围，可能会终止您的保修。软件本身不附带任何担保，使用风险由您自行承担。对自己拥有的设备进行逆向工程在多大程度上被允许，取决于您所在的国家或地区。
about-links-heading = 项目
about-link-original = Sony-PMCA-RE — 原始项目
about-link-fwtool = fwtool.py — 更新映像生成所依据的固件工具
about-link-tweak = OpenMemories: Tweak — 推荐用于受支持相机的应用
about-link-applist = OpenMemories: App list — 更多应用
about-link-platform = OpenMemories: Platform — 随附的更新器主体
about-link-devices = 受支持的相机型号
about-link-nexhack = Nex-Hack — 本项目所依托的社区研究
about-link-zve10 = zv-e10-re — 关于 ZV-E10 接口与设置存储的研究
about-version = 版本
btn-copy-url = 复制地址
msg-url-copied = 地址已复制到剪贴板。

# 安全提示
notice-experimental = 这是实验性软件。使用风险由您自行承担。

# 网络设置（GPS / Wi-Fi / 直播）
gps-downloading = 正在下载 GPS 辅助数据…
gps-writing = 正在向相机写入 GPS 辅助数据…
gps-bad-size = GPS 辅助数据大小异常，未写入。
wifi-read = 正在从相机读取直播接入点…
wifi-written = 直播接入点已写入。
stream-read = 正在从相机读取直播设置…
stream-written = 直播设置已写入。
net-secret-warning = 警告：此文件以明文包含 Wi-Fi 密码和访问令牌。
wifi-secret-warning = 相机以未加密的方式保存这些密码，此处按其保存的原样显示。它们不会离开这台计算机；若您将其保存到文件，请注意把该文件交给了谁。
gps-heading = GPS 辅助数据
gps-hint = 将文件拖到此处、输入路径，或留空以从索尼获取最新数据。
btn-gps-devices = 兼容设备
btn-gps-devices-hide = 隐藏兼容设备
gps-devices-note = 以下型号内置 GPS 模块。其中许多型号只有名称以 V 结尾的版本配备该模块；不带 V 的版本没有 GPS。
gps-devices-compact = 便携相机 (DSC)
gps-devices-slt = SLT 相机 (A 卡口)
gps-devices-action = 运动相机
gps-devices-camcorder = 摄像机 (Handycam)
gps-col-variant = GPS 版本
gps-col-arch = 架构
gps-col-module = GPS 模块
gps-col-apps = 应用支持
gps-yes = 是
gps-no = 否
gps-unknown = 未知
gps-devices-untested = 带 GPS，支持情况未经测试
gps-devices-untested-note = 以下型号内置 GPS 模块，但不在 OpenMemories 的设备列表中。尚未测试本应用能否与它们配合使用，因此其架构和应用支持情况未知。
wifi-heading = 直播接入点
wifi-hint = 这些是直播功能使用的接入点，而不是相机本身的 Wi-Fi 设置。不具备直播功能的机身会拒绝此操作。
wifi-multi = 多接入点列表
wifi-none = 未存储任何接入点。
wifi-writing = 正在向相机写入直播接入点…
wifi-confirm = 此操作将替换相机上存储的所有接入点。是否继续？
wifi-empty-refused = 拒绝写入空列表，否则会清除所有已存储的接入点。
stream-heading = 直播
stream-writing = 正在向相机写入直播设置…
stream-confirm = 此操作将覆盖相机的直播设置。是否继续？
stream-invalid-json = 设置不是有效的 JSON：

# 固件更新与官方商店
firmware-warning = 警告：此操作会覆盖相机固件。更新开始后请勿中断：不要拔下 USB 线，也不要关闭相机。只能使用专为此相机型号提供的固件文件，并确保电池电量充足。否则相机可能永久无法使用。
firmware-untested = 未经测试：固件刷写功能已完成移植，但从未在真实相机上试验过。写入失败或中断无法撤销，因此请将本页的一切视为未经验证。
firmware-confirm-required = 未使用 --confirm，拒绝刷写。
firmware-truncated = 固件文件被截断：FDAT 有效载荷超出了文件末尾。
firmware-updating = 正在更新固件：
firmware-invalid = 该文件不是有效的索尼固件映像
firmware-dat-path = 固件文件 (.dat)
firmware-drop-hint = 将 .dat 固件文件拖放到此处
firmware-file-required = 请先选择固件文件。
firmware-check-ok = 相机接受此映像。尚未写入任何内容。
firmware-cancelled = 刷写已取消。相机未被修改。
firmware-current-version = 已安装版本
firmware-new-version = 此映像中的版本
firmware-models-hint = 升级模式终端能够访问的相机即为兼容相机。
btn-firmware-check = 检查映像
btn-firmware-flash = 刷写固件
firmware-own-device = 请确认您了解这些风险，并且只更新您自己的设备。
btn-firmware-accept = 已了解风险 – 刷写

# 文件工具：完全不需要相机的操作。
tools-hint = 这些操作只处理文件，从不触碰相机，因此这里的任何操作都不会损坏它。它们就是 pmca-console 以 apk2spk、spk2apk、print-backup、print-usagelog、guess-firmware 和 xpd-checksum 提供的那些操作。
tools-operation = 操作
tools-apk2spk = 将 .apk 打包进 .spk 容器
tools-spk2apk = 将 .spk 解包回 .apk
tools-print-backup = 显示 Backup.bin 的内容
tools-print-usagelog = 显示 Usagelog.bin 的内容
tools-guess-firmware = 在固件 .dat 中查找有效载荷
tools-xpd-checksum = 计算文件的 CIC 校验和
tools-input = 输入文件
tools-file-required = 请先选择此操作所需的文件。
tools-apk-invalid = 无法将该文件打包为 .spk
tools-spk-invalid = 该文件不是有效的 .spk 包
tools-backup-invalid = 该文件不是有效的 Backup.bin 镜像
tools-usagelog-invalid = 该文件不是有效的使用日志
tools-reset-value = 重置值：
tools-fdat-found = 在偏移 { $offset } 处找到有效载荷，大小 { $size }。
tools-checksum = 校验和
tools-written = 已写入 { $path }

# 指向项目文档的链接
docs-compatibility = 相机兼容性
docs-compatibility-hint = 原项目提供了一份已验证相机的概览。其中主要是较早的机型，地址如下：
devices-note = 数据取自原项目的概览。应用支持旁的数字是相机应用平台的版本。
devices-more-note = 以下相机未经本程序专门测试。最后一列中的勾号表示社区曾报告过使用情况；若服务模式终端同时标记为 ✖，则表示使用未成功。
btn-more-models = 更多设备
btn-more-models-hide = 隐藏更多设备
check-updater-yes = 您的设备 { $model } 兼容：根据设备列表，升级模式终端可以访问它。
check-updater-no = 您的设备 { $model } 不兼容：根据设备列表，升级模式终端无法访问它。
check-updater-unknown = 您的设备 { $model } 的兼容性未知：它不在本程序的任何设备列表中，因此不清楚升级模式终端能否访问它。
check-apps-no = 您的设备 { $model } 不兼容：根据设备列表，它不支持应用。
check-apps-untested = 您的设备 { $model } 的兼容性未知：它在设备列表中，但应用支持尚未确认。
check-apps-unknown = 您的设备 { $model } 的兼容性未知：它不在本程序的任何设备列表中，因此不清楚它是否支持应用。
check-gps-no = 您的设备 { $model } 不兼容：它不在带 GPS 接收器的设备列表中。
check-gps-untested = 您的设备 { $model } 的兼容性未知：根据设备列表，它带有 GPS 接收器，但向该型号传输数据尚未经过测试。
check-gps-unknown = 您的设备 { $model } 的兼容性未知：它不在本程序的任何设备列表中，因此不清楚它是否带有 GPS 接收器。
check-unidentified = 无法识别已连接的设备，因此其兼容性未知。
check-proceed = 仍要继续吗？
check-cancelled = 操作已取消。设备未被修改。
devices-col-confirmed = 已确认
devices-group-ilc = 可换镜头相机（E 卡口和 A 卡口）
devices-group-remote = 实时取景遥控器
btn-copy-link = 复制链接
shutter-heading = 快门计数
shutter-hint = 相机会统计每一次快门释放，并把当前数值写入它保存的每张照片（ARW、JPEG 或 HEIF）。请选择最近拍摄的照片来读取当前数值。文件只会被读取，绝不会被修改。
shutter-source = 计数在文件中的位置因机型而异。这些位置来自公开记录的研究（ExifTool），并未在所有机型上得到确认。
shutter-drop-hint = 将照片（ARW、JPEG、HEIF）拖放到此处
btn-shutter-read = 分析照片
shutter-file-required = 请先选择一张照片。
shutter-file = 文件
shutter-taken = 拍摄日期
shutter-count = 快门次数
shutter-type = 快门
shutter-type-mechanical = 机械快门
shutter-type-electronic = 电子快门
shutter-electronic-hint = 这张照片是用电子快门拍摄的。在许多机型上，这类拍摄不计入快门次数；此时显示的是最后一次机械快门释放时的计数。
shutter-not-sony = 这张照片不是用索尼相机拍摄的，无法从中读取快门计数。
shutter-unknown-model = 尚不清楚该相机型号将快门计数存放在文件中的哪个位置，因此不显示数值。
shutter-not-recorded = 该文件不包含可读取的快门计数。经图像编辑软件保存或导出的照片，以及部分机型使用电子快门拍摄的照片会出现这种情况。请选择存储卡上的原始文件。
shutter-model-mismatch = 这张照片来自与已连接相机不同的相机型号。
shutter-invalid = 该文件不是可读取的照片
file-filter-photo = 照片 (ARW、JPEG、HEIF)
usage-log-heading = 使用日志
usage-log-hint = 相机以未公开的格式保存内部日志。日志将原样保存；此操作仅从相机读取。
usage-log-written = 已写入
btn-usage-log-info = 关于使用日志
btn-usage-log-info-hide = 隐藏说明
usage-log-info-what-heading = 这是什么日志
usage-log-info-what = 索尼在内部将此日志称为“KikiLog”。日志由相机自行写入，本程序只读取它。目前不知道有任何公开的格式说明。
usage-log-info-read = 读取不会改变任何内容：连续多次下载的结果逐字节完全相同。
usage-log-info-flush-heading = 为什么缺少最新记录
usage-log-info-flush = 新记录往往不会立即出现：相机可能要到关机时才保存它们，有时分散在多次开关机中。请关闭并重新打开相机，再次下载，并重复此操作，直到不再出现新记录。
usage-log-info-content-heading = 可以读出什么
usage-log-info-shutter = 快门释放：某些事件各自对应一次拍摄。但这些记录的数量并不是快门计数：日志大小固定，不一定涵盖相机的整个使用期。相机在别处保存总数，并将其写入所保存的照片；可在“快门计数”部分读取。
usage-log-info-clock = 时间戳的准确度取决于相机时钟。更换电池后时钟可能倒退，并生成带有相机初始日期的记录。
usage-log-info-unknown = 目前不知道有任何关于事件编号的公开说明。程序只显示编号，不作解释。
usage-log-info-privacy-heading = 隐私
usage-log-info-privacy = 该文件包含插入过的存储卡的序列号。请勿随意分享或公开发布。
btn-usage-log = 保存日志
usagelog-size = 日志区域
usagelog-cursor = 写入位置
usagelog-spare = 第二区域
usagelog-count = 条目数
usagelog-events = 按编号统计的事件
usagelog-entries = 按顺序的条目
msg-link-copied = 链接已复制到剪贴板。

# 后端故障消息，按后端报告的稳定代码索引。
# 消息中的可变部分（路径、状态字节、设备错误）由前端附加，
# 因此这些句子在不含该部分时也应完整可读。
# 这里刻意没有 `err-operation`：该代码标记的是后端已翻译好的消息，必须原样显示。
err-usb-no-device = 未找到相机。请连接相机并将其开机。
err-usb-camera-not-ready = 相机已连接但尚未就绪：它已关机，或正在屏幕上等待选择 USB 模式。请开机，或在相机上选择“大容量存储 (MSC)”。
camera-choose-usb-mode = 相机正在等待选择 USB 模式。请在相机上选择“大容量存储 (MSC)”。
err-usb-too-many-devices = 连接了多台相机。请只保留您要操作的那一台。
err-usb-transfer = USB 传输失败。请更换数据线或接口，并避免使用 USB 集线器。
err-usb-permission = 程序无权通过 USB 打开相机。在 Linux 上，此权限通过一条针对索尼厂商 ID 的 udev 规则一次性授予；源码树中的 scripts/linux-prepare.sh 会安装该规则。之后请拔下相机并重新接入。
err-usb-busy = 另一个程序正在使用相机，因此无法打开。请关闭其他可能访问它的程序；在 Linux 上，桌面的媒体监视器可能占用它，scripts/linux-prepare.sh 会将其停止。在 macOS 上，请将相机的 USB 连接设为 MTP：在大容量存储模式下，系统自身的磁盘驱动会占用它。
err-usb-needs-root = macOS 只会把相机的大容量存储连接交给以管理员权限运行的程序。请用 sudo 启动命令行程序，例如：sudo pmca-console tweaks。若将相机的 USB 连接设为 MTP，不用 sudo 也能读取相机信息。
macos-admin-prompt = PMCA-RE RELOADED 需要管理员权限，才能从 macOS 接管相机的大容量存储连接。操作期间存储卡会从访达中消失；如果卡上有打开的文件，请先在访达中将其推出。
macos-admin-declined = 未授予管理员权限，因此未对相机做任何操作。在 macOS 上，此功能需要该权限。
macos-admin-failed = 程序无法以管理员权限启动其辅助程序。
err-usb-stall = 相机中止了 USB 传输。
err-usb-protocol = 相机返回了意外的响应。
err-usb-msc-status = 相机拒绝了一条存储命令。
cli-error-prefix = 错误
err-usb-driver = Windows 为此相机分配了不允许直接访问的驱动。此模式需要 WinUSB 驱动，本程序可以自行分配——但仅限于以管理员权限运行时。
err-needs-elevation = 分配或释放驱动程序需要管理员权限，而 Windows 的提示被拒绝。请通过“启动驱动助手”重试；相机上无需更改任何设置。
err-usb-no-mass-storage-volume = 相机已以大容量存储模式连接，但未提供任何驱动器。请唤醒相机，如屏幕上询问 USB 模式请予以确认，并检查是否已插入存储卡。请勿更换驱动：此模式正需要 Windows 的标准驱动。
err-usb-msc-sense = 相机报告了存储错误。
err-usb-ptp-response = 相机拒绝了该命令。
err-usb-unsupported = 此相机型号不支持该功能。
err-usb-updater-status = 相机上的固件更新器拒绝了该操作。
err-usb-senser = 服务模式命令失败。
err-market-core = 无法读取相机数据文件。
err-market-json = 接收到的数据格式有误。
err-market-missing-field = 响应中缺少预期的字段。
err-market-no-apk = 尚未选择要安装的应用。
err-market-http = 网络请求失败。
err-market-io = 本地安装服务器出错。
err-market-tls = 无法与相机建立加密连接。
err-apk = 无法读取应用文件。
err-io = 无法读取或写入文件。
err-json = 相机发送了格式错误的数据。
err-unknown-locale = 该语言不可用。
err-catalog-encoding = 翻译文件未以 UTF-8 文本保存。请使用 UTF-8 编码重新保存。
err-internal = 发生内部错误。这是应用程序的缺陷。

## Function analysis
nav-analysis = 分析
analysis-title = 功能分析
analysis-experimental = 实验性功能。此功能是新的，仅在少数相机上试用过。
analysis-own-hardware = 仅在您自己的相机上使用。服务模式本身就是 Sony 未预期的操作；责任由您承担。
analysis-explanation-toggle = 说明：用途、流程、风险
analysis-purpose-heading = 用途是什么？
analysis-purpose = 功能分析记录相机把自身的某项菜单设置保存在其设置存储器（备份）中的哪个值里。结果描述的是这台相机的设置存储器；可以与其他相机比较，但不会改变相机所提供的功能。
analysis-how-heading = 流程如何？
analysis-how = 对您输入的每个菜单值，程序都会完整读取一次设置存储器，每次都在单独的短暂服务模式会话中进行。两次读取之间，您拔下数据线并在相机菜单中更改设置；相机保持开机。最后再读取一次第一个值。跟随菜单变化且最后回到初始值的值就是候选值。会自行变化的值（计数器、时钟）会被排除。
analysis-requirements-heading = 前提条件
analysis-requirements = 相机通过 USB 连接，USB 模式设为“自动”，每次会话后在相机上选择“大容量存储 (MSC)”。在 Windows 上，程序需要管理员权限，因为它会为服务模式短暂分配驱动程序。请给电池充电；每次会话约需一分钟。
analysis-read-heading = 读取步骤
analysis-read = 快照只读取。相机的设置不会有任何改变。风险很低但不为零：切换到服务模式本身就是 Sony 未预期的操作。
analysis-write-heading = 写入步骤（写入测试）
analysis-write = 仅在您明确启用时：程序将找到的值写入相机，您在重启后于菜单中检查是否生效，之后再写回原始值。只会写入相机自己保存过的值，并且仅在恰好找到一个匹配候选值时进行。旧值会事先保存。尽管如此，此步骤仍会改变相机；故障可能使其处于只有维修中心才能修复的状态。
analysis-legal-heading = 法律信息
analysis-legal = 本软件不提供任何担保（MIT 许可证）。对相机的操作可能影响其保修。仅在属于您的相机上使用此功能。分析报告不包含序列号，也不包含设置存储器中除所找到候选值以外的任何内容；它绝不会自动发送。
analysis-start-heading = 新分析
analysis-setting-label = 菜单设置（按相机上的名称）
analysis-values-label = 菜单值，用逗号分隔
analysis-values-hint = 第一个值是当前设置的值。两到八个值；每个值需要一次服务模式会话。
analysis-write-option = 然后进行写入测试（会改变相机）
analysis-write-warning = 写入测试会改变相机：它向设置存储器写入一个值，之后再写回原始值。仅限您自己的相机，风险自负。
analysis-write-consent = 这台相机属于我，我承担风险。
btn-analysis-write-enable = 启用写入测试
btn-analysis-start = 开始分析
btn-analysis-snapshot = 读取备份
btn-analysis-new = 新分析
analysis-discard-confirm = 放弃正在进行的分析？到目前为止的报告仍会保留。
analysis-access-read = 读取
analysis-access-write = 写入
analysis-access-user = 手动
analysis-access-local = 电脑
analysis-step-snapshot = 读取备份，设置为“{ $value }”
analysis-step-change = 在相机菜单中切换到“{ $value }”
analysis-step-evaluate = 比较快照
analysis-step-write = 将“{ $value }”的值写入相机
analysis-step-check = 关闭并重新打开相机，查看菜单（预期：“{ $value }”）
analysis-step-restore = 写回“{ $value }”的原始值
analysis-step-report = 保存分析报告
analysis-do-prepare = 准备： 1. 在相机菜单中将“{ $setting }”设为“{ $value }”，然后按 MENU 按钮退出菜单，使相机保存该设置。 2. 保持相机开机。 3. 插上数据线，在相机上选择“大容量存储 (MSC)”。
analysis-do-change = 轮到您： 1. 拔下数据线。 2. 相机保持开机。 3. 在相机菜单中将“{ $setting }”设为“{ $value }”，然后按 MENU 按钮退出菜单，使相机保存该设置。 4. 重新插上数据线并选择“大容量存储 (MSC)”。
analysis-do-check = 轮到您： 1. 拔下数据线。 2. 关闭相机后重新打开。 3. 在菜单中查看“{ $setting }”显示的内容。
analysis-do-reconnect = 重新插上数据线，在相机上选择“大容量存储 (MSC)”。
analysis-reading = 快照 { $index } / { $total }：读取期间请勿拔下数据线。
analysis-cli-continue = 按 Enter 继续，按 q 取消：
analysis-confirm-word = 是
analysis-cli-confirm = 输入“{ $word }”以确认：
analysis-write-declined = 未确认。未写入任何内容。
analysis-aborted = 已取消。未写入任何内容。
analysis-snapshot-done = 快照已读取。相机已退出服务模式。
analysis-result = 已比较 { $compared } 个值：{ $count } 个候选值，{ $running } 个值自行发生了变化。
analysis-candidate-consistent = 与菜单明确对应
analysis-candidate-inconsistent = 不能明确对应
analysis-write-preview-heading = 将写入以下内容
analysis-write-preview = 值 { $id }（{ $size } 字节）：从 { $old }（“{ $old-label }”）改为 { $new }（“{ $new-label }”）。新值是相机在设为“{ $new-label }”时自己保存的。
btn-analysis-write = 写入
btn-analysis-skip-write = 不进行写入测试并结束
analysis-written = 值已写入、回读并永久保存。
analysis-menu-question = 菜单现在显示什么？
analysis-menu-other = 其他
btn-analysis-menu-confirm = 确定
btn-analysis-restore = 写回原始值
analysis-restored = 原始值已写回并永久保存。
analysis-survived = 重启后写入的值仍然保存着。
analysis-not-survived = 重启后写入的值已不再保存。
analysis-report-saved = 分析报告已保存：{ $path }
analysis-report-hint = 报告不包含序列号，也不包含备份。您可以将其转交他人，以便核实和记录这一发现。
btn-analysis-export = 导出报告…
file-filter-analysis = 分析报告
analysis-pending = 有一个写入测试未完成。原始值已保存，可以写回。
analysis-pending-none = 此相机没有保存未完成的写入测试。
analysis-invalid-setting = 请按相机上的名称输入菜单设置。
analysis-invalid-values = 请输入两到 { $max } 个不同的菜单值。第一个是当前设置的值。
analysis-already-complete = 所有快照都已读取。
analysis-other-camera = 这不是第一次快照时的同一台相机。分析已停止。
analysis-write-not-allowed = 当前状态下无法进行写入测试。
analysis-camera-changed = 相机现在的值与第一次快照时不同。未写入任何内容。
analysis-write-refused = 相机拒绝了该值；一切保持原样。({ $detail })
analysis-refuse-none = 无法进行写入测试：没有值跟随菜单变化。
analysis-refuse-ambiguous = 无法进行写入测试：有 { $count } 个值跟随菜单变化，无法确定是哪一个。
analysis-refuse-inconsistent = 无法进行写入测试：该值与菜单值不能明确对应。
analysis-refuse-size = 无法进行写入测试：该值为 { $size } 字节；只写入较小的值。
analysis-refuse-read-only = 无法进行写入测试：相机将该值标记为只读。
analysis-refuse-protected = 无法进行写入测试：相机将该值标记为受保护。
settings-analysis-heading = 功能分析
settings-analysis-purpose = 功能分析记录相机把自身的某项菜单设置保存在其设置存储器中的哪个值里。它面向希望了解自己机型的设置存储器并分享结果的用户。
settings-analysis-why-off = 它默认关闭，因为正常使用不需要它，它会多次把相机切换到服务模式，并且在要求时还会写入相机。它是实验性的，仅在少数型号上试用过。
settings-analysis-toggle = 启用功能分析（菜单中的“分析”项）
settings-analysis-confirm = 启用功能分析？它会多次把相机切换到服务模式，并可在要求时写入相机。仅在您自己的相机上使用。
btn-analysis-enable = 启用
settings-analysis-on = 功能分析已启用。
settings-analysis-off = 功能分析已关闭。
analysis-needs-admin = 每次快照都需要服务模式的管理员权限。Windows 每个程序会话只询问一次；您可以现在启动驱动助手，也可以等到第一次快照时再请求。
analysis-changed-hint = 请检查：未找到候选值。以下值发生了变化，但没有回到初始值。如果其中某个值只在两个值之间变化了一次，那么第一次快照时相机很可能不在第一个菜单值上。此时请正确设置初始值后重新分析。
analysis-changed-info = 信息：分析期间自行变化的值（例如计数器、时钟）会被忽略。您无需执行任何操作。
elevation-admin = 管理员
elevation-standard = 标准权限
elevation-admin-hint = 程序正以管理员权限运行，可以分配服务模式所需的驱动程序。
elevation-standard-hint = 程序以标准权限运行。用于服务模式、调整和功能分析时，它会在需要时启动具有管理员权限的驱动助手；Windows 每个会话只询问一次。单击即可启动。
driver-helper-declined = 管理员权限请求被拒绝。没有该权限就无法为服务模式分配驱动程序；未做任何更改。
btn-remove-foreign-drivers = 删除旧驱动程序
btn-not-now = 暂不
btn-ok = 确定
foreign-drivers-found = Windows 中仍保留着 Zadig 或类似工具为 Sony 设备安装的驱动程序：{ $packages }。在本程序分配自己的驱动程序之前，Windows 就会把相机交给这些驱动程序，因此维修模式会失败，相机也可能不显示为驱动器。现在删除吗？删除后相机将使用 Windows 自带的驱动程序。删除需要一次管理员权限。
foreign-drivers-removed = 旧驱动程序已删除。请拔下相机后重新连接。
foreign-drivers-remaining = 以下驱动程序无法删除：{ $packages }。请重新启动 Windows 后再试，或在设备管理器中连同驱动程序一起卸载该设备。
foreign-drivers-hint = 仍存在 Zadig 或类似工具为 Sony 设备安装的驱动程序（{ $packages }）。在本程序分配自己的驱动程序之前，Windows 就会把相机交给这些驱动程序，可能导致维修模式失败。删除命令：pmca-console driver zadig --remove
driver-zadig-none = 没有为 Sony 设备安装 Zadig 或类似工具的驱动程序。
driver-zadig-entry = { $package }（{ $provider }）：{ $devices }
driver-zadig-remove-hint = 删除命令：pmca-console driver zadig --remove
driver-zadig-removed = 已删除 { $count } 个旧驱动程序包。请拔下相机后重新连接。
elevation-helper = 驱动助手已启用
elevation-helper-hint = 一个具有管理员权限的助手为服务模式分配驱动程序。它没有窗口，并随程序结束；不会再有其他提示。
elevation-helper-confirm = 现在启动驱动助手？Windows 只询问一次管理员权限，在程序关闭前不会再询问。
analysis-cross-heading = 在另一台相机上核对
analysis-cross-intro = 可选：检查另一台相机是否也有找到的值，以及其中设置的是什么。只读取。相同编号不一定代表相同功能；只有查看另一台相机的菜单才能确认。结果会写入同一份分析报告。
analysis-cross-do = 1. 拔下已分析的相机。 2. 打开另一台相机，将其 USB 模式设为“自动”，插上数据线并选择“大容量存储 (MSC)”。
btn-analysis-cross = 检查另一台相机
analysis-cross-present = 存在，值 { $value }
analysis-cross-match = 对应“{ $label }”
analysis-cross-nomatch = 与测得的任何菜单值都不对应
analysis-cross-absent = 不存在
analysis-cross-same-camera = 这就是已分析的相机本身。请连接另一台相机。
analysis-cross-not-now = 只有在所有快照都已读取且没有待写回的写入值时才能核对。
analysis-cross-none = 没有可核对的候选值。
analysis-wrong-camera = 连接的不是已分析的相机。请连接 { $model }；未写入任何内容。
keys-fetching = 正在下载来源…
keys-scanning = 正在搜索密钥材料…
keys-complete = 所有密钥材料均已具备。
keys-incomplete = 仍缺少部分密钥材料。
keys-present = 已具备
keys-missing = 缺少
keys-class-senser = 服务模式密钥（服务终端、调整、分析）
keys-class-cic = XPD 校验和密钥（应用安装）
keys-class-spk = SPK 密钥材料（应用安装、apk2spk）
keys-class-fdat = FDAT 密钥材料（更新器 shell 固件）
plugin-heading = 插件
plugin-intro = 插件可以为本程序添加语言、相机设置开关、兼容性信息、工具和诊断。插件只来自您自己选择的文件，没有插件商店。插件代码在沙箱中运行，无法访问您的文件、网络或密钥材料。请只安装您信任的作者提供的插件；更改将在下次启动时生效。
plugin-none = 未安装插件。
plugin-inactive = （重启后生效）
plugin-author = 作者
plugin-restart = 请重新启动程序以使更改生效。
btn-plugin-install = 安装插件…
plugin-install-confirm = 此插件可以：
btn-confirm-plugin-install = 安装插件
plugin-installed = 插件已安装。
plugin-remove-confirm = 要移除插件“{ $name }”吗？
plugin-removed = 插件 { $id } 已移除。
plugin-perm-languages = 为程序添加语言
plugin-perm-camera-settings = 更改相机设置（仅限其指定的两个值之间，且须经您确认）
plugin-perm-compatibility = 向兼容性概览添加条目
plugin-perm-code = 在沙箱中运行自己的代码（工具或诊断）
plugin-perm-host-os = 查询当前操作系统
plugin-perm-host-cameras = 查看连接了哪些 Sony 相机及其模式
plugin-invalid = 这不是可用的插件：{ $reason }
plugin-no-folder = 无法确定用户数据文件夹，因此无法安装插件。
plugin-not-installed = 未安装 ID 为 { $id } 的插件。
plugin-failed = 插件运行失败：{ $reason }
plugin-unknown-entry = 没有已加载的插件提供 { $id }。
plugin-run-done = 插件已完成。
plugin-run-warning = 插件已完成，但有提示。
plugin-run-error = 插件报告了错误。
plugin-tools-heading = 插件工具
plugin-tools-hint = 这些来自已安装的插件，并在沙箱中运行。插件只能获得您交给它的内容以及它声明的内容，且无法更改相机。
plugin-diagnostics-heading = 插件诊断
plugin-compat-heading = 来自插件
plugin-col-note = 备注
file-filter-plugin = 插件文件
keys-not-set-up = 此操作所需的密钥材料尚未配置。
err-missing-keys = 此操作所需的密钥材料尚未配置。
keys-heading = 需要密钥材料
keys-intro = 服务模式密钥、XPD 校验和密钥和 SPK 密钥材料不包含在本程序中。请输入您拥有的来源地址——仓库、压缩包或文件——程序会找到并导入其识别出的部分。不会运行任何内容，材料仅保存在本计算机上。
keys-law-notice = 请遵守您所在地的法律，并仅对属于您自己的相机和数据进行操作。
keys-url-label = 来源地址（每行一个）
keys-url-required = 请至少输入一个来源地址。
btn-keys-import = 导入
btn-import-keys = 设置密钥材料
