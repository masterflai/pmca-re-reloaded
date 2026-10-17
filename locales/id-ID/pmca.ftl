# Pelokalan bahasa Indonesia untuk PMCA-RE RELOADED.
# Terjemahkan hanya teks di sebelah kanan tanda =. Jangan pernah mengubah ID.

app-title = PMCA-RE RELOADED – Perkakas Kamera

# Navigasi / tab
nav-camera-info = Info kamera
nav-install-app = Aplikasi
nav-tweaks = Penyesuaian
nav-network = Jaringan
nav-firmware = Firmware
nav-updater = Shell updater
nav-service = Shell servis
nav-tools = Perkakas berkas
nav-settings = Pengaturan
nav-license = Lisensi

# Halaman sambutan: halaman yang dibuka pertama kali oleh jendela ini.
nav-welcome = Selamat datang
welcome-intro-heading = Apa yang dilakukan program ini
welcome-intro-body = Program ini membantu mempertahankan fungsi kamera Sony Anda dan menyesuaikannya dengan kebutuhan Anda. Setelah Sony menutup toko PlayMemories Camera Apps, tidak ada lagi cara untuk mencadangkan atau memasang ulang aplikasi yang dibeli atau sudah terpasang. Reset kamera menghapus aplikasi yang terpasang secara permanen. Program ini memungkinkan Anda mencadangkan aplikasi yang ada dan memasangnya kembali, serta dapat menyediakan aplikasi komunitas yang tersedia secara bebas. Anda juga dapat memasang aplikasi Anda sendiri (berkas *.apk). Saat terjadi masalah, program ini membaca info kamera dan log internal untuk analisis. Pengaturan yang sudah dimiliki kamera tetapi tidak ditawarkan menunya dapat diubah dan dibatalkan kembali. Program ini berbicara langsung dengan kamera melalui kabel USB. Model mana saja yang didukung tercantum di halaman Info kamera.
welcome-start-heading = Langkah pertama
welcome-start-body = Nyalakan kamera lebih dulu dan, selagi kabel belum terpasang, atur “Koneksi USB” di menunya ke Penyimpanan massal atau MTP. “Auto” biasanya juga berhasil, karena kamera lalu memilih sendiri salah satu dari keduanya. Urutannya penting: begitu kabel terpasang, menu kamera tidak dapat dicapai lagi. Barulah setelah itu hubungkan kamera melalui USB, buka Info kamera dan tekan “Ambil info kamera”. Begitu model dan versi firmware muncul, sambungan sudah benar dan semua fungsi lain program ini tersedia.
welcome-start-driver = Jika yang muncul justru pemberitahuan tentang driver, tangani itu lebih dulu. Di Windows sebuah driver sistem menguasai kamera, dan driver itu harus diganti sebelum akses USB langsung yang dibutuhkan perangkat ini menjadi mungkin. Program melakukannya sendiri, tetapi menetapkan driver adalah hak yang hanya diberikan Windows kepada administrator. Dalam hal ini, program memberi Anda pilihan untuk memulai ulang program sebagai administrator, dan Windows akan meminta Anda mengonfirmasi tindakan tersebut. Penetapan itu dilepas kembali saat sesi berakhir. Di Linux dan macOS hal ini tidak diperlukan.
welcome-start-driver-linux = Jika yang muncul justru pemberitahuan tentang driver, tangani itu lebih dulu. Di Linux tidak ada yang perlu diganti: kamera dapat dijangkau langsung. Yang diperlukan sekali saja adalah hak untuk membuka perangkat — sebuah aturan udev untuk kode produsen Sony — dan, untuk mode penyimpanan massal, melepaskan driver kernel usb-storage. Keduanya disiapkan di luar program ini dengan hak root; skrip scripts/linux-prepare.sh dalam kode sumber melakukan keduanya. Jendela ini sendiri tidak pernah meminta hak semacam itu.
welcome-start-driver-macos = Jika yang muncul justru pemberitahuan tentang driver, tangani itu lebih dulu. Di macOS tidak ada yang perlu dipasang atau diganti, dan jendela ini tidak pernah meminta hak administrator. Setel koneksi USB kamera ke MTP: dalam mode penyimpanan massal, driver disk sistem menahan kamera sehingga kamera tidak dapat dijangkau.
welcome-tweaks-heading = Penyesuaian dan bahasa
welcome-tweaks-body = Penyesuaian adalah pengaturan yang sudah dimiliki kamera tetapi tidak ditawarkan di menunya: batas durasi perekaman, batas perekaman 4K, pilihan antara PAL dan NTSC, pemasang aplikasi lewat USB — dan bahasa menu.
welcome-tweaks-languages-heading = Bahasa pada kamera
welcome-tweaks-languages-body = Kamera Sony hanya menampilkan bahasa menu yang lazim di wilayah penjualannya. Karena itu, orang yang tinggal di luar negeri sering tidak menemukan bahasanya sendiri dan harus puas dengan bahasa asing. Padahal bahasa lain sudah tersimpan di firmware. Program ini menghadirkannya di menu agar Anda dapat mengoperasikan kamera dalam bahasa yang akrab.
welcome-tweaks-ui-heading = Bahasa program ini
welcome-tweaks-ui-body = Terpisah dari semua itu adalah pemilih di bagian atas. Ia hanya mengubah bahasa program ini dan tidak pernah menyentuh kamera.
welcome-tweaks-safety = Setiap penyesuaian ditulis ke memori cadangan kamera, dan setiap penyesuaian dapat ditarik kembali dengan sakelar yang sama. Penulisan yang terputus dapat merusak kamera secara permanen, jadi jangan cabut kabel maupun lepas baterai selama proses berlangsung.
welcome-privacy-heading = Data Anda
welcome-privacy-where = Halaman "Lisensi" menguraikan semua ini sekali lagi secara lengkap, beserta lisensi seluruh pustaka yang membangun program ini.
welcome-where-heading = Di mana menemukan apa
welcome-where-info = Model, nomor seri, dan versi firmware. Sekaligus uji sambungan.
welcome-where-install = Aplikasi dari daftar komunitas, atau berkas .apk milik Anda sendiri.
welcome-where-tweaks = Pengaturan yang tidak dijangkau menu kamera, termasuk bahasa.
welcome-where-network = Data bantuan GPS, serta titik akses dan layanan untuk siaran langsung.
welcome-where-firmware = Menulis berkas citra firmware Sony ke kamera.
welcome-where-tools = Membongkar dan memeriksa berkas firmware dan paket. Tanpa kamera.
welcome-where-shells = Akses langsung ke memori dan sistem berkas kamera. Tampilkan dulu di Pengaturan.
welcome-where-settings = Tema, mode lanjutan, dan penyuntingan terjemahan program ini.
welcome-where-license = Dari mana alih bahasa program ini berasal, di atas karya siapa ia dibangun, dan dengan syarat apa ia terbit.
welcome-where-hint = Tidak ada halaman yang menyentuh kamera dengan sendirinya. Tidak ada yang dibaca atau ditulis sampai Anda menekan tombol yang menyatakannya.

# Tombol
btn-get-info = Ambil info kamera
btn-refresh = Segarkan
btn-load-apps = Muat aplikasi yang tersedia
btn-list-tweaks = Tampilkan penyesuaian yang tersedia
btn-install = Pasang aplikasi terpilih
btn-install-file = Pasang berkas .apk…
btn-apply = Terapkan
btn-save = Simpan ke kamera
btn-add = Tambah
btn-remove = Hapus
btn-reveal = Tampilkan
btn-update-gps = Transfer data bantuan GPS ke perangkat
btn-cancel = Batal
btn-browse = Telusuri…
btn-switch-msc = Beralih ke mode penyimpanan massal
btn-start-driver-helper = Mulai pembantu driver

# Filter jenis berkas pada dialog berkas
file-filter-apk = Aplikasi Android (*.apk)
file-filter-firmware = Citra firmware Sony (*.dat)
file-filter-usage-log = Log penggunaan (*.bin)
file-filter-catalog = Berkas terjemahan (*.ftl)
file-filter-spk = Paket aplikasi Sony (*.spk)
file-filter-any = Semua berkas
file-filter-gps = Data bantuan GPS (*.dat)

# Status / pesan
msg-connecting = Menghubungkan…
msg-usage-log-written = Log penggunaan ditulis ke { $path }
msg-apps-found = Ditemukan { $count } aplikasi
msg-switch-mode = Silakan alihkan kamera ke mode MTP atau penyimpanan massal.
msg-no-tweaks = Tidak ada penyesuaian yang tersedia untuk kamera ini.
msg-done = Selesai.

# Peralihan mode USB kamera (MTP → penyimpanan massal). Ditawarkan di bawah
# pesan galat "mode salah"; tidak ada perintah USB untuk arah sebaliknya.
mode-switching = Meminta kamera beralih ke mode penyimpanan massal…
mode-switch-wait = Menunggu kamera terhubung kembali…
camera-confirm-hint = Pesan di kamera (mis. tentang www.sony.net/pm) dapat ditutup; pesan itu tidak mengganggu proses.
mode-switch-done = Kamera kini berada dalam mode penyimpanan massal.
mode-switch-timeout = Kamera tidak muncul kembali dalam mode penyimpanan massal. Matikan dan hidupkan kembali, lalu coba lagi.
mode-switch-impossible = Mode ini tidak dapat diubah melalui USB. Silakan matikan dan hidupkan kembali kamera.
mode-switch-hint = Hanya kamera dalam mode MTP yang dapat dialihkan melalui USB. Perubahan ini bersifat sementara: mode USB yang dipilih di menu kamera itu sendiri tidak berubah, sehingga mematikan lalu menghidupkan kamera — atau mencabut dan memasang kembali kabel — akan mengembalikan mode sebelumnya. Tidak ada perintah USB untuk arah sebaliknya.

# Ruas info kamera
info-model = Model
info-product-code = Kode produk
info-serial = Nomor seri
info-firmware = Versi firmware
info-lens = Lensa
info-region = Wilayah
info-mode = Mode
mode-mass-storage = Penyimpanan massal
mode-mtp = MTP
mode-updater = Mode updater
mode-app-install = Mode pemasangan aplikasi
mode-service = Mode servis
mode-unknown = Tidak diketahui
info-android-version = Versi platform Android
info-photographer = Fotografer
info-copyright = Hak cipta
info-photographer-1 = Fotografer (entri 1)
info-photographer-2 = Fotografer (entri 2)
info-copyright-1 = Hak cipta (entri 1)
info-copyright-2 = Hak cipta (entri 2)
diagnose-services-heading = Kamera tidak terjangkau?
diagnose-services-hint = Beberapa layanan Windows dapat menahan kamera dalam mode MTP sehingga aplikasi ini tidak dapat menjangkaunya. Pemeriksaan ini hanya menanyakan kepada Windows layanan mana yang berjalan; tidak ada yang diubah.
btn-diagnose-services = Periksa layanan Windows
diagnose-services-none = Tidak ada layanan pengganggu yang dikenal sedang berjalan.
diagnose-services-found = Layanan yang sedang berjalan ini dapat mengganggu akses ke kamera:
diagnose-services-advice = Hentikan layanan di Services (services.msc) atau dengan perintah berikut sebagai administrator, lalu sambungkan ulang kamera:

# Updater firmware / shell
updater-model = Model kamera
updater-firmware-current = Versi firmware saat ini
updater-guard-ok = Citra firmware cocok dengan kamera ini.
updater-no-fdat = Tidak ada firmware updater bawaan untuk model { $model }.
updater-model-required = Kamera sudah berada dalam mode updater dan di sana tidak dapat melaporkan modelnya. Matikan kamera lalu nyalakan lagi agar ia mulai dalam mode normal, lalu coba lagi. Sebagai alternatif, tentukan berkas payload (--fdat-file) atau, di konsol, modelnya (--model <MODEL>).
updater-already-updater = Kamera sudah berada dalam mode updater.
updater-flash-required = Kamera tidak akan diubah tanpa konfirmasi. Jalankan ulang dengan --flash untuk beralih ke mode updater dan menulis payload.
updater-flashing = Menulis payload updater ke kamera…
updater-switch-wait = Ikuti petunjuk di layar kamera. Menunggu kamera beralih ke mode updater…
updater-switch-timeout = Waktu tunggu kamera habis. Jalankan perintah lagi setelah kamera terhubung kembali.
updater-shell-ready = Payload updater sedang berjalan. Ketik 'help' untuk daftar perintah, 'exit' untuk keluar.
updater-shell-done = Sesi shell berakhir.
updater-session-panic = Sesi updater berakhir tanpa diduga. Matikan lalu nyalakan lagi kamera sebelum mencoba ulang.

# Jawaban shell updater. Kesalahan saat membaca satu baris hanya dirumuskan
# sekali, di kunci service-*: kedua shell memakai pengurai yang sama, jadi
# jalur dalam tanda kutip dan nilai heksadesimal yang keliru gagal dengan cara
# yang sama di keduanya.
updater-backup-synced = Cadangan ditulis ke memori flash.
updater-backup-locked = Pengaturan cadangan yang terlindungi kini terkunci.
updater-backup-writable = Pengaturan cadangan yang terlindungi kini dapat ditulis.
updater-shell-console-only = Shell root memerlukan terminal dan hanya tersedia dari baris perintah: jalankan pmca-console updatershell --flash lalu ketik “shell” di sana.

# Halaman shell updater.
updater-warning = Peringatan: untuk mencapai shell ini kamera dinyalakan ulang ke mode updater dan sebuah payload ditulis ke dalamnya — program kecil dari proyek ini yang diterima kamera seperti pembaruan firmware dan yang kemudian menjawab perintah shell ini. Payload itu hilang begitu kamera dimatikan, tetapi perintah yang ditawarkannya menulis ke memori pengaturan permanen kamera, dan penulisan yang terputus dapat merusak kamera secara permanen. Selain itu, semua pengaturan kamera diatur ulang dalam prosesnya — berbeda dengan shell servis.
updater-console-heading = Menjalankan shell updater dari baris perintah
updater-driver-hint = Shell updater menuntut kamera tetap memakai driver penyimpanan massal bawaan Windows, USBSTOR, karena lewat driver itulah payload dikirim. Ini kebalikan dari yang dibutuhkan shell servis. Aplikasi menetapkan dan melepas driver WinUSB sendiri, jadi tidak ada yang perlu disiapkan secara manual — tetapi driver yang ditetapkan alat lain harus dibatalkan lebih dulu. Kamera dalam mode updater muncul dengan id produknya sendiri dan juga tidak boleh dialihkan drivernya.
updater-fdat-label = Berkas payload (opsional)
updater-confirm = Kamera akan dinyalakan ulang ke mode updater dan sebuah payload akan ditulis ke dalamnya. Lanjutkan?
updater-cancelled = Dibatalkan. Kamera tidak diubah.

btn-shell-primer = Hal penting tentang shell ini
btn-shell-primer-hide = Sembunyikan penjelasan
shell-primer-compare-heading = Dua shell, dua protokol
shell-primer-compare = Kedua halaman menyediakan baris perintah ke kamera, tetapi keduanya bukan shell yang sama. Shell servis berbicara dengan protokol servis pabrik milik Sony sendiri, yang dijawab kamera secara langsung setelah otentikasi; tidak ada apa pun yang ditulis ke kamera untuk sampai ke sana. Shell pembaru hanya ada karena proyek ini menulis programnya sendiri ke dalam kamera: kamera memulai ulang ke mode pembaru, menerima payload itu seolah-olah firmware, dan payload itulah yang kemudian menjawab perintah di bawah. Hanya satu dari keduanya yang dapat terbuka pada satu waktu.
shell-primer-payload = “Payload” (bahasa Inggris untuk muatan) adalah isi sebenarnya yang dibawa oleh suatu jalur pengiriman. Pada shell pembaru, itu adalah program kecil dari proyek ini: kamera menerimanya lewat jalur yang sama dengan pembaruan firmware, tetapi hanya menyimpannya sampai dimatikan. Shell servis tidak memerlukan payload, karena kamera menjawab perintahnya dengan firmware-nya sendiri. Pada berkas firmware, istilah ini berarti hal yang sama: citra firmware sebenarnya di dalam wadah berkas .dat.
shell-primer-when-service = Shell servis dipakai untuk segala hal yang membaca atau mengubah pengaturan tersimpan kamera selagi kamera tetap menjalankan firmware-nya sendiri: membaca blok HASP, menyimpan salinan pemulihan area cadangan, mengubah entri satuan, menarik berkas dari kamera. Halaman Penyesuaian juga memakainya di baliknya; semua dari sana bisa dikerjakan di sini secara manual.
shell-primer-when-updater = Shell pembaru adalah pilihan ketika kamera tidak dapat dijangkau dengan cara lain, atau ketika Anda memerlukan shell pada sistem yang sedang berjalan alih-alih perintah satuan. Hanya jalur ini yang memberi shell root sungguhan, dan ia tetap bekerja pada kamera yang menolak mode servis. Ia menuntut satu kali mulai ulang, satu kali penulisan, dan pengaturan kamera, jadi ini pilihan kedua, bukan pertama.
shell-primer-driver-heading = Driver: keduanya menuntut hal yang berlawanan
shell-primer-driver = Shell servis berbicara USB mentah dan di Windows menuntut kamera terikat ke WinUSB. Aplikasi menetapkan driver itu sendiri dalam perjalanan ke mode servis dan melepasnya kembali saat sesi berakhir, jadi tidak ada alat terpisah yang terlibat. Shell pembaru lewat USBSTOR, driver penyimpanan massal bawaan Windows, dan berhenti bekerja selama kamera terikat ke WinUSB — karena itu melepas ikatan termasuk bagian dari mengakhiri sesi servis. Menetapkan driver memerlukan hak administrator. Di Linux tidak perlu pergantian driver untuk keduanya, cukup akses ke node perangkat.
shell-primer-risk-heading = Apa yang bisa salah
shell-primer-risk = Perintah yang ditandai di bawah menulis ke memori pengaturan permanen kamera. Penulisan yang terputus dapat membuat kamera tidak bisa menyala, dan tidak satu pun dari itu bisa dibatalkan — justru untuk itulah bk d ada: simpan salinan area cadangan sebelum Anda mengubah apa pun. Payload pembaru sendiri tidak tinggal di kamera; mematikan kamera akan menghapusnya. Namun, jalan melalui mode pembaru mengatur ulang semua pengaturan kamera.
shell-primer-commands-heading = Perintah pada shell ini
shell-primer-writes = Perintah bertanda (WRITES) mengubah kamera. Selebihnya hanya membaca.
shell-cmd-hasp = Membaca blok HASP, area identitas kamera yang terlindungi.
shell-cmd-pull = Mengambil berkas dari kamera. Tanpa nama lokal, nama di kamera dipertahankan.
shell-cmd-push = Menyalin berkas lokal ke kamera, menggantikan apa yang ada di sana. (WRITES)
shell-cmd-rm = Menghapus berkas di kamera. (WRITES)
shell-cmd-rmem = Menulis satu wilayah memori perangkat ke berkas lokal. Ofset dan panjang dalam heksadesimal.
shell-cmd-bk-r = Membaca satu entri area cadangan dan menampilkannya sebagai dump heksadesimal.
shell-cmd-bk-d = Menyimpan seluruh area cadangan ke sebuah berkas. Lakukan ini sebelum mengubah apa pun: inilah satu-satunya jalan kembali.
shell-cmd-bk-dump-txt = Menyimpan seluruh area cadangan sebagai <file>.bin dan daftar yang mudah dibaca sebagai <file>.txt.
shell-cmd-bk-stat = Menampilkan blok status cadangan kamera, termasuk wilayahnya.
shell-cmd-bk-w = Menulis satu entri area cadangan, diberikan sebagai bita heksadesimal dipisahkan spasi. (WRITES)
shell-cmd-bk-s-sub = Menetapkan area cadangan satu subsistem ke flash agar perubahan bertahan setelah mulai ulang. (WRITES)
shell-cmd-bk-s = Menetapkan area cadangan ke flash agar perubahan bertahan setelah mulai ulang. (WRITES)
shell-cmd-bk-lock = Mengaktifkan atau menonaktifkan perlindungan tulis pada entri cadangan yang terlindungi; sebagian pengaturan menolak berubah selama perlindungan aktif. (WRITES)
shell-cmd-term = Menyalakan atau mematikan terminal awakutu kamera. (WRITES)
shell-cmd-info = Membaca blok status cadangan, yang memberitahu apakah area itu dapat ditulisi sama sekali.
shell-cmd-shell = Membuka shell root pada kamera. Ia memerlukan terminal, jadi hanya tersedia di konsol; shell tcp <port> menawarkannya di 127.0.0.1 untuk klien lain. (WRITES)
shell-cmd-help = Menampilkan daftar perintah yang diterima shell ini.
shell-cmd-exit = Mengakhiri sesi dan keluar dari shell.
shell-primer-diagnostics-heading = Diagnostik dan perbaikan
shell-cmd-bk-trace = Merekam ke sebuah file, paket demi paket, cara kamera mengirim seluruh area cadangan melalui USB. Untuk menyelidiki cadangan berukuran besar; dapat membuat kamera macet di tengah balasan. Jika demikian, cabut kabel lalu matikan dan nyalakan kamera, bila perlu dengan melepas baterai.
shell-cmd-bk-rewrite = Membaca area cadangan, menulisnya kembali tanpa perubahan, lalu membacanya lagi. Menunjukkan apakah kamera menerima image utuh, tanpa mengubah pengaturan apa pun. (WRITES)
shell-cmd-bk-restore-attrs = Mengembalikan atribut semua entri cadangan, misalnya perlindungan tulis, ke keadaan dalam salinan yang disimpan dengan bk d atau bk dump-txt. Nilainya tidak berubah. Gunakan hanya salinan dari kamera yang sama. (WRITES)
shell-primer-diagnostics-env = Tiga variabel lingkungan berlaku untuk konsol maupun antarmuka grafis jika diatur sebelum dijalankan, misalnya di PowerShell dengan $env:PMCA_TWEAKS_DIRECT = "1". PMCA_TWEAKS_DIRECT membuat tweak menulis nilai satu per satu, meskipun cadangan lebih kecil dari 1 MB. PMCA_TWEAKS_IMAGE membuatnya menulis image utuh, meskipun lebih besar dari 1 MB; jika keduanya diatur, PMCA_TWEAKS_DIRECT yang berlaku. PMCA_TLS_DUMP=<file> menyimpan pesan TLS pertama kamera (ClientHello) saat instalasi aplikasi, untuk menyelidiki koneksi yang gagal.

# Pemasangan aplikasi
install-switching = Beralih ke mode pemasangan aplikasi…
install-disabled = Kamera menolak masuk ke mode pemasangan aplikasi. Biasanya penyebabnya adalah pengaturan "Aktifkan pemasang aplikasi USB": pengaturan itu harus dinyalakan di bagian "Penyesuaian".
install-stays-app-mode = Kamera tetap berada dalam mode pemasangan aplikasi, tempat aplikasi lain masih dapat dipasang. Untuk semua fungsi lainnya, cabut kabel USB, lalu matikan dan hidupkan kembali kamera.
err-usb-app-install-mode = Kamera masih berada dalam mode pemasangan aplikasi dan dalam mode ini hanya menerima pemasangan aplikasi. Cabut kabel USB, lalu matikan dan hidupkan kembali kamera — jika dinyalakan ulang dalam keadaan terhubung, kamera langsung kembali ke mode ini.
install-switch-wait = Menunggu kamera terhubung kembali dalam mode pemasangan aplikasi…
install-switch-timeout = Waktu tunggu kamera masuk ke mode pemasangan aplikasi habis. Jalankan perintah lagi setelah kamera terhubung kembali.
install-bad-mode = Kamera tidak dapat digunakan dalam mode ini. Silakan alihkan ke mode MTP atau penyimpanan massal.
install-starting = Memulai pemasangan…
install-error = Galat komunikasi saat pemasangan
install-server-failed = Server pemasangan lokal tidak menerima koneksi dari kamera:
install-done = Tugas berhasil diselesaikan.

# Pemeriksaan APK
apk-package = Paket:
apk-unreadable = Peringatan: berkas .apk tidak dapat dianalisis.
apk-incompatible = Peringatan: aplikasi ini menyatakan level SDK minimum yang mungkin tidak disediakan kamera.
apk-unsigned = Peringatan: berkas .apk tidak memiliki tanda tangan.

# Toko aplikasi komunitas
store-loading = Memuat daftar aplikasi…
store-resolving = Mencari rilis terbaru…
store-downloading = Mengunduh
store-unknown-app = Aplikasi tidak dikenal
store-no-release = Tidak ada rilis yang dapat diunduh untuk aplikasi ini
store-not-installable = tanpa rilis
msg-no-apps = Tidak ada aplikasi ditemukan.
store-heading = Toko aplikasi komunitas
store-discontinued = Sony telah menutup toko resmi PlayMemories Camera Apps; servernya tidak lagi menjawab, sehingga aplikasi tidak dapat dibeli atau diunduh dari sana.
store-use-own-apk = Anda tetap dapat memasang .apk milik Anda sendiri: pilih berkasnya di bawah atau seret ke jendela ini. Toko aplikasi komunitas di bawah juga masih berfungsi.
btn-store-sync = Periksa kamera
btn-install-selected = Pasang yang dipilih
btn-store-stop = Berhenti setelah aplikasi ini
btn-store-stopping = Akan berhenti setelah aplikasi ini…
btn-store-retry = Coba lagi yang gagal
btn-continue = Lanjutkan
btn-reinstall = Instal ulang
btn-skip = Lewati
btn-confirm-updater = Mulai ulang dan tulis
btn-confirm-tweaks = Tulis penyesuaian
btn-confirm-languages = Aktifkan bahasa
btn-confirm-wifi = Ganti titik akses
btn-confirm-stream = Timpa pengaturan
btn-confirm-show-shells = Tampilkan shell
btn-confirm-discard = Buang terjemahan
store-select-all = Pilih semua aplikasi
store-col-app = Aplikasi
store-col-version = Versi
store-col-author = Pembuat
store-col-camera = Versi di kamera
store-col-status = Status
store-camera-absent = tidak terpasang
store-version-unknown = tidak diketahui
store-batch-query = Membaca aplikasi yang terpasang dari kamera…
store-sync-done = Kamera diperiksa. Aplikasi dari daftar yang terpasang: { $count }
store-batch-downloading = Mengunduh aplikasi { $current } dari { $total }…
store-batch-installing = Aplikasi { $current } dari { $total }: memasang { $name }…
store-reinstall-confirm = { $name } sudah terpasang di kamera. Versi terpasang: { $installed }. Versi di toko aplikasi: { $store }. Pasang ulang?
store-state-queued = menunggu
store-state-installed = terpasang
store-state-failed = gagal
store-state-skipped = dilewati
store-state-not-run = tidak dijalankan
store-batch-summary = Terpasang: { $done } · gagal: { $failed } · dilewati: { $skipped } · tidak dijalankan: { $notrun }

# Mode servis (senser)
usb-mode-advice = Untuk tweak, cadangan, dan shell servis, atur mode koneksi USB kamera ke “Otomatis”, bukan tetap ke penyimpanan massal. Jika kamera menanyakan mode setelah sesi, pilih “Penyimpanan Massal (MSC)”. Dengan penyimpanan massal tetap, kamera sering menolak peralihan berikutnya ke mode servis.
service-switch-refused = Kamera tidak menyelesaikan peralihan ke mode servis dan tidak tersambung ulang melalui USB; biasanya kamera lalu menampilkan “Periksa perangkat yang tersambung”. Cabut kabel USB, sambungkan lagi, lalu coba lagi.
service-switching = Beralih ke mode servis…
service-switch-wait = Menunggu kamera terhubung kembali dalam mode servis…
service-switch-timeout = Kamera tidak muncul dalam mode servis. Kamera terdaftar ulang sebagai perangkat USB tersendiri (id produk 0336) yang memerlukan penetapan driver WinUSB sendiri; periksa di Device Manager apakah ada perangkat Sony tanpa driver. Matikan dan hidupkan kembali kamera untuk mengembalikannya ke mode normal.
service-authenticating = Melakukan autentikasi…
# Ditampilkan saat kamera masih berada dalam mode servis dari sesi sebelumnya,
# dan saat hal itu tidak dapat dibatalkan.
service-stranded-reset = Kamera masih berada dalam mode servis dari sesi sebelumnya. Sesi tersebut diakhiri terlebih dahulu…
service-stranded-failed = Kamera tersangkut dalam mode servis dari sesi sebelumnya dan tidak menanggapi permintaan untuk keluar. Matikan kamera lalu hidupkan kembali, kemudian coba lagi.
service-bad-mode = Kamera tidak dapat digunakan dalam mode ini. Silakan alihkan ke mode penyimpanan massal.
service-driver-required = Akses USB langsung ditolak dan driver WinUSB tidak dapat ditetapkan. Menetapkan driver memerlukan hak administrator.
service-warning = Peringatan: mode servis memberi akses langsung ke memori, sistem berkas, dan data kalibrasi kamera. Perintah tulis dapat merusak kamera secara permanen.
service-shell-ready = Shell servis siap. Ketik 'help' untuk daftar perintah, 'exit' untuk keluar.
service-shell-done = Sesi shell servis berakhir.
# Dilaporkan saat mulai bila sesi sebelumnya berhenti sebelum sempat melepas
# driver yang telah ditetapkannya.
driver-swept = { $count } penetapan driver yang tertinggal dari sesi sebelumnya telah dihapus.
# Dilaporkan saat mulai bila sesi terputus yang sama juga meninggalkan kamera
# dalam mode servis, di mana kamera akan menolak sesi berikutnya.
driver-swept-camera = Kamera yang tertinggal dalam mode servis dari sesi sebelumnya telah dikembalikan ke mode normalnya.
driver-release-replug = Windows sedang menunggu kamera saat driver dilepas. Cabut kabel USB lalu sambungkan lagi agar pembersihan dapat selesai.
# Hasil perintah `driver`.
driver-bound = { $id } kini memakai WinUSB (paket driver { $package }).
driver-released = { $count } penetapan driver dilepaskan; kamera kembali memakai driver bawaan Windows.
driver-needs-elevation = Menetapkan driver memerlukan hak administrator. Jalankan perintah ini di konsol yang dibuka sebagai administrator.
driver-leftover-needs-elevation = Proses sebelumnya meninggalkan driver yang ditetapkan pada kamera dan belum dilepaskan. Selama masih ada, kamera meminta Anda memeriksa perangkat yang terhubung dan penyimpanan massalnya tidak berfungsi. Melepaskannya memerlukan hak administrator; harap konfirmasi permintaan Windows.
driver-binding = Menetapkan driver WinUSB ke kamera…
driver-bind-wait = Menunggu kamera terhubung kembali dengan driver baru…
driver-bind-timeout = Kamera tidak muncul lagi setelah driver ditetapkan. Cabut, sambungkan kembali, lalu coba lagi.
driver-bind-failed = Driver WinUSB tidak dapat ditetapkan: { $detail }
# Jawaban shell. Semuanya berasal dari inti aplikasi agar konsol dan jendela
# memakai kata yang sama.
service-usage = Penggunaan: { $usage }
service-bad-hex = Bukan nilai heksadesimal: { $value }
service-unknown-command = Perintah tidak dikenal: { $command }. Ketik 'help' untuk melihat daftarnya.
service-unterminated-quote = Baris berakhir di tengah argumen dalam tanda kutip.
service-help = Daftar perintah:
service-read-bytes = { $count } bita dibaca.
service-pulled = { $remote } disalin dari kamera ke { $local }.
service-pushed = { $local } disalin ke { $remote } di kamera.
service-removed = { $remote } dihapus dari kamera.
service-dumped = { $size } bita mulai dari { $offset } disimpan ke { $local }.
service-backup-saved = Berkas cadangan ({ $count } bita) disimpan ke { $local }.
service-backup-text-saved = Berkas cadangan ({ $count } bita) disimpan ke { $bin } dan daftarnya ke { $txt }.
service-backup-status = Status cadangan, wilayah: { $region }
service-backup-rewritten = Image ({ $count } byte) ditulis kembali tanpa perubahan lalu dibaca ulang; tidak disimpan secara permanen.
service-backup-attrs-restored = Atribut { $count } entri dikembalikan sesuai { $local }.
service-backup-written = Entri cadangan { $id } ditulis.
service-backup-persisted = Cadangan subsistem { $id } ditulis ke memori flash.
service-terminal-on = Terminal awakutu kini aktif.
service-terminal-off = Terminal awakutu kini nonaktif.

# Halaman servis tanpa mode lanjutan: cara mencapai shell dari konsol dan apa
# yang harus disiapkan lebih dulu.
service-console-heading = Menjalankan shell servis di konsol
service-console-hint = Buka prompt perintah di folder yang berisi pmca-console lalu jalankan perintah ini:
service-driver-heading = Sebelum memulai
service-driver-hint = Mode servis memerlukan akses USB langsung, yang tidak diberikan Windows lewat driver penyimpanan massal. Aplikasi menetapkan driver WinUSB sendiri — dua kali, sekali saat kamera dalam mode penyimpanan massal dan sekali lagi setelah kamera muncul kembali dalam mode servis dengan identitas produk kedua — lalu melepas kedua penetapan itu saat sesi berakhir. Menetapkan driver memerlukan hak administrator, jadi jalankan konsol sebagai administrator; tidak ada alat lain yang dibutuhkan.
driver-note-linux = Di sini tidak ada yang perlu dipasang: libusb menjangkau kamera secara langsung, tanpa menggeser pengandar kernel. Yang diperlukan hanyalah hak untuk membuka perangkat, dan destop yang belum menahannya. Tambahkan aturan udev untuk identitas pabrikan Sony 054c, dan lepaskan pengandar usb-storage sebelum memakai mode penyimpanan massal. Skrip scripts/linux-prepare.sh di dalam sumber melakukan keduanya sekaligus menghentikan pemantau volume gvfs.
driver-note-macos = Untuk akses USB langsung tidak ada yang perlu dipasang. Setel koneksi USB kamera ke MTP: dalam mode penyimpanan massal, driver disk sistem menahan kamera dan program tidak dapat menjangkaunya. Dalam mode MTP, program menghentikan layanan kamera macOS ptpcamerad bila layanan itu menduduki kamera; sistem menjalankannya kembali bila diperlukan. Landasan ini paling sedikit diuji di antara ketiganya; bersiaplah melaporkan masalah alih-alih menemukannya sudah tertulis.
service-advanced-hint = Shell juga dapat ditampilkan di sini, pada Pengaturan, Mode lanjutan.
btn-copy-command = Salin perintah
msg-command-copied = Perintah disalin ke papan klip.

# Halaman servis dengan mode lanjutan aktif.
btn-service-open = Sambungkan
btn-service-close = Putuskan
btn-service-run = Jalankan
btn-service-browse = Sisipkan jalur berkas…
shell-path-quote = Jalur ini tidak dapat disisipkan: jalur memuat tanda kutip ganda, karakter yang dipakai baris perintah untuk menandai awal dan akhir sebuah argumen. Ganti nama berkas atau pindahkan ke jalur tanpa karakter tersebut.
service-command-label = Perintah
service-not-connected = Tidak tersambung.

# Teks khusus antarmuka grafis
install-apk-path = Jalur ke berkas .apk lokal (opsional)
install-drop-hint = Letakkan berkas .apk di sini

# Deskripsi penyesuaian
tweak-rec-limit = Rekam video tanpa batas waktu
tweak-rec-limit-4k = Rekam video 4K tanpa batas waktu
tweak-language = Tampilkan semua bahasa menu
tweak-pal-ntsc = Aktifkan pemilih PAL / NTSC dan peringatannya
tweak-pal-ntsc-mode = Sistem video (PAL / NTSC)
tweak-pal-ntsc-pal = PAL
tweak-pal-ntsc-ntsc = NTSC
tweak-pal-ntsc-note = Setelah sistem video diubah, kamera meminta kartu memori diformat. Cadangkan dulu seluruh isinya — foto, video, LUT. Paling mudah adalah mengeluarkan kartu sebelum beralih.
tweaks-set-malformed = Diharapkan --set ID=NILAI, diterima: { $arg }
tweaks-set-unknown-value = Nilai tidak dikenal "{ $value }". Diharapkan salah satu dari: { $options }
tweaks-set-not-a-choice = { $id } adalah sakelar, bukan pilihan. Gunakan --enable atau --disable.
tweaks-needs-set = { $id } adalah pilihan, bukan sakelar. Gunakan --set { $id }=NILAI dengan salah satu dari: { $options }
tweak-usb-installer = Aktifkan pemasang aplikasi USB

# Alur penyesuaian
tweaks-warning = Peringatan: menerapkan penyesuaian menulis ulang memori cadangan kamera. Penulisan yang terputus dapat merusak kamera secara permanen.
tweaks-reading = Membaca pengaturan kamera…
tweaks-writing = Menulis pengaturan kamera…
tweaks-done = Pengaturan kamera diperbarui.
tweaks-write-failed = Kamera tidak menyimpan pengaturan baru sesuai yang dikirim. Sebagian mungkin tetap sudah tertulis: baca ulang pengaturan dan batalkan bila perlu. Cadangan yang dibuat sebelum perubahan adalah jalan kembali yang aman.
tweaks-confirm-required = Penulisan ditolak tanpa konfirmasi. Jalankan ulang dengan --confirm setelah Anda membaca peringatan di atas.
tweaks-state-enabled = aktif
tweaks-state-disabled = nonaktif
tweaks-full-read-failed = Kamera tidak mengirim cadangannya secara lengkap ({ $detail }). Setelah itu kamera sering tidak merespons lagi: matikan kamera, lepas baterai beberapa detik, pasang kembali, lalu sambungkan ulang.
tweaks-direct-mode = Cadangan kamera ini lebih besar dari 1 MB, sehingga pengaturan dibaca dan ditulis satu per satu. Nilai lama disimpan sebelum setiap perubahan dan dapat ditulis kembali dengan “Batalkan perubahan”.
tweaks-direct-forced = PMCA_TWEAKS_DIRECT memaksa jalur per nilai, sehingga pengaturan dibaca dan ditulis satu per satu. Nilai lama disimpan sebelum setiap perubahan dan dapat ditulis kembali dengan “Batalkan perubahan”.
tweaks-direct-refused = Kamera menolak perubahan ({ $detail }). Nilai yang sudah ditulis telah dikembalikan; tidak ada yang berubah. Beberapa model, terutama model Jepang, hanya mengizinkan pengaturan ini melalui cadangan lengkap.
tweaks-direct-revert-failed = Kamera menolak perubahan ({ $detail }), dan tidak semua nilai yang sudah ditulis dapat dikembalikan. Nilai lama tersimpan: coba “Batalkan perubahan”.
tweaks-direct-restart = Pengaturan kamera diperbarui. Matikan lalu nyalakan kembali kamera agar perubahan berlaku.
tweaks-rollback-unsaved = Nilai saat ini tidak dapat disimpan untuk pembatalan; tidak ada yang berubah.
tweaks-image-saved = Image cadangan dari sebelum perubahan telah disimpan: { $path }
tweaks-whole-image-hint = Untuk menulis perubahan melalui image cadangan lengkap, jalankan lagi perintah yang sama dengan --whole-image. Image dari sebelum perubahan disimpan terlebih dahulu.
tweaks-whole-image-confirm = Tulis perubahan melalui image cadangan lengkap? Kamera menolaknya sebagai nilai tunggal. Seluruh image ditulis kembali bersama perubahan; image dari sebelum perubahan disimpan terlebih dahulu di komputer ini.
btn-whole-image = Tulis cadangan lengkap
tweaks-undo-none = Tidak ada nilai tersimpan untuk kamera ini.
tweaks-undo-confirm = Tulis kembali nilai yang tersimpan ke kamera? Ini membatalkan semua perubahan yang ditulis satu per satu sejak nilai tersebut disimpan.
tweaks-undo-done = Nilai yang tersimpan telah ditulis kembali. Matikan lalu nyalakan kembali kamera.
tweaks-undo-available = Nilai dari sebelum perubahan pertama tersimpan dan dapat ditulis kembali.
btn-undo-tweaks = Batalkan perubahan
tweaks-identity-heading = Identitas kamera (hanya baca)
tweaks-caution-languages = Dua bahasa — kemungkinan Tionghoa (Sederhana) dan Tionghoa (Tradisional) — tidak dapat ditampilkan kamera ini. Menggulir ke bahasa tersebut membuat menu bahasa mulai ulang, jadi keduanya dibiarkan nonaktif sebagai tindakan pencegahan.
tweaks-caution-accept = Tetap aktifkan kedua bahasa ini (risiko ditanggung sendiri)
tweaks-caution-confirm = Kedua bahasa ini dapat membuat menu bahasa kamera mulai ulang. Tidak ada yang rusak dan perubahan dapat dibatalkan, tetapi Anda mungkin harus mencari pengaturannya dari awal. Tetap aktifkan?

# Pengaturan
settings-language = Bahasa
settings-theme = Tema
settings-theme-system = Sistem
settings-theme-light = Terang
settings-theme-dark = Gelap

# Mode lanjutan. Perlindungan dari salah klik, bukan dari penyerang: semua yang
# dibukanya tetap dapat dicapai dari konsol.
settings-advanced-heading = Mode lanjutan
settings-advanced-hint = Mode lanjutan menampilkan shell servis dan shell updater pada halaman dengan nama yang sama. Mode servis memberi akses langsung ke memori kamera, sistem berkasnya, dan data kalibrasinya; satu perintah yang salah ketik dapat membuat kamera tidak berfungsi. Biarkan nonaktif selama Anda belum tahu apa yang akan Anda tulis.
settings-advanced-toggle = Tampilkan shell servis dan shell updater
settings-advanced-on = Mode lanjutan aktif. Halaman Shell servis dan Shell updater kini menyediakan shell itu sendiri.
settings-advanced-off = Mode lanjutan nonaktif. Halaman Shell servis dan Shell updater justru menjelaskan jalur lewat konsol.
settings-advanced-confirm = Shell servis dan shell updater dapat merusak kamera Anda secara permanen. Tampilkan kedua shell?

# Menyunting terjemahan. Berkas bahasa dapat diunduh, diubah di editor teks
# mana pun, lalu diunggah kembali; berkas yang diunggah diperiksa sebelum
# digunakan.
settings-catalog-heading = Sunting terjemahan
settings-catalog-hint = Unduh berkas bahasa, ubah di editor teks pilihan Anda, lalu unggah kembali. Hanya teks di sebelah kanan tanda sama dengan yang diterjemahkan, dan entri yang Anda hilangkan tetap memakai teks bawaan. Bahasa yang dipilih di sini tidak mengubah bahasa aplikasi.
settings-catalog-locale = Berkas bahasa
settings-catalog-builtin = Terjemahan bawaan sedang digunakan.
settings-catalog-custom = Terjemahan Anda sendiri sedang digunakan:
btn-catalog-export = Unduh…
btn-catalog-import = Unggah…
btn-catalog-reset = Pulihkan bawaan
catalog-exported = Ditulis ke
catalog-imported = Terjemahan diterapkan.
catalog-reset-done = Terjemahan bawaan telah dipulihkan.
catalog-reset-none = Tidak ada terjemahan sendiri yang perlu dipulihkan.
catalog-reset-confirm = Buang terjemahan Anda sendiri untuk bahasa ini?
catalog-rejected = Berkas tidak diterapkan. Perbaiki baris berikut:
catalog-count-accepted = Entri yang diterapkan
catalog-count-missing = Entri yang dipertahankan dari terjemahan bawaan
catalog-count-untranslated = Entri yang sama persis dengan teks asli bahasa Inggris
catalog-line = Baris
catalog-problem-syntax = bukan entri berbentuk “id = teks”
catalog-problem-continuation = dilanjutkan pada baris berikutnya, yang tidak didukung
catalog-problem-duplicate = didefinisikan lebih dari sekali
catalog-problem-unknown-key = bukan entri yang digunakan aplikasi ini
catalog-problem-empty = tidak memiliki teks
catalog-problem-placeholder = penanda diubah, seharusnya:

# Asal usul, ucapan terima kasih, dan lisensi. Ditampilkan di halaman Lisensi
# dan oleh `pmca-console license`. Teks lisensinya sendiri tidak diterjemahkan:
# ia adalah dokumen hukum dan ditampilkan apa adanya.
about-thanks-heading = Terima kasih
about-thanks-body-1 = Perkakas ini ada berkat ma1co, yang memecahkan cara kamera Sony berkomunikasi melalui USB dan merilis Sony-PMCA-RE pada 2015, serta berkat semua orang yang ikut berkontribusi padanya dan pada proyek-proyek OpenMemories pada tahun-tahun berikutnya.
about-thanks-body-2 = Deteksi kamera, protokol perintah eksternal, prosedur pembaru firmware, dan mode servis semuanya adalah karya mereka, dan port ini meneruskan karya tersebut. Terima kasih. Di atas fondasi itu, PMCA-RE RELOADED menambahkan banyak fitur baru yang disumbangkan oleh masterflai. Yang terpenting adalah pergantian driver perangkat secara otomatis di Windows yang kini sudah terpadu: kamera tidak perlu lagi diberi driver lain secara manual dengan alat terpisah, sehingga prosesnya lebih nyaman, lebih cepat, dan lebih kecil risikonya.
about-contributors-heading = Kontributor
about-contributors-body = Selain ma1co, orang-orang berikut berkontribusi pada Sony-PMCA-RE atau menerbitkan perubahan di salinan mereka sendiri yang kemudian diambil ke dalam port ini:
about-purpose-heading = Mengapa port ini ada
about-purpose-body-1 = Aplikasi aslinya tidak lagi dikembangkan. Aplikasi itu ditulis untuk Python 2, yang masa hidupnya berakhir pada 2020, dan sebagian darinya bergantung pada layanan yang sejak itu ditutup oleh Sony, sehingga tidak lagi berjalan tanpa batasan di sistem masa kini.
about-purpose-body-2 = Tujuan port ini adalah pelestarian, bukan penggantian, dan sama pentingnya, penggunaan yang lancar saat ini: menjaga agar kamera-kamera ini tetap dapat dipakai dan agar program berjalan tanpa kesulitan di komputer masa kini maupun dengan model kamera yang lebih baru. Segala sesuatu yang dapat dilakukan aplikasi asli seharusnya tetap dapat dilakukan di sini.
about-origin-heading = Bagaimana ini dibuat
about-origin-body-1 = Port dari Python ke Rust dikerjakan dengan bantuan AI.
about-origin-body-2 = Apa yang hal itu tidak berarti perlu dikatakan sama terus terangnya: perilakunya tercakup oleh rangkaian pengujian otomatis dan oleh rencana uji perangkat keras. Terutama versi Windows telah diperiksa secara menyeluruh, karena driver perangkat menjadikannya platform yang paling rumit; sebaliknya macOS dan Linux belum dapat diuji sepenuhnya. Perlakukan ini sebagai perangkat lunak eksperimental.
about-license-heading = Lisensi
about-license-body = Port ini diterbitkan di bawah lisensi MIT, lisensi yang sama dengan proyek aslinya. Pemberitahuan hak cipta asli dipertahankan tanpa perubahan, sebagaimana diwajibkan lisensi tersebut.
about-license-full = Teks lisensi
about-third-party-heading = Lisensi pustaka yang digunakan
about-third-party-body = Program ini ditautkan secara statis: kode setiap pustaka yang dipakainya berada di dalam berkas yang dijalankan, dan lisensi pustaka tersebut mewajibkan pemberitahuan hak ciptanya ikut disertakan. Semuanya dimuat lengkap di bawah ini, satu entri per paket.
about-third-party-toggle = Tampilkan semua pemberitahuan pihak ketiga
about-privacy-heading = Data
about-privacy-body-1 = Program ini berjalan di komputer Anda sendiri. Program ini tidak mengirim data penggunaan, tidak ada telemetri, dan tidak ada keharusan membuat akun. Tidak ada satu pun yang Anda lakukan di sini yang dilaporkan ke mana pun.
about-privacy-body-2 = Namun ada dua fungsi yang menghubungi jaringan dengan sendirinya, dan pihak seberang akan mengetahui alamat IP Anda: data bantuan GPS diunduh dari Sony, dan daftar aplikasi dari GitHub. Tanpa fungsi-fungsi itu, program tetap sepenuhnya luring.
about-privacy-body-3 = Apa yang dibaca dari kamera — termasuk nomor serinya dan kata sandi Wi-Fi yang tersimpan — tidak meninggalkan komputer Anda. Jika Anda menuliskannya ke sebuah berkas, perhatikan kepada siapa berkas itu Anda berikan.
about-privacy-body-4 = Tidak ada layanan yang dijalankan di balik program ini dan tidak ada data yang sampai ke para penulisnya, jadi di sini tidak ada yang bisa diminta maupun dihapus. Apa yang dilakukan kedua layanan tersebut terhadap alamat yang mereka lihat tunduk pada pernyataan privasi mereka sendiri, bukan pernyataan ini.
about-legal-heading = Pemberitahuan hukum
about-legal-trademark = Ini adalah proyek independen. Proyek ini tidak dibuat oleh, tidak didukung oleh, dan tidak berkaitan dengan Sony dalam bentuk apa pun. "Sony", "PlayMemories", dan nama model kamera adalah merek dagang Sony Group Corporation, yang disebut di sini semata-mata untuk menunjukkan kamera mana yang dapat diajak berkomunikasi oleh alat ini.
about-legal-own-device = Program ini ditujukan untuk digunakan dengan kamera milik Anda sendiri.
about-legal-warranty = Apa yang dilakukan alat ini berada di luar yang dimaksudkan Sony untuk kamera-kamera tersebut, dan melakukannya dapat mengakhiri garansi Anda. Perangkat lunak ini sendiri diberikan tanpa jaminan apa pun, dan penggunaannya adalah risiko Anda sendiri. Sejauh mana rekayasa balik atas perangkat milik Anda diperbolehkan bergantung pada negara Anda.
about-links-heading = Proyek
about-link-original = Sony-PMCA-RE — proyek asli
about-link-fwtool = fwtool.py — alat firmware yang menjadi dasar pembuatan image pembaruan
about-link-tweak = OpenMemories: Tweak — aplikasi yang disarankan untuk kamera yang mendukungnya
about-link-applist = OpenMemories: App list — aplikasi lainnya
about-link-platform = OpenMemories: Platform — bodi updater yang disertakan
about-link-devices = Model kamera yang didukung
about-link-nexhack = Nex-Hack — riset komunitas yang menjadi dasar proyek ini
about-link-zve10 = zv-e10-re — riset tentang antarmuka dan memori pengaturan ZV-E10
about-version = Versi
btn-copy-url = Salin alamat
msg-url-copied = Alamat disalin ke papan klip.

# Pemberitahuan keselamatan
notice-experimental = Ini adalah perangkat lunak eksperimental. Gunakan dengan risiko Anda sendiri.

# Pengaturan jaringan (GPS / Wi-Fi / siaran langsung)
gps-downloading = Mengunduh data bantuan GPS…
gps-writing = Menulis data bantuan GPS ke kamera…
gps-bad-size = Ukuran data bantuan GPS tidak sesuai dan tidak ditulis.
wifi-read = Membaca titik akses siaran langsung dari kamera…
wifi-written = Titik akses siaran langsung ditulis.
stream-read = Membaca pengaturan siaran langsung dari kamera…
stream-written = Pengaturan siaran langsung ditulis.
net-secret-warning = Peringatan: berkas ini memuat kata sandi Wi-Fi dan token akses dalam teks biasa.
wifi-secret-warning = Kamera menyimpan kata sandi ini tanpa enkripsi, dan di sini semuanya ditampilkan persis seperti tersimpan. Semuanya tetap berada di komputer ini; jika Anda menyimpannya ke sebuah berkas, perhatikan kepada siapa berkas itu Anda berikan.
gps-heading = Data bantuan GPS
gps-hint = Seret berkas ke sini, masukkan jalur, atau kosongkan kolom untuk mengambil data terkini dari Sony.
btn-gps-devices = Perangkat yang kompatibel
btn-gps-devices-hide = Sembunyikan perangkat yang kompatibel
gps-devices-note = Model-model ini memiliki modul GPS bawaan. Pada banyak di antaranya, hanya varian yang namanya berakhiran V yang memilikinya; varian tanpa V tidak memiliki GPS.
gps-devices-compact = Kamera saku (DSC)
gps-devices-slt = Kamera SLT (dudukan A)
gps-devices-action = Kamera aksi
gps-devices-camcorder = Kamera video (Handycam)
gps-col-variant = Varian GPS
gps-col-arch = Arsitektur
gps-col-module = Modul GPS
gps-col-apps = Dukungan aplikasi
gps-yes = ya
gps-no = tidak
gps-unknown = tidak diketahui
gps-devices-untested = Ada GPS, dukungan belum diuji
gps-devices-untested-note = Model-model ini memiliki modul GPS bawaan, tetapi tidak tercantum dalam daftar perangkat OpenMemories. Belum diuji apakah aplikasi ini dapat bekerja dengannya, sehingga arsitektur dan dukungan aplikasinya tidak diketahui.
wifi-heading = Titik akses untuk siaran langsung
wifi-hint = Ini adalah titik akses yang dipakai fitur siaran langsung, bukan pengaturan Wi-Fi kamera itu sendiri. Bodi tanpa fitur siaran akan menolak perintah ini.
wifi-multi = Daftar banyak titik akses
wifi-none = Tidak ada titik akses tersimpan.
wifi-writing = Menulis titik akses siaran langsung ke kamera…
wifi-confirm = Ini akan mengganti semua titik akses yang tersimpan di kamera. Lanjutkan?
wifi-empty-refused = Daftar kosong tidak akan ditulis, karena itu akan menghapus semua titik akses tersimpan.
stream-heading = Siaran langsung
stream-writing = Menulis pengaturan siaran langsung ke kamera…
stream-confirm = Ini akan menimpa pengaturan siaran langsung kamera. Lanjutkan?
stream-invalid-json = Pengaturan bukan JSON yang valid:

# Pembaruan firmware dan toko resmi
firmware-warning = Peringatan: ini menimpa firmware kamera. Jangan menyela pembaruan setelah dimulai: jangan mencabut kabel USB dan jangan mematikan kamera. Gunakan hanya berkas firmware yang ditujukan tepat untuk model kamera ini, dan pastikan baterai terisi cukup. Jika tidak, kamera dapat menjadi tidak dapat digunakan secara permanen.
firmware-untested = Belum teruji: penulisan firmware sudah dialihkan, tetapi belum pernah dicoba pada kamera sungguhan. Penulisan yang gagal atau terputus tidak dapat dibatalkan, jadi anggaplah semua di halaman ini belum terbukti.
firmware-confirm-required = Penulisan firmware ditolak tanpa --confirm.
firmware-truncated = Berkas firmware terpotong: payload FDAT melampaui akhir berkas.
firmware-updating = Memperbarui firmware:
firmware-invalid = Berkas ini bukan citra firmware Sony yang valid
firmware-dat-path = Berkas firmware (.dat)
firmware-drop-hint = Letakkan berkas firmware .dat di sini
firmware-file-required = Pilih berkas firmware terlebih dahulu.
firmware-check-ok = Kamera menerima citra ini. Belum ada yang ditulis.
firmware-cancelled = Penulisan dibatalkan. Kamera tidak diubah.
firmware-current-version = Versi terpasang
firmware-new-version = Versi dalam citra ini
firmware-models-hint = Kamera dianggap kompatibel jika shell updater dapat menjangkaunya.
btn-firmware-check = Periksa citra
btn-firmware-flash = Tulis firmware
firmware-own-device = Konfirmasikan bahwa Anda memahami risiko ini dan hanya memperbarui perangkat milik Anda sendiri.
btn-firmware-accept = Risiko dipahami – tulis

# Perkakas berkas: operasi yang sama sekali tidak memerlukan kamera.
tools-hint = Operasi ini bekerja pada berkas dan tidak pernah menyentuh kamera, jadi tidak ada di sini yang bisa merusaknya. Ini operasi yang sama dengan yang ditawarkan pmca-console sebagai apk2spk, spk2apk, print-backup, print-usagelog, guess-firmware, dan xpd-checksum.
tools-operation = Operasi
tools-apk2spk = Bungkus sebuah .apk ke dalam kontainer .spk
tools-spk2apk = Buka sebuah .spk kembali menjadi .apk
tools-print-backup = Tampilkan isi sebuah Backup.bin
tools-print-usagelog = Tampilkan isi sebuah Usagelog.bin
tools-guess-firmware = Cari payload di dalam berkas .dat firmware
tools-xpd-checksum = Hitung checksum CIC sebuah berkas
tools-input = Berkas masukan
tools-file-required = Pilih dulu berkas yang dibutuhkan operasi ini.
tools-apk-invalid = Berkas tidak dapat dibungkus sebagai .spk
tools-spk-invalid = Berkas bukan paket .spk yang sah
tools-backup-invalid = Berkas bukan citra Backup.bin yang sah
tools-usagelog-invalid = Berkas bukan log penggunaan yang sah
tools-reset-value = nilai setel ulang:
tools-fdat-found = Payload ditemukan pada ofset { $offset }, ukuran { $size }.
tools-checksum = Checksum
tools-written = Ditulis ke { $path }

# Tautan ke dokumentasi proyek
docs-compatibility = Kompatibilitas kamera
docs-compatibility-hint = Proyek aslinya menyediakan ikhtisar kamera yang telah diuji. Ikhtisar ini terutama mencakup model kamera lama dan tersedia di alamat berikut:
devices-note = Data diambil dari ikhtisar proyek asli. Angka di samping dukungan aplikasi adalah versi platform aplikasi kamera.
devices-more-note = Kamera berikut belum diuji secara khusus dengan program ini. Tanda centang di kolom terakhir berarti komunitas telah melaporkan penggunaannya. Jika shell servis juga ditandai ✖, penggunaan tersebut gagal.
btn-more-models = Perangkat lainnya
btn-more-models-hide = Sembunyikan perangkat lainnya
check-updater-yes = Perangkat Anda { $model } kompatibel: menurut daftar perangkat, shell updater dapat menjangkaunya.
check-updater-no = Perangkat Anda { $model } tidak kompatibel: menurut daftar perangkat, shell updater tidak dapat menjangkaunya.
check-updater-unknown = Kompatibilitas perangkat Anda { $model } tidak diketahui: perangkat ini tidak ada di daftar perangkat mana pun dalam program ini, sehingga tidak diketahui apakah shell updater dapat menjangkaunya.
check-apps-no = Perangkat Anda { $model } tidak kompatibel: menurut daftar perangkat, perangkat ini tidak mendukung aplikasi.
check-apps-untested = Kompatibilitas perangkat Anda { $model } tidak diketahui: perangkat ini ada di daftar perangkat, tetapi dukungan aplikasinya belum dikonfirmasi.
check-apps-unknown = Kompatibilitas perangkat Anda { $model } tidak diketahui: perangkat ini tidak ada di daftar perangkat mana pun dalam program ini, sehingga tidak diketahui apakah mendukung aplikasi.
check-gps-no = Perangkat Anda { $model } tidak kompatibel: perangkat ini tidak ada di daftar perangkat dengan penerima GPS.
check-gps-untested = Kompatibilitas perangkat Anda { $model } tidak diketahui: menurut daftar perangkat, perangkat ini memiliki penerima GPS, tetapi transfer ke model ini belum diuji.
check-gps-unknown = Kompatibilitas perangkat Anda { $model } tidak diketahui: perangkat ini tidak ada di daftar perangkat mana pun dalam program ini, sehingga tidak diketahui apakah memiliki penerima GPS.
check-unidentified = Perangkat yang terhubung tidak dapat diidentifikasi, sehingga kompatibilitasnya tidak diketahui.
check-proceed = Tetap lanjutkan?
check-cancelled = Operasi dibatalkan. Perangkat tidak diubah.
devices-col-confirmed = Terkonfirmasi
devices-group-ilc = Kamera lensa tukar (dudukan E dan A)
devices-group-remote = Remote Live-View
btn-copy-link = Salin tautan
shutter-heading = Penghitung rana
shutter-hint = Kamera menghitung setiap pelepasan rana dan menulis nilai terkini ke setiap foto yang disimpannya (ARW, JPEG, atau HEIF). Pilih foto yang paling akhir diambil untuk membaca nilai terkini. Berkas hanya dibaca, tidak pernah diubah.
shutter-source = Letak penghitung di dalam berkas berbeda-beda menurut model. Posisinya berasal dari riset yang didokumentasikan secara publik (ExifTool) dan belum dikonfirmasi untuk setiap model.
shutter-drop-hint = Letakkan foto (ARW, JPEG, HEIF) di sini
btn-shutter-read = Analisis foto
shutter-file-required = Pilih foto terlebih dahulu.
shutter-file = Berkas
shutter-taken = Tanggal pengambilan
shutter-count = Jumlah jepretan
shutter-type = Rana
shutter-type-mechanical = Mekanis
shutter-type-electronic = Elektronik
shutter-electronic-hint = Foto ini diambil dengan rana elektronik. Pada banyak model, jepretan seperti ini tidak dihitung; nilainya lalu menunjukkan hitungan pada pelepasan mekanis terakhir.
shutter-not-sony = Foto ini tidak diambil dengan kamera Sony. Penghitung rana tidak dapat dibaca darinya.
shutter-unknown-model = Tidak diketahui di mana model kamera ini menyimpan penghitung rana di dalam berkas, sehingga tidak ada nilai yang ditampilkan.
shutter-not-recorded = Berkas tidak berisi penghitung rana yang dapat dibaca. Hal ini terjadi pada foto yang disimpan atau diekspor oleh program penyunting gambar, dan pada beberapa model pada jepretan dengan rana elektronik. Pilih berkas asli dari kartu memori.
shutter-model-mismatch = Foto ini berasal dari model kamera yang berbeda dengan kamera yang tersambung.
shutter-invalid = Berkas ini bukan foto yang dapat dibaca
file-filter-photo = Foto (ARW, JPEG, HEIF)
usage-log-heading = Log penggunaan
usage-log-hint = Kamera menyimpan log internal dalam format yang tidak terdokumentasi. Log disimpan apa adanya; kamera hanya dibaca.
usage-log-written = Ditulis ke
btn-usage-log-info = Tentang log penggunaan
btn-usage-log-info-hide = Sembunyikan penjelasan
usage-log-info-what-heading = Apa log ini
usage-log-info-what = Secara internal Sony menyebut log ini "KikiLog". Kamera menulisnya sendiri; program ini hanya membacanya. Tidak ada deskripsi publik tentang formatnya yang diketahui.
usage-log-info-read = Membaca tidak mengubah apa pun: beberapa unduhan berturut-turut identik byte demi byte.
usage-log-info-flush-heading = Mengapa entri terbaru belum ada
usage-log-info-flush = Entri baru sering tidak langsung muncul: kamera mungkin baru menyimpannya saat dimatikan, kadang tersebar di beberapa siklus nyala/mati. Matikan dan nyalakan kamera, unduh lagi, dan ulangi sampai tidak ada entri baru yang muncul.
usage-log-info-content-heading = Apa yang bisa dibaca darinya
usage-log-info-shutter = Jepretan: beberapa peristiwa masing-masing mewakili satu pengambilan gambar. Namun jumlahnya bukan penghitung rana: log ini berukuran tetap dan belum tentu mencakup seluruh masa pakai kamera. Kamera menyimpan nilai totalnya di tempat lain dan menuliskannya ke foto yang disimpannya; nilai itu dapat dibaca di bagian "Penghitung rana".
usage-log-info-clock = Stempel waktu hanya seakurat jam kamera. Setelah baterai diganti, jam bisa mundur dan menghasilkan entri dengan tanggal awal kamera.
usage-log-info-unknown = Tidak ada deskripsi publik tentang nomor peristiwa yang diketahui. Program menampilkan nomor tanpa menafsirkannya.
usage-log-info-privacy-heading = Privasi
usage-log-info-privacy = Berkas ini berisi nomor seri kartu memori yang pernah dimasukkan. Jangan membagikan atau mengunggahnya sembarangan.
btn-usage-log = Simpan log
usagelog-size = Area log
usagelog-cursor = Posisi tulis
usagelog-spare = Area kedua
usagelog-count = Entri
usagelog-events = Peristiwa menurut nomor
usagelog-entries = Entri berurutan
msg-link-copied = Tautan disalin ke papan klip.

# Pesan kegagalan backend, dikunci oleh kode stabil yang dilaporkan backend.
# Bagian variabel dari pesan (jalur, bita status, galat perangkat) ditambahkan
# oleh antarmuka, sehingga kalimat ini tetap utuh tanpa bagian tersebut.
# `err-operation` sengaja tidak ada: kode itu menandai pesan yang sudah
# diterjemahkan backend dan harus ditampilkan apa adanya.
err-usb-no-device = Kamera tidak ditemukan. Hubungkan kamera dan nyalakan.
err-usb-camera-not-ready = Kamera terhubung tetapi belum siap: kamera mati atau menunggu pemilihan mode USB di layarnya. Nyalakan kamera atau pilih “Penyimpanan Massal (MSC)” di kamera.
camera-choose-usb-mode = Kamera menunggu pemilihan mode USB. Pilih “Penyimpanan Massal (MSC)” di kamera.
err-usb-too-many-devices = Lebih dari satu kamera terhubung. Cabut semuanya kecuali yang ingin Anda gunakan.
err-usb-transfer = Transfer USB gagal. Coba kabel atau port lain, dan hindari hub USB.
err-usb-permission = Program tidak diizinkan membuka kamera melalui USB. Di Linux hak ini diberikan sekali melalui aturan udev untuk kode produsen Sony; skrip scripts/linux-prepare.sh dalam kode sumber memasangnya. Setelah itu cabut kamera lalu colokkan kembali.
err-usb-busy = Program lain sedang menggunakan kamera, sehingga tidak dapat dibuka. Tutup program lain yang mungkin mengaksesnya; di Linux pemantau media destop dapat menahannya, yang dihentikan oleh scripts/linux-prepare.sh. Di macOS, setel koneksi USB kamera ke MTP: dalam mode penyimpanan massal, driver disk milik sistem menahannya.
err-usb-stall = Kamera membatalkan transfer USB.
err-usb-protocol = Kamera mengirim respons yang tidak terduga.
err-usb-msc-status = Kamera menolak sebuah perintah penyimpanan.
cli-error-prefix = galat
err-usb-driver = Windows menetapkan driver untuk kamera ini yang tidak mengizinkan akses langsung. Mode ini memerlukan driver WinUSB, yang dapat ditetapkan aplikasi sendiri — tetapi hanya ketika berjalan dengan hak administrator.
err-needs-elevation = Menetapkan atau melepaskan driver memerlukan hak administrator, dan permintaan Windows ditolak. Coba lagi dengan “Mulai pembantu driver”; tidak ada yang perlu diubah di kamera.
err-usb-no-mass-storage-volume = Kamera tersambung dalam mode penyimpanan massal tetapi tidak menyediakan drive apa pun. Bangunkan kamera, konfirmasi mode USB di layarnya jika diminta, dan periksa apakah kartu memori sudah terpasang. Jangan ganti drivernya: mode ini justru memerlukan driver bawaan Windows.
err-usb-msc-sense = Kamera melaporkan galat penyimpanan.
err-usb-ptp-response = Kamera menolak perintah tersebut.
err-usb-unsupported = Model kamera ini tidak mendukung fungsi tersebut.
err-usb-updater-status = Updater firmware di kamera menolak operasi ini.
err-usb-senser = Sebuah perintah mode servis gagal.
err-market-core = Berkas data kamera tidak dapat dibaca.
err-market-json = Data yang diterima salah bentuk.
err-market-missing-field = Ruas yang diharapkan tidak ada dalam respons.
err-market-no-apk = Belum ada aplikasi yang dipilih untuk dipasang.
err-market-http = Permintaan jaringan gagal.
err-market-io = Server pemasangan lokal gagal.
err-market-tls = Koneksi terenkripsi ke kamera tidak dapat dibangun.
err-apk = Berkas aplikasi tidak dapat dibaca.
err-io = Sebuah berkas tidak dapat dibaca atau ditulis.
err-json = Kamera mengirim data yang salah bentuk.
err-unknown-locale = Bahasa ini tidak tersedia.
err-catalog-encoding = Berkas terjemahan tidak disimpan sebagai teks UTF-8. Simpan ulang dengan pengodean UTF-8.
err-internal = Terjadi kesalahan internal. Ini adalah bug pada aplikasi.

## Function analysis
nav-analysis = Analisis
analysis-title = Analisis fungsi
analysis-experimental = Eksperimental. Fungsi ini baru dan baru dicoba pada sedikit kamera.
analysis-own-hardware = Gunakan hanya pada kamera Anda sendiri. Mode servis saja sudah merupakan campur tangan yang tidak dimaksudkan Sony; tanggung jawab ada pada Anda.
analysis-explanation-toggle = Penjelasan: tujuan, alur, risiko
analysis-purpose-heading = Untuk apa?
analysis-purpose = Analisis fungsi mendokumentasikan di nilai mana dalam memori pengaturannya (cadangan) kamera menyimpan salah satu pengaturan menunya sendiri. Hasilnya menggambarkan memori pengaturan kamera ini; dapat dibandingkan dengan kamera lain, tetapi tidak mengubah apa yang ditawarkan kamera.
analysis-how-heading = Bagaimana alurnya?
analysis-how = Untuk setiap nilai menu yang Anda masukkan, program membaca seluruh memori pengaturan satu kali, setiap kali dalam sesi mode servis singkat tersendiri. Di antara pembacaan, Anda mencabut kabel dan mengubah pengaturan di menu kamera; kamera tetap menyala. Terakhir, nilai pertama dibaca sekali lagi. Nilai yang mengikuti menu dan pada akhirnya kembali ke nilai awal adalah kandidat. Nilai yang berubah sendiri (penghitung, jam) disisihkan.
analysis-requirements-heading = Persyaratan
analysis-requirements = Kamera terhubung lewat USB, mode USB-nya diatur ke “Otomatis”, dan setelah setiap sesi Anda memilih “Penyimpanan Massal (MSC)” di kamera. Di Windows program memerlukan hak administrator, karena sebentar menetapkan driver untuk mode servis. Isi daya baterai; setiap sesi memakan waktu sekitar satu menit.
analysis-read-heading = Langkah membaca
analysis-read = Cuplikan hanya membaca. Tidak ada yang berubah pada pengaturan kamera. Risikonya kecil, tetapi tidak nol: beralih ke mode servis saja sudah merupakan campur tangan yang tidak dimaksudkan Sony.
analysis-write-heading = Langkah menulis (uji tulis)
analysis-write = Hanya jika Anda mengaktifkannya secara tegas: program menulis nilai yang ditemukan ke kamera, Anda memeriksa di menu setelah memulai ulang apakah nilai itu berpengaruh, lalu nilai asli ditulis kembali. Yang ditulis hanya nilai yang disimpan oleh kamera itu sendiri, dan hanya jika ditemukan tepat satu kandidat yang cocok. Nilai lama disimpan terlebih dahulu. Meski begitu, langkah ini mengubah kamera; kesalahan dapat membuatnya dalam keadaan yang hanya bisa diperbaiki pusat servis.
analysis-legal-heading = Hukum
analysis-legal = Perangkat lunak disediakan tanpa jaminan apa pun (lisensi MIT). Campur tangan pada kamera dapat memengaruhi garansinya. Gunakan fungsi ini hanya pada kamera milik Anda. Laporan analisis tidak berisi nomor seri dan tidak berisi apa pun dari memori pengaturan selain kandidat yang ditemukan; laporan tidak pernah dikirim secara otomatis.
analysis-start-heading = Analisis baru
analysis-setting-label = Pengaturan menu, sesuai nama di kamera
analysis-values-label = Nilai menu, dipisahkan koma
analysis-values-hint = Nilai pertama adalah yang sedang diatur sekarang. Dua sampai delapan nilai; masing-masing memerlukan satu sesi mode servis.
analysis-write-option = Lalu lakukan uji tulis (mengubah kamera)
analysis-write-warning = Uji tulis mengubah kamera: menulis sebuah nilai ke memori pengaturannya lalu menulis kembali nilai aslinya. Hanya pada kamera Anda sendiri, dengan risiko Anda sendiri.
analysis-write-consent = Kamera ini milik saya dan saya menanggung risikonya.
btn-analysis-write-enable = Aktifkan uji tulis
btn-analysis-start = Mulai analisis
btn-analysis-snapshot = Baca cadangan
btn-analysis-new = Analisis baru
analysis-discard-confirm = Buang analisis yang sedang berjalan? Laporan sejauh ini tetap tersimpan.
analysis-access-read = MEMBACA
analysis-access-write = MENULIS
analysis-access-user = MANUAL
analysis-access-local = PC
analysis-step-snapshot = Baca cadangan, pengaturan berada di “{ $value }”
analysis-step-change = Ubah ke “{ $value }” di menu kamera
analysis-step-evaluate = Bandingkan cuplikan
analysis-step-write = Tulis nilai untuk “{ $value }” ke kamera
analysis-step-check = Matikan dan nyalakan kamera, baca menu (diharapkan: “{ $value }”)
analysis-step-restore = Tulis kembali nilai asli untuk “{ $value }”
analysis-step-report = Simpan laporan analisis
analysis-do-prepare = PERSIAPAN: 1. Di menu kamera, atur “{ $setting }” ke “{ $value }” lalu keluar dari menu dengan tombol MENU agar kamera menyimpan pengaturan. 2. Biarkan kamera menyala. 3. Pasang kabel dan pilih “Penyimpanan Massal (MSC)” di kamera.
analysis-do-change = GILIRAN ANDA: 1. Cabut kabel. 2. Kamera tetap menyala. 3. Di menu kamera, atur “{ $setting }” ke “{ $value }” lalu keluar dari menu dengan tombol MENU agar kamera menyimpan pengaturan. 4. Pasang kembali kabel dan pilih “Penyimpanan Massal (MSC)”.
analysis-do-check = GILIRAN ANDA: 1. Cabut kabel. 2. Matikan lalu nyalakan lagi kamera. 3. Lihat di menu apa yang tertera pada “{ $setting }”.
analysis-do-reconnect = Pasang kembali kabel dan pilih “Penyimpanan Massal (MSC)” di kamera.
analysis-reading = Cuplikan { $index } dari { $total }: JANGAN cabut kabel selama pembacaan.
analysis-cli-continue = Enter untuk lanjut, q untuk batal:
analysis-confirm-word = ya
analysis-cli-confirm = Ketik “{ $word }” untuk mengonfirmasi:
analysis-write-declined = Tidak dikonfirmasi. Tidak ada yang ditulis.
analysis-aborted = Dibatalkan. Tidak ada yang ditulis.
analysis-snapshot-done = Cuplikan terbaca. Kamera telah keluar dari mode servis.
analysis-result = { $compared } nilai dibandingkan: { $count } kandidat, { $running } nilai berubah sendiri.
analysis-candidate-consistent = cocok dengan menu secara jelas
analysis-candidate-inconsistent = tidak cocok secara jelas
analysis-write-preview-heading = Ini yang akan ditulis
analysis-write-preview = Nilai { $id } ({ $size } byte): dari { $old } (“{ $old-label }”) menjadi { $new } (“{ $new-label }”). Kamera sendiri menyimpan nilai baru ini saat “{ $new-label }” diatur.
btn-analysis-write = Tulis
btn-analysis-skip-write = Selesai tanpa uji tulis
analysis-written = Nilai ditulis, dibaca kembali, dan disimpan permanen.
analysis-menu-question = Apa yang ditampilkan menu sekarang?
analysis-menu-other = hal lain
btn-analysis-menu-confirm = Terapkan
btn-analysis-restore = Tulis kembali nilai asli
analysis-restored = Nilai asli ditulis kembali dan disimpan permanen.
analysis-survived = Nilai yang ditulis masih tersimpan setelah mulai ulang.
analysis-not-survived = Nilai yang ditulis tidak lagi tersimpan setelah mulai ulang.
analysis-report-saved = Laporan analisis disimpan: { $path }
analysis-report-hint = Laporan tidak berisi nomor seri maupun cadangan. Anda dapat membagikannya agar temuan dapat diperiksa dan didokumentasikan.
btn-analysis-export = Ekspor laporan…
file-filter-analysis = Laporan analisis
analysis-pending = Sebuah uji tulis belum selesai. Nilai asli tersimpan dan dapat ditulis kembali.
analysis-pending-none = Tidak ada uji tulis yang belum selesai tersimpan untuk kamera ini.
analysis-invalid-setting = Masukkan pengaturan menu sesuai nama di kamera.
analysis-invalid-values = Masukkan dua sampai { $max } nilai menu yang berbeda. Yang pertama adalah yang sedang diatur sekarang.
analysis-already-complete = Semua cuplikan sudah dibaca.
analysis-other-camera = Ini bukan kamera yang sama seperti pada cuplikan pertama. Analisis dihentikan.
analysis-write-not-allowed = Uji tulis tidak dimungkinkan dalam keadaan ini.
analysis-camera-changed = Kamera kini memuat nilai yang berbeda dari cuplikan pertama. Tidak ada yang ditulis.
analysis-write-refused = Kamera menolak nilai tersebut; semuanya seperti sebelumnya. ({ $detail })
analysis-refuse-none = Uji tulis tidak dimungkinkan: tidak ada nilai yang mengikuti menu.
analysis-refuse-ambiguous = Uji tulis tidak dimungkinkan: { $count } nilai mengikuti menu, tidak satu pun dapat ditetapkan dengan pasti.
analysis-refuse-inconsistent = Uji tulis tidak dimungkinkan: nilai tidak cocok secara jelas dengan nilai menu.
analysis-refuse-size = Uji tulis tidak dimungkinkan: nilainya { $size } byte; hanya nilai kecil yang ditulis.
analysis-refuse-read-only = Uji tulis tidak dimungkinkan: kamera menandai nilai sebagai hanya-baca.
analysis-refuse-protected = Uji tulis tidak dimungkinkan: kamera menandai nilai sebagai terlindungi.
settings-analysis-heading = Analisis fungsi
settings-analysis-purpose = Analisis fungsi mendokumentasikan di nilai mana dalam memori pengaturannya kamera menyimpan salah satu pengaturan menunya sendiri. Ditujukan bagi yang ingin memahami memori pengaturan model mereka dan membagikan hasilnya.
settings-analysis-why-off = Fungsi ini nonaktif karena penggunaan biasa tidak memerlukannya, karena beberapa kali mengalihkan kamera ke mode servis, dan karena atas permintaan juga menulis ke kamera. Fungsi ini eksperimental dan baru dicoba pada sedikit model.
settings-analysis-toggle = Aktifkan analisis fungsi (entri “Analisis” di menu)
settings-analysis-confirm = Aktifkan analisis fungsi? Fungsi ini beberapa kali mengalihkan kamera ke mode servis dan atas permintaan dapat menulis ke kamera. Gunakan hanya pada kamera Anda sendiri.
btn-analysis-enable = Aktifkan
settings-analysis-on = Analisis fungsi aktif.
settings-analysis-off = Analisis fungsi nonaktif.
analysis-needs-admin = Setiap cuplikan memerlukan hak administrator untuk mode servis. Windows memintanya sekali per sesi program; Anda dapat memulai pembantu driver sekarang atau menunggu hingga cuplikan pertama memintanya.
analysis-changed-hint = Harap periksa: tidak ada kandidat yang ditemukan. Nilai-nilai berikut berubah tetapi tidak kembali ke nilai awalnya. Jika salah satunya hanya berganti sekali di antara dua nilai, kamera kemungkinan tidak berada pada nilai menu pertama saat cuplikan pertama. Ulangi analisis dengan nilai awal yang diatur dengan benar.
analysis-changed-info = Info: nilai yang berubah sendiri selama analisis (mis. penghitung, jam) diabaikan. Anda tidak perlu melakukan apa pun.
elevation-admin = Administrator
elevation-standard = Hak standar
elevation-admin-hint = Program berjalan dengan hak administrator dan dapat menetapkan driver yang diperlukan mode servis.
elevation-standard-hint = Program berjalan dengan hak standar. Untuk mode servis, tweak, dan analisis fungsi, program memulai pembantu driver dengan hak administrator bila perlu; Windows bertanya sekali per sesi. Klik untuk memulainya.
driver-helper-declined = Permintaan hak administrator ditolak. Tanpa hak tersebut driver untuk mode servis tidak dapat ditetapkan; tidak ada yang diubah.
btn-remove-foreign-drivers = Hapus driver lama
btn-not-now = Nanti saja
btn-ok = OK
foreign-drivers-found = Windows masih menyimpan driver yang dipasang Zadig atau alat serupa untuk perangkat Sony: { $packages }. Windows memberikan kamera kepada driver tersebut sebelum program ini sempat menetapkan driver-nya sendiri, sehingga mode servis gagal dan kamera mungkin tidak muncul sebagai drive. Hapus sekarang? Setelah itu kamera memakai driver bawaan Windows. Penghapusan memerlukan hak administrator satu kali.
foreign-drivers-removed = Driver lama sudah dihapus. Cabut kamera lalu sambungkan kembali.
foreign-drivers-remaining = Driver berikut tidak dapat dihapus: { $packages }. Mulai ulang Windows dan coba lagi, atau copot pemasangan perangkat beserta driver-nya di Pengelola Perangkat.
foreign-drivers-hint = Driver yang dipasang Zadig atau alat serupa untuk perangkat Sony masih ada ({ $packages }). Windows memberikan kamera kepada driver tersebut sebelum program ini sempat menetapkan driver-nya sendiri, sehingga mode servis bisa gagal. Hapus dengan: pmca-console driver zadig --remove
driver-zadig-none = Tidak ada driver dari Zadig atau alat serupa yang terpasang untuk perangkat Sony.
driver-zadig-entry = { $package } ({ $provider }): { $devices }
driver-zadig-remove-hint = Hapus dengan: pmca-console driver zadig --remove
driver-zadig-removed = { $count } paket driver lama dihapus. Cabut kamera lalu sambungkan kembali.
elevation-helper = Pembantu driver aktif
elevation-helper-hint = Pembantu dengan hak administrator menetapkan driver untuk mode servis. Ia tidak memiliki jendela dan berakhir bersama program; tidak akan ada permintaan lagi.
elevation-helper-confirm = Mulai pembantu driver sekarang? Windows meminta hak administrator sekali dan tidak lagi sampai program ditutup.
analysis-cross-heading = Pemeriksaan silang pada kamera lain
analysis-cross-intro = Opsional: memeriksa apakah kamera lain juga memiliki nilai yang ditemukan dan apa yang diatur di sana. Hanya membaca. Nomor yang sama belum tentu berarti fungsi yang sama; hanya melihat menu kamera lain yang memastikannya. Hasilnya masuk ke laporan analisis yang sama.
analysis-cross-do = 1. Cabut kamera yang dianalisis. 2. Nyalakan kamera lain, atur mode USB-nya ke “Otomatis”, pasang kabel, dan pilih “Penyimpanan Massal (MSC)”.
btn-analysis-cross = Periksa kamera lain
analysis-cross-present = ada, nilai { $value }
analysis-cross-match = sama dengan “{ $label }”
analysis-cross-nomatch = tidak sama dengan nilai menu terukur mana pun
analysis-cross-absent = tidak ada
analysis-cross-same-camera = Ini adalah kamera yang dianalisis itu sendiri. Hubungkan kamera yang lain.
analysis-cross-not-now = Pemeriksaan silang baru dapat dilakukan setelah semua cuplikan dibaca dan tidak ada nilai tertulis yang menunggu dipulihkan.
analysis-cross-none = Tidak ada kandidat untuk diperiksa.
analysis-wrong-camera = Kamera yang terhubung bukan kamera yang dianalisis. Hubungkan { $model }; tidak ada yang ditulis.
keys-fetching = Mengunduh sumber…
keys-scanning = Mencari materi kunci…
keys-complete = Semua materi kunci tersedia.
keys-incomplete = Sebagian materi kunci masih hilang.
keys-present = tersedia
keys-missing = hilang
keys-class-senser = Kunci mode servis (shell servis, tweak, analisis)
keys-class-cic = Kunci checksum XPD (pemasangan aplikasi)
keys-class-spk = Materi kunci SPK (pemasangan aplikasi, apk2spk)
keys-class-fdat = Materi kunci FDAT (firmware updater-shell)
plugin-heading = Plugin
plugin-intro = Plugin menambahkan bahasa, sakelar untuk pengaturan kamera, informasi kompatibilitas, alat, dan diagnostik ke program ini. Plugin hanya berasal dari berkas yang Anda pilih sendiri — tidak ada toko plugin. Kodenya berjalan di sandbox tanpa akses ke berkas Anda, jaringan, atau materi kunci. Pasang hanya plugin dari pembuat yang Anda percayai; perubahan berlaku pada saat program dijalankan berikutnya.
plugin-none = Tidak ada plugin yang terpasang.
plugin-inactive = (aktif setelah dimulai ulang)
plugin-author = Pembuat
plugin-restart = Mulai ulang program agar perubahan berlaku.
btn-plugin-install = Pasang plugin…
plugin-install-confirm = Plugin ini dapat:
btn-confirm-plugin-install = Pasang plugin
plugin-installed = Plugin telah dipasang.
plugin-remove-confirm = Hapus plugin “{ $name }”?
plugin-removed = Plugin { $id } telah dihapus.
plugin-perm-languages = menambahkan bahasa ke program
plugin-perm-camera-settings = mengubah pengaturan kamera (hanya di antara dua nilai yang disebutkannya, dan hanya setelah Anda konfirmasi)
plugin-perm-compatibility = menambahkan entri ke ikhtisar kompatibilitas
plugin-perm-code = menjalankan kodenya sendiri di sandbox (alat atau diagnostik)
plugin-perm-host-os = menanyakan sistem operasi yang digunakan
plugin-perm-host-cameras = melihat kamera Sony mana yang terhubung dan dalam mode apa
plugin-invalid = Ini bukan plugin yang dapat digunakan: { $reason }
plugin-no-folder = Folder data pengguna tidak dapat ditentukan, sehingga plugin tidak dapat dipasang.
plugin-not-installed = Tidak ada plugin dengan ID { $id } yang terpasang.
plugin-failed = Plugin gagal: { $reason }
plugin-unknown-entry = Tidak ada plugin termuat yang menyediakan { $id }.
plugin-run-done = Plugin telah selesai.
plugin-run-warning = Plugin telah selesai dengan catatan.
plugin-run-error = Plugin melaporkan kesalahan.
plugin-tools-heading = Alat plugin
plugin-tools-hint = Alat ini berasal dari plugin yang terpasang dan berjalan di sandbox. Plugin hanya menerima apa yang Anda berikan dan apa yang telah dideklarasikannya, serta tidak dapat mengubah kamera.
plugin-diagnostics-heading = Diagnostik plugin
plugin-compat-heading = Dari plugin
plugin-col-note = Catatan
file-filter-plugin = Berkas plugin
keys-not-set-up = Materi kunci yang diperlukan operasi ini belum disiapkan.
err-missing-keys = Materi kunci yang diperlukan operasi ini belum disiapkan.
keys-heading = Perlu materi kunci
keys-intro = Kunci mode servis, kunci checksum XPD, dan materi kunci SPK bukan bagian dari program ini. Masukkan alamat sumber yang Anda miliki — repositori, arsip, atau berkas — dan program akan menemukan serta mengimpor bagian yang dikenalinya. Tidak ada yang dijalankan, dan materi hanya disimpan di komputer ini.
keys-law-notice = Patuhi hukum yang berlaku di tempat Anda, dan kerjakan hanya kamera dan data milik Anda sendiri.
keys-url-label = Alamat sumber (satu per baris)
keys-url-required = Masukkan setidaknya satu alamat sumber.
btn-keys-import = Impor
btn-import-keys = Siapkan materi kunci
