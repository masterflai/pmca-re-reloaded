# Bản địa hóa tiếng Việt cho PMCA-RE RELOADED.
# Mỗi dòng một thông điệp: <id> = <văn bản>. Để thêm ngôn ngữ mới, hãy sao chép
# tệp này vào locales/<lang>/pmca.ftl và chỉ dịch phần bên phải.

app-title = PMCA-RE RELOADED – Bộ công cụ máy ảnh

# Điều hướng / thẻ
nav-camera-info = Thông tin máy ảnh
nav-install-app = Ứng dụng
nav-tweaks = Tinh chỉnh
nav-network = Mạng
nav-firmware = Phần sụn
nav-updater = Trình bao cập nhật
nav-service = Trình bao dịch vụ
nav-tools = Công cụ tệp
nav-settings = Cài đặt
nav-license = Giấy phép

# Trang chào mừng: trang hiển thị khi cửa sổ mở ra. Trang này cho biết bộ công cụ
# dùng để làm gì, cần làm gì trước tiên và trang nào trả lời câu hỏi nào.
#
# "Ngôn ngữ" có hai nghĩa khác nhau trong ứng dụng này: ngôn ngữ của cửa sổ
# này và các ngôn ngữ mà menu riêng của máy ảnh cung cấp. Mỗi nghĩa có một
# tiêu đề riêng thay vì chỉ một mệnh đề, vì nhầm lẫn giữa hai nghĩa này chính là
# lỗi mà trang này được tạo ra để ngăn chặn.
nav-welcome = Chào mừng
welcome-intro-heading = Chương trình này làm gì
welcome-intro-body = Chương trình này giúp duy trì các chức năng của máy ảnh Sony và điều chỉnh máy theo nhu cầu của bạn. Sau khi Sony đóng cửa hàng PlayMemories Camera Apps, không còn cách nào để sao lưu hoặc cài đặt lại các ứng dụng đã mua hoặc đã cài đặt. Việc đặt lại máy ảnh sẽ xóa vĩnh viễn các ứng dụng đã cài đặt. Chương trình này cho phép sao lưu các ứng dụng hiện có và cài đặt lại chúng, đồng thời có thể cung cấp các ứng dụng cộng đồng miễn phí. Bạn cũng có thể cài đặt ứng dụng của riêng mình (tệp *.apk). Khi có sự cố, chương trình sẽ đọc thông tin máy ảnh và nhật ký nội bộ để phân tích. Các cài đặt mà máy ảnh đã có nhưng không hiển thị trong menu có thể được chuyển đổi và hoàn tác lại. Chương trình giao tiếp trực tiếp với máy ảnh qua cáp USB. Các kiểu máy được hỗ trợ được liệt kê trên trang Thông tin máy ảnh.
welcome-start-heading = Bước đầu tiên
welcome-start-body = Trước tiên, hãy bật máy ảnh và, khi cáp vẫn chưa được cắm, đặt mục Kết nối USB trong menu của máy thành Lưu trữ dung lượng lớn hoặc MTP. Chế độ Tự động thường cũng hoạt động, vì khi đó máy ảnh sẽ tự chọn một trong hai chế độ này. Thứ tự rất quan trọng: khi cáp đã được cắm, bạn không thể truy cập menu của máy ảnh nữa. Chỉ sau đó mới kết nối máy ảnh qua USB, mở trang Thông tin máy ảnh và nhấn "Lấy thông tin máy ảnh". Khi kiểu máy và phiên bản phần sụn hiển thị, kết nối đã ổn định và mọi tính năng khác của chương trình này đều sẵn sàng.
welcome-start-driver = Nếu thay vào đó xuất hiện thông báo về trình điều khiển, hãy làm theo thông báo đó trước. Trên Windows, một trình điều khiển hệ thống chiếm giữ máy ảnh, và trình điều khiển này phải được thay thế thì bộ công cụ mới có thể truy cập USB trực tiếp như cần thiết. Chương trình tự làm việc đó, nhưng việc gán trình điều khiển là quyền mà Windows chỉ cấp cho quản trị viên. Trong trường hợp này, chương trình cho phép bạn khởi động lại nó với quyền quản trị viên, và Windows sẽ yêu cầu bạn xác nhận thao tác. Việc gán trình điều khiển sẽ được gỡ bỏ khi phiên làm việc kết thúc. Điều này không cần thiết trên Linux và macOS.
welcome-start-driver-linux = Nếu thay vào đó xuất hiện thông báo về trình điều khiển, hãy làm theo thông báo đó trước. Trên Linux không cần thay thế gì: máy ảnh được truy cập trực tiếp. Điều cần làm một lần là cấp quyền mở thiết bị — một quy tắc udev cho mã nhà cung cấp của Sony — và, đối với chế độ lưu trữ dung lượng lớn, giải phóng trình điều khiển nhân usb-storage. Cả hai việc được chuẩn bị bên ngoài chương trình này với quyền root; tập lệnh scripts/linux-prepare.sh trong cây mã nguồn thực hiện cả hai. Bản thân cửa sổ này không bao giờ yêu cầu các quyền như vậy.
welcome-start-driver-macos = Nếu thay vào đó xuất hiện thông báo về trình điều khiển, hãy làm theo thông báo đó trước. Trên macOS không cần cài đặt hay thay thế gì. Hãy đặt kết nối USB của máy ảnh thành MTP. Đối với các tinh chỉnh, trình bao dịch vụ, phân tích chức năng, cập nhật phần sụn và trình bao cập nhật, cửa sổ này sẽ hỏi mật khẩu quản trị viên của bạn một lần: các chức năng này đi qua kết nối lưu trữ dung lượng lớn của máy ảnh, mà macOS chỉ giao cho chương trình có quyền quản trị viên.
welcome-tweaks-heading = Tinh chỉnh và ngôn ngữ
welcome-tweaks-body = Tinh chỉnh là những cài đặt mà máy ảnh đã có nhưng không hiển thị trong menu: giới hạn thời gian quay, giới hạn quay 4K, lựa chọn giữa PAL và NTSC, trình cài đặt ứng dụng qua USB — và các ngôn ngữ menu.
welcome-tweaks-languages-heading = Ngôn ngữ của máy ảnh
welcome-tweaks-languages-body = Máy ảnh Sony chỉ hiển thị các ngôn ngữ menu phổ biến tại khu vực bán hàng của chúng. Nếu bạn sống ở nước ngoài, bạn thường không tìm thấy ngôn ngữ của mình và phải dùng tạm một ngôn ngữ xa lạ. Tuy nhiên, phần sụn đã tích hợp sẵn nhiều ngôn ngữ hơn. Chương trình này đưa chúng vào menu để bạn có thể sử dụng máy ảnh bằng ngôn ngữ quen thuộc.
welcome-tweaks-ui-heading = Ngôn ngữ của chương trình này
welcome-tweaks-ui-body = Tách biệt với tất cả những điều trên là bộ chọn ở phía trên cùng. Bộ chọn này chỉ thay đổi ngôn ngữ của chương trình này và không bao giờ tác động đến máy ảnh.
welcome-tweaks-safety = Mỗi tinh chỉnh được ghi vào bộ nhớ sao lưu của máy ảnh, và mỗi tinh chỉnh đều có thể được hoàn tác bằng chính công tắc đó. Việc ghi bị gián đoạn có thể làm hỏng máy ảnh vĩnh viễn, vì vậy đừng rút cáp hoặc tháo pin trong khi đang ghi.
welcome-privacy-heading = Dữ liệu của bạn
welcome-privacy-where = Trang Giấy phép trình bày lại đầy đủ tất cả những điều này, cùng với giấy phép của mọi thư viện được dùng để xây dựng chương trình này.
welcome-where-heading = Tìm gì ở đâu
welcome-where-info = Kiểu máy, số sê-ri và phiên bản phần sụn. Đây cũng là bước kiểm tra kết nối.
welcome-where-install = Ứng dụng từ danh sách cộng đồng hoặc tệp .apk của riêng bạn.
welcome-where-tweaks = Các cài đặt mà menu của máy ảnh không truy cập được, trong đó có các ngôn ngữ.
welcome-where-network = Dữ liệu hỗ trợ GPS, cùng các điểm truy cập và dịch vụ dùng để phát trực tiếp.
welcome-where-firmware = Ghi một bản phần sụn Sony vào máy ảnh.
welcome-where-tools = Giải nén và kiểm tra các tệp phần sụn và tệp gói. Không cần máy ảnh.
welcome-where-shells = Truy cập trực tiếp vào bộ nhớ và hệ thống tệp của máy ảnh. Cần bật trước trong Cài đặt.
welcome-where-settings = Chủ đề, chế độ nâng cao và chỉnh sửa bản dịch của chương trình này.
welcome-where-license = Bản chuyển đổi này bắt nguồn từ đâu, dựa trên công sức của ai và tuân theo những điều khoản nào.
welcome-where-hint = Không trang nào tự tác động đến máy ảnh. Không có gì được đọc hoặc ghi cho đến khi bạn nhấn một nút ghi rõ sẽ làm việc đó.

# Nút
btn-get-info = Lấy thông tin máy ảnh
btn-refresh = Làm mới
btn-load-apps = Tải danh sách ứng dụng có sẵn
btn-list-tweaks = Hiển thị các tinh chỉnh có sẵn
btn-install = Cài đặt ứng dụng đã chọn
btn-install-file = Cài đặt tệp .apk…
btn-apply = Áp dụng
btn-save = Lưu vào máy ảnh
btn-add = Thêm
btn-remove = Xóa
btn-reveal = Hiện
btn-update-gps = Truyền dữ liệu hỗ trợ GPS sang thiết bị
btn-cancel = Hủy
btn-browse = Duyệt…
btn-switch-msc = Chuyển sang chế độ lưu trữ dung lượng lớn
btn-start-driver-helper = Khởi động trình hỗ trợ trình điều khiển

# Nhãn cho các bộ lọc loại tệp trong hộp thoại tệp
file-filter-apk = Ứng dụng Android (*.apk)
file-filter-firmware = Bản phần sụn Sony (*.dat)
file-filter-usage-log = Nhật ký sử dụng (*.bin)
file-filter-catalog = Tệp bản dịch (*.ftl)
file-filter-spk = Gói ứng dụng Sony (*.spk)
file-filter-any = Mọi tệp
file-filter-gps = Dữ liệu hỗ trợ GPS (*.dat)

# Trạng thái / thông báo
msg-connecting = Đang kết nối…
msg-usage-log-written = Đã ghi nhật ký sử dụng vào { $path }
msg-apps-found = Đã tìm thấy { $count } ứng dụng
msg-switch-mode = Vui lòng chuyển máy ảnh sang chế độ MTP hoặc lưu trữ dung lượng lớn.
msg-no-tweaks = Không có tinh chỉnh nào cho máy ảnh này.
msg-done = Xong.

# Chuyển chế độ USB của máy ảnh (MTP -> lưu trữ dung lượng lớn). Được đề xuất bên
# dưới lỗi "sai chế độ"; không có lệnh USB nào cho chiều ngược lại.
mode-switching = Đang yêu cầu máy ảnh chuyển sang chế độ lưu trữ dung lượng lớn…
mode-switch-wait = Đang chờ máy ảnh kết nối lại…
camera-confirm-hint = Có thể bỏ qua các thông báo trên máy ảnh (ví dụ về www.sony.net/pm); chúng không gây cản trở.
mode-switch-done = Máy ảnh hiện đang ở chế độ lưu trữ dung lượng lớn.
mode-switch-timeout = Máy ảnh không xuất hiện lại ở chế độ lưu trữ dung lượng lớn. Vui lòng tắt rồi bật lại máy ảnh và thử lại.
mode-switch-impossible = Không thể thay đổi chế độ này qua USB. Vui lòng tắt rồi bật lại máy ảnh.
mode-switch-hint = Chỉ máy ảnh ở chế độ MTP mới có thể được chuyển qua USB. Thay đổi này chỉ là tạm thời: chế độ USB được chọn trong menu của máy ảnh vẫn giữ nguyên, vì vậy tắt rồi bật lại máy ảnh — hoặc rút cáp ra rồi cắm lại — sẽ khôi phục chế độ trước đó. Không có lệnh USB nào để chuyển ngược lại.

# Các trường thông tin máy ảnh
info-model = Kiểu máy
info-product-code = Mã sản phẩm
info-serial = Số sê-ri
info-firmware = Phiên bản phần sụn
info-lens = Ống kính
info-region = Khu vực
info-mode = Chế độ
mode-mass-storage = Lưu trữ dung lượng lớn
mode-mtp = MTP
mode-updater = Chế độ cập nhật
mode-app-install = Chế độ cài đặt ứng dụng
mode-service = Chế độ dịch vụ
mode-unknown = Không xác định
info-android-version = Phiên bản nền tảng Android
info-photographer = Nhiếp ảnh gia
info-copyright = Bản quyền
info-photographer-1 = Nhiếp ảnh gia (mục 1)
info-photographer-2 = Nhiếp ảnh gia (mục 2)
info-copyright-1 = Bản quyền (mục 1)
info-copyright-2 = Bản quyền (mục 2)
diagnose-services-heading = Không kết nối được với máy ảnh?
diagnose-services-hint = Một số dịch vụ Windows có thể chiếm giữ máy ảnh ở chế độ MTP, khiến ứng dụng này không truy cập được. Bước kiểm tra chỉ hỏi Windows những dịch vụ nào đang chạy; nó không thay đổi gì.
btn-diagnose-services = Kiểm tra dịch vụ Windows
diagnose-services-none = Không có dịch vụ gây cản trở nào đã biết đang chạy.
diagnose-services-found = Các dịch vụ đang chạy sau đây có thể cản trở việc truy cập máy ảnh:
diagnose-services-advice = Hãy dừng dịch vụ trong Dịch vụ (services.msc) hoặc bằng lệnh sau với quyền quản trị viên, rồi kết nối lại máy ảnh:

# Trình cập nhật phần sụn / trình bao
updater-model = Kiểu máy ảnh
updater-firmware-current = Phiên bản phần sụn hiện tại
updater-guard-ok = Bản phần sụn khớp với máy ảnh này.
updater-no-fdat = Không có phần sụn cập nhật đi kèm cho kiểu máy { $model }.
updater-model-required = Máy ảnh đã ở chế độ cập nhật và không thể báo kiểu máy ở chế độ đó. Hãy tắt rồi bật lại máy ảnh để máy khởi động ở chế độ bình thường, sau đó thử lại. Ngoài ra, bạn có thể chỉ định tệp tải trọng (--fdat-file) hoặc, trong dòng lệnh, chỉ định kiểu máy (--model <MODEL>).
updater-already-updater = Máy ảnh đã ở chế độ cập nhật.
updater-flash-required = Từ chối sửa đổi máy ảnh khi chưa có xác nhận. Hãy chạy lại với --flash để chuyển sang chế độ cập nhật và ghi tải trọng.
updater-flashing = Đang ghi tải trọng cập nhật vào máy ảnh…
updater-switch-wait = Vui lòng làm theo hướng dẫn trên màn hình máy ảnh. Đang chờ máy ảnh chuyển sang chế độ cập nhật…
updater-switch-timeout = Đã hết thời gian chờ máy ảnh. Vui lòng chạy lại lệnh khi máy ảnh đã kết nối lại.
updater-shell-ready = Tải trọng cập nhật đang chạy. Gõ 'help' để xem các lệnh, 'exit' để thoát.
updater-shell-done = Phiên trình bao đã kết thúc.
updater-session-panic = Phiên cập nhật đã kết thúc bất ngờ. Hãy tắt rồi bật lại máy ảnh trước khi thử lại.

# Những gì trình bao cập nhật trả lời. Các lỗi phân tích cú pháp chỉ được diễn đạt
# một lần, trong các khóa service-*: cả hai trình bao dùng chung một bộ tách từ tố, nên
# một đường dẫn trong dấu ngoặc kép và một giá trị thập lục phân sai đều lỗi như nhau ở cả hai.
updater-backup-synced = Đã đồng bộ bản sao lưu vào bộ nhớ flash.
updater-backup-locked = Các cài đặt sao lưu được bảo vệ hiện đã bị khóa.
updater-backup-writable = Các cài đặt sao lưu được bảo vệ hiện có thể ghi.
updater-shell-console-only = Trình bao root cần một cửa sổ dòng lệnh và chỉ có trong chương trình dòng lệnh: hãy chạy pmca-console updatershell --flash và gõ 'shell' tại đó.

# Trang trình bao cập nhật.
updater-warning = Cảnh báo: để vào trình bao này, máy ảnh sẽ khởi động lại vào chế độ cập nhật và một tải trọng sẽ được ghi vào máy — một chương trình nhỏ của dự án này mà máy ảnh chấp nhận như một bản cập nhật phần sụn và sau đó trả lời các lệnh của trình bao này. Bản thân tải trọng sẽ bị loại bỏ khi tắt máy ảnh, nhưng các lệnh mà nó cung cấp sẽ ghi vào bộ nhớ cài đặt lâu dài của máy ảnh, và việc ghi bị gián đoạn có thể làm hỏng máy ảnh vĩnh viễn. Ngoài ra, toàn bộ cài đặt của máy ảnh sẽ bị đặt lại trong quá trình này — khác với trình bao dịch vụ.
updater-console-heading = Chạy trình bao cập nhật trong dòng lệnh
updater-driver-hint = Trình bao cập nhật cần máy ảnh dùng trình điều khiển lưu trữ dung lượng lớn của chính Windows, USBSTOR, vì tải trọng được truyền qua trình điều khiển đó. Đây là điều ngược lại với những gì trình bao dịch vụ cần. Ứng dụng tự gán và gỡ trình điều khiển WinUSB, nên không cần chuẩn bị gì thủ công — nhưng trình điều khiển do một công cụ khác gán phải được gỡ bỏ trước. Máy ảnh ở chế độ cập nhật được nhận dạng với mã sản phẩm riêng và cũng không được gán lại trình điều khiển.
updater-fdat-label = Tệp tải trọng (tùy chọn)
updater-confirm = Máy ảnh sẽ khởi động lại vào chế độ cập nhật và một tải trọng sẽ được ghi vào máy. Tiếp tục?
updater-cancelled = Đã hủy. Máy ảnh không bị sửa đổi.

# Phần giải thích thu gọn hiển thị trên cả hai trang trình bao. Bản thân cú pháp
# lệnh không bao giờ được dịch — đó là những từ phải được gõ chính xác —
# nên chỉ phần giải thích bên cạnh mỗi lệnh nằm ở đây.
btn-shell-primer = Về trình bao này
btn-shell-primer-hide = Ẩn phần giải thích
shell-primer-compare-heading = Hai trình bao, hai giao thức
shell-primer-compare = Cả hai trang đều cung cấp dòng lệnh đến máy ảnh, nhưng đó không phải cùng một trình bao. Trình bao dịch vụ dùng giao thức dịch vụ nhà máy riêng của Sony, mà máy ảnh tự trả lời sau khi đã xác thực bạn; không có gì được ghi vào máy ảnh để đạt được điều đó. Trình bao cập nhật tồn tại chỉ vì dự án này ghi một chương trình riêng vào máy ảnh: máy ảnh khởi động lại vào chế độ cập nhật, chấp nhận tải trọng như thể đó là một bản phần sụn, và sau đó tải trọng trả lời các lệnh được liệt kê bên dưới. Mỗi lúc chỉ có thể mở một trong hai trình bao.
shell-primer-payload = "Tải trọng" (payload) là nội dung thực sự mà một đường truyền mang theo. Đối với trình bao cập nhật, đó là một chương trình nhỏ của dự án này: máy ảnh chấp nhận nó theo cùng cách chấp nhận một bản cập nhật phần sụn, nhưng chỉ giữ nó cho đến khi tắt máy. Trình bao dịch vụ không cần tải trọng, vì máy ảnh trả lời các lệnh của nó bằng phần sụn của chính mình. Đối với tệp phần sụn, thuật ngữ này có cùng ý nghĩa: bản phần sụn thực sự nằm bên trong vỏ chứa của tệp .dat.
shell-primer-when-service = Hãy dùng trình bao dịch vụ cho mọi việc đọc hoặc chỉnh sửa các cài đặt đã lưu của máy ảnh trong khi máy vẫn chạy phần sụn của chính nó — đọc khối HASP, lưu một bản sao khôi phục của vùng sao lưu, thay đổi từng mục, sao chép tệp ra khỏi máy ảnh. Đây cũng là thứ mà trang Tinh chỉnh sử dụng bên dưới, nên mọi việc bạn làm được ở đó đều có thể làm thủ công ở đây.
shell-primer-when-updater = Hãy dùng trình bao cập nhật khi không thể kết nối máy ảnh bằng cách nào khác, hoặc khi bạn cần một trình bao trên hệ thống đang chạy thay vì từng lệnh riêng lẻ. Đây là cách duy nhất cung cấp trình bao root thực sự, và nó vẫn hoạt động trên máy ảnh từ chối chế độ dịch vụ. Cách này phải trả giá bằng một lần khởi động lại, một lần ghi và các cài đặt của máy ảnh, nên đây là lựa chọn thứ hai chứ không phải thứ nhất.
shell-primer-driver-heading = Trình điều khiển: hai trình bao cần những điều ngược nhau
shell-primer-driver = Trình bao dịch vụ giao tiếp USB thô và cần máy ảnh được gán với WinUSB trên Windows. Ứng dụng tự gán trình điều khiển đó khi chuyển vào chế độ dịch vụ và gỡ nó khi phiên kết thúc, nên không cần công cụ riêng nào. Trình bao cập nhật được truyền qua trình điều khiển lưu trữ dung lượng lớn của chính Windows, USBSTOR, và ngừng hoạt động khi máy ảnh được gán với WinUSB — vì vậy việc gỡ gán là một phần của việc kết thúc phiên dịch vụ, không phải việc làm thêm sau đó. Việc gán trình điều khiển cần quyền quản trị viên. Trên Linux, cả hai đều không cần đổi trình điều khiển; chỉ cần quyền truy cập vào nút thiết bị.
shell-primer-risk-heading = Những gì có thể gặp sự cố
shell-primer-risk = Các lệnh được đánh dấu bên dưới ghi vào bộ nhớ cài đặt lâu dài của máy ảnh. Việc ghi bị gián đoạn có thể khiến máy ảnh không khởi động được, và không thao tác nào ở đây có thể hoàn tác — đó chính là lý do bk d tồn tại: hãy lưu một bản sao của vùng sao lưu trước khi bạn thay đổi bất cứ điều gì. Bản thân tải trọng cập nhật không lưu lại trên máy ảnh; tắt máy ảnh sẽ loại bỏ nó. Tuy nhiên, con đường qua chế độ cập nhật sẽ đặt lại toàn bộ cài đặt của máy ảnh.
shell-primer-commands-heading = Các lệnh trên trình bao này
shell-primer-writes = Các lệnh được đánh dấu (GHI) sẽ thay đổi máy ảnh. Mọi lệnh khác chỉ đọc.
shell-cmd-hasp = Đọc khối HASP, vùng nhận dạng được bảo vệ của máy ảnh.
shell-cmd-pull = Sao chép một tệp ra khỏi máy ảnh. Nếu không có tên cục bộ, tên tệp trên máy ảnh được giữ nguyên.
shell-cmd-push = Sao chép một tệp cục bộ vào máy ảnh, thay thế mọi thứ đang có ở đó. (GHI)
shell-cmd-rm = Xóa một tệp trên máy ảnh. (GHI)
shell-cmd-rmem = Ghi một vùng bộ nhớ thiết bị ra tệp cục bộ. Độ lệch và độ dài ở dạng thập lục phân.
shell-cmd-bk-r = Đọc một mục của vùng sao lưu và hiển thị dưới dạng bản kết xuất hex.
shell-cmd-bk-d = Lưu toàn bộ vùng sao lưu vào tệp. Hãy làm việc này trước khi thay đổi bất cứ điều gì: đó là cách duy nhất để quay lại.
shell-cmd-bk-dump-txt = Lưu toàn bộ vùng sao lưu dưới dạng <file>.bin và một bản liệt kê dễ đọc dưới dạng <file>.txt.
shell-cmd-bk-stat = Hiển thị khối trạng thái sao lưu của máy ảnh, bao gồm cả khu vực.
shell-cmd-bk-w = Ghi một mục của vùng sao lưu, được nhập dưới dạng các byte hex cách nhau bằng dấu cách. (GHI)
shell-cmd-bk-s-sub = Lưu vĩnh viễn vùng sao lưu của một hệ thống con vào bộ nhớ flash để thay đổi vẫn còn sau khi khởi động lại. (GHI)
shell-cmd-bk-s = Lưu vĩnh viễn vùng sao lưu vào bộ nhớ flash để thay đổi vẫn còn sau khi khởi động lại. (GHI)
shell-cmd-bk-lock = Bật hoặc tắt chống ghi cho các mục sao lưu được bảo vệ; một số cài đặt không thể thay đổi khi chống ghi đang bật. (GHI)
shell-cmd-term = Bật hoặc tắt thiết bị đầu cuối gỡ lỗi của máy ảnh. (GHI)
shell-cmd-info = Đọc khối trạng thái sao lưu, cho biết vùng này có thể ghi được hay không.
shell-cmd-shell = Mở trình bao root trên máy ảnh. Lệnh này cần cửa sổ dòng lệnh, nên chỉ có trong chương trình dòng lệnh; thay vào đó, shell tcp <port> cung cấp trình bao trên 127.0.0.1 cho một máy khách khác. (GHI)
shell-cmd-help = Liệt kê các lệnh mà trình bao này chấp nhận.
shell-cmd-exit = Kết thúc phiên và rời khỏi trình bao.
shell-primer-diagnostics-heading = Chẩn đoán và sửa chữa
shell-cmd-bk-trace = Ghi lại vào tệp, theo từng gói, cách máy ảnh gửi toàn bộ vùng sao lưu qua USB. Dùng để điều tra các bản sao lưu lớn; máy ảnh có thể bị treo giữa chừng khi đang trả lời. Khi đó hãy rút cáp và tắt rồi bật lại máy ảnh, tháo pin nếu cần.
shell-cmd-bk-rewrite = Đọc vùng sao lưu, ghi lại y nguyên và đọc lại lần nữa. Cho biết máy ảnh có chấp nhận toàn bộ ảnh sao lưu hay không, mà không thay đổi cài đặt nào. (GHI)
shell-cmd-bk-restore-attrs = Đặt thuộc tính của mọi mục sao lưu, chẳng hạn như chống ghi, trở lại như trong bản sao đã lưu bằng bk d hoặc bk dump-txt. Các giá trị được giữ nguyên. Chỉ dùng bản sao từ cùng một máy ảnh. (GHI)
shell-primer-diagnostics-env = Ba biến môi trường ảnh hưởng đến cả chương trình dòng lệnh lẫn giao diện đồ họa khi được đặt trước khi khởi động, ví dụ trong PowerShell bằng $env:PMCA_TWEAKS_DIRECT = "1". PMCA_TWEAKS_DIRECT khiến các tinh chỉnh ghi từng giá trị riêng lẻ ngay cả khi bản sao lưu nhỏ hơn 1 MB. PMCA_TWEAKS_IMAGE khiến chúng ghi toàn bộ ảnh sao lưu ngay cả khi lớn hơn 1 MB; khi đặt cả hai, PMCA_TWEAKS_DIRECT được ưu tiên. PMCA_TLS_DUMP=<file> lưu thông điệp TLS đầu tiên của máy ảnh (ClientHello) trong khi cài đặt ứng dụng, để điều tra kết nối thất bại.

# Cài đặt ứng dụng
install-switching = Đang chuyển sang chế độ cài đặt ứng dụng…
install-disabled = Máy ảnh từ chối vào chế độ cài đặt ứng dụng. Nguyên nhân thường là cài đặt "Bật trình cài đặt ứng dụng qua USB": cài đặt này phải được bật trong "Tinh chỉnh".
install-stays-app-mode = Máy ảnh vẫn ở chế độ cài đặt ứng dụng, nơi có thể cài đặt thêm ứng dụng. Đối với mọi chức năng khác, hãy rút cáp USB rồi tắt và bật lại máy ảnh.
err-usb-app-install-mode = Máy ảnh vẫn đang ở chế độ cài đặt ứng dụng, chế độ chỉ chấp nhận việc cài đặt ứng dụng. Hãy rút cáp USB, sau đó tắt rồi bật lại máy ảnh — khởi động lại khi vẫn còn kết nối sẽ đưa máy trở lại ngay chế độ này.
install-switch-wait = Đang chờ máy ảnh kết nối lại ở chế độ cài đặt ứng dụng…
install-switch-timeout = Đã hết thời gian chờ máy ảnh vào chế độ cài đặt ứng dụng. Vui lòng chạy lại lệnh khi máy ảnh đã kết nối lại.
install-bad-mode = Không thể sử dụng máy ảnh ở chế độ này. Vui lòng chuyển máy sang chế độ MTP hoặc lưu trữ dung lượng lớn.
install-starting = Đang bắt đầu cài đặt…
install-error = Lỗi giao tiếp khi cài đặt
install-server-failed = Máy chủ cài đặt cục bộ không chấp nhận kết nối của máy ảnh:
install-done = Tác vụ đã hoàn tất thành công.

# Kiểm tra APK
apk-package = Gói:
apk-unreadable = Cảnh báo: không thể phân tích tệp .apk.
apk-incompatible = Cảnh báo: ứng dụng này khai báo mức SDK tối thiểu mà máy ảnh có thể không đáp ứng.
apk-unsigned = Cảnh báo: tệp .apk không có chữ ký.

# Cửa hàng ứng dụng cộng đồng
store-loading = Đang tải danh sách ứng dụng…
store-resolving = Đang tìm bản phát hành mới nhất…
store-downloading = Đang tải xuống
store-unknown-app = Ứng dụng không xác định
store-no-release = Không có bản phát hành nào có thể tải xuống cho ứng dụng này
store-not-installable = không có bản phát hành
msg-no-apps = Không tìm thấy ứng dụng nào.
store-heading = Cửa hàng ứng dụng cộng đồng
store-discontinued = Sony đã đóng cửa hàng PlayMemories Camera Apps chính thức; máy chủ của cửa hàng không còn phản hồi, nên không thể mua hoặc tải xuống ứng dụng từ đó nữa.
store-use-own-apk = Bạn vẫn có thể cài đặt tệp .apk của riêng mình: chọn tệp bên dưới hoặc kéo tệp vào cửa sổ này. Cửa hàng ứng dụng cộng đồng ở phía dưới cũng vẫn hoạt động.
btn-store-sync = Kiểm tra máy ảnh
btn-install-selected = Cài đặt mục đã chọn
btn-store-stop = Dừng sau ứng dụng này
btn-store-stopping = Sẽ dừng sau ứng dụng này…
btn-store-retry = Thử lại các ứng dụng bị lỗi
btn-continue = Tiếp tục
btn-reinstall = Cài đặt lại
btn-skip = Bỏ qua
btn-confirm-updater = Khởi động lại và ghi
btn-confirm-tweaks = Ghi tinh chỉnh
btn-confirm-languages = Bật ngôn ngữ
btn-confirm-wifi = Thay thế điểm truy cập
btn-confirm-stream = Ghi đè cài đặt
btn-confirm-show-shells = Hiển thị trình bao
btn-confirm-discard = Hủy bỏ bản dịch
store-select-all = Chọn tất cả ứng dụng
store-col-app = Ứng dụng
store-col-version = Phiên bản
store-col-author = Tác giả
store-col-camera = Phiên bản trên máy ảnh
store-col-status = Trạng thái
store-camera-absent = chưa cài đặt
store-version-unknown = không xác định
store-batch-query = Đang đọc các ứng dụng đã cài đặt từ máy ảnh…
store-sync-done = Đã kiểm tra máy ảnh. Số ứng dụng trong danh sách đã được cài đặt: { $count }
store-batch-downloading = Đang tải xuống ứng dụng { $current } trên { $total }…
store-batch-installing = Ứng dụng { $current } trên { $total }: đang cài đặt { $name }…
store-reinstall-confirm = { $name } đã được cài đặt trên máy ảnh. Phiên bản đã cài đặt: { $installed }. Phiên bản trong cửa hàng ứng dụng: { $store }. Cài đặt lại?
store-state-queued = đang chờ
store-state-installed = đã cài đặt
store-state-failed = thất bại
store-state-skipped = đã bỏ qua
store-state-not-run = chưa chạy
store-batch-summary = Đã cài đặt: { $done } · thất bại: { $failed } · đã bỏ qua: { $skipped } · chưa chạy: { $notrun }
store-sources-hint = Các ứng dụng đến từ những nguồn bạn đã thêm trong Cài đặt. Chương trình này không kiểm tra chúng.
app-backup-heading = Sao lưu ứng dụng đã cài đặt
app-backup-intro = Sao chép các ứng dụng đã cài đặt trên máy ảnh sang máy tính này để bạn có thể cài đặt lại sau này, kể cả khi không còn nơi nào cung cấp chúng nữa. Để làm việc này, máy ảnh khởi động lại vào chế độ cập nhật; bộ nhớ ứng dụng của máy chỉ được đọc.
app-backup-own-camera = Chỉ sao lưu ứng dụng của chính máy ảnh của bạn và không chuyển các tệp cho người khác: quyền đối với một ứng dụng thuộc về tác giả của nó.
app-backup-restore-hint = Để khôi phục một ứng dụng, hãy chọn tệp .apk đã lưu ở mục “Cài đặt tệp .apk…” phía trên.
btn-app-backup = Sao lưu ứng dụng
app-backup-confirm = Máy ảnh sẽ khởi động lại vào chế độ cập nhật và một tải trọng chạy cho đến khi tắt máy ảnh sẽ được ghi vào máy. Bộ nhớ ứng dụng chỉ được đọc. Tiếp tục?
btn-confirm-app-backup = Khởi động lại và sao lưu
app-backup-cancelled = Đã hủy sao lưu.
app-backup-confirm-required = Quá trình sao lưu sẽ khởi động lại máy ảnh vào chế độ cập nhật. Hãy xác nhận bằng --confirm.
app-backup-no-folder = Máy tính này không có thư mục dữ liệu cho các bản sao lưu. Hãy chọn một thư mục bằng --out.
app-backup-no-model = Không thể đọc kiểu máy ảnh. Hãy tắt rồi bật lại máy ảnh, kết nối ở chế độ lưu trữ dung lượng lớn hoặc MTP rồi thử lại.
app-backup-mounting = Đang tìm bộ nhớ ứng dụng của máy ảnh…
app-backup-no-partition = Không tìm thấy bộ nhớ ứng dụng trên máy ảnh này. Có lẽ máy không hỗ trợ ứng dụng.
app-backup-copying = Đang sao chép { $name } ({ $current }/{ $total })…
app-backup-none = Không có ứng dụng nào được cài đặt trên máy ảnh.
app-backup-done = Đã lưu { $count } ứng dụng.
app-backup-power-cycle = Hãy tắt rồi bật lại máy ảnh để thoát khỏi chế độ cập nhật.
app-backup-folder = Thư mục
app-backup-col-file = Tệp
app-backup-state-new = đã lưu
app-backup-state-kept = đã lưu trước đó
btn-copy-path = Sao chép đường dẫn
msg-path-copied = Đã sao chép đường dẫn vào bảng tạm.
store-no-sources = Chưa thêm nguồn nào. Hãy thêm một nguồn trong Cài đặt → Nguồn ứng dụng và plugin; danh sách ứng dụng của dự án gốc được đề xuất sẵn ở đó.
store-col-source = Nguồn
sources-heading = Nguồn ứng dụng và plugin
sources-intro = Ứng dụng và plugin đến từ các nguồn do chính bạn thêm vào đây: các kho mã GitHub công khai, nơi các nhà phát triển công bố sản phẩm của họ. Chương trình này không lưu trữ, không kiểm tra và không khuyến nghị bất kỳ nguồn nào hay bất cứ thứ gì trong đó; chỉ tác giả của một nguồn chịu trách nhiệm về nguồn đó. Hãy xem kỹ một kho mã trước khi thêm nó.
sources-none = Chưa thêm nguồn nào.
sources-add-label = Kho mã GitHub (chủ sở hữu/tên hoặc địa chỉ của kho mã)
btn-source-add = Thêm nguồn
btn-confirm-source-add = Thêm nguồn
sources-add-confirm = Thêm { $source } làm nguồn? Chương trình này chưa kiểm tra nguồn này. Những gì nguồn này cung cấp sẽ được cài đặt trên trách nhiệm của tác giả. Hãy xem kho mã trước khi đồng ý: { $url }
sources-adding = Đang đọc nguồn…
sources-added = Đã thêm nguồn: { $source }
sources-check-repo = Hãy xem kho mã trước khi cài đặt bất cứ thứ gì từ đó: { $url }
sources-removed = Đã xóa nguồn: { $source }
sources-remove-confirm = Xóa nguồn { $source }? Những người ký đã được ghi nhớ cho các ứng dụng của nguồn này cũng sẽ bị xóa theo.
sources-stars-title = Số sao trên GitHub: số người dùng đã gắn sao cho kho mã. Con số này không nói lên điều gì về chất lượng hay độ an toàn.
sources-suggestion-heading = Đề xuất
sources-suggestion-applist = Danh sách ứng dụng của dự án gốc Sony-PMCA-RE. Danh sách này do tác giả ma1co duy trì, không phải do chương trình này, và đã nhiều năm không được cập nhật.
btn-source-activate = Kích hoạt
sources-search-heading = Tìm nguồn trên GitHub
sources-search-hint = Tìm trên GitHub các kho mã có chủ đề pmca-source. Kết quả không được chương trình này kiểm tra và không phải là khuyến nghị. Hãy xem kỹ một kho mã trước khi thêm nó; số sao chỉ cho biết có bao nhiêu người dùng GitHub đã gắn sao cho kho mã đó.
btn-source-search = Tìm trên GitHub
sources-searching = Đang tìm trên GitHub…
sources-search-none = GitHub không tìm thấy kho mã nào có chủ đề pmca-source.
sources-search-found = Đã tìm thấy { $count } kho mã. Chưa kho mã nào trong số đó được chương trình này kiểm tra.
sources-updated = Thay đổi gần nhất: { $date }
sources-archived = đã lưu trữ
sources-already = đã thêm
sources-no-folder = Không thể xác định thư mục dữ liệu người dùng, nên không thể lưu nguồn.
source-invalid = Đây không phải là kho mã GitHub. Hãy nhập chủ sở hữu/tên, ví dụ ma1co/OpenMemories-AppList, hoặc địa chỉ của kho mã.
source-exists = { $source } đã được thêm.
source-unknown = Không có nguồn này: { $source }
source-unreadable = { $source } không có tệp { $file } đọc được trong nhánh mặc định.
source-failed = Không thể đọc nguồn { $source }: { $reason }
source-signer-changed = { $name } từ { $source } được ký bởi một người khác so với trước đây nên đã không được cài đặt. Nếu tác giả thực sự đã đổi khóa ký, hãy xóa nguồn rồi thêm lại.
source-ambiguous = { $package } được nhiều nguồn cung cấp: { $sources }. Hãy chọn một nguồn bằng --source.
source-plugin-mismatch = Plugin đã tải xuống có mã { $found }, nhưng nguồn công bố mã { $id }. Plugin đã không được cài đặt.
plugin-sources-heading = Plugin từ các nguồn bạn đã thêm
plugin-sources-hint = Bất kỳ ai cũng có thể công bố plugin trong nguồn của riêng mình. Danh sách này hiển thị plugin của tất cả các nguồn bạn đã thêm trong Nguồn ứng dụng và plugin, bất kể ai công bố chúng; chương trình này không kiểm tra chúng.
plugin-signed = Đã ký, khóa { $key }
plugin-unsigned = Chưa ký
plugin-signer-changed = { $id } đã được cài đặt với một khóa ký khác với khóa của tệp này, hoặc tệp này không được ký. Plugin đã không được thay thế. Nếu tác giả thực sự đã đổi khóa, hãy xóa plugin đã cài đặt trước.
plugin-key-written = Đã ghi khóa ký vào { $path }. Hãy giữ bí mật tệp này và sao lưu nó: bất kỳ ai có tệp này đều có thể ký plugin dưới danh nghĩa của bạn, và nếu không có nó, bạn không thể công bố các bản cập nhật mà các bản cài đặt hiện có chấp nhận. Khóa công khai: { $key }
btn-details = Chi tiết
details-repository = Kho mã
details-homepage = Trang dự án
details-source = Nguồn
msg-url-copied-browser = Đã sao chép địa chỉ vào bảng tạm. Hãy dán địa chỉ vào trình duyệt để xem trang.
store-activate-suggestion = Chưa thêm nguồn nào. Kích hoạt danh sách ứng dụng của dự án gốc ({ $source })? Danh sách này do tác giả ma1co duy trì, không phải do chương trình này, và chương trình này chưa kiểm tra danh sách đó. Sau đó, danh sách sẽ xuất hiện trong Cài đặt → Nguồn ứng dụng và plugin.
btn-activate-list = Kích hoạt danh sách
btn-plugin-sources-load = Hiển thị plugin từ các nguồn
plugin-sources-none = Các nguồn của bạn không cung cấp plugin nào.
plugin-source-unchecked = Nguồn: { $source }. Chương trình này chưa kiểm tra plugin này.
btn-plugin-source-install = Cài đặt

# Chế độ dịch vụ (senser)
usb-mode-advice = Đối với tinh chỉnh, sao lưu và trình bao dịch vụ, hãy đặt chế độ kết nối USB của máy ảnh thành “Tự động”, không cố định ở lưu trữ dung lượng lớn. Khi máy ảnh hỏi chế độ sau một phiên, hãy chọn “Lưu trữ dung lượng lớn (MSC)”. Nếu đặt cố định ở lưu trữ dung lượng lớn, máy ảnh thường từ chối lần chuyển sang chế độ dịch vụ tiếp theo.
service-switch-refused = Máy ảnh không hoàn tất việc chuyển sang chế độ dịch vụ và không kết nối lại qua USB; khi đó máy thường hiển thị “Kiểm tra thiết bị đã kết nối”. Hãy rút cáp USB, cắm lại và thử lại.
service-switching = Đang chuyển sang chế độ dịch vụ…
service-switch-wait = Đang chờ máy ảnh kết nối lại ở chế độ dịch vụ…
service-switch-timeout = Máy ảnh không xuất hiện ở chế độ dịch vụ. Máy được nhận dạng lại như một thiết bị USB riêng (mã sản phẩm 0336), cần được gán trình điều khiển WinUSB riêng; hãy kiểm tra trong Trình quản lý Thiết bị xem có thiết bị Sony nào chưa có trình điều khiển không. Tắt rồi bật lại máy ảnh để đưa máy về chế độ bình thường.
service-authenticating = Đang xác thực…
# Hiển thị khi phát hiện máy ảnh vẫn ở chế độ dịch vụ từ lần chạy trước, và
# khi điều đó không thể hoàn tác.
service-stranded-reset = Máy ảnh vẫn ở chế độ dịch vụ từ lần chạy trước. Đang kết thúc phiên đó trước…
service-stranded-failed = Máy ảnh bị kẹt ở chế độ dịch vụ từ lần chạy trước và không phản hồi yêu cầu thoát khỏi chế độ đó. Hãy tắt rồi bật lại máy ảnh, sau đó thử lại.
service-bad-mode = Không thể sử dụng máy ảnh ở chế độ này. Vui lòng chuyển máy sang chế độ lưu trữ dung lượng lớn.
service-driver-required = Quyền truy cập USB thô bị từ chối và không thể gán trình điều khiển WinUSB. Việc gán trình điều khiển cần quyền quản trị viên.
service-warning = Cảnh báo: chế độ dịch vụ cấp quyền truy cập thô vào bộ nhớ, hệ thống tệp và dữ liệu hiệu chuẩn của máy ảnh. Các lệnh ghi có thể làm hỏng máy ảnh vĩnh viễn.
service-shell-ready = Trình bao dịch vụ đã sẵn sàng. Gõ 'help' để xem các lệnh, 'exit' để thoát.
service-shell-done = Phiên trình bao dịch vụ đã kết thúc.
# Thông báo khi khởi động nếu lần chạy trước bị ngắt giữa chừng trước khi kịp
# gỡ trình điều khiển mà nó đã gán.
driver-swept = Đã xóa { $count } lần gán trình điều khiển còn sót lại từ lần chạy trước.
# Thông báo khi khởi động nếu chính lần chạy bị gián đoạn đó cũng để máy ảnh ở
# chế độ dịch vụ, khiến máy từ chối phiên tiếp theo.
driver-swept-camera = Máy ảnh bị một lần chạy trước để lại ở chế độ dịch vụ đã được đưa về chế độ bình thường.
driver-release-replug = Windows đang chờ máy ảnh trong khi gỡ trình điều khiển. Hãy rút cáp USB ra rồi cắm lại để hoàn tất việc dọn dẹp.
# Kết quả của lệnh `driver`.
driver-bound = { $id } hiện đang dùng WinUSB (gói trình điều khiển { $package }).
driver-released = Đã gỡ { $count } lần gán trình điều khiển; máy ảnh đã trở lại dùng các trình điều khiển đi kèm Windows.
driver-needs-elevation = Việc gán trình điều khiển cần quyền quản trị viên. Vui lòng chạy lệnh này từ cửa sổ dòng lệnh được khởi động với quyền quản trị viên.
driver-leftover-needs-elevation = Một lần chạy trước đã để lại trình điều khiển được gán cho máy ảnh. Khi trình điều khiển này còn đó, máy ảnh sẽ yêu cầu bạn kiểm tra thiết bị đã kết nối và chế độ lưu trữ dung lượng lớn của máy không hoạt động. Việc gỡ bỏ cần quyền quản trị viên; vui lòng xác nhận lời nhắc của Windows.
# Việc gán trình điều khiển mà quy trình dịch vụ tự thực hiện, để không cần
# công cụ bên ngoài nào.
driver-binding = Đang gán trình điều khiển WinUSB cho máy ảnh…
driver-bind-wait = Đang chờ máy ảnh kết nối lại với trình điều khiển mới…
driver-bind-timeout = Máy ảnh không xuất hiện lại sau khi gán trình điều khiển. Vui lòng rút máy ảnh ra, kết nối lại và thử lại.
driver-bind-failed = Không thể gán trình điều khiển WinUSB: { $detail }

# Những gì trình bao trả lời. Các thông báo này do phần phụ trợ tạo ra, nên cả
# chương trình dòng lệnh và cửa sổ đều diễn đạt giống nhau.
service-usage = Cách dùng: { $usage }
service-bad-hex = Không phải giá trị thập lục phân: { $value }
service-unknown-command = Lệnh không xác định: { $command }. Gõ 'help' để xem danh sách.
service-unterminated-quote = Dòng kết thúc bên trong một đối số có dấu ngoặc kép.
service-help = Tham chiếu lệnh:
service-read-bytes = Đã đọc { $count } byte.
service-pulled = Đã sao chép { $remote } từ máy ảnh sang { $local }.
service-pushed = Đã sao chép { $local } vào { $remote } trên máy ảnh.
service-removed = Đã xóa { $remote } khỏi máy ảnh.
service-dumped = Đã kết xuất { $size } byte từ { $offset } vào { $local }.
service-backup-saved = Đã lưu ảnh sao lưu ({ $count } byte) vào { $local }.
service-backup-text-saved = Đã lưu ảnh sao lưu ({ $count } byte) vào { $bin } và bản liệt kê vào { $txt }.
service-backup-status = Trạng thái sao lưu, khu vực: { $region }
service-backup-rewritten = Đã ghi lại y nguyên ảnh sao lưu ({ $count } byte) và đọc lại; chưa được lưu vĩnh viễn.
service-backup-attrs-restored = Đã đặt thuộc tính của { $count } mục trở lại như trong { $local }.
service-backup-written = Đã ghi mục sao lưu { $id }.
service-backup-persisted = Đã lưu vĩnh viễn bản sao lưu của hệ thống con { $id } vào bộ nhớ flash.
service-terminal-on = Thiết bị đầu cuối gỡ lỗi hiện đã bật.
service-terminal-off = Thiết bị đầu cuối gỡ lỗi hiện đã tắt.
btn-service-terminal-open = Mở bảng điều khiển nối tiếp
btn-service-terminal-close = Đóng bảng điều khiển nối tiếp
service-terminal-heading = Bảng điều khiển nối tiếp
service-terminal-hint = Trên các máy ảnh chạy Linux, thiết bị đầu cuối gỡ lỗi là một trình bao root qua USB. Hãy mở nó, rồi gõ lệnh như trong cửa sổ dòng lệnh.
service-terminal-label = Bảng điều khiển
service-terminal-interactive = Bảng điều khiển nối tiếp đã mở. Hãy gõ lệnh; gõ 'exit' để quay lại trình bao dịch vụ.

# Trang dịch vụ khi không bật chế độ nâng cao: cách mở trình bao từ chương trình
# dòng lệnh và những gì cần chuẩn bị trước.
service-console-heading = Chạy trình bao dịch vụ trong dòng lệnh
service-console-hint = Mở cửa sổ dòng lệnh trong thư mục chứa pmca-console và chạy lệnh sau:
service-driver-heading = Trước khi bắt đầu
service-driver-hint = Chế độ dịch vụ cần quyền truy cập USB thô, điều mà Windows không cấp qua trình điều khiển lưu trữ dung lượng lớn. Ứng dụng tự gán trình điều khiển WinUSB — hai lần, một lần khi máy ảnh ở chế độ lưu trữ dung lượng lớn và một lần nữa sau khi máy xuất hiện lại ở chế độ dịch vụ với mã sản phẩm thứ hai — và gỡ cả hai lần gán khi phiên kết thúc. Việc gán trình điều khiển cần quyền quản trị viên, vì vậy hãy khởi động chương trình dòng lệnh với quyền quản trị viên; không cần công cụ nào khác.
driver-note-linux = Không cần cài đặt gì ở đây: libusb truy cập trực tiếp máy ảnh mà không thay thế trình điều khiển nhân. Điều cần có là quyền mở thiết bị và một môi trường máy tính để bàn chưa chiếm giữ thiết bị đó. Hãy cài đặt quy tắc udev cho mã nhà cung cấp 054c của Sony và tách trình điều khiển usb-storage trước khi dùng chế độ lưu trữ dung lượng lớn. Tập lệnh scripts/linux-prepare.sh trong cây mã nguồn thực hiện cả hai việc và đồng thời dừng các trình giám sát ổ đĩa gvfs.
driver-note-macos = Không cần cài đặt gì để truy cập USB thô. Hãy đặt kết nối USB của máy ảnh thành MTP. Ở chế độ MTP, chương trình sẽ kết thúc dịch vụ máy ảnh ptpcamerad của macOS nếu dịch vụ này chiếm giữ máy ảnh; hệ thống sẽ khởi động lại dịch vụ khi cần. Tinh chỉnh, trình bao dịch vụ, phân tích chức năng, cập nhật phần sụn và trình bao cập nhật đi qua kết nối lưu trữ dung lượng lớn, mà macOS chỉ giao cho chương trình có quyền quản trị viên: cửa sổ sẽ hỏi mật khẩu của bạn một lần mỗi lần chạy, còn dòng lệnh cần sudo. Nền tảng này được kiểm thử ít nhất trong ba nền tảng, vì vậy nhiều khả năng bạn sẽ cần báo cáo sự cố thay vì thấy nó đã được mô tả sẵn.
service-advanced-hint = Trình bao cũng có thể được hiển thị ở đây, trong Cài đặt, Chế độ nâng cao.
btn-copy-command = Sao chép lệnh
msg-command-copied = Đã sao chép lệnh vào bảng tạm.

# Trang dịch vụ khi bật chế độ nâng cao.
btn-service-open = Kết nối
btn-service-close = Ngắt kết nối
btn-service-run = Chạy
btn-service-browse = Chèn đường dẫn tệp…
shell-path-quote = Không thể chèn đường dẫn này: nó chứa dấu ngoặc kép, ký tự mà dòng lệnh dùng để đánh dấu nơi một đối số bắt đầu và kết thúc. Hãy đổi tên tệp hoặc di chuyển tệp đến đường dẫn không có ký tự đó.
service-command-label = Lệnh
service-not-connected = Chưa kết nối.

# Chuỗi chỉ dùng trong giao diện đồ họa
install-apk-path = Đường dẫn đến tệp .apk cục bộ (tùy chọn)
install-drop-hint = Thả tệp .apk vào đây

# Mô tả tinh chỉnh
tweak-rec-limit = Quay video không giới hạn thời gian
tweak-rec-limit-4k = Quay video 4K không giới hạn thời gian
tweak-language = Hiển thị tất cả ngôn ngữ menu
tweak-language-note = Máy ảnh chỉ hiển thị các ngôn ngữ mà phần sụn của nó biết, nên số ngôn ngữ xuất hiện có thể ít hơn số được bật ở đây. Ví dụ, DSC-RX1R hiển thị 25 trong số 35 ngôn ngữ.
tweak-pal-ntsc = Bật bộ chọn và cảnh báo PAL / NTSC
tweak-pal-ntsc-mode = Hệ video (PAL / NTSC)
tweak-pal-ntsc-pal = PAL
tweak-pal-ntsc-ntsc = NTSC
tweak-pal-ntsc-note = Sau khi thay đổi hệ video, máy ảnh yêu cầu định dạng thẻ nhớ. Hãy sao lưu mọi thứ trên thẻ trước — ảnh, video, LUT. Cách đơn giản nhất là tháo thẻ ra trước khi chuyển đổi.
tweaks-set-malformed = Cần --set ID=VALUE, nhận được: { $arg }
tweaks-set-unknown-value = Giá trị không xác định "{ $value }". Cần một trong các giá trị: { $options }
tweaks-set-not-a-choice = { $id } là công tắc, không phải lựa chọn. Hãy dùng --enable hoặc --disable.
tweaks-needs-set = { $id } là lựa chọn, không phải công tắc. Hãy dùng --set { $id }=VALUE với một trong các giá trị: { $options }
tweak-usb-installer = Bật trình cài đặt ứng dụng qua USB

# Quy trình tinh chỉnh
tweaks-warning = Cảnh báo: áp dụng tinh chỉnh sẽ ghi lại bộ nhớ sao lưu của máy ảnh. Việc ghi bị gián đoạn có thể làm hỏng máy ảnh vĩnh viễn.
tweaks-reading = Đang đọc cài đặt máy ảnh…
tweaks-writing = Đang ghi cài đặt máy ảnh…
tweaks-done = Đã cập nhật cài đặt máy ảnh.
tweaks-write-failed = Máy ảnh không lưu các cài đặt mới đúng như khi được gửi. Một phần có thể vẫn đã được ghi: hãy đọc lại cài đặt và hoàn tác nếu cần. Một bản sao lưu được tạo trước khi thay đổi là cách an toàn để quay lại.
tweaks-confirm-required = Từ chối ghi khi chưa có xác nhận. Hãy chạy lại với --confirm sau khi bạn đã đọc cảnh báo ở trên.
tweaks-state-enabled = đã bật
tweaks-state-disabled = đã tắt
tweaks-full-read-failed = Máy ảnh không cung cấp đầy đủ bản sao lưu ({ $detail }). Sau đó máy thường ngừng phản hồi: hãy tắt máy, tháo pin ra vài giây, lắp lại rồi kết nối lại.
tweaks-direct-mode = Bản sao lưu của máy ảnh này lớn hơn 1 MB, nên các cài đặt được đọc và ghi lần lượt từng cái một. Các giá trị cũ được lưu trước mỗi lần thay đổi và có thể được ghi lại bằng “Hoàn tác thay đổi”.
tweaks-direct-forced = PMCA_TWEAKS_DIRECT buộc dùng cách ghi theo từng thuộc tính, nên các cài đặt của máy ảnh này được đọc và ghi lần lượt từng cái một. Các giá trị cũ được lưu trước mỗi lần thay đổi và có thể được ghi lại bằng “Hoàn tác thay đổi”.
tweaks-direct-refused = Máy ảnh từ chối thay đổi ({ $detail }). Các giá trị đã ghi được khôi phục lại; không có gì bị thay đổi. Một số kiểu máy, đặc biệt là máy Nhật Bản, chỉ cho phép cài đặt này thông qua toàn bộ ảnh sao lưu.
tweaks-direct-revert-failed = Máy ảnh từ chối thay đổi ({ $detail }), và không phải mọi giá trị đã ghi đều có thể được khôi phục. Các giá trị cũ đã được lưu: vui lòng thử “Hoàn tác thay đổi”.
tweaks-direct-restart = Đã cập nhật cài đặt máy ảnh. Hãy tắt rồi bật lại máy ảnh để thay đổi có hiệu lực.
tweaks-rollback-unsaved = Không thể lưu các giá trị hiện tại để hoàn tác, nên không có gì bị thay đổi.
tweaks-image-saved = Đã lưu ảnh sao lưu từ trước khi thay đổi: { $path }
tweaks-whole-image-hint = Để ghi thay đổi thông qua toàn bộ ảnh sao lưu, hãy chạy lại cùng lệnh với --whole-image. Ảnh từ trước khi thay đổi sẽ được lưu trước.
tweaks-whole-image-confirm = Ghi thay đổi thông qua toàn bộ ảnh sao lưu? Máy ảnh đã từ chối khi ghi dưới dạng một giá trị riêng lẻ. Toàn bộ ảnh sẽ được ghi lại kèm theo thay đổi; ảnh từ trước khi thay đổi sẽ được lưu trên máy tính này trước.
btn-whole-image = Ghi toàn bộ bản sao lưu
tweaks-undo-none = Không có giá trị nào được lưu cho máy ảnh này.
tweaks-undo-confirm = Ghi các giá trị đã lưu trở lại máy ảnh? Thao tác này hoàn tác mọi thay đổi được ghi theo từng cài đặt kể từ khi các giá trị được lưu.
tweaks-undo-done = Đã ghi lại các giá trị đã lưu. Hãy tắt rồi bật lại máy ảnh.
tweaks-undo-available = Các giá trị từ trước lần thay đổi đầu tiên đã được lưu và có thể được ghi lại.
btn-undo-tweaks = Hoàn tác thay đổi
tweaks-identity-heading = Nhận dạng máy ảnh (chỉ đọc)
tweaks-caution-languages = Hai ngôn ngữ — có thể là tiếng Trung (Giản thể) và tiếng Trung (Phồn thể) — không thể hiển thị trên máy ảnh này. Cuộn đến chúng sẽ khiến menu ngôn ngữ khởi động lại, nên chúng được để tắt như một biện pháp phòng ngừa.
tweaks-caution-accept = Vẫn bật hai ngôn ngữ này (bạn tự chịu rủi ro)
tweaks-caution-confirm = Hai ngôn ngữ này có thể khiến menu ngôn ngữ của máy ảnh khởi động lại. Không có gì bị hỏng và thay đổi có thể hoàn tác, nhưng có thể bạn sẽ phải tìm lại cài đặt từ đầu. Vẫn bật chúng?

# Cài đặt
settings-language = Ngôn ngữ
settings-theme = Chủ đề
settings-theme-system = Hệ thống
settings-theme-light = Sáng
settings-theme-dark = Tối

# Chế độ nâng cao. Đây là biện pháp phòng tránh thao tác nhầm chứ không phải chống
# lại kẻ tấn công: mọi thứ nó hiển thị đều có thể truy cập từ dòng lệnh.
settings-advanced-heading = Chế độ nâng cao
settings-advanced-hint = Chế độ nâng cao hiển thị trình bao dịch vụ và trình bao cập nhật trên các trang cùng tên. Chế độ dịch vụ cấp quyền truy cập thô vào bộ nhớ, hệ thống tệp và dữ liệu hiệu chuẩn của máy ảnh; một lệnh gõ sai có thể khiến máy ảnh không dùng được nữa. Hãy để tắt trừ khi bạn biết mình sắp ghi gì.
settings-advanced-toggle = Hiển thị trình bao dịch vụ và trình bao cập nhật
settings-advanced-on = Chế độ nâng cao đang bật. Các trang Trình bao dịch vụ và Trình bao cập nhật hiện cung cấp chính các trình bao đó.
settings-advanced-off = Chế độ nâng cao đang tắt. Các trang Trình bao dịch vụ và Trình bao cập nhật giải thích cách dùng chương trình dòng lệnh thay thế.
settings-advanced-confirm = Trình bao dịch vụ và trình bao cập nhật có thể làm hỏng máy ảnh vĩnh viễn. Hiển thị cả hai trình bao?
settings-update-heading = Thông báo cập nhật
settings-update-purpose = Khi bật tính năng này, mỗi lần khởi động chương trình sẽ hỏi GitHub xem đã có phiên bản mới hơn được phát hành chưa và, nếu có, sẽ thông báo ở đầu cửa sổ. Trong quá trình này GitHub biết được địa chỉ IP của bạn; không có gì khác được gửi đi. Các bản phát hành thử nghiệm không được thông báo.
settings-update-toggle = Thông báo cho tôi khi khởi động nếu có phiên bản mới
settings-update-on = Thông báo cập nhật đang bật. Từ lần khởi động tiếp theo, phiên bản mới sẽ được thông báo ở đầu cửa sổ.
settings-update-off = Thông báo cập nhật đang tắt. Chương trình không tìm phiên bản mới.
update-available = Đã có phiên bản mới
update-current = Phiên bản đã cài đặt
update-latest = Phiên bản mới
btn-update-dismiss = Ẩn

# Chỉnh sửa bản dịch. Tệp ngôn ngữ có thể được tải xuống, sửa trong bất kỳ trình soạn
# thảo văn bản nào và tải lên lại; tệp được tải lên sẽ được kiểm tra trước khi sử dụng.
settings-catalog-heading = Chỉnh sửa bản dịch
settings-catalog-hint = Tải xuống tệp ngôn ngữ, sửa trong bất kỳ trình soạn thảo văn bản nào rồi tải lên lại. Chỉ phần văn bản bên phải dấu bằng được dịch, và các mục bạn bỏ qua sẽ giữ nguyên nội dung tích hợp sẵn. Việc chọn ngôn ngữ ở đây không thay đổi ngôn ngữ của ứng dụng.
settings-catalog-locale = Tệp ngôn ngữ
settings-catalog-builtin = Đang dùng bản dịch tích hợp sẵn.
settings-catalog-custom = Đang dùng bản dịch của riêng bạn:
btn-catalog-export = Tải xuống…
btn-catalog-import = Tải lên…
btn-catalog-reset = Khôi phục bản tích hợp sẵn
catalog-exported = Đã ghi vào
catalog-imported = Đã áp dụng bản dịch.
catalog-reset-done = Đã khôi phục bản dịch tích hợp sẵn.
catalog-reset-none = Không có bản dịch tùy chỉnh nào, nên không có gì để khôi phục.
catalog-reset-confirm = Hủy bỏ bản dịch của riêng bạn cho ngôn ngữ này?
catalog-rejected = Tệp chưa được áp dụng. Vui lòng sửa các dòng sau:
catalog-count-accepted = Số mục đã áp dụng
catalog-count-missing = Số mục giữ nguyên từ bản dịch tích hợp sẵn
catalog-count-untranslated = Số mục giống hệt bản gốc tiếng Anh
catalog-line = Dòng
catalog-problem-syntax = không phải là mục có dạng "id = văn bản"
catalog-problem-continuation = tiếp tục ở dòng sau, điều này không được hỗ trợ
catalog-problem-duplicate = được định nghĩa nhiều lần
catalog-problem-unknown-key = không phải là mục mà ứng dụng này sử dụng
catalog-problem-empty = không có văn bản
catalog-problem-placeholder = phần giữ chỗ đã bị thay đổi, cần có:

# Nguồn gốc, lời cảm ơn và giấy phép. Hiển thị trên trang Giấy phép và bởi
# `pmca-console license`. Bản thân văn bản giấy phép không được dịch: đó là một
# văn bản pháp lý và được sao chép nguyên văn.
about-thanks-heading = Lời cảm ơn
about-thanks-body-1 = Công cụ này tồn tại nhờ ma1co, người đã tìm ra cách máy ảnh Sony giao tiếp qua USB và phát hành Sony-PMCA-RE vào năm 2015, và nhờ tất cả những người đã đóng góp cho dự án đó cũng như cho các dự án OpenMemories trong những năm sau đó.
about-thanks-body-2 = Việc nhận diện máy ảnh, giao thức lệnh bên ngoài, quy trình cập nhật phần sụn và chế độ dịch vụ đều là thành quả của họ, và bản chuyển đổi này tiếp nối thành quả đó. Xin cảm ơn. Trên nền tảng đó, PMCA-RE RELOADED bổ sung nhiều tính năng mới do masterflai đóng góp. Nổi bật nhất là việc tự động chuyển đổi trình điều khiển thiết bị trên Windows, nay đã được tích hợp vào chương trình: không còn phải gán thủ công một trình điều khiển khác cho máy ảnh bằng một công cụ riêng, giúp quy trình thuận tiện hơn, nhanh hơn và ít rủi ro hơn.
about-contributors-heading = Người đóng góp
about-contributors-body = Ngoài ma1co, những người sau đã đóng góp cho Sony-PMCA-RE hoặc đã công bố các thay đổi trong bản sao riêng của họ mà sau đó được đưa vào bản chuyển đổi này:
about-purpose-heading = Vì sao có bản chuyển đổi này
about-purpose-body-1 = Ứng dụng gốc không còn được phát triển. Ứng dụng được viết cho Python 2, phiên bản đã ngừng hỗ trợ vào năm 2020, và một số phần dựa vào các dịch vụ mà Sony đã đóng cửa, nên nó không còn chạy được mà không bị hạn chế trên các hệ thống hiện nay.
about-purpose-body-2 = Mục đích của bản chuyển đổi này là bảo tồn chứ không phải thay thế, và cũng quan trọng không kém là sử dụng suôn sẻ ngày nay: giữ cho những máy ảnh này còn dùng được, và hoạt động không gặp khó khăn trên máy tính hiện tại cũng như với các kiểu máy ảnh mới hơn. Mọi thứ bản gốc làm được, bản này vẫn phải làm được.
about-origin-heading = Bản chuyển đổi được tạo ra như thế nào
about-origin-body-1 = Việc chuyển đổi từ Python sang Rust được thực hiện với sự hỗ trợ của AI.
about-origin-body-2 = Điều đó không có nghĩa là gì cũng đáng được nói rõ như vậy: hoạt động của chương trình được kiểm chứng bằng một bộ kiểm thử tự động và một kế hoạch kiểm thử phần cứng. Đặc biệt, phiên bản Windows đã được kiểm thử kỹ lưỡng, vì các trình điều khiển thiết bị khiến đây là nền tảng phức tạp nhất; macOS và Linux chưa thể được kiểm thử toàn diện như vậy. Vui lòng xem đây là phần mềm thử nghiệm.
about-license-heading = Giấy phép
about-license-body = Bản chuyển đổi này được phát hành theo giấy phép MIT, cùng giấy phép với dự án gốc. Thông báo bản quyền gốc được giữ nguyên, đúng như giấy phép đó yêu cầu.
about-license-full = Văn bản giấy phép
about-third-party-heading = Giấy phép của bên thứ ba
about-third-party-body = Chương trình này được liên kết tĩnh: mã của mọi thư viện mà nó sử dụng đều nằm bên trong tệp thực thi, và giấy phép của các thư viện đó yêu cầu thông báo bản quyền của chúng phải đi kèm. Các thông báo này được trình bày đầy đủ bên dưới, mỗi gói một mục.
about-third-party-toggle = Hiển thị tất cả thông báo của bên thứ ba
about-privacy-heading = Dữ liệu
about-privacy-body-1 = Chương trình này chạy trên máy tính của riêng bạn. Chương trình không gửi dữ liệu sử dụng, không thu thập dữ liệu đo từ xa và không cần tài khoản. Không điều gì bạn làm ở đây được báo cáo đi bất cứ đâu.
about-privacy-body-2 = Có hai tính năng tự kết nối mạng, và phía bên kia biết được địa chỉ IP của bạn trong quá trình đó: dữ liệu hỗ trợ GPS được tải xuống từ Sony, và các nguồn bạn đã thêm được đọc từ GitHub, kèm theo số sao của chúng và, khi bạn yêu cầu, một lượt tìm kiếm ở đó; ứng dụng và plugin được tải xuống từ các địa chỉ mà những nguồn đó nêu ra. Nếu không dùng các tính năng đó, chương trình hoàn toàn ngoại tuyến. Nếu bạn bật thông báo cập nhật trong cài đặt, mỗi lần khởi động chương trình cũng sẽ hỏi GitHub về phiên bản mới nhất.
about-privacy-body-3 = Những gì được đọc ra từ máy ảnh — trong đó có số sê-ri và mọi mật khẩu Wi-Fi đã lưu — không rời khỏi máy tính của bạn. Nếu bạn ghi chúng vào tệp, hãy cẩn thận khi chuyển tệp đó cho người khác.
about-privacy-body-4 = Không có dịch vụ nào được vận hành phía sau chương trình này và không có dữ liệu nào đến được với các tác giả của nó, vì vậy ở đây không có gì để yêu cầu cung cấp hay yêu cầu xóa. Việc hai dịch vụ nêu trên làm gì với các địa chỉ mà họ thấy được điều chỉnh bởi tuyên bố quyền riêng tư của chính họ, không phải bởi tuyên bố này.
about-legal-heading = Thông báo pháp lý
about-legal-trademark = Đây là một dự án độc lập. Dự án không do Sony tạo ra, không được Sony xác nhận và không có bất kỳ liên hệ nào với Sony. "Sony", "PlayMemories" và tên các kiểu máy ảnh là nhãn hiệu của Sony Group Corporation, chỉ được dùng ở đây để cho biết công cụ này giao tiếp với những máy ảnh nào.
about-legal-own-device = Chương trình này dành cho việc sử dụng với máy ảnh thuộc sở hữu của bạn.
about-legal-warranty = Những gì công cụ này làm nằm ngoài mục đích Sony dự định cho các máy ảnh này, và việc đó có thể làm mất hiệu lực bảo hành của bạn. Bản thân phần mềm không đi kèm bất kỳ bảo hành nào, và bạn tự chịu rủi ro khi sử dụng. Việc thiết kế ngược một thiết bị bạn sở hữu có được phép hay không, và ở mức độ nào, tùy thuộc vào nơi bạn sinh sống.
about-links-heading = Dự án
about-link-original = Sony-PMCA-RE — dự án gốc
about-link-fwtool = fwtool.py — các công cụ phần sụn làm cơ sở cho việc tạo bản cập nhật
about-link-tweak = OpenMemories: Tweak — ứng dụng được khuyên dùng cho các máy ảnh hỗ trợ ứng dụng này
about-link-applist = OpenMemories: App list — thêm các ứng dụng khác
about-link-platform = OpenMemories: Platform — phần lõi trình cập nhật đi kèm công cụ này
about-link-devices = Các kiểu máy ảnh được hỗ trợ
about-link-nexhack = Nex-Hack — nghiên cứu cộng đồng mà dự án này dựa vào
about-link-zve10 = zv-e10-re — nghiên cứu về giao diện và bộ nhớ cài đặt của ZV-E10
about-version = Phiên bản
btn-copy-url = Sao chép địa chỉ
msg-url-copied = Đã sao chép địa chỉ vào bảng tạm.

# Thông báo an toàn
notice-experimental = Đây là phần mềm thử nghiệm. Bạn tự chịu rủi ro khi sử dụng.

# Cài đặt mạng (hỗ trợ GPS, điểm truy cập và dịch vụ phát trực tiếp)
gps-downloading = Đang tải xuống dữ liệu hỗ trợ GPS…
gps-writing = Đang ghi dữ liệu hỗ trợ GPS vào máy ảnh…
gps-bad-size = Dữ liệu hỗ trợ GPS có kích thước không như mong đợi và đã không được ghi.
wifi-read = Đang đọc các điểm truy cập phát trực tiếp từ máy ảnh…
wifi-written = Đã ghi các điểm truy cập phát trực tiếp.
stream-read = Đang đọc cài đặt phát trực tiếp từ máy ảnh…
stream-written = Đã ghi cài đặt phát trực tiếp.
net-secret-warning = Cảnh báo: tệp này chứa mật khẩu Wi-Fi và mã truy cập ở dạng văn bản thuần.
wifi-secret-warning = Máy ảnh lưu các mật khẩu này mà không mã hóa, và chúng được hiển thị ở đây đúng như khi được lưu. Chúng chỉ nằm trên máy tính này; nếu bạn lưu chúng vào tệp, hãy cẩn thận khi chuyển tệp đó cho người khác.
gps-heading = Dữ liệu hỗ trợ GPS
gps-hint = Kéo tệp vào đây, nhập đường dẫn hoặc để trống trường này để lấy dữ liệu hiện tại từ Sony.
btn-gps-devices = Thiết bị tương thích
btn-gps-devices-hide = Ẩn thiết bị tương thích
gps-devices-note = Các kiểu máy này có mô-đun GPS tích hợp. Với nhiều kiểu máy, chỉ biến thể có tên kết thúc bằng V mới có GPS; các biến thể không có chữ V thì không có GPS.
gps-devices-compact = Máy ảnh nhỏ gọn (DSC)
gps-devices-slt = Máy ảnh SLT (ngàm A)
gps-devices-action = Máy quay hành động
gps-devices-camcorder = Máy quay phim (Handycam)
gps-col-variant = Biến thể GPS
gps-col-arch = Kiến trúc
gps-col-module = Mô-đun GPS
gps-col-apps = Hỗ trợ ứng dụng
gps-yes = có
gps-no = không
gps-unknown = không xác định
gps-devices-untested = Có GPS, chưa kiểm thử khả năng hỗ trợ
gps-devices-untested-note = Các kiểu máy này có mô-đun GPS tích hợp nhưng không có trong danh sách thiết bị của OpenMemories. Chưa kiểm thử xem ứng dụng này có thể giao tiếp với chúng hay không, nên kiến trúc và khả năng hỗ trợ ứng dụng của chúng chưa được biết.
wifi-heading = Điểm truy cập để phát trực tiếp
wifi-hint = Đây là các điểm truy cập mà tính năng phát trực tiếp của máy ảnh sử dụng, không phải cài đặt Wi-Fi riêng của máy ảnh. Thân máy không có tính năng phát trực tiếp sẽ từ chối thao tác này.
wifi-multi = Danh sách nhiều điểm truy cập
wifi-none = Chưa lưu điểm truy cập nào.
wifi-writing = Đang ghi các điểm truy cập phát trực tiếp vào máy ảnh…
wifi-confirm = Thao tác này thay thế mọi điểm truy cập đã lưu trên máy ảnh. Tiếp tục?
wifi-empty-refused = Từ chối ghi danh sách trống, vì việc này sẽ xóa mọi điểm truy cập đã lưu.
stream-heading = Phát trực tiếp
stream-writing = Đang ghi cài đặt phát trực tiếp vào máy ảnh…
stream-confirm = Thao tác này ghi đè cài đặt phát trực tiếp của máy ảnh. Tiếp tục?
stream-invalid-json = Cài đặt không phải là JSON hợp lệ:

# Cập nhật phần sụn
firmware-warning = Cảnh báo: thao tác này ghi đè phần sụn của máy ảnh. Không được làm gián đoạn quá trình cập nhật sau khi đã bắt đầu: không rút cáp USB và không tắt máy ảnh. Chỉ sử dụng tệp phần sụn dành riêng cho đúng kiểu máy ảnh này, và đảm bảo pin được sạc đủ. Nếu không, máy ảnh có thể bị hỏng vĩnh viễn.
firmware-untested = Chưa kiểm thử: chức năng ghi phần sụn đã được chuyển đổi nhưng chưa bao giờ được thử trên máy ảnh thật. Việc ghi thất bại hoặc bị gián đoạn không thể hoàn tác, vì vậy hãy xem mọi thứ trên trang này là chưa được kiểm chứng.
firmware-confirm-required = Từ chối ghi phần sụn khi không có --confirm.
firmware-truncated = Tệp phần sụn bị cắt cụt: tải trọng FDAT vượt quá cuối tệp.
firmware-updating = Đang cập nhật phần sụn:
firmware-invalid = Tệp không phải là bản phần sụn Sony hợp lệ
firmware-dat-path = Tệp phần sụn (.dat)
firmware-drop-hint = Thả tệp phần sụn .dat vào đây
firmware-file-required = Hãy chọn tệp phần sụn trước.
firmware-check-ok = Máy ảnh chấp nhận bản phần sụn này. Chưa có gì được ghi.
firmware-cancelled = Đã hủy ghi phần sụn. Máy ảnh không bị sửa đổi.
firmware-current-version = Phiên bản đã cài đặt
firmware-new-version = Phiên bản trong bản phần sụn này
firmware-models-hint = Máy ảnh tương thích nếu trình bao cập nhật có thể kết nối với nó.
btn-firmware-check = Kiểm tra bản phần sụn
btn-firmware-flash = Ghi phần sụn
firmware-own-device = Hãy xác nhận rằng bạn đã biết những rủi ro này và chỉ cập nhật thiết bị của chính mình.
btn-firmware-accept = Đã hiểu rủi ro – ghi phần sụn

# Công cụ tệp: các thao tác hoàn toàn không cần máy ảnh.
tools-hint = Các thao tác này làm việc trên tệp và không bao giờ tác động đến máy ảnh, nên không gì ở đây có thể làm hỏng máy. Đây là các thao tác mà pmca-console cung cấp dưới dạng apk2spk, spk2apk, print-backup, print-usagelog, guess-firmware và xpd-checksum.
tools-operation = Thao tác
tools-apk2spk = Đóng gói tệp .apk vào vỏ chứa .spk
tools-spk2apk = Giải nén tệp .spk trở lại thành .apk
tools-print-backup = Hiển thị nội dung của Backup.bin
tools-print-usagelog = Hiển thị nội dung của Usagelog.bin
tools-guess-firmware = Tìm tải trọng bên trong tệp phần sụn .dat
tools-xpd-checksum = Tính tổng kiểm CIC của một tệp
tools-input = Tệp đầu vào
tools-file-required = Hãy chọn các tệp mà thao tác này cần trước.
tools-apk-invalid = Không thể đóng gói tệp thành .spk
tools-spk-invalid = Tệp không phải là gói .spk hợp lệ
tools-backup-invalid = Tệp không phải là ảnh Backup.bin hợp lệ
tools-usagelog-invalid = Tệp không phải là nhật ký sử dụng hợp lệ
tools-reset-value = giá trị đặt lại:
tools-fdat-found = Đã tìm thấy tải trọng tại độ lệch { $offset }, kích thước { $size }.
tools-checksum = Tổng kiểm
tools-written = Đã ghi vào { $path }

# Liên kết đến tài liệu dự án
docs-compatibility = Khả năng tương thích của máy ảnh
docs-compatibility-hint = Dự án gốc đã cung cấp bảng tổng quan về các máy ảnh đã được kiểm thử. Bảng này chủ yếu bao gồm các kiểu máy ảnh cũ và có tại địa chỉ sau:
devices-note = Lấy từ bảng tổng quan của dự án gốc. Con số bên cạnh mục hỗ trợ ứng dụng là phiên bản nền tảng ứng dụng của máy ảnh.
devices-more-note = Các máy ảnh sau chưa được kiểm thử cụ thể với chương trình này. Dấu tích ở cột cuối cùng nghĩa là cộng đồng đã báo cáo có sử dụng chúng. Nếu trình bao dịch vụ cũng được đánh dấu ✖, việc sử dụng đó đã thất bại.
btn-more-models = Thêm thiết bị
btn-more-models-hide = Ẩn bớt thiết bị
check-updater-yes = Thiết bị { $model } của bạn tương thích: theo danh sách thiết bị, trình bao cập nhật có thể kết nối với thiết bị này.
check-updater-no = Thiết bị { $model } của bạn không tương thích: theo danh sách thiết bị, trình bao cập nhật không thể kết nối với thiết bị này.
check-updater-unknown = Chưa rõ khả năng tương thích của thiết bị { $model }: thiết bị không có trong bất kỳ danh sách thiết bị nào của chương trình này, nên không biết trình bao cập nhật có thể kết nối với thiết bị hay không.
check-apps-no = Thiết bị { $model } của bạn không tương thích: theo danh sách thiết bị, thiết bị này không hỗ trợ ứng dụng.
check-apps-untested = Chưa rõ khả năng tương thích của thiết bị { $model }: thiết bị có trong danh sách thiết bị, nhưng khả năng hỗ trợ ứng dụng chưa được xác nhận.
check-apps-unknown = Chưa rõ khả năng tương thích của thiết bị { $model }: thiết bị không có trong bất kỳ danh sách thiết bị nào của chương trình này, nên không biết thiết bị có hỗ trợ ứng dụng hay không.
check-gps-no = Thiết bị { $model } của bạn không tương thích: thiết bị không có trong danh sách các thiết bị có bộ thu GPS.
check-gps-untested = Chưa rõ khả năng tương thích của thiết bị { $model }: theo danh sách thiết bị, thiết bị có bộ thu GPS, nhưng việc truyền dữ liệu sang kiểu máy này chưa được kiểm thử.
check-gps-unknown = Chưa rõ khả năng tương thích của thiết bị { $model }: thiết bị không có trong bất kỳ danh sách thiết bị nào của chương trình này, nên không biết thiết bị có bộ thu GPS hay không.
check-unidentified = Không thể nhận dạng thiết bị đã kết nối, nên chưa rõ khả năng tương thích của thiết bị.
check-proceed = Bạn vẫn muốn tiếp tục?
check-cancelled = Đã hủy. Thiết bị không bị sửa đổi.
devices-col-confirmed = Đã xác nhận
devices-group-ilc = Máy ảnh ống kính rời (ngàm E và ngàm A)
devices-group-remote = Bộ điều khiển từ xa Live-View
btn-copy-link = Sao chép liên kết
shutter-heading = Số lần chụp
shutter-hint = Máy ảnh đếm mỗi lần nhả màn trập và ghi con số hiện tại vào mỗi ảnh mà nó lưu (ARW, JPEG hoặc HEIF). Hãy chọn ảnh được chụp gần đây nhất để đọc số lần chụp hiện tại. Tệp chỉ được đọc, không bao giờ bị sửa đổi.
shutter-source = Vị trí lưu số lần chụp trong tệp khác nhau giữa các kiểu máy. Các vị trí này lấy từ nghiên cứu được công bố công khai (ExifTool) và chưa được xác nhận cho mọi kiểu máy.
shutter-drop-hint = Thả ảnh (ARW, JPEG, HEIF) vào đây
btn-shutter-read = Phân tích ảnh
shutter-file-required = Vui lòng chọn ảnh trước.
shutter-file = Tệp
shutter-taken = Ngày chụp
shutter-count = Số lần nhả màn trập
shutter-type = Màn trập
shutter-type-mechanical = Cơ học
shutter-type-electronic = Điện tử
shutter-electronic-hint = Ảnh này được chụp bằng màn trập điện tử. Trên nhiều kiểu máy, những lần chụp như vậy không được đếm; khi đó giá trị hiển thị là số đếm tại lần nhả màn trập cơ học gần nhất.
shutter-not-sony = Ảnh không được chụp bằng máy ảnh Sony. Không thể đọc số lần chụp từ ảnh này.
shutter-unknown-model = Chưa biết kiểu máy ảnh này lưu số lần chụp ở đâu trong tệp, nên không có giá trị nào được hiển thị.
shutter-not-recorded = Tệp không chứa số lần chụp có thể đọc được. Điều này xảy ra với ảnh được lưu hoặc xuất bởi phần mềm chỉnh sửa ảnh, và trên một số kiểu máy với ảnh chụp bằng màn trập điện tử. Hãy chọn tệp gốc từ thẻ nhớ.
shutter-model-mismatch = Ảnh được chụp bằng kiểu máy ảnh khác với máy ảnh đang kết nối.
shutter-invalid = Tệp không phải là ảnh có thể đọc được
file-filter-photo = Ảnh (ARW, JPEG, HEIF)
usage-log-heading = Nhật ký sử dụng
usage-log-hint = Máy ảnh lưu một nhật ký nội bộ ở định dạng không được công bố. Nhật ký được lưu nguyên văn; máy ảnh chỉ được đọc.
usage-log-written = Đã ghi vào
btn-usage-log-info = Về nhật ký sử dụng
btn-usage-log-info-hide = Ẩn phần giải thích
usage-log-info-what-heading = Nhật ký này là gì
usage-log-info-what = Nội bộ Sony gọi nhật ký này là "KikiLog". Máy ảnh tự ghi nhật ký; chương trình này chỉ đọc nó. Chưa có mô tả công khai nào về định dạng này.
usage-log-info-read = Việc đọc không thay đổi gì: nhiều lần tải xuống liên tiếp cho kết quả giống hệt nhau đến từng byte.
usage-log-info-flush-heading = Vì sao thiếu các mục gần đây
usage-log-info-flush = Các mục mới thường không xuất hiện ngay: máy ảnh có thể chỉ lưu chúng khi tắt máy, đôi khi rải rác qua nhiều lần tắt/bật. Hãy tắt rồi bật máy ảnh, tải xuống lại và lặp lại cho đến khi không còn mục mới xuất hiện.
usage-log-info-content-heading = Nhật ký cho bạn biết gì
usage-log-info-shutter = Số lần nhả màn trập: mỗi sự kiện thuộc một số loại nhất định tương ứng với một lần phơi sáng. Tuy nhiên, đếm các sự kiện này không cho ra số lần chụp: nhật ký có kích thước cố định và không nhất thiết bao quát toàn bộ thời gian sử dụng của máy ảnh. Máy ảnh lưu tổng số ở nơi khác và ghi vào các ảnh mà nó lưu; có thể đọc số này trong mục "Số lần chụp".
usage-log-info-clock = Dấu thời gian chỉ chính xác bằng đồng hồ của máy ảnh. Sau khi thay pin, đồng hồ có thể bị đặt lại và tạo ra các mục mang ngày ban đầu của máy ảnh.
usage-log-info-unknown = Chưa có mô tả công khai nào về số hiệu sự kiện. Chương trình hiển thị các số này mà không diễn giải.
usage-log-info-privacy-heading = Quyền riêng tư
usage-log-info-privacy = Tệp chứa số sê-ri của các thẻ nhớ đã được lắp vào. Đừng chia sẻ hoặc đăng tải tệp này một cách thiếu cẩn trọng.
btn-usage-log = Lưu nhật ký
usagelog-size = Vùng nhật ký
usagelog-cursor = Vị trí ghi
usagelog-spare = Vùng thứ hai
usagelog-count = Số mục
usagelog-events = Sự kiện theo số hiệu
usagelog-entries = Các mục theo thứ tự
msg-link-copied = Đã sao chép liên kết vào bảng tạm.

# Thông báo lỗi của phần phụ trợ, theo mã ổn định mà phần phụ trợ báo cáo.
# Phần thay đổi của thông báo (đường dẫn, byte trạng thái, lỗi thiết bị) được giao
# diện người dùng nối thêm vào, nên các thông báo này là câu hoàn chỉnh khi không có phần đó.
# Cố ý không có `err-operation`: mã đó đánh dấu một thông báo mà phần phụ trợ
# đã dịch sẵn, phải được hiển thị nguyên văn.
err-usb-no-device = Không tìm thấy máy ảnh. Hãy kết nối máy ảnh và bật máy. Nếu màn hình máy ảnh cứ báo đang kết nối, máy tính không nhận được máy ảnh: hãy dùng cáp dữ liệu thay vì cáp chỉ sạc, kết nối máy ảnh trực tiếp không qua bộ chia hay bộ chuyển đổi, rồi tắt và bật lại máy ảnh; nếu vẫn không được, hãy tháo pin ra trong giây lát. Qua cáp dữ liệu, ngay cả máy ảnh đang tắt cũng được phát hiện và báo là chưa sẵn sàng; nếu thông báo này vẫn xuất hiện khi máy ảnh đang tắt, có thể cáp không truyền dữ liệu.
err-usb-camera-not-ready = Máy ảnh đã được kết nối nhưng chưa sẵn sàng: máy đang tắt, hoặc đang chờ chọn chế độ USB trên màn hình. Hãy bật máy, hoặc chọn “Lưu trữ dung lượng lớn (MSC)” trên máy ảnh.
camera-choose-usb-mode = Máy ảnh đang chờ chọn chế độ USB. Vui lòng chọn “Lưu trữ dung lượng lớn (MSC)” trên máy ảnh.
err-usb-too-many-devices = Có nhiều hơn một máy ảnh được kết nối. Hãy ngắt kết nối tất cả trừ máy ảnh bạn muốn làm việc cùng.
err-usb-transfer = Truyền dữ liệu USB thất bại. Hãy thử cáp hoặc cổng khác và tránh dùng bộ chia USB.
err-usb-permission = Chương trình không được phép mở máy ảnh qua USB. Trên Linux, quyền này được cấp một lần bằng quy tắc udev cho mã nhà cung cấp của Sony; tập lệnh scripts/linux-prepare.sh trong cây mã nguồn sẽ cài đặt quy tắc này. Sau đó hãy rút máy ảnh ra rồi cắm lại.
err-usb-busy = Một chương trình khác đang sử dụng máy ảnh, nên không thể mở máy. Hãy đóng các chương trình khác có thể truy cập máy ảnh; trên Linux, trình giám sát phương tiện của môi trường máy tính để bàn có thể chiếm giữ máy ảnh, và scripts/linux-prepare.sh sẽ dừng trình này. Trên macOS, hãy đặt kết nối USB của máy ảnh thành MTP: ở chế độ lưu trữ dung lượng lớn, trình điều khiển ổ đĩa của chính hệ thống sẽ chiếm giữ máy.
err-usb-needs-root = macOS chỉ giao kết nối lưu trữ dung lượng lớn của máy ảnh cho một chương trình khi chương trình đó chạy với quyền quản trị viên. Hãy khởi động chương trình dòng lệnh bằng sudo, ví dụ: sudo pmca-console tweaks. Việc đọc thông tin máy ảnh vẫn hoạt động mà không cần quyền này, khi kết nối USB của máy ảnh được đặt thành MTP.
macos-admin-prompt = PMCA-RE RELOADED cần quyền quản trị viên để lấy kết nối lưu trữ dung lượng lớn của máy ảnh từ macOS. Thẻ nhớ của máy ảnh sẽ biến mất khỏi Finder trong khi chương trình làm việc; nếu có tệp trên thẻ đang mở, hãy đẩy thẻ ra trong Finder trước.
macos-admin-declined = Quyền quản trị viên không được cấp, nên máy ảnh không bị tác động. Trên macOS, chức năng này cần quyền đó.
macos-admin-failed = Chương trình không thể khởi động trình hỗ trợ của nó với quyền quản trị viên.
err-usb-stall = Máy ảnh đã hủy bỏ quá trình truyền USB.
err-usb-protocol = Máy ảnh đã gửi phản hồi không mong đợi.
err-usb-msc-status = Máy ảnh đã từ chối một lệnh lưu trữ.
cli-error-prefix = lỗi
cli-hint-mass-storage = Chuyển sang chế độ lưu trữ dung lượng lớn: pmca-console mass-storage
err-usb-driver = Windows đã gán máy ảnh này cho một trình điều khiển không cho phép truy cập trực tiếp. Chế độ này cần trình điều khiển WinUSB, mà ứng dụng có thể tự gán — nhưng chỉ khi đang chạy với quyền quản trị viên.
err-needs-elevation = Việc gán hoặc gỡ trình điều khiển cần quyền quản trị viên, và lời nhắc của Windows đã bị từ chối. Hãy thử lại bằng “Khởi động trình hỗ trợ trình điều khiển”; không cần thay đổi gì trên máy ảnh.
err-usb-no-mass-storage-volume = Máy ảnh đã kết nối ở chế độ lưu trữ dung lượng lớn nhưng không cung cấp ổ đĩa. Hãy đánh thức máy, xác nhận chế độ USB trên màn hình nếu máy yêu cầu, và kiểm tra xem thẻ nhớ đã được lắp chưa. Đừng thay đổi trình điều khiển: chế độ này cần trình điều khiển chuẩn của Windows.
err-usb-msc-sense = Máy ảnh đã báo lỗi lưu trữ.
err-usb-ptp-response = Máy ảnh đã từ chối lệnh.
err-usb-unsupported = Kiểu máy ảnh này không hỗ trợ chức năng đó.
err-usb-updater-status = Trình cập nhật phần sụn trên máy ảnh đã từ chối thao tác.
err-usb-senser = Một lệnh của chế độ dịch vụ đã thất bại.
err-market-core = Không thể đọc tệp dữ liệu máy ảnh.
err-market-json = Dữ liệu nhận được bị sai định dạng.
err-market-missing-field = Thiếu một trường cần có trong phản hồi.
err-market-no-apk = Chưa chọn ứng dụng nào để cài đặt.
err-market-http = Yêu cầu mạng thất bại.
err-market-io = Máy chủ cài đặt cục bộ đã gặp lỗi.
err-market-tls = Không thể thiết lập kết nối mã hóa với máy ảnh.
err-apk = Không thể đọc tệp ứng dụng.
err-io = Không thể đọc hoặc ghi một tệp.
err-json = Máy ảnh đã gửi dữ liệu sai định dạng.
err-unknown-locale = Ngôn ngữ này không có sẵn.
err-catalog-encoding = Tệp bản dịch không được lưu dưới dạng văn bản UTF-8. Hãy lưu lại bằng mã hóa UTF-8.
err-internal = Đã xảy ra lỗi nội bộ. Đây là lỗi của ứng dụng.

## Phân tích chức năng
nav-analysis = Phân tích
analysis-title = Phân tích chức năng
analysis-experimental = Thử nghiệm. Tính năng này mới và chỉ mới được thử trên một số ít máy ảnh.
analysis-own-hardware = Chỉ sử dụng trên máy ảnh của chính bạn. Bản thân chế độ dịch vụ đã là một can thiệp mà Sony không dự định; bạn chịu trách nhiệm.
analysis-explanation-toggle = Giải thích: mục đích, quy trình, rủi ro
analysis-purpose-heading = Dùng để làm gì?
analysis-purpose = Phân tích chức năng ghi lại việc máy ảnh lưu một cài đặt menu của chính nó trong giá trị nào của bộ nhớ cài đặt (sao lưu). Kết quả mô tả bộ nhớ cài đặt của máy ảnh này; có thể so sánh với các máy ảnh khác nhưng không thay đổi những gì máy ảnh cung cấp.
analysis-how-heading = Hoạt động như thế nào?
analysis-how = Với mỗi giá trị menu bạn nhập, chương trình đọc toàn bộ bộ nhớ cài đặt một lần, mỗi lần trong một phiên chế độ dịch vụ ngắn riêng. Giữa các lần đó, bạn rút cáp và thay đổi cài đặt trong menu máy ảnh; máy ảnh vẫn bật. Cuối cùng, giá trị đầu tiên được đọc thêm một lần nữa. Các giá trị thay đổi theo menu và trở về giá trị ban đầu ở cuối là các ứng viên. Các giá trị tự thay đổi (bộ đếm, đồng hồ) được để riêng.
analysis-requirements-heading = Yêu cầu
analysis-requirements = Máy ảnh được kết nối qua USB, chế độ USB được đặt thành “Tự động”, và sau mỗi phiên bạn chọn “Lưu trữ dung lượng lớn (MSC)” trên máy ảnh. Trên Windows, chương trình cần quyền quản trị viên vì nó tạm thời gán trình điều khiển cho chế độ dịch vụ. Hãy sạc pin; mỗi phiên mất khoảng một phút.
analysis-read-heading = Các bước đọc
analysis-read = Các bản chụp nhanh chỉ đọc. Không có gì trong cài đặt của máy ảnh bị thay đổi. Rủi ro thấp nhưng không phải bằng không: việc chuyển sang chế độ dịch vụ đã là một can thiệp mà Sony không dự định.
analysis-write-heading = Bước ghi (kiểm tra ghi)
analysis-write = Chỉ khi bạn bật rõ ràng: chương trình ghi giá trị tìm được vào máy ảnh, bạn kiểm tra trong menu sau khi khởi động lại xem giá trị đó có hiệu lực không, rồi giá trị gốc được ghi trở lại. Chỉ giá trị mà chính máy ảnh đã lưu mới được ghi, và chỉ khi tìm thấy đúng một ứng viên phù hợp. Giá trị cũ được lưu trước đó. Dù vậy, bước này vẫn thay đổi máy ảnh; một lỗi có thể khiến máy rơi vào trạng thái mà chỉ trung tâm bảo hành mới sửa được.
analysis-legal-heading = Pháp lý
analysis-legal = Phần mềm được cung cấp mà không có bất kỳ bảo hành nào (giấy phép MIT). Việc can thiệp vào máy ảnh có thể ảnh hưởng đến bảo hành. Chỉ sử dụng chức năng này trên máy ảnh bạn sở hữu. Báo cáo phân tích không chứa số sê-ri và không chứa gì từ bộ nhớ cài đặt ngoại trừ các ứng viên tìm được; báo cáo không bao giờ được tự động gửi đi đâu cả.
analysis-start-heading = Phân tích mới
analysis-setting-label = Cài đặt menu, theo tên máy ảnh gọi
analysis-values-label = Các giá trị menu, phân cách bằng dấu phẩy
analysis-values-hint = Giá trị đầu tiên là giá trị đang được đặt. Từ hai đến tám giá trị; mỗi giá trị tốn một phiên chế độ dịch vụ.
analysis-write-option = Sau đó chạy kiểm tra ghi (thay đổi máy ảnh)
analysis-write-warning = Kiểm tra ghi sẽ thay đổi máy ảnh: nó ghi một giá trị vào bộ nhớ cài đặt và sau đó ghi lại giá trị gốc. Chỉ trên máy ảnh của chính bạn, và bạn tự chịu rủi ro.
analysis-write-consent = Máy ảnh này là của tôi và tôi chịu rủi ro.
btn-analysis-write-enable = Bật kiểm tra ghi
btn-analysis-start = Bắt đầu phân tích
btn-analysis-snapshot = Đọc bản sao lưu
btn-analysis-new = Phân tích mới
analysis-discard-confirm = Hủy bỏ phân tích đang thực hiện? Báo cáo cho đến hiện tại vẫn được lưu.
analysis-access-read = ĐỌC
analysis-access-write = GHI
analysis-access-user = THỦ CÔNG
analysis-access-local = MÁY TÍNH
analysis-step-snapshot = Đọc bản sao lưu, cài đặt đang ở “{ $value }”
analysis-step-change = Đổi sang “{ $value }” trong menu máy ảnh
analysis-step-evaluate = So sánh các bản chụp nhanh
analysis-step-write = Ghi giá trị cho “{ $value }” vào máy ảnh
analysis-step-check = Tắt rồi bật máy ảnh, đọc menu (dự kiến: “{ $value }”)
analysis-step-restore = Ghi lại giá trị gốc cho “{ $value }”
analysis-step-report = Lưu báo cáo phân tích
analysis-do-prepare = CHUẨN BỊ: 1. Trong menu máy ảnh, đặt “{ $setting }” thành “{ $value }” rồi thoát menu bằng nút MENU để máy ảnh lưu cài đặt. 2. Để máy ảnh tiếp tục bật. 3. Cắm cáp và chọn “Lưu trữ dung lượng lớn (MSC)” trên máy ảnh.
analysis-do-change = ĐẾN LƯỢT BẠN: 1. Rút cáp. 2. Máy ảnh vẫn bật. 3. Trong menu máy ảnh, đặt “{ $setting }” thành “{ $value }” rồi thoát menu bằng nút MENU để máy ảnh lưu cài đặt. 4. Cắm lại cáp và chọn “Lưu trữ dung lượng lớn (MSC)”.
analysis-do-check = ĐẾN LƯỢT BẠN: 1. Rút cáp. 2. Tắt rồi bật lại máy ảnh. 3. Xem menu hiển thị gì cho “{ $setting }”.
analysis-do-reconnect = Cắm lại cáp và chọn “Lưu trữ dung lượng lớn (MSC)” trên máy ảnh.
analysis-reading = Bản chụp nhanh { $index } trên { $total }: KHÔNG rút cáp trong khi đang đọc.
analysis-cli-continue = Nhấn Enter để tiếp tục, q để hủy:
analysis-confirm-word = có
analysis-cli-confirm = Gõ “{ $word }” để xác nhận:
analysis-write-declined = Chưa xác nhận. Không có gì được ghi.
analysis-aborted = Đã hủy. Không có gì được ghi.
analysis-snapshot-done = Đã đọc bản chụp nhanh. Máy ảnh đã thoát khỏi chế độ dịch vụ.
analysis-result = Đã so sánh { $compared } giá trị: { $count } ứng viên, { $running } giá trị tự thay đổi.
analysis-candidate-consistent = khớp rõ ràng với menu
analysis-candidate-inconsistent = không khớp rõ ràng
analysis-write-preview-heading = Nội dung sẽ được ghi
analysis-write-preview = Giá trị { $id } ({ $size } byte): từ { $old } (“{ $old-label }”) thành { $new } (“{ $new-label }”). Chính máy ảnh đã lưu giá trị mới khi “{ $new-label }” được đặt.
btn-analysis-write = Ghi
btn-analysis-skip-write = Hoàn tất mà không kiểm tra ghi
analysis-written = Đã ghi giá trị, đọc lại và lưu vĩnh viễn.
analysis-menu-question = Menu hiện đang hiển thị gì?
analysis-menu-other = giá trị khác
btn-analysis-menu-confirm = Xác nhận
btn-analysis-restore = Ghi lại giá trị gốc
analysis-restored = Đã ghi lại giá trị gốc và lưu vĩnh viễn.
analysis-survived = Giá trị đã ghi vẫn được lưu sau khi khởi động lại.
analysis-not-survived = Giá trị đã ghi không còn được lưu sau khi khởi động lại.
analysis-report-saved = Đã lưu báo cáo phân tích: { $path }
analysis-report-hint = Báo cáo không chứa số sê-ri và không chứa bản sao lưu. Bạn có thể chia sẻ báo cáo để kết quả được kiểm chứng và ghi nhận.
btn-analysis-export = Xuất báo cáo…
file-filter-analysis = Báo cáo phân tích
analysis-pending = Có một lần kiểm tra ghi chưa hoàn tất. Giá trị gốc đã được lưu và có thể được ghi lại.
analysis-pending-none = Không có lần kiểm tra ghi nào chưa hoàn tất được lưu cho máy ảnh này.
analysis-invalid-setting = Vui lòng nhập cài đặt menu theo tên máy ảnh gọi.
analysis-invalid-values = Vui lòng nhập từ hai đến { $max } giá trị menu khác nhau. Giá trị đầu tiên là giá trị đang được đặt.
analysis-already-complete = Đã đọc tất cả các bản chụp nhanh.
analysis-other-camera = Đây không phải cùng máy ảnh như trong bản chụp nhanh đầu tiên. Phân tích đã dừng.
analysis-write-not-allowed = Không thể kiểm tra ghi ở trạng thái này.
analysis-camera-changed = Máy ảnh hiện giữ một giá trị khác so với bản chụp nhanh đầu tiên. Không có gì được ghi.
analysis-write-refused = Máy ảnh đã từ chối giá trị; mọi thứ vẫn như trước. ({ $detail })
analysis-refuse-none = Không thể kiểm tra ghi: không có giá trị nào thay đổi theo menu.
analysis-refuse-ambiguous = Không thể kiểm tra ghi: có { $count } giá trị thay đổi theo menu, không giá trị nào có thể được xác định chắc chắn.
analysis-refuse-inconsistent = Không thể kiểm tra ghi: giá trị không khớp rõ ràng với các giá trị menu.
analysis-refuse-size = Không thể kiểm tra ghi: giá trị có kích thước { $size } byte; chỉ các giá trị nhỏ mới được ghi.
analysis-refuse-read-only = Không thể kiểm tra ghi: máy ảnh đánh dấu giá trị này là chỉ đọc.
analysis-refuse-protected = Không thể kiểm tra ghi: máy ảnh đánh dấu giá trị này là được bảo vệ.
settings-analysis-heading = Phân tích chức năng
settings-analysis-purpose = Phân tích chức năng ghi lại việc máy ảnh lưu một cài đặt menu của chính nó trong giá trị nào của bộ nhớ cài đặt. Tính năng này dành cho những ai muốn hiểu bộ nhớ cài đặt của kiểu máy mình và chia sẻ kết quả.
settings-analysis-why-off = Tính năng này bị tắt vì việc sử dụng thông thường không cần đến nó, vì nó chuyển máy ảnh sang chế độ dịch vụ nhiều lần, và vì khi được yêu cầu, nó còn ghi vào máy ảnh. Đây là tính năng thử nghiệm và mới chỉ được thử trên một số ít kiểu máy.
settings-analysis-toggle = Bật phân tích chức năng (mục “Phân tích” trong menu)
settings-analysis-confirm = Bật phân tích chức năng? Tính năng này chuyển máy ảnh sang chế độ dịch vụ nhiều lần và có thể ghi vào máy ảnh khi được yêu cầu. Chỉ sử dụng trên máy ảnh của chính bạn.
btn-analysis-enable = Bật
settings-analysis-on = Phân tích chức năng đang bật.
settings-analysis-off = Phân tích chức năng đang tắt.
analysis-needs-admin = Mỗi bản chụp nhanh cần quyền quản trị viên cho chế độ dịch vụ. Windows yêu cầu quyền này một lần cho mỗi phiên chương trình; bạn có thể khởi động trình hỗ trợ trình điều khiển ngay bây giờ hoặc chờ đến khi bản chụp nhanh đầu tiên yêu cầu.
analysis-changed-hint = Vui lòng kiểm tra: không tìm thấy ứng viên nào. Các giá trị sau đã thay đổi nhưng không trở về giá trị ban đầu. Nếu một trong số chúng chỉ thay đổi một lần, giữa hai giá trị, có lẽ máy ảnh đã không ở giá trị menu đầu tiên khi chụp bản chụp nhanh đầu tiên. Khi đó hãy lặp lại phân tích với giá trị ban đầu được đặt đúng.
analysis-changed-info = Thông tin: các giá trị tự thay đổi trong quá trình phân tích (ví dụ bộ đếm, đồng hồ) được bỏ qua. Không cần làm gì.
elevation-admin = Quản trị viên
elevation-standard = Quyền tiêu chuẩn
elevation-admin-hint = Chương trình đang chạy với quyền quản trị viên và có thể gán trình điều khiển mà chế độ dịch vụ cần.
elevation-standard-hint = Chương trình đang chạy với quyền tiêu chuẩn. Đối với chế độ dịch vụ, tinh chỉnh và phân tích chức năng, khi cần chương trình sẽ khởi động một trình hỗ trợ trình điều khiển với quyền quản trị viên; Windows hỏi một lần mỗi phiên. Nhấp để khởi động.
driver-helper-declined = Lời nhắc cấp quyền quản trị viên đã bị từ chối. Nếu không có quyền này, không thể gán trình điều khiển cho chế độ dịch vụ; không có gì bị thay đổi.
btn-remove-foreign-drivers = Xóa trình điều khiển cũ
btn-not-now = Để sau
btn-ok = OK
foreign-drivers-found = Windows vẫn còn giữ các trình điều khiển mà Zadig hoặc công cụ tương tự đã cài đặt cho thiết bị Sony: { $packages }. Windows giao máy ảnh cho các trình điều khiển này trước khi chương trình này kịp gán trình điều khiển riêng, nên chế độ dịch vụ thất bại và máy ảnh có thể không hiển thị dưới dạng ổ đĩa. Xóa chúng ngay bây giờ? Khi đó máy ảnh sẽ dùng các trình điều khiển đi kèm Windows. Việc xóa cần quyền quản trị viên một lần.
foreign-drivers-removed = Đã xóa các trình điều khiển cũ. Hãy rút máy ảnh ra rồi cắm lại.
foreign-drivers-remaining = Không thể xóa các trình điều khiển sau: { $packages }. Hãy khởi động lại Windows và thử lại, hoặc gỡ cài đặt thiết bị cùng với trình điều khiển của nó trong Trình quản lý Thiết bị.
foreign-drivers-hint = Vẫn còn các trình điều khiển mà Zadig hoặc công cụ tương tự đã cài đặt cho thiết bị Sony ({ $packages }). Windows giao máy ảnh cho các trình điều khiển này trước khi chương trình này kịp gán trình điều khiển riêng, điều này có thể khiến chế độ dịch vụ thất bại. Hãy xóa chúng bằng: pmca-console driver zadig --remove
driver-zadig-none = Không có trình điều khiển nào từ Zadig hoặc công cụ tương tự được cài đặt cho thiết bị Sony.
driver-zadig-entry = { $package } ({ $provider }): { $devices }
driver-zadig-remove-hint = Hãy xóa chúng bằng: pmca-console driver zadig --remove
driver-zadig-removed = Đã xóa { $count } gói trình điều khiển cũ. Hãy rút máy ảnh ra rồi cắm lại.
elevation-helper = Trình hỗ trợ trình điều khiển đang hoạt động
elevation-helper-hint = Một trình hỗ trợ trình điều khiển có quyền quản trị viên sẽ gán trình điều khiển cho chế độ dịch vụ. Trình này không có cửa sổ và kết thúc cùng chương trình; sẽ không có lời nhắc nào nữa.
elevation-helper-confirm = Khởi động trình hỗ trợ trình điều khiển ngay bây giờ? Windows sẽ hỏi quyền quản trị viên một lần và không hỏi lại cho đến khi chương trình kết thúc.
analysis-cross-heading = Kiểm tra chéo trên máy ảnh khác
analysis-cross-intro = Tùy chọn: kiểm tra xem máy ảnh khác cũng có giá trị đã tìm được hay không và giá trị đó được đặt thế nào. Thao tác này chỉ đọc. Cùng một số không nhất thiết có nghĩa là cùng một chức năng; chỉ khi xem menu của máy ảnh kia mới xác nhận được điều đó. Kết quả được đưa vào cùng báo cáo phân tích.
analysis-cross-do = 1. Rút máy ảnh đã phân tích ra. 2. Bật máy ảnh kia, đặt chế độ USB thành “Tự động”, cắm cáp và chọn “Lưu trữ dung lượng lớn (MSC)”.
btn-analysis-cross = Kiểm tra máy ảnh khác
analysis-cross-present = có, giá trị { $value }
analysis-cross-match = bằng “{ $label }”
analysis-cross-nomatch = không bằng giá trị menu nào đã đo
analysis-cross-absent = không có
analysis-cross-same-camera = Đây chính là máy ảnh đã được phân tích. Vui lòng kết nối máy ảnh kia.
analysis-cross-not-now = Chỉ có thể kiểm tra chéo khi đã đọc mọi bản chụp nhanh và không còn giá trị đã ghi nào đang chờ khôi phục.
analysis-cross-none = Không có ứng viên nào để kiểm tra chéo.
analysis-wrong-camera = Máy ảnh đang kết nối không phải là máy đã được phân tích. Vui lòng kết nối { $model }; không có gì được ghi.
keys-fetching = Đang tải xuống nguồn…
keys-scanning = Đang tìm dữ liệu khóa…
keys-complete = Đã có đầy đủ dữ liệu khóa.
keys-incomplete = Vẫn còn thiếu một số dữ liệu khóa.
keys-present = có
keys-missing = thiếu
keys-class-senser = Khóa chế độ dịch vụ (trình bao dịch vụ, tinh chỉnh, phân tích)
keys-class-cic = Khóa tổng kiểm XPD (cài đặt ứng dụng)
keys-class-spk = Dữ liệu khóa SPK (cài đặt ứng dụng, apk2spk)
keys-class-fdat = Dữ liệu khóa FDAT (phần sụn của trình bao cập nhật)
plugin-heading = Plugin
plugin-intro = Plugin mở rộng chương trình này bằng ngôn ngữ, công tắc cho cài đặt máy ảnh, ghi chú tương thích, công cụ và chẩn đoán. Plugin đến từ các tệp do chính bạn chọn hoặc từ các nguồn bạn đã thêm; chương trình này không kiểm tra chúng. Mã của plugin chạy trong môi trường cách ly, không có quyền truy cập vào tệp của bạn, mạng hay dữ liệu khóa. Chỉ cài đặt plugin có tác giả mà bạn tin tưởng; các thay đổi có hiệu lực ở lần khởi động tiếp theo.
plugin-none = Chưa cài đặt plugin nào.
plugin-inactive = (hoạt động sau khi khởi động lại)
plugin-author = Tác giả
plugin-restart = Khởi động lại chương trình để thay đổi có hiệu lực.
btn-plugin-install = Cài đặt plugin…
plugin-install-confirm = Plugin này có thể:
btn-confirm-plugin-install = Cài đặt plugin
plugin-installed = Đã cài đặt plugin.
plugin-remove-confirm = Xóa plugin “{ $name }”?
plugin-removed = Đã xóa plugin { $id }.
plugin-perm-languages = thêm ngôn ngữ vào chương trình
plugin-perm-camera-settings = thay đổi cài đặt máy ảnh (chỉ giữa hai giá trị mà plugin nêu ra và chỉ sau khi bạn xác nhận)
plugin-perm-compatibility = thêm mục vào bảng tổng quan khả năng tương thích
plugin-perm-code = chạy mã riêng trong môi trường cách ly (công cụ hoặc chẩn đoán)
plugin-perm-host-os = hỏi đây là hệ điều hành nào
plugin-perm-host-cameras = xem những máy ảnh Sony nào đang kết nối và ở chế độ nào
plugin-invalid = Đây không phải là plugin dùng được: { $reason }
plugin-unknown-field = Plugin này dùng một tính năng mà phiên bản chương trình này không biết ({ $field }). Plugin được tạo cho một phiên bản mới hơn hoặc một ấn bản khác của chương trình.
plugin-no-folder = Không thể xác định thư mục dữ liệu người dùng, nên không thể cài đặt plugin.
plugin-not-installed = Không có plugin nào với mã { $id } được cài đặt.
plugin-failed = Plugin đã gặp lỗi: { $reason }
plugin-unknown-entry = Không có plugin nào đã tải cung cấp { $id }.
plugin-run-done = Plugin đã hoàn tất.
plugin-run-warning = Plugin đã hoàn tất kèm theo ghi chú.
plugin-run-error = Plugin đã báo lỗi.
plugin-tools-heading = Công cụ plugin
plugin-tools-hint = Các công cụ này đến từ plugin đã cài đặt và chạy trong môi trường cách ly. Plugin chỉ nhận những gì bạn giao cho nó và những gì nó đã khai báo, và không thể thay đổi máy ảnh.
plugin-diagnostics-heading = Chẩn đoán plugin
plugin-compat-heading = Từ một plugin
plugin-col-note = Ghi chú
file-filter-plugin = Tệp plugin
keys-not-set-up = Dữ liệu khóa mà thao tác này cần chưa được thiết lập.
err-missing-keys = Dữ liệu khóa mà thao tác này cần chưa được thiết lập.
keys-heading = Cần có dữ liệu khóa
keys-intro = Khóa chế độ dịch vụ, khóa tổng kiểm XPD và dữ liệu khóa SPK không phải là một phần của chương trình này. Hãy nhập địa chỉ của một nguồn mà bạn có — kho mã, tệp nén hoặc một tệp — và chương trình sẽ tìm và nhập những phần mà nó nhận ra. Không có gì được thực thi, và dữ liệu chỉ được lưu trên máy tính này.
keys-law-notice = Hãy tuân thủ pháp luật áp dụng tại nơi bạn sinh sống, và chỉ làm việc với máy ảnh và dữ liệu của chính bạn.
keys-url-label = Địa chỉ nguồn (mỗi dòng một địa chỉ)
keys-url-required = Vui lòng nhập ít nhất một địa chỉ nguồn.
btn-keys-import = Nhập
btn-import-keys = Thiết lập dữ liệu khóa
