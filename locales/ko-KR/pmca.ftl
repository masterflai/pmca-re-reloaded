# PMCA-RE RELOADED 한국어 현지화.
# 한 줄에 메시지 하나씩: <id> = <text>. 새 언어를 추가하려면 이 파일을
# locales/<lang>/pmca.ftl 로 복사한 뒤 오른쪽 부분만 번역합니다.

app-title = PMCA-RE RELOADED – 카메라 툴킷

# 탐색 / 탭
nav-camera-info = 카메라 정보
nav-install-app = 앱
nav-tweaks = 트윅
nav-network = 네트워크
nav-firmware = 펌웨어
nav-updater = 업데이터 셸
nav-service = 서비스 셸
nav-tools = 파일 도구
nav-settings = 설정
nav-license = 라이선스

# 시작 페이지: 창이 열릴 때 처음 표시되는 페이지입니다. 이 툴킷의 용도와
# 가장 먼저 할 일, 그리고 어느 페이지가 어떤 질문에 답하는지 알려 줍니다.
#
# 이 애플리케이션에서 "언어"는 두 가지를 뜻합니다. 이 창의 언어와 카메라 자체
# 메뉴가 제공하는 언어입니다. 둘은 각각 한 구절이 아니라 별도의 제목으로
# 다룹니다. 둘을 혼동하는 것이 바로 이 페이지가 막으려는
# 실수이기 때문입니다.
nav-welcome = 시작
welcome-intro-heading = 이 프로그램이 하는 일
welcome-intro-body = 이 프로그램은 Sony 카메라의 기능을 보존하고 필요에 맞게 조정하도록 도와줍니다. Sony가 PlayMemories Camera Apps 스토어를 종료한 뒤로는 구입했거나 이미 설치한 앱을 백업하거나 다시 설치할 방법이 없었습니다. 카메라를 초기화하면 설치된 앱이 복구할 수 없게 삭제됩니다. 이 프로그램을 사용하면 기존 앱을 백업하고 다시 설치할 수 있으며, 자유롭게 사용할 수 있는 커뮤니티 앱도 제공받을 수 있습니다. 직접 만든 앱(*.apk 파일)도 설치할 수 있습니다. 문제가 생기면 프로그램이 분석을 위해 카메라 정보와 내부 로그를 읽어 옵니다. 카메라에 이미 있지만 메뉴에서는 제공하지 않는 설정을 전환하고 다시 되돌릴 수도 있습니다. 프로그램은 USB 케이블을 통해 카메라와 직접 통신합니다. 지원되는 모델은 카메라 정보 페이지에 나와 있습니다.
welcome-start-heading = 첫 단계
welcome-start-body = 먼저 카메라를 켜고, 케이블을 아직 연결하지 않은 상태에서 카메라 메뉴의 USB 연결을 대용량 저장장치 또는 MTP로 설정합니다. 자동으로 두어도 대개 문제없습니다. 이 경우 카메라가 두 방식 중 하나를 스스로 선택하기 때문입니다. 순서가 중요합니다. 케이블을 연결하면 카메라 메뉴에 더 이상 들어갈 수 없습니다. 그다음에 카메라를 USB로 연결하고 카메라 정보를 열어 "카메라 정보 가져오기"를 누릅니다. 모델과 펌웨어 버전이 표시되면 연결이 정상이며, 이 프로그램의 다른 모든 기능을 사용할 수 있습니다.
welcome-start-driver = 대신 드라이버 안내가 표시되면 먼저 그 안내를 따르십시오. Windows에서는 시스템 드라이버가 카메라를 점유하므로, 이 툴킷에 필요한 직접 USB 액세스를 사용하려면 그 드라이버를 교체해야 합니다. 프로그램이 이 작업을 직접 수행하지만, 드라이버 할당은 Windows가 관리자에게만 허용하는 권한입니다. 이 경우 프로그램이 관리자 권한으로 다시 시작하는 옵션을 제공하며, Windows에서 작업을 확인하라는 메시지가 표시됩니다. 할당은 세션이 끝나면 다시 해제됩니다. Linux와 macOS에서는 이 작업이 필요하지 않습니다.
welcome-start-driver-linux = 대신 드라이버 안내가 표시되면 먼저 그 안내를 따르십시오. Linux에서는 아무것도 교체할 필요가 없으며 카메라에 직접 접근합니다. 한 번만 준비하면 되는 것은 장치를 열 수 있는 권한, 즉 Sony 공급업체 ID에 대한 udev 규칙과, 대용량 저장장치 모드의 경우 커널 드라이버 usb-storage의 해제입니다. 두 가지 모두 이 프로그램 밖에서 root 권한으로 준비합니다. 소스 트리에 있는 scripts/linux-prepare.sh 스크립트가 둘 다 처리합니다. 이 창 자체는 그러한 권한을 절대 요청하지 않습니다.
welcome-start-driver-macos = 대신 드라이버 안내가 표시되면 먼저 그 안내를 따르십시오. macOS에서는 아무것도 설치하거나 교체할 필요가 없습니다. 카메라의 USB 연결을 MTP로 설정합니다. 트윅, 서비스 셸, 기능 분석, 펌웨어 업데이트, 업데이터 셸을 사용할 때는 이 창이 관리자 암호를 한 번 묻습니다. 이 기능들은 카메라의 대용량 저장장치 연결을 거치는데, macOS는 이 연결을 그러한 권한을 가진 프로그램에만 넘겨주기 때문입니다.
welcome-tweaks-heading = 트윅과 언어
welcome-tweaks-body = 트윅은 카메라에 이미 있지만 메뉴에서 제공하지 않는 설정입니다. 녹화 시간 제한, 4K 녹화 제한, PAL과 NTSC 선택, USB 앱 설치 프로그램, 그리고 메뉴 언어가 여기에 해당합니다.
welcome-tweaks-languages-heading = 카메라의 언어
welcome-tweaks-languages-body = Sony 카메라는 판매 지역에서 흔히 쓰이는 메뉴 언어만 표시합니다. 해외에 거주하는 경우 자신의 언어를 찾지 못하고 외국어로 만족해야 할 때가 많습니다. 그러나 펌웨어에는 이미 더 많은 언어가 내장되어 있습니다. 이 프로그램은 그 언어들을 메뉴에 불러와 익숙한 언어로 카메라를 조작할 수 있게 합니다.
welcome-tweaks-ui-heading = 이 프로그램의 언어
welcome-tweaks-ui-body = 이 모든 것과 별개로 위쪽에 언어 선택 상자가 있습니다. 이 상자는 이 프로그램의 언어만 바꾸며 카메라에는 전혀 영향을 주지 않습니다.
welcome-tweaks-safety = 모든 트윅은 카메라의 백업 메모리에 기록되며, 모두 같은 스위치로 되돌릴 수 있습니다. 쓰기가 중단되면 카메라가 영구적으로 손상될 수 있으므로, 쓰기가 진행되는 동안에는 케이블을 뽑거나 배터리를 빼지 마십시오.
welcome-privacy-heading = 사용자 데이터
welcome-privacy-where = 라이선스 페이지에서 이 모든 내용을 이 프로그램을 구성하는 모든 라이브러리의 라이선스와 함께 다시 빠짐없이 안내합니다.
welcome-where-heading = 어디에서 무엇을 찾을 수 있나요
welcome-where-info = 모델, 일련번호, 펌웨어 버전입니다. 연결 테스트 역할도 합니다.
welcome-where-install = 커뮤니티 목록의 앱 또는 직접 준비한 .apk 파일입니다.
welcome-where-tweaks = 카메라 메뉴로는 접근할 수 없는 설정이며, 언어도 여기에 포함됩니다.
welcome-where-network = GPS 보조 데이터, 그리고 라이브 스트리밍에 사용하는 액세스 포인트와 서비스입니다.
welcome-where-firmware = Sony 펌웨어 이미지를 카메라에 기록합니다.
welcome-where-tools = 펌웨어 파일과 패키지 파일의 압축을 풀고 검사합니다. 카메라가 필요하지 않습니다.
welcome-where-shells = 카메라의 메모리와 파일 시스템에 직접 접근합니다. 먼저 설정에서 켜야 합니다.
welcome-where-settings = 테마, 고급 모드, 이 프로그램의 번역 편집입니다.
welcome-where-license = 이 포트의 출처, 기반이 된 작업의 저작자, 적용되는 라이선스 조건입니다.
welcome-where-hint = 어떤 페이지도 스스로 카메라에 작업하지 않습니다. 해당 작업을 한다고 표시된 버튼을 누르기 전에는 아무것도 읽거나 쓰지 않습니다.

# 버튼
btn-get-info = 카메라 정보 가져오기
btn-refresh = 새로 고침
btn-load-apps = 사용 가능한 앱 불러오기
btn-list-tweaks = 사용 가능한 트윅 표시
btn-install = 선택한 앱 설치
btn-install-file = .apk 파일 설치…
btn-apply = 적용
btn-save = 카메라에 저장
btn-add = 추가
btn-remove = 제거
btn-reveal = 표시
btn-update-gps = GPS 보조 데이터를 장치로 전송
btn-cancel = 취소
btn-browse = 찾아보기…
btn-switch-msc = 대용량 저장장치 모드로 전환
btn-start-driver-helper = 드라이버 도우미 시작

# 파일 대화 상자에서 제공하는 파일 형식 필터의 레이블
file-filter-apk = Android 애플리케이션 (*.apk)
file-filter-firmware = Sony 펌웨어 이미지 (*.dat)
file-filter-usage-log = 사용 로그 (*.bin)
file-filter-catalog = 번역 파일 (*.ftl)
file-filter-spk = Sony 앱 패키지 (*.spk)
file-filter-any = 모든 파일
file-filter-gps = GPS 보조 데이터 (*.dat)

# 상태 / 메시지
msg-connecting = 연결 중…
msg-usage-log-written = 사용 로그를 { $path }에 저장했습니다
msg-apps-found = 앱 { $count }개를 찾았습니다
msg-switch-mode = 카메라를 MTP 또는 대용량 저장장치 모드로 전환하십시오.
msg-no-tweaks = 이 카메라에 사용할 수 있는 트윅이 없습니다.
msg-done = 완료되었습니다.

# 카메라 USB 모드 전환(MTP -> 대용량 저장장치). "잘못된 모드" 오류 아래에
# 표시됩니다. 반대 방향으로 전환하는 USB 명령은 없습니다.
mode-switching = 카메라에 대용량 저장장치 모드로 전환하도록 요청하는 중…
mode-switch-wait = 카메라가 다시 연결되기를 기다리는 중…
camera-confirm-hint = 카메라에 표시되는 메시지(예: www.sony.net/pm 관련)는 닫아도 됩니다. 작업에 방해가 되지 않습니다.
mode-switch-done = 카메라가 이제 대용량 저장장치 모드입니다.
mode-switch-timeout = 카메라가 대용량 저장장치 모드로 다시 나타나지 않았습니다. 카메라 전원을 껐다 켠 뒤 다시 시도하십시오.
mode-switch-impossible = 이 모드는 USB로 변경할 수 없습니다. 카메라 전원을 껐다 켜십시오.
mode-switch-hint = USB로 전환할 수 있는 것은 MTP 모드의 카메라뿐입니다. 이 변경은 일시적입니다. 카메라 자체 메뉴에서 선택한 USB 모드는 그대로 유지되므로, 카메라를 껐다 다시 켜거나 케이블을 뽑았다 다시 연결하면 이전 모드로 돌아갑니다. 되돌아가는 방향의 USB 명령은 없습니다.

# 카메라 정보 필드
info-model = 모델
info-product-code = 제품 코드
info-serial = 일련번호
info-firmware = 펌웨어 버전
info-lens = 렌즈
info-region = 지역
info-mode = 모드
mode-mass-storage = 대용량 저장장치
mode-mtp = MTP
mode-updater = 업데이터 모드
mode-app-install = 앱 설치 모드
mode-service = 서비스 모드
mode-unknown = 알 수 없음
info-android-version = Android 플랫폼 버전
info-photographer = 촬영자
info-copyright = 저작권
info-photographer-1 = 촬영자 (항목 1)
info-photographer-2 = 촬영자 (항목 2)
info-copyright-1 = 저작권 (항목 1)
info-copyright-2 = 저작권 (항목 2)
diagnose-services-heading = 카메라에 연결할 수 없나요?
diagnose-services-hint = 일부 Windows 서비스가 MTP 모드의 카메라를 점유하여 이 애플리케이션이 카메라에 접근하지 못할 수 있습니다. 이 검사는 Windows에 실행 중인 서비스를 묻기만 할 뿐 아무것도 변경하지 않습니다.
btn-diagnose-services = Windows 서비스 확인
diagnose-services-none = 방해가 되는 것으로 알려진 서비스가 실행되고 있지 않습니다.
diagnose-services-found = 실행 중인 다음 서비스가 카메라 접근을 방해할 수 있습니다:
diagnose-services-advice = 서비스(services.msc)에서 해당 서비스를 중지하거나 관리자 권한으로 다음 명령을 실행한 뒤 카메라를 다시 연결하십시오:

# 펌웨어 업데이터 / 셸
updater-model = 카메라 모델
updater-firmware-current = 현재 펌웨어 버전
updater-guard-ok = 펌웨어 이미지가 이 카메라와 일치합니다.
updater-no-fdat = 모델 { $model }에 사용할 수 있는 내장 업데이터 펌웨어가 없습니다.
updater-model-required = 카메라가 이미 업데이터 모드에 있어 모델을 알려 줄 수 없습니다. 카메라를 껐다 다시 켜서 일반 모드로 시작되게 한 뒤 다시 시도하십시오. 또는 페이로드 파일(--fdat-file)을 지정하거나, 콘솔에서 모델(--model <MODEL>)을 지정하십시오.
updater-already-updater = 카메라가 이미 업데이터 모드에 있습니다.
updater-flash-required = 확인 없이는 카메라를 수정하지 않습니다. 업데이터 모드로 전환하고 페이로드를 기록하려면 --flash 옵션을 붙여 다시 실행하십시오.
updater-flashing = 업데이터 페이로드를 카메라에 기록하는 중…
updater-switch-wait = 카메라 화면의 안내를 따르십시오. 카메라가 업데이터 모드로 전환되기를 기다리는 중…
updater-switch-timeout = 카메라를 기다리는 동안 시간이 초과되었습니다. 카메라가 다시 연결되면 명령을 다시 실행하십시오.
updater-shell-ready = 업데이터 페이로드가 실행 중입니다. 명령 목록을 보려면 'help', 종료하려면 'exit'를 입력하십시오.
updater-shell-done = 셸 세션이 종료되었습니다.
updater-session-panic = 업데이터 세션이 예기치 않게 종료되었습니다. 다시 시도하기 전에 카메라 전원을 껐다 켜십시오.

# 업데이터 셸의 응답입니다. 구문 분석 실패 메시지는 service-* 키에 한 번만
# 작성되어 있습니다. 두 셸은 같은 토크나이저를 사용하므로, 따옴표로 묶인 경로와
# 잘못된 16진수 값은 어느 셸에서든 같은 방식으로 실패합니다.
updater-backup-synced = 백업을 플래시에 동기화했습니다.
updater-backup-locked = 보호된 백업 설정이 이제 잠겼습니다.
updater-backup-writable = 보호된 백업 설정에 이제 쓸 수 있습니다.
updater-shell-console-only = 루트 셸은 터미널이 필요하므로 콘솔에서만 사용할 수 있습니다. pmca-console updatershell --flash 를 실행한 뒤 그곳에서 'shell'을 입력하십시오.

# 업데이터 셸 페이지.
updater-warning = 경고: 이 셸에 들어가면 카메라가 업데이터 모드로 재부팅되고 페이로드가 기록됩니다. 페이로드는 이 프로젝트의 작은 프로그램으로, 카메라가 펌웨어 업데이트처럼 받아들인 뒤 이 셸의 명령에 응답합니다. 페이로드 자체는 카메라 전원을 끄면 사라지지만, 페이로드가 제공하는 명령은 카메라의 영구 설정 메모리에 기록하며, 쓰기가 중단되면 카메라가 영구적으로 손상될 수 있습니다. 또한 서비스 셸과 달리 이 과정에서 카메라의 모든 설정이 초기화됩니다.
updater-console-heading = 콘솔에서 업데이터 셸 실행
updater-driver-hint = 업데이터 셸은 페이로드가 Windows 자체 대용량 저장장치 드라이버인 USBSTOR를 통해 전달되므로 카메라가 이 드라이버를 사용해야 합니다. 이는 서비스 셸에 필요한 것과 정반대입니다. 애플리케이션이 WinUSB 드라이버를 직접 할당하고 해제하므로 수동으로 준비할 것은 없지만, 다른 도구로 할당한 드라이버는 먼저 되돌려야 합니다. 업데이터 모드의 카메라는 자체 제품 ID로 열거되며, 이 장치에도 드라이버를 다시 바인딩해서는 안 됩니다.
updater-fdat-label = 페이로드 파일 (선택 사항)
updater-confirm = 카메라가 업데이터 모드로 다시 시작되고 페이로드가 기록됩니다. 계속하시겠습니까?
updater-cancelled = 취소되었습니다. 카메라는 수정되지 않았습니다.

# 두 셸 페이지에 접힌 상태로 표시되는 설명입니다. 명령 구문 자체는 정확히
# 입력해야 하는 단어이므로 절대 번역하지 않습니다. 따라서 여기에는 각 명령
# 옆에 표시되는 설명만 있습니다.
btn-shell-primer = 이 셸에 대하여
btn-shell-primer-hide = 설명 숨기기
shell-primer-compare-heading = 두 개의 셸, 두 개의 프로토콜
shell-primer-compare = 두 페이지 모두 카메라에 대한 명령줄을 제공하지만 같은 셸은 아닙니다. 서비스 셸은 Sony 자체의 공장 서비스 프로토콜을 사용하며, 카메라가 사용자를 인증한 뒤에는 카메라가 스스로 응답합니다. 이를 위해 카메라에 아무것도 기록하지 않습니다. 업데이터 셸은 이 프로젝트가 자체 프로그램을 카메라에 기록하기 때문에 존재합니다. 카메라가 업데이터 모드로 다시 시작하고 페이로드를 펌웨어 이미지처럼 받아들이며, 그 후 페이로드가 아래에 나열된 명령에 응답합니다. 두 셸 중 한 번에 하나만 열 수 있습니다.
shell-primer-payload = "페이로드"는 전달 경로가 실어 나르는 실제 내용을 뜻합니다. 업데이터 셸의 경우 이 프로젝트의 작은 프로그램입니다. 카메라는 이를 펌웨어 업데이트와 같은 방식으로 받아들이지만, 전원을 끌 때까지만 보관합니다. 서비스 셸은 카메라가 자체 펌웨어로 명령에 응답하므로 페이로드가 필요하지 않습니다. 펌웨어 파일에서도 이 용어는 같은 의미로, .dat 파일 컨테이너 안에 있는 실제 펌웨어 이미지를 가리킵니다.
shell-primer-when-service = 카메라가 자체 펌웨어를 계속 실행하는 동안 저장된 설정을 읽거나 편집하는 모든 작업에는 서비스 셸을 사용하십시오. HASP 블록 읽기, 백업 영역의 복원용 사본 저장, 개별 항목 변경, 카메라에서 파일 복사 등이 이에 해당합니다. 트윅 페이지도 내부적으로 이 셸을 사용하므로, 트윅 페이지에서 할 수 있는 모든 작업을 여기서 직접 할 수 있습니다.
shell-primer-when-updater = 다른 방법으로는 카메라에 접근할 수 없을 때, 또는 개별 명령이 아니라 실행 중인 시스템의 셸이 필요할 때 업데이터 셸을 사용하십시오. 진짜 루트 셸을 제공하는 유일한 경로이며, 서비스 모드를 거부하는 카메라에서도 작동합니다. 대신 재시작과 쓰기 작업이 필요하고 카메라 설정이 초기화되므로, 첫 번째가 아닌 두 번째 선택지입니다.
shell-primer-driver-heading = 드라이버: 두 셸은 서로 반대되는 것이 필요합니다
shell-primer-driver = 서비스 셸은 원시 USB로 통신하므로 Windows에서 카메라가 WinUSB에 바인딩되어 있어야 합니다. 애플리케이션이 서비스 모드로 들어갈 때 이 드라이버를 직접 할당하고 세션이 끝나면 다시 해제하므로 별도의 도구가 필요 없습니다. 업데이터 셸은 Windows 자체 대용량 저장장치 드라이버인 USBSTOR를 통해 전달되며, 카메라가 WinUSB에 바인딩되어 있는 동안에는 작동하지 않습니다. 따라서 할당 해제는 나중에 덧붙이는 일이 아니라 서비스 세션을 마치는 과정의 일부입니다. 드라이버를 할당하려면 관리자 권한이 필요합니다. Linux에서는 두 셸 모두 드라이버 교체가 필요 없으며, 장치 노드에 대한 접근 권한만 있으면 됩니다.
shell-primer-risk-heading = 발생할 수 있는 문제
shell-primer-risk = 아래에 표시된 명령은 카메라의 영구 설정 메모리에 기록합니다. 쓰기가 중단되면 카메라가 시작되지 않을 수 있으며, 여기서 한 작업은 되돌릴 수 없습니다. 바로 이 때문에 bk d가 있습니다. 무엇이든 변경하기 전에 백업 영역의 사본을 저장하십시오. 업데이터 페이로드 자체는 카메라에 남지 않으며, 카메라 전원을 끄면 사라집니다. 그러나 업데이터 모드를 거치는 경로는 카메라의 모든 설정을 초기화합니다.
shell-primer-commands-heading = 이 셸의 명령
shell-primer-writes = (WRITES)로 표시된 명령은 카메라를 변경합니다. 그 밖의 명령은 읽기만 합니다.
shell-cmd-hasp = 카메라의 보호된 식별 영역인 HASP 블록을 읽습니다.
shell-cmd-pull = 카메라에서 파일을 복사해 옵니다. 로컬 이름을 지정하지 않으면 원격 파일 이름이 유지됩니다.
shell-cmd-push = 로컬 파일을 카메라로 복사하며, 그 위치에 있던 파일을 대체합니다. (WRITES)
shell-cmd-rm = 카메라의 파일을 삭제합니다. (WRITES)
shell-cmd-rmem = 장치 메모리의 한 영역을 로컬 파일에 기록합니다. 오프셋과 길이는 16진수입니다.
shell-cmd-bk-r = 백업 영역의 항목 하나를 읽어 16진수 덤프로 표시합니다.
shell-cmd-bk-d = 백업 영역 전체를 파일로 저장합니다. 무엇이든 변경하기 전에 이 작업을 하십시오. 되돌릴 수 있는 유일한 방법입니다.
shell-cmd-bk-dump-txt = 백업 영역 전체를 <file>.bin으로, 읽을 수 있는 목록을 <file>.txt로 저장합니다.
shell-cmd-bk-stat = 지역 정보를 포함한 카메라의 백업 상태 블록을 표시합니다.
shell-cmd-bk-w = 백업 영역의 항목 하나를 공백으로 구분한 16진수 바이트로 기록합니다. (WRITES)
shell-cmd-bk-s-sub = 한 서브시스템의 백업 영역을 플래시에 저장하여 재시작 후에도 변경 사항이 유지되게 합니다. (WRITES)
shell-cmd-bk-s = 백업 영역을 플래시에 저장하여 재시작 후에도 변경 사항이 유지되게 합니다. (WRITES)
shell-cmd-bk-lock = 보호된 백업 항목의 쓰기 보호를 설정하거나 해제합니다. 쓰기 보호가 설정되어 있으면 일부 설정은 변경되지 않습니다. (WRITES)
shell-cmd-term = 카메라의 디버그 터미널을 켜거나 끕니다. (WRITES)
shell-cmd-info = 영역에 쓸 수 있는지 여부를 알려 주는 백업 상태 블록을 읽습니다.
shell-cmd-shell = 카메라에서 루트 셸을 엽니다. 터미널이 필요하므로 콘솔에서만 사용할 수 있습니다. 대신 shell tcp <port>를 사용하면 다른 클라이언트용으로 127.0.0.1에서 셸을 제공합니다. (WRITES)
shell-cmd-help = 이 셸에서 사용할 수 있는 명령을 나열합니다.
shell-cmd-exit = 세션을 끝내고 셸을 나갑니다.
shell-primer-diagnostics-heading = 진단 및 복구
shell-cmd-bk-trace = 카메라가 백업 영역 전체를 USB로 보내는 과정을 패킷 단위로 파일에 기록합니다. 대용량 백업을 조사하기 위한 기능이며, 카메라가 응답 도중에 멈출 수 있습니다. 그런 경우 케이블을 뽑고 카메라를 껐다 켜십시오. 필요하면 배터리를 분리하십시오.
shell-cmd-bk-rewrite = 백업 영역을 읽어 변경하지 않은 채 다시 기록한 뒤 다시 읽습니다. 설정을 하나도 바꾸지 않고 카메라가 전체 이미지를 받아들이는지 보여 줍니다. (WRITES)
shell-cmd-bk-restore-attrs = 모든 백업 항목의 속성(예: 쓰기 보호)을 bk d 또는 bk dump-txt로 저장한 사본의 속성으로 되돌립니다. 값은 그대로 유지됩니다. 같은 카메라의 사본만 사용하십시오. (WRITES)
shell-primer-diagnostics-env = 다음 세 환경 변수는 시작하기 전에 설정하면 콘솔과 그래픽 인터페이스 모두에 적용됩니다. 예를 들어 PowerShell에서는 $env:PMCA_TWEAKS_DIRECT = "1"로 설정합니다. PMCA_TWEAKS_DIRECT는 백업이 1 MB보다 작아도 트윅이 값을 하나씩 기록하게 합니다. PMCA_TWEAKS_IMAGE는 이미지가 1 MB보다 커도 전체 이미지를 기록하게 합니다. 둘 다 설정되면 PMCA_TWEAKS_DIRECT가 우선합니다. PMCA_TLS_DUMP=<file>은 앱 설치 중에 카메라의 첫 TLS 메시지(ClientHello)를 저장하여 실패한 연결을 조사할 수 있게 합니다.

# 앱 설치
install-switching = 앱 설치 모드로 전환하는 중…
install-disabled = 카메라가 앱 설치 모드로 들어가기를 거부했습니다. 대개 "USB 앱 설치 프로그램 사용" 설정 때문입니다. 이 설정을 "트윅"에서 켜야 합니다.
install-stays-app-mode = 카메라는 앱 설치 모드에 머물러 있으며, 이 모드에서 다른 앱을 더 설치할 수 있습니다. 다른 모든 기능을 사용하려면 USB 케이블을 뽑고 카메라를 껐다 다시 켜십시오.
err-usb-app-install-mode = 카메라가 아직 앱 설치 모드에 있으며, 이 모드에서는 앱 설치 외에는 아무것도 받아들이지 않습니다. USB 케이블을 뽑은 다음 카메라를 껐다 다시 켜십시오. 연결된 상태로 다시 시작하면 곧바로 이 모드로 돌아갑니다.
install-switch-wait = 카메라가 앱 설치 모드로 다시 연결되기를 기다리는 중…
install-switch-timeout = 카메라가 앱 설치 모드로 들어가기를 기다리는 동안 시간이 초과되었습니다. 카메라가 다시 연결되면 명령을 다시 실행하십시오.
install-bad-mode = 이 모드에서는 카메라를 사용할 수 없습니다. 카메라를 MTP 또는 대용량 저장장치 모드로 전환하십시오.
install-starting = 설치를 시작하는 중…
install-error = 설치 통신 오류
install-server-failed = 로컬 설치 서버가 카메라의 연결을 받아들이지 않았습니다:
install-done = 작업이 완료되었습니다.

# APK 검사
apk-package = 패키지:
apk-unreadable = 경고: .apk 파일을 분석할 수 없습니다.
apk-incompatible = 경고: 이 앱은 카메라가 제공하지 않을 수 있는 최소 SDK 수준을 선언합니다.
apk-unsigned = 경고: .apk 파일에 서명이 없습니다.

# 커뮤니티 앱 스토어
store-loading = 앱 목록을 불러오는 중…
store-resolving = 최신 릴리스를 찾는 중…
store-downloading = 다운로드 중
store-unknown-app = 알 수 없는 앱
store-no-release = 이 앱에는 다운로드할 수 있는 릴리스가 없습니다
store-not-installable = 릴리스 없음
msg-no-apps = 앱을 찾을 수 없습니다.
store-heading = 커뮤니티 앱 스토어
store-discontinued = Sony가 공식 PlayMemories Camera Apps 스토어를 종료했습니다. 서버가 더 이상 응답하지 않으므로 그곳에서 앱을 구입하거나 다운로드할 수 없습니다.
store-use-own-apk = 직접 준비한 .apk 파일은 여전히 설치할 수 있습니다. 아래에서 파일을 선택하거나 이 창으로 끌어다 놓으십시오. 아래쪽의 커뮤니티 앱 스토어도 계속 작동합니다.
btn-store-sync = 카메라 확인
btn-install-selected = 선택 항목 설치
btn-store-stop = 이 앱 다음에 중지
btn-store-stopping = 이 앱 다음에 중지하는 중…
btn-store-retry = 실패한 앱 다시 시도
btn-continue = 계속
btn-reinstall = 다시 설치
btn-skip = 건너뛰기
btn-confirm-updater = 다시 시작하고 기록
btn-confirm-tweaks = 트윅 기록
btn-confirm-languages = 언어 활성화
btn-confirm-wifi = 액세스 포인트 바꾸기
btn-confirm-stream = 설정 덮어쓰기
btn-confirm-show-shells = 셸 표시
btn-confirm-discard = 번역 버리기
store-select-all = 모든 앱 선택
store-col-app = 앱
store-col-version = 버전
store-col-author = 작성자
store-col-camera = 카메라의 버전
store-col-status = 상태
store-camera-absent = 설치되지 않음
store-version-unknown = 알 수 없음
store-batch-query = 카메라에서 설치된 앱을 읽는 중…
store-sync-done = 카메라를 확인했습니다. 목록에 있는 앱 중 설치된 앱: { $count }개
store-batch-downloading = { $total }개 중 { $current }번째 앱 다운로드 중…
store-batch-installing = { $total }개 중 { $current }번째 앱: { $name } 설치 중…
store-reinstall-confirm = { $name }이(가) 이미 카메라에 설치되어 있습니다. 설치된 버전: { $installed }. 앱 스토어의 버전: { $store }. 다시 설치하시겠습니까?
store-state-queued = 대기 중
store-state-installed = 설치됨
store-state-failed = 실패
store-state-skipped = 건너뜀
store-state-not-run = 실행 안 됨
store-batch-summary = 설치됨: { $done } · 실패: { $failed } · 건너뜀: { $skipped } · 실행 안 됨: { $notrun }
store-sources-hint = 앱은 설정에서 추가한 소스에서 가져옵니다. 이 프로그램은 이 앱들을 검사하지 않습니다.
app-backup-heading = 설치된 앱 백업
app-backup-intro = 카메라에 설치된 앱을 이 컴퓨터로 복사하여, 더 이상 어디에서도 구할 수 없게 되더라도 나중에 다시 설치할 수 있게 합니다. 이를 위해 카메라가 업데이터 모드로 다시 시작되며, 앱 저장 영역은 읽기만 합니다.
app-backup-own-camera = 본인 카메라의 앱만 백업하고 파일을 다른 사람에게 넘기지 마십시오. 앱에 대한 권리는 해당 작성자에게 있습니다.
app-backup-restore-hint = 앱을 복원하려면 위의 “.apk 파일 설치…”에서 저장한 .apk 파일을 선택하십시오.
btn-app-backup = 앱 백업
app-backup-confirm = 카메라가 업데이터 모드로 다시 시작되고, 카메라를 끌 때까지 실행되는 페이로드가 기록됩니다. 앱 저장 영역은 읽기만 합니다. 계속하시겠습니까?
btn-confirm-app-backup = 다시 시작하고 백업
app-backup-cancelled = 백업을 취소했습니다.
app-backup-confirm-required = 백업을 하면 카메라가 업데이터 모드로 다시 시작됩니다. --confirm 옵션으로 확인하십시오.
app-backup-no-folder = 이 컴퓨터에 백업용 데이터 폴더가 없습니다. --out 옵션으로 폴더를 지정하십시오.
app-backup-no-model = 카메라 모델을 읽을 수 없습니다. 카메라를 껐다 다시 켜고 대용량 저장장치 또는 MTP 모드로 연결한 뒤 다시 시도하십시오.
app-backup-mounting = 카메라의 앱 저장 영역을 찾는 중…
app-backup-no-partition = 이 카메라에서 앱 저장 영역을 찾을 수 없습니다. 앱을 지원하지 않는 카메라로 보입니다.
app-backup-copying = { $name } 복사 중({ $current }/{ $total })…
app-backup-none = 카메라에 설치된 앱이 없습니다.
app-backup-done = 앱 { $count }개를 저장했습니다.
app-backup-power-cycle = 업데이터 모드를 종료하려면 카메라를 껐다 다시 켜십시오.
app-backup-folder = 폴더
app-backup-col-file = 파일
app-backup-state-new = 저장됨
app-backup-state-kept = 이미 저장됨
btn-copy-path = 경로 복사
msg-path-copied = 경로를 클립보드에 복사했습니다.
store-no-sources = 아직 추가된 소스가 없습니다. 설정 → 앱 및 플러그인 소스에서 소스를 추가하십시오. 그곳에서 원래 프로젝트의 앱 목록이 제안으로 표시됩니다.
store-col-source = 소스
store-filter-source = 표시할 소스
store-filter-all = 모든 소스
sources-heading = 앱 및 플러그인 소스
sources-intro = 앱과 플러그인은 사용자가 여기에서 직접 추가하는 소스에서 가져옵니다. 소스는 개발자가 자신의 작업물을 공개하는 공개 GitHub 저장소입니다. 이 프로그램은 어떤 소스나 그 안의 어떤 것도 호스팅하거나 검사하거나 추천하지 않으며, 소스에 대한 책임은 오직 해당 소스의 작성자에게 있습니다. 저장소를 추가하기 전에 먼저 살펴보십시오.
sources-none = 아직 추가된 소스가 없습니다.
sources-add-label = GitHub 저장소 (소유자/이름 또는 저장소 주소)
btn-source-add = 소스 추가
btn-confirm-source-add = 소스 추가
sources-add-confirm = { $source }을(를) 소스로 추가하시겠습니까? 이 프로그램은 이 소스를 검사하지 않았습니다. 이 소스가 제공하는 것은 작성자의 책임하에 설치됩니다. 동의하기 전에 저장소를 살펴보십시오: { $url }
sources-adding = 소스를 읽는 중…
sources-added = 소스를 추가했습니다: { $source }
sources-check-repo = 저장소에서 무엇이든 설치하기 전에 저장소를 살펴보십시오: { $url }
sources-removed = 소스를 제거했습니다: { $source }
sources-remove-confirm = 소스 { $source }을(를) 제거하시겠습니까? 이 소스의 앱에 대해 기억해 둔 서명자 정보도 함께 삭제됩니다.
sources-stars-title = GitHub 스타: 이 저장소에 스타를 표시한 사용자 수입니다. 품질이나 안전성에 대해서는 아무것도 알려 주지 않습니다.
sources-suggestion-heading = 제안
sources-suggestion-community = 작성자가 프로젝트를 보관 처리한 카메라 앱입니다. 이 목록은 이 프로그램의 작성자 masterflai가 관리하며, 앱 작성자가 직접 게시한 릴리스만 가리키고 자체 파일은 포함하지 않습니다. 앱은 검사되지 않았습니다.
sources-suggestion-applist = 원래 프로젝트인 Sony-PMCA-RE의 앱 목록입니다. 이 프로그램이 아니라 작성자 ma1co가 관리하며, 수년째 업데이트되지 않았습니다.
btn-source-activate = 활성화
sources-search-heading = GitHub에서 소스 찾기
sources-search-hint = GitHub에서 pmca-source 토픽이 있는 저장소를 검색합니다. 검색 결과는 이 프로그램이 검사한 것이 아니며 추천도 아닙니다. 저장소를 추가하기 전에 먼저 살펴보십시오. 스타는 그 저장소에 스타를 표시한 GitHub 사용자 수만 보여 줍니다.
btn-source-search = GitHub 검색
sources-searching = GitHub를 검색하는 중…
sources-search-none = GitHub에서 pmca-source 토픽이 있는 저장소를 찾지 못했습니다.
sources-search-found = 저장소 { $count }개를 찾았습니다. 이 중 어느 것도 이 프로그램이 검사하지 않았습니다.
sources-updated = 마지막 변경: { $date }
sources-archived = 보관됨
sources-already = 이미 추가됨
sources-no-folder = 사용자 데이터 폴더를 확인할 수 없어 소스를 저장할 수 없습니다.
source-invalid = GitHub 저장소가 아닙니다. 소유자/이름(예: ma1co/OpenMemories-AppList) 또는 저장소 주소를 입력하십시오.
source-exists = { $source }은(는) 이미 추가되어 있습니다.
source-unknown = 해당 소스가 없습니다: { $source }
source-unreadable = { $source }의 기본 브랜치에 읽을 수 있는 { $file }이(가) 없습니다.
source-failed = 소스 { $source }을(를) 읽을 수 없습니다: { $reason }
source-signer-changed = { $source }의 { $name }이(가) 이전과 다른 사람에 의해 서명되어 설치하지 않았습니다. 작성자가 실제로 서명 키를 변경했다면 소스를 제거한 후 다시 추가하십시오.
source-signer-pinned = { $source }의 { $name }이(가) 소스에 지정된 인증서로 서명되지 않아 설치하지 않았습니다.
source-ambiguous = 여러 소스에서 { $package }을(를) 제공합니다: { $sources }. --source로 하나를 선택하십시오.
source-plugin-mismatch = 다운로드한 플러그인의 ID는 { $found }이지만 소스에서 알린 ID는 { $id }입니다. 플러그인을 설치하지 않았습니다.
plugin-sources-heading = 추가한 소스의 플러그인
plugin-sources-hint = 누구나 자신의 소스에 플러그인을 게시할 수 있습니다. 이 목록에는 앱 및 플러그인 소스에서 추가한 모든 소스의 플러그인이 게시자와 관계없이 표시되며, 이 프로그램은 이를 검사하지 않습니다.
plugin-signed = 서명됨, 키 { $key }
plugin-unsigned = 서명되지 않음
plugin-signer-changed = { $id }은(는) 이 파일과 다른 서명 키로 설치되어 있거나 이 파일에 서명이 없습니다. 플러그인을 교체하지 않았습니다. 작성자가 실제로 키를 변경했다면 먼저 설치된 플러그인을 제거하십시오.
plugin-key-written = 서명 키를 { $path }에 저장했습니다. 이 파일을 비밀로 유지하고 백업해 두십시오. 이 파일을 가진 사람은 누구나 사용자의 이름으로 플러그인에 서명할 수 있으며, 이 파일이 없으면 기존 설치에서 받아들이는 업데이트를 게시할 수 없습니다. 공개 키: { $key }
btn-details = 세부 정보
details-repository = 저장소
details-homepage = 프로젝트 페이지
details-source = 소스
msg-url-copied-browser = 주소를 클립보드에 복사했습니다. 페이지를 보려면 브라우저에 붙여 넣으십시오.
store-activate-suggestion = 아직 추가된 소스가 없습니다. 원래 프로젝트의 앱 목록({ $source })을 활성화하시겠습니까? 이 목록은 이 프로그램이 아니라 작성자 ma1co가 관리하며, 이 프로그램은 이 목록을 검사하지 않았습니다. 그러면 목록이 설정 → 앱 및 플러그인 소스에 표시됩니다.
btn-activate-list = 목록 활성화
btn-plugin-sources-load = 소스의 플러그인 표시
plugin-sources-none = 추가한 소스에서 제공하는 플러그인이 없습니다.
plugin-source-unchecked = 소스: { $source }. 이 프로그램은 이 플러그인을 검사하지 않았습니다.
btn-plugin-source-install = 설치

# 서비스(senser) 모드
usb-mode-advice = 트윅, 백업, 서비스 셸을 사용하려면 카메라의 USB 연결 모드를 대용량 저장장치로 고정하지 말고 “자동”으로 설정하십시오. 세션이 끝난 뒤 카메라가 모드를 물으면 “대용량 저장장치(MSC)”를 선택하십시오. 대용량 저장장치로 고정되어 있으면 카메라가 다음 서비스 모드 전환을 거부하는 경우가 많습니다.
service-switch-refused = 카메라가 서비스 모드로의 전환을 완료하지 못했고 USB로 다시 연결되지 않았습니다. 이 경우 카메라에는 대개 “연결된 장치를 확인하십시오”가 표시됩니다. USB 케이블을 뽑았다가 다시 꽂고 다시 시도하십시오.
service-switching = 서비스 모드로 전환하는 중…
service-switch-wait = 카메라가 서비스 모드로 다시 연결되기를 기다리는 중…
service-switch-timeout = 카메라가 서비스 모드로 나타나지 않았습니다. 카메라는 별도의 USB 장치(제품 ID 0336)로 다시 열거되며, 이 장치에는 별도의 WinUSB 드라이버 할당이 필요합니다. 장치 관리자에서 드라이버가 없는 Sony 장치가 있는지 확인하십시오. 카메라 전원을 껐다 켜면 일반 모드로 돌아갑니다.
service-authenticating = 인증하는 중…
# 이전 실행 때문에 카메라가 아직 서비스 모드에 있는 것을 발견했을 때, 그리고
# 이를 되돌릴 수 없었을 때 표시됩니다.
service-stranded-reset = 이전 실행 때문에 카메라가 아직 서비스 모드에 있습니다. 먼저 해당 세션을 종료하는 중…
service-stranded-failed = 이전 실행 때문에 카메라가 서비스 모드에서 벗어나지 못하고 있으며, 서비스 모드 종료 요청에 응답하지 않았습니다. 카메라를 껐다 다시 켠 뒤 다시 시도하십시오.
service-bad-mode = 이 모드에서는 카메라를 사용할 수 없습니다. 카메라를 대용량 저장장치 모드로 전환하십시오.
service-driver-required = 원시 USB 액세스가 거부되었고 WinUSB 드라이버를 할당할 수 없었습니다. 드라이버를 할당하려면 관리자 권한이 필요합니다.
service-warning = 경고: 서비스 모드는 카메라의 메모리, 파일 시스템, 보정 데이터에 대한 원시 액세스를 허용합니다. 쓰기 명령은 카메라를 영구적으로 손상시킬 수 있습니다.
service-shell-ready = 서비스 셸이 준비되었습니다. 명령 목록을 보려면 'help', 종료하려면 'exit'를 입력하십시오.
service-shell-done = 서비스 셸 세션이 종료되었습니다.
# 이전 실행이 할당한 드라이버를 해제하기 전에 중단되었을 때 시작 시
# 보고됩니다.
driver-swept = 이전 실행에서 남은 드라이버 할당 { $count }개를 제거했습니다.
# 같은 중단된 실행이 카메라를 서비스 모드에 남겨 두어 다음 세션을 거부하게
# 된 경우에도 시작 시 보고됩니다.
driver-swept-camera = 이전 실행 때문에 서비스 모드에 남아 있던 카메라를 일반 모드로 되돌렸습니다.
driver-release-replug = 드라이버를 해제하는 동안 Windows가 카메라를 기다리고 있습니다. 정리 작업을 마칠 수 있도록 USB 케이블을 뽑았다가 다시 꽂으십시오.
# `driver` 명령의 결과입니다.
driver-bound = { $id }이(가) 이제 WinUSB를 사용합니다(드라이버 패키지 { $package }).
driver-released = 드라이버 할당 { $count }개를 해제했습니다. 카메라가 Windows 기본 제공 드라이버로 돌아갔습니다.
driver-needs-elevation = 드라이버를 할당하려면 관리자 권한이 필요합니다. 관리자 권한으로 시작한 콘솔에서 이 명령을 실행하십시오.
driver-leftover-needs-elevation = 이전 실행에서 카메라에 할당된 드라이버가 남아 있습니다. 이 드라이버가 남아 있는 동안 카메라는 연결된 장치를 확인하라는 메시지를 표시하며 대용량 저장장치가 작동하지 않습니다. 드라이버를 해제하려면 관리자 권한이 필요합니다. Windows 확인 메시지에서 승인하십시오.
# 외부 도구가 필요 없도록 서비스 경로가 직접 수행하는 드라이버
# 할당입니다.
driver-binding = 카메라에 WinUSB 드라이버를 할당하는 중…
driver-bind-wait = 카메라가 새 드라이버로 다시 연결되기를 기다리는 중…
driver-bind-timeout = 드라이버를 할당한 뒤 카메라가 다시 나타나지 않았습니다. 카메라를 뽑았다가 다시 연결한 뒤 다시 시도하십시오.
driver-bind-failed = WinUSB 드라이버를 할당할 수 없습니다: { $detail }

# 셸의 응답입니다. 백엔드에서 생성되므로 콘솔과 창에서 같은 문구로
# 표시됩니다.
service-usage = 사용법: { $usage }
service-bad-hex = 16진수 값이 아닙니다: { $value }
service-unknown-command = 알 수 없는 명령: { $command }. 목록을 보려면 'help'를 입력하십시오.
service-unterminated-quote = 따옴표로 묶인 인수 안에서 줄이 끝났습니다.
service-help = 명령 참조:
service-read-bytes = { $count }바이트를 읽었습니다.
service-pulled = 카메라의 { $remote }을(를) { $local }(으)로 복사했습니다.
service-pushed = { $local }을(를) 카메라의 { $remote }(으)로 복사했습니다.
service-removed = 카메라에서 { $remote }을(를) 삭제했습니다.
service-dumped = { $offset }부터 { $size }바이트를 { $local }에 덤프했습니다.
service-backup-saved = 백업 이미지({ $count }바이트)를 { $local }에 저장했습니다.
service-backup-text-saved = 백업 이미지({ $count }바이트)를 { $bin }에, 목록을 { $txt }에 저장했습니다.
service-backup-status = 백업 상태, 지역: { $region }
service-backup-rewritten = 이미지({ $count }바이트)를 변경 없이 다시 기록하고 다시 읽었습니다. 영구 저장되지는 않았습니다.
service-backup-attrs-restored = 항목 { $count }개의 속성을 { $local }에 있는 속성으로 되돌렸습니다.
service-backup-written = 백업 항목 { $id }을(를) 기록했습니다.
service-backup-persisted = 서브시스템 { $id }의 백업을 플래시에 저장했습니다.
service-terminal-on = 디버그 터미널이 이제 켜졌습니다.
service-terminal-off = 디버그 터미널이 이제 꺼졌습니다.
btn-service-terminal-open = 직렬 콘솔 열기
btn-service-terminal-close = 직렬 콘솔 닫기
service-terminal-heading = 직렬 콘솔
service-terminal-hint = Linux 기반 카메라에서 디버그 터미널은 USB를 통한 루트 셸입니다. 콘솔을 연 다음 터미널에서처럼 명령을 입력하십시오.
service-terminal-label = 콘솔
service-terminal-interactive = 직렬 콘솔이 열렸습니다. 명령을 입력하십시오. 서비스 셸로 돌아가려면 'exit'를 입력하십시오.

# 고급 모드가 꺼져 있을 때의 서비스 페이지: 콘솔에서 셸에 들어가는 방법과
# 먼저 준비해야 할 것을 설명합니다.
service-console-heading = 콘솔에서 서비스 셸 실행
service-console-hint = pmca-console이 있는 폴더에서 터미널을 열고 다음 명령을 실행하십시오:
service-driver-heading = 시작하기 전에
service-driver-hint = 서비스 모드에는 원시 USB 액세스가 필요한데, Windows는 대용량 저장장치 드라이버를 통해서는 이를 허용하지 않습니다. 애플리케이션이 WinUSB 드라이버를 직접 할당합니다. 카메라가 대용량 저장장치 모드일 때 한 번, 서비스 모드에서 두 번째 제품 ID로 다시 나타난 뒤 한 번 더 할당하며, 세션이 끝나면 두 할당을 모두 해제합니다. 드라이버를 할당하려면 관리자 권한이 필요하므로 콘솔을 관리자 권한으로 시작하십시오. 다른 도구는 필요하지 않습니다.
driver-note-linux = 여기서는 아무것도 설치할 필요가 없습니다. libusb는 커널 드라이버를 밀어내지 않고 카메라에 직접 접근합니다. 다만 장치를 열 수 있는 권한과, 장치를 이미 점유하고 있지 않은 데스크톱 환경이 필요합니다. Sony 공급업체 ID 054c에 대한 udev 규칙을 설치하고, 대용량 저장장치 모드를 사용하기 전에 usb-storage 드라이버를 분리하십시오. 소스 트리에 있는 scripts/linux-prepare.sh 스크립트가 두 가지를 모두 처리하고 gvfs 볼륨 모니터도 중지합니다.
driver-note-macos = 원시 USB 액세스를 위해 설치할 것은 없습니다. 카메라의 USB 연결을 MTP로 설정하십시오. MTP 모드에서는 macOS 카메라 서비스 ptpcamerad가 카메라를 점유하고 있으면 프로그램이 이를 종료합니다. 시스템은 필요할 때 이 서비스를 다시 시작합니다. 트윅, 서비스 셸, 기능 분석, 펌웨어 업데이트, 업데이터 셸은 대용량 저장장치 연결을 거치는데, macOS는 이 연결을 관리자 권한이 있는 프로그램에만 제공합니다. 창에서는 실행할 때마다 암호를 한 번 묻고, 명령줄에서는 sudo가 필요합니다. 이 플랫폼은 세 플랫폼 중 테스트가 가장 적게 되었으므로, 문제가 이미 설명되어 있기보다는 직접 보고하게 될 가능성이 높습니다.
service-advanced-hint = 설정의 고급 모드에서 셸을 여기에 표시할 수도 있습니다.
btn-copy-command = 명령 복사
msg-command-copied = 명령을 클립보드에 복사했습니다.

# 고급 모드가 켜져 있을 때의 서비스 페이지.
btn-service-open = 연결
btn-service-close = 연결 끊기
btn-service-run = 실행
btn-service-browse = 파일 경로 삽입…
shell-path-quote = 이 경로는 삽입할 수 없습니다. 명령줄에서 인수의 시작과 끝을 표시하는 문자인 큰따옴표가 포함되어 있습니다. 파일 이름을 바꾸거나 이 문자가 없는 경로로 파일을 옮기십시오.
service-command-label = 명령
service-not-connected = 연결되지 않았습니다.

# GUI 전용 문자열
install-apk-path = 로컬 .apk 파일 경로 (선택 사항)
install-drop-hint = 여기에 .apk 파일을 놓으십시오

# 트윅 설명
tweak-rec-limit = 시간 제한 없이 동영상 녹화
tweak-rec-limit-4k = 시간 제한 없이 4K 동영상 녹화
tweak-language = 모든 메뉴 언어 표시
tweak-language-note = 카메라는 펌웨어가 알고 있는 언어만 표시하므로, 여기서 활성화한 것보다 적게 나타날 수 있습니다. 예를 들어 DSC-RX1R은 35개 중 25개를 표시합니다.
tweak-pal-ntsc = PAL / NTSC 선택 및 경고 활성화
tweak-pal-ntsc-mode = 비디오 시스템 (PAL / NTSC)
tweak-pal-ntsc-pal = PAL
tweak-pal-ntsc-ntsc = NTSC
tweak-pal-ntsc-note = 비디오 시스템을 변경하면 카메라에서 메모리 카드를 포맷해야 합니다. 먼저 카드에 있는 사진, 동영상, LUT 등 모든 것을 백업하십시오. 가장 간단한 방법은 전환하기 전에 카드를 빼 두는 것입니다.
tweaks-set-malformed = --set ID=VALUE 형식이 필요하지만 다음을 받았습니다: { $arg }
tweaks-set-unknown-value = 알 수 없는 값 "{ $value }". 다음 중 하나여야 합니다: { $options }
tweaks-set-not-a-choice = { $id }은(는) 선택 항목이 아니라 스위치입니다. --enable 또는 --disable을 사용하십시오.
tweaks-needs-set = { $id }은(는) 스위치가 아니라 선택 항목입니다. 다음 중 하나와 함께 --set { $id }=VALUE를 사용하십시오: { $options }
tweak-usb-installer = USB 앱 설치 프로그램 사용

# 트윅 작업 흐름
tweaks-warning = 경고: 트윅을 적용하면 카메라의 백업 메모리를 다시 씁니다. 쓰기가 중단되면 카메라가 영구적으로 손상될 수 있습니다.
tweaks-info-toggle = 트윅은 무엇을 하나요?
tweaks-info-what-heading = 트윅이란 무엇인가요
tweaks-info-what = 트윅은 카메라에 이미 있지만 메뉴에서 제공하지 않는 설정입니다. 이 프로그램은 이러한 설정을 사용할 수 있게 해 줍니다. 시간 제한 없는 동영상 녹화(4K 포함), 펌웨어에 들어 있는 모든 메뉴 언어, PAL과 NTSC 선택, USB 앱 설치 프로그램이 여기에 해당합니다. 플러그인으로 더 추가할 수도 있습니다. 어떤 트윅이 표시되는지는 카메라 모델에 따라 다릅니다.
tweaks-info-how-heading = 작동 방식
tweaks-info-how = “사용 가능한 트윅 표시”는 카메라를 잠시 서비스 모드로 전환하여 백업 메모리를 읽습니다. 이 과정에서는 아무것도 변경되지 않습니다. 그런 다음 각 트윅이 켜져 있는지 꺼져 있는지와 카메라 식별 정보가 표시되며, 식별 정보는 표시만 됩니다. “적용”은 변경한 항목만 기록합니다. 그 후 프로그램이 메모리를 다시 읽고 카메라가 모든 내용을 반영했는지 확인합니다.
tweaks-info-undo-heading = 되돌리는 방법
tweaks-info-undo = 프로그램은 매번 기록하기 전에 이전 상태를 이 컴퓨터에 저장합니다. 모든 트윅은 같은 스위치로 다시 끌 수 있습니다. 값을 하나씩 기록하는 카메라에서는 “변경 취소”도 제공됩니다.
tweaks-info-needs-heading = 필요한 것
tweaks-info-needs = 키 자료(“시작” 페이지 참조)가 필요하며, 아래 설명대로 카메라의 USB 연결을 “자동”으로 설정해야 합니다. Windows와 macOS에서는 프로그램이 서비스 모드의 카메라에 접근할 수 있도록 관리자 권한을 요청합니다. 기록하는 동안에는 케이블을 뽑거나 배터리를 빼지 마십시오.
app-backup-target = 대상 폴더
btn-choose-folder = 폴더 선택…
btn-default-folder = 기본 폴더 사용
tweaks-reading = 카메라 설정을 읽는 중…
tweaks-writing = 카메라 설정을 기록하는 중…
tweaks-done = 카메라 설정을 업데이트했습니다.
tweaks-write-failed = 카메라가 새 설정을 보낸 그대로 저장하지 않았습니다. 일부는 기록되었을 수 있으므로 설정을 다시 읽고 필요하면 되돌리십시오. 변경 전에 만든 백업 사본이 안전하게 되돌리는 방법입니다.
tweaks-confirm-required = 확인 없이는 기록하지 않습니다. 위의 경고를 읽은 뒤 --confirm 옵션을 붙여 다시 실행하십시오.
tweaks-state-enabled = 사용
tweaks-state-disabled = 사용 안 함
tweaks-full-read-failed = 카메라가 전체 백업을 전달하지 않았습니다({ $detail }). 이후 응답하지 않는 경우가 많습니다. 카메라를 끄고 배터리를 몇 초간 뺐다가 다시 넣은 뒤 다시 연결하십시오.
tweaks-direct-mode = 이 카메라의 백업은 1 MB보다 크므로 설정을 하나씩 읽고 기록합니다. 변경할 때마다 이전 값이 저장되며 “변경 취소”로 다시 기록할 수 있습니다.
tweaks-direct-forced = PMCA_TWEAKS_DIRECT가 속성별 경로를 강제하므로 이 카메라의 설정을 하나씩 읽고 기록합니다. 변경할 때마다 이전 값이 저장되며 “변경 취소”로 다시 기록할 수 있습니다.
tweaks-direct-refused = 카메라가 변경을 거부했습니다({ $detail }). 이미 기록한 값은 되돌렸으며 아무것도 변경되지 않았습니다. 일부 모델, 특히 일본 모델은 전체 백업 이미지를 통해서만 이 설정을 허용합니다.
tweaks-direct-revert-failed = 카메라가 변경을 거부했으며({ $detail }) 이미 기록한 값을 모두 되돌리지는 못했습니다. 이전 값은 저장되어 있으므로 “변경 취소”를 시도하십시오.
tweaks-direct-restart = 카메라 설정을 업데이트했습니다. 변경 사항을 적용하려면 카메라를 껐다 다시 켜십시오.
tweaks-rollback-unsaved = 되돌리기용으로 현재 값을 저장할 수 없어 아무것도 변경하지 않았습니다.
tweaks-image-saved = 변경 전의 백업 이미지를 저장했습니다: { $path }
tweaks-whole-image-hint = 대신 전체 백업 이미지를 통해 변경 사항을 기록하려면 같은 명령을 --whole-image 옵션과 함께 다시 실행하십시오. 변경 전 이미지가 먼저 저장됩니다.
tweaks-whole-image-confirm = 전체 백업 이미지를 통해 변경 사항을 기록하시겠습니까? 카메라가 단일 값으로는 변경을 거부했습니다. 변경 사항이 포함된 전체 이미지가 다시 기록되며, 변경 전 이미지는 먼저 이 컴퓨터에 저장됩니다.
btn-whole-image = 전체 백업 기록
tweaks-undo-none = 이 카메라에 대해 저장된 값이 없습니다.
tweaks-undo-confirm = 저장된 값을 카메라에 다시 기록하시겠습니까? 값을 저장한 이후 설정을 하나씩 기록한 모든 변경이 취소됩니다.
tweaks-undo-done = 저장된 값을 다시 기록했습니다. 카메라를 껐다 다시 켜십시오.
tweaks-undo-available = 첫 번째 변경 이전의 값이 저장되어 있으며 다시 기록할 수 있습니다.
btn-undo-tweaks = 변경 취소
tweaks-identity-heading = 카메라 식별 정보 (읽기 전용)
tweaks-caution-languages = 두 언어(아마도 중국어 간체와 중국어 번체)는 이 카메라에서 표시할 수 없습니다. 해당 언어로 스크롤하면 언어 메뉴가 다시 시작되므로, 예방 차원에서 꺼 둔 상태로 둡니다.
tweaks-caution-accept = 그래도 이 두 언어 활성화 (사용자 책임)
tweaks-caution-confirm = 이 두 언어로 인해 카메라의 언어 메뉴가 다시 시작될 수 있습니다. 손상되는 것은 없으며 변경을 되돌릴 수 있지만, 설정을 처음부터 다시 찾아야 할 수 있습니다. 그래도 활성화하시겠습니까?

# 설정
settings-language = 언어
settings-theme = 테마
settings-theme-system = 시스템
settings-theme-light = 밝게
settings-theme-dark = 어둡게

# 고급 모드. 공격자가 아니라 실수로 손이 미끄러지는 것을 막기 위한
# 장치입니다. 여기서 표시되는 모든 것은 어차피 콘솔에서 사용할 수 있습니다.
settings-advanced-heading = 고급 모드
settings-advanced-hint = 고급 모드는 같은 이름의 페이지에 서비스 셸과 업데이터 셸을 표시합니다. 서비스 모드는 카메라의 메모리, 파일 시스템, 보정 데이터에 대한 원시 액세스를 허용하므로, 명령을 잘못 입력하면 카메라를 사용할 수 없게 될 수 있습니다. 무엇을 기록하려는지 정확히 알지 못한다면 이 설정을 끈 상태로 두십시오.
settings-advanced-toggle = 서비스 셸과 업데이터 셸 표시
settings-advanced-on = 고급 모드가 켜져 있습니다. 이제 서비스 셸 및 업데이터 셸 페이지에서 셸을 직접 사용할 수 있습니다.
settings-advanced-off = 고급 모드가 꺼져 있습니다. 서비스 셸 및 업데이터 셸 페이지에서는 대신 콘솔 사용 방법을 설명합니다.
settings-advanced-confirm = 서비스 셸과 업데이터 셸은 카메라를 영구적으로 손상시킬 수 있습니다. 두 셸을 모두 표시하시겠습니까?
settings-update-heading = 업데이트 알림
settings-update-purpose = 이 설정을 켜면 프로그램이 시작할 때마다 GitHub에 새 버전이 출시되었는지 묻고, 출시되었으면 창 위쪽에 알려 줍니다. 이 과정에서 GitHub는 사용자의 IP 주소를 알게 되며, 그 밖에는 아무것도 전송되지 않습니다. 시험판은 알리지 않습니다.
settings-update-toggle = 새 버전이 있으면 시작할 때 알림
settings-update-on = 업데이트 알림이 켜져 있습니다. 다음 시작부터 새 버전이 창 위쪽에 표시됩니다.
settings-update-off = 업데이트 알림이 꺼져 있습니다. 프로그램이 새 버전을 확인하지 않습니다.
update-available = 새 버전을 사용할 수 있습니다
update-current = 설치된 버전
update-latest = 새 버전
btn-update-dismiss = 숨기기

# 번역 편집. 언어 파일을 다운로드하여 아무 텍스트 편집기에서 변경한 뒤
# 다시 업로드할 수 있습니다. 업로드한 파일은 사용하기 전에 검사합니다.
settings-catalog-heading = 번역 편집
settings-catalog-hint = 언어 파일을 다운로드하여 아무 텍스트 편집기에서 변경한 뒤 다시 업로드하십시오. 등호 오른쪽의 텍스트만 번역하며, 생략한 항목은 기본 제공 문구를 유지합니다. 여기서 언어를 선택해도 애플리케이션의 언어는 바뀌지 않습니다.
settings-catalog-locale = 언어 파일
settings-catalog-builtin = 기본 제공 번역을 사용 중입니다.
settings-catalog-custom = 사용자 번역을 사용 중입니다:
btn-catalog-export = 다운로드…
btn-catalog-import = 업로드…
btn-catalog-reset = 기본 제공 번역 복원
catalog-exported = 저장 위치
catalog-imported = 번역을 적용했습니다.
catalog-reset-done = 기본 제공 번역을 복원했습니다.
catalog-reset-none = 복원할 사용자 번역이 없었습니다.
catalog-reset-confirm = 이 언어에 대한 사용자 번역을 버리시겠습니까?
catalog-rejected = 파일을 적용하지 않았습니다. 다음 줄을 수정하십시오:
catalog-count-accepted = 적용된 항목
catalog-count-missing = 기본 제공 번역에서 유지된 항목
catalog-count-untranslated = 영어 원문과 동일한 항목
catalog-line = 줄
catalog-problem-syntax = "id = 텍스트" 형식의 항목이 아닙니다
catalog-problem-continuation = 다음 줄로 이어지며, 이는 지원되지 않습니다
catalog-problem-duplicate = 두 번 이상 정의되었습니다
catalog-problem-unknown-key = 이 애플리케이션에서 사용하는 항목이 아닙니다
catalog-problem-empty = 텍스트가 없습니다
catalog-problem-placeholder = 자리 표시자가 변경되었습니다. 예상 값:

# 출처, 감사의 말, 라이선스. 라이선스 페이지와 `pmca-console license`에서
# 표시됩니다. 라이선스 원문 자체는 법률 문서이므로 번역하지 않고 그대로
# 수록합니다.
about-thanks-heading = 감사의 말
about-thanks-body-1 = 이 도구가 존재하는 것은 Sony 카메라가 USB로 통신하는 방식을 밝혀내고 2015년에 Sony-PMCA-RE를 공개한 ma1co, 그리고 이후 수년간 이 프로젝트와 OpenMemories 프로젝트에 기여한 모든 분들 덕분입니다.
about-thanks-body-2 = 카메라 감지, 외부 명령 프로토콜, 펌웨어 업데이터 절차, 서비스 모드는 모두 그분들의 작업이며, 이 포트는 그 작업을 이어 갑니다. 감사합니다. 이 기반 위에 PMCA-RE RELOADED는 masterflai가 기여한 많은 새 기능을 더했습니다. 그중 가장 중요한 것은 이제 프로그램에 내장된 Windows 장치 드라이버 자동 전환입니다. 더 이상 별도의 도구로 카메라에 다른 드라이버를 수동으로 할당할 필요가 없으므로, 과정이 더 편리하고 빨라졌으며 위험도 줄었습니다.
about-contributors-heading = 기여자
about-contributors-body = ma1co 외에도 다음 분들이 Sony-PMCA-RE에 기여했거나, 자신의 사본에 공개한 변경 사항이 이 포트에 반영되었습니다:
about-purpose-heading = 이 포트가 존재하는 이유
about-purpose-body-1 = 원래 애플리케이션은 더 이상 개발되지 않습니다. 2020년에 지원이 종료된 Python 2용으로 작성되었으며, 일부 기능은 Sony가 이후 종료한 서비스에 의존하므로 현재 시스템에서는 더 이상 제한 없이 실행되지 않습니다.
about-purpose-body-2 = 이 포트의 목적은 대체가 아니라 보존이며, 그에 못지않게 오늘날 문제없이 사용할 수 있도록 하는 것입니다. 즉 이 카메라들을 계속 사용할 수 있게 하고, 최신 컴퓨터와 새로운 카메라 모델에서도 어려움 없이 작동하게 하는 것입니다. 원래 프로그램이 할 수 있었던 모든 것을 이 프로그램도 여전히 할 수 있어야 합니다.
about-origin-heading = 제작 방식
about-origin-body-1 = Python에서 Rust로의 이식은 AI의 도움을 받아 수행되었습니다.
about-origin-body-2 = 그것이 무엇을 의미하지 않는지도 똑같이 분명히 밝혀 둘 필요가 있습니다. 동작은 자동화된 테스트 모음과 하드웨어 테스트 계획으로 검증됩니다. 특히 Windows 버전은 장치 드라이버 때문에 가장 복잡한 플랫폼이므로 철저히 테스트되었으며, macOS와 Linux는 그만큼 포괄적으로 테스트할 수 없었습니다. 이 소프트웨어를 실험적인 소프트웨어로 취급하십시오.
about-license-heading = 라이선스
about-license-body = 이 포트는 원래 프로젝트와 같은 MIT 라이선스로 배포됩니다. 원래 저작권 고지는 해당 라이선스의 요구에 따라 변경 없이 유지됩니다.
about-license-full = 라이선스 전문
about-third-party-heading = 타사 라이선스
about-third-party-body = 이 프로그램은 정적으로 링크되어 있습니다. 사용하는 모든 라이브러리의 코드가 실행 파일 안에 들어 있으며, 해당 라이브러리의 라이선스는 저작권 고지를 함께 제공하도록 요구합니다. 아래에 패키지별로 한 항목씩 전문을 수록했습니다.
about-third-party-toggle = 모든 타사 고지 표시
about-privacy-heading = 데이터
about-privacy-body-1 = 이 프로그램은 사용자의 컴퓨터에서 실행됩니다. 사용 데이터를 전송하지 않으며, 원격 분석(텔레메트리)이 없고, 계정도 필요하지 않습니다. 여기서 하는 어떤 작업도 어디에도 보고되지 않습니다.
about-privacy-body-2 = 두 가지 기능은 자체적으로 네트워크에 접속하며, 이 과정에서 상대방이 사용자의 IP 주소를 알게 됩니다. GPS 보조 데이터는 Sony에서 다운로드하고, 사용자가 추가한 소스는 스타와 함께 GitHub에서 읽어 오며, 요청하면 그곳에서 검색도 수행합니다. 앱과 플러그인은 해당 소스에 명시된 주소에서 다운로드합니다. 이 기능들을 사용하지 않으면 프로그램은 완전히 오프라인 상태로 유지됩니다. 설정에서 업데이트 알림을 켜면 프로그램은 시작할 때마다 GitHub에 최신 버전도 묻습니다.
about-privacy-body-3 = 카메라에서 읽어 온 정보(일련번호와 저장된 Wi-Fi 암호 포함)는 사용자의 컴퓨터를 벗어나지 않습니다. 이를 파일로 저장한 경우 그 파일을 누구에게 전달하는지 주의하십시오.
about-privacy-body-4 = 이 프로그램 뒤에서 운영되는 서비스는 없으며 어떤 데이터도 개발자에게 전달되지 않으므로, 여기서 열람을 요청하거나 삭제를 요구할 정보가 없습니다. 위에서 언급한 두 서비스가 확인한 주소로 무엇을 하는지는 이 고지가 아니라 해당 서비스의 개인정보 처리방침에 따릅니다.
about-legal-heading = 법적 고지
about-legal-trademark = 이 프로젝트는 독립 프로젝트입니다. Sony가 제작하거나 보증하지 않았으며 Sony와 어떤 방식으로도 관련이 없습니다. "Sony", "PlayMemories" 및 카메라 모델 이름은 Sony Group Corporation의 상표이며, 여기서는 이 도구가 어떤 카메라와 통신하는지 알리기 위해서만 사용됩니다.
about-legal-own-device = 이 프로그램은 사용자가 소유한 카메라에 사용하기 위한 것입니다.
about-legal-warranty = 이 도구가 하는 작업은 Sony가 이 카메라에 대해 의도한 범위를 벗어나며, 이로 인해 보증이 종료될 수 있습니다. 소프트웨어 자체에는 어떠한 보증도 없으며, 사용에 따른 위험은 사용자가 부담합니다. 소유한 장치를 리버스 엔지니어링하는 것이 허용되는지, 어느 범위까지 허용되는지는 거주 지역에 따라 다릅니다.
about-links-heading = 프로젝트
about-link-original = Sony-PMCA-RE — 원래 프로젝트
about-link-fwtool = fwtool.py — 업데이트 이미지 생성의 기반이 된 펌웨어 도구
about-link-tweak = OpenMemories: Tweak — 지원되는 카메라에 권장되는 앱
about-link-applist = OpenMemories: App list — 추가 앱
about-link-platform = OpenMemories: Platform — 이 도구에 포함된 업데이터 본체
about-link-devices = 지원되는 카메라 모델
about-link-nexhack = Nex-Hack — 이 프로젝트의 기반이 된 커뮤니티 연구
about-link-zve10 = zv-e10-re — ZV-E10의 인터페이스와 설정 메모리에 관한 연구
about-version = 버전
btn-copy-url = 주소 복사
msg-url-copied = 주소를 클립보드에 복사했습니다.

# 안전 고지
notice-experimental = 실험적인 소프트웨어입니다. 사용에 따른 위험은 사용자가 부담합니다.

# 네트워크 설정(GPS 보조, 라이브 스트리밍 액세스 포인트 및 서비스)
gps-downloading = GPS 보조 데이터를 다운로드하는 중…
gps-writing = GPS 보조 데이터를 카메라에 기록하는 중…
gps-bad-size = GPS 보조 데이터의 크기가 예상과 달라 기록하지 않았습니다.
wifi-read = 카메라에서 라이브 스트리밍 액세스 포인트를 읽는 중…
wifi-written = 라이브 스트리밍 액세스 포인트를 기록했습니다.
stream-read = 카메라에서 라이브 스트리밍 설정을 읽는 중…
stream-written = 라이브 스트리밍 설정을 기록했습니다.
net-secret-warning = 경고: 이 파일에는 Wi-Fi 암호와 액세스 토큰이 일반 텍스트로 들어 있습니다.
wifi-secret-warning = 카메라는 이 암호를 암호화하지 않고 저장하며, 여기에는 저장된 그대로 표시됩니다. 암호는 이 컴퓨터에만 남습니다. 파일로 저장하는 경우 그 파일을 누구에게 전달하는지 주의하십시오.
gps-heading = GPS 보조 데이터
gps-hint = 여기로 파일을 끌어 오거나 경로를 입력하십시오. Sony에서 최신 데이터를 가져오려면 필드를 비워 두십시오.
btn-gps-devices = 호환 장치
btn-gps-devices-hide = 호환 장치 숨기기
gps-devices-note = 이 모델들에는 GPS 모듈이 내장되어 있습니다. 다수의 모델은 이름이 V로 끝나는 변형에만 GPS가 있으며, V가 없는 변형에는 GPS가 없습니다.
gps-devices-compact = 콤팩트 카메라 (DSC)
gps-devices-slt = SLT 카메라 (A 마운트)
gps-devices-action = 액션캠
gps-devices-camcorder = 캠코더 (Handycam)
gps-col-variant = GPS 변형
gps-col-arch = 아키텍처
gps-col-module = GPS 모듈
gps-col-apps = 앱 지원
gps-yes = 예
gps-no = 아니요
gps-unknown = 알 수 없음
gps-devices-untested = GPS 있음, 지원 여부 미확인
gps-devices-untested-note = 이 모델들은 GPS 모듈이 내장되어 있지만 OpenMemories 장치 목록에 없습니다. 이 애플리케이션이 해당 모델과 통신할 수 있는지 테스트되지 않았으므로 아키텍처와 앱 지원 여부는 알 수 없습니다.
wifi-heading = 라이브 스트리밍용 액세스 포인트
wifi-hint = 이것은 카메라 자체의 Wi-Fi 설정이 아니라 카메라의 라이브 스트리밍 기능이 사용하는 액세스 포인트입니다. 스트리밍 기능이 없는 본체는 이 작업을 거부합니다.
wifi-multi = 다중 액세스 포인트 목록
wifi-none = 저장된 액세스 포인트가 없습니다.
wifi-writing = 라이브 스트리밍 액세스 포인트를 카메라에 기록하는 중…
wifi-confirm = 카메라에 저장된 모든 액세스 포인트를 바꿉니다. 계속하시겠습니까?
wifi-empty-refused = 빈 목록은 기록하지 않습니다. 저장된 모든 액세스 포인트가 지워지기 때문입니다.
stream-heading = 라이브 스트리밍
stream-writing = 라이브 스트리밍 설정을 카메라에 기록하는 중…
stream-confirm = 카메라의 라이브 스트리밍 설정을 덮어씁니다. 계속하시겠습니까?
stream-invalid-json = 설정이 올바른 JSON이 아닙니다:

# 펌웨어 업데이트
firmware-warning = 경고: 카메라의 펌웨어를 덮어씁니다. 업데이트가 시작되면 중단하지 마십시오. USB 케이블을 뽑거나 카메라 전원을 끄지 마십시오. 정확히 이 카메라 모델용 펌웨어 파일만 사용하고, 배터리가 충분히 충전되어 있는지 확인하십시오. 그렇지 않으면 카메라를 영구적으로 사용할 수 없게 될 수 있습니다.
firmware-untested = 테스트되지 않음: 펌웨어 기록 기능은 이식되었지만 실제 카메라에서 시험한 적이 없습니다. 실패하거나 중단된 쓰기는 되돌릴 수 없으므로, 이 페이지의 모든 기능은 검증되지 않은 것으로 간주하십시오.
firmware-confirm-required = --confirm 없이는 펌웨어를 기록하지 않습니다.
firmware-truncated = 펌웨어 파일이 잘려 있습니다. FDAT 페이로드가 파일 끝을 넘어갑니다.
firmware-updating = 펌웨어 업데이트 중:
firmware-invalid = 이 파일은 올바른 Sony 펌웨어 이미지가 아닙니다
firmware-dat-path = 펌웨어 파일 (.dat)
firmware-drop-hint = 여기에 .dat 펌웨어 파일을 놓으십시오
firmware-file-required = 먼저 펌웨어 파일을 선택하십시오.
firmware-check-ok = 카메라가 이 이미지를 받아들입니다. 아무것도 기록하지 않았습니다.
firmware-cancelled = 펌웨어 기록을 취소했습니다. 카메라는 수정되지 않았습니다.
firmware-current-version = 설치된 버전
firmware-new-version = 이 이미지의 버전
firmware-models-hint = 업데이터 셸이 접근할 수 있는 카메라는 호환됩니다.
btn-firmware-check = 이미지 확인
btn-firmware-flash = 펌웨어 기록
firmware-own-device = 이러한 위험을 알고 있으며 본인 소유의 장치만 업데이트한다는 것을 확인하십시오.
btn-firmware-accept = 위험을 이해함 – 기록

# 파일 도구: 카메라가 전혀 필요 없는 작업.
tools-hint = 이 작업들은 파일에서만 작동하며 카메라에는 전혀 접근하지 않으므로, 여기서는 카메라가 손상될 일이 없습니다. pmca-console이 apk2spk, spk2apk, print-backup, print-usagelog, guess-firmware, xpd-checksum으로 제공하는 작업과 같습니다.
tools-operation = 작업
tools-apk2spk = .apk를 .spk 컨테이너로 감싸기
tools-spk2apk = .spk를 다시 .apk로 풀기
tools-print-backup = Backup.bin의 내용 표시
tools-print-usagelog = Usagelog.bin의 내용 표시
tools-guess-firmware = 펌웨어 .dat 안의 페이로드 찾기
tools-xpd-checksum = 파일의 CIC 체크섬 계산
tools-input = 입력 파일
tools-file-required = 먼저 이 작업에 필요한 파일을 선택하십시오.
tools-apk-invalid = 이 파일을 .spk로 패키징할 수 없습니다
tools-spk-invalid = 이 파일은 올바른 .spk 패키지가 아닙니다
tools-backup-invalid = 이 파일은 올바른 Backup.bin 이미지가 아닙니다
tools-usagelog-invalid = 이 파일은 올바른 사용 로그가 아닙니다
tools-reset-value = 초기화 값:
tools-fdat-found = 오프셋 { $offset }에서 페이로드를 찾았습니다. 크기: { $size }.
tools-checksum = 체크섬
tools-written = { $path }에 저장했습니다

# 프로젝트 문서 링크
docs-compatibility = 카메라 호환성
docs-compatibility-hint = 원래 프로젝트에서 테스트된 카메라 개요를 제공했습니다. 주로 이전 카메라 모델을 다루며 다음 주소에서 확인할 수 있습니다:
devices-note = 원래 프로젝트의 개요에서 가져온 정보입니다. 앱 지원 옆의 숫자는 카메라 앱 플랫폼의 버전입니다.
devices-more-note = 다음 카메라는 이 프로그램으로 명시적으로 테스트되지 않았습니다. 마지막 열의 체크 표시는 커뮤니티에서 사용했다고 보고한 경우를 뜻합니다. 서비스 셸에도 ✖가 표시된 경우 그 사용은 실패한 것입니다.
btn-more-models = 추가 장치
btn-more-models-hide = 추가 장치 숨기기
check-updater-yes = 장치 { $model }은(는) 호환됩니다. 장치 목록에 따르면 업데이터 셸이 이 장치에 접근할 수 있습니다.
check-updater-no = 장치 { $model }은(는) 호환되지 않습니다. 장치 목록에 따르면 업데이터 셸이 이 장치에 접근할 수 없습니다.
check-updater-unknown = 장치 { $model }의 호환성을 알 수 없습니다. 이 프로그램의 어떤 장치 목록에도 없으므로 업데이터 셸이 이 장치에 접근할 수 있는지 알 수 없습니다.
check-apps-no = 장치 { $model }은(는) 호환되지 않습니다. 장치 목록에 따르면 앱을 지원하지 않습니다.
check-apps-untested = 장치 { $model }의 호환성을 알 수 없습니다. 장치 목록에는 있지만 앱 지원 여부가 확인되지 않았습니다.
check-apps-unknown = 장치 { $model }의 호환성을 알 수 없습니다. 이 프로그램의 어떤 장치 목록에도 없으므로 앱 지원 여부를 알 수 없습니다.
check-gps-no = 장치 { $model }은(는) 호환되지 않습니다. GPS 수신기가 있는 장치 목록에 없습니다.
check-gps-untested = 장치 { $model }의 호환성을 알 수 없습니다. 장치 목록에 따르면 GPS 수신기가 있지만 이 모델로의 전송은 테스트되지 않았습니다.
check-gps-unknown = 장치 { $model }의 호환성을 알 수 없습니다. 이 프로그램의 어떤 장치 목록에도 없으므로 GPS 수신기가 있는지 알 수 없습니다.
check-unidentified = 연결된 장치를 식별할 수 없어 호환성을 알 수 없습니다.
check-proceed = 그래도 계속하시겠습니까?
check-cancelled = 취소되었습니다. 장치는 수정되지 않았습니다.
devices-col-confirmed = 확인됨
devices-group-ilc = 렌즈 교환식 카메라 (E 마운트 및 A 마운트)
devices-group-remote = 라이브 뷰 리모컨
btn-copy-link = 링크 복사
shutter-heading = 셔터 횟수
shutter-hint = 카메라는 셔터를 누를 때마다 횟수를 세고, 저장하는 모든 사진(ARW, JPEG 또는 HEIF)에 현재 값을 기록합니다. 현재 횟수를 읽으려면 가장 최근에 촬영한 사진을 선택하십시오. 파일은 읽기만 하며 수정하지 않습니다.
shutter-source = 파일 안에서 횟수가 저장되는 위치는 모델마다 다릅니다. 이 위치 정보는 공개적으로 문서화된 연구(ExifTool)에서 가져온 것이며 모든 모델에 대해 확인된 것은 아닙니다.
shutter-drop-hint = 여기에 사진(ARW, JPEG, HEIF)을 놓으십시오
btn-shutter-read = 사진 분석
shutter-file-required = 먼저 사진을 선택하십시오.
shutter-file = 파일
shutter-taken = 촬영 날짜
shutter-count = 셔터 횟수
shutter-type = 셔터
shutter-type-mechanical = 기계식
shutter-type-electronic = 전자식
shutter-electronic-hint = 이 사진은 전자 셔터로 촬영되었습니다. 많은 모델에서 이러한 촬영은 횟수에 포함되지 않으며, 이 경우 표시되는 값은 마지막 기계식 셔터 작동 시점의 횟수입니다.
shutter-not-sony = 이 사진은 Sony 카메라로 촬영되지 않았습니다. 셔터 횟수를 읽을 수 없습니다.
shutter-unknown-model = 이 카메라 모델이 파일의 어느 위치에 셔터 횟수를 저장하는지 알려져 있지 않으므로 값을 표시하지 않습니다.
shutter-not-recorded = 파일에 읽을 수 있는 셔터 횟수가 없습니다. 이미지 편집 소프트웨어로 저장하거나 내보낸 사진, 그리고 일부 모델에서 전자 셔터로 촬영한 사진에서 이런 경우가 발생합니다. 메모리 카드에 있는 원본 파일을 선택하십시오.
shutter-model-mismatch = 이 사진은 연결된 카메라와 다른 카메라 모델로 촬영되었습니다.
shutter-invalid = 이 파일은 읽을 수 있는 사진이 아닙니다
file-filter-photo = 사진 (ARW, JPEG, HEIF)
usage-log-heading = 사용 로그
usage-log-hint = 카메라는 문서화되지 않은 형식으로 내부 로그를 유지합니다. 로그는 그대로 저장되며, 카메라에서는 읽기만 합니다.
usage-log-written = 저장 위치
btn-usage-log-info = 사용 로그 정보
btn-usage-log-info-hide = 설명 숨기기
usage-log-info-what-heading = 이 로그는 무엇인가요
usage-log-info-what = Sony는 내부적으로 이 로그를 "KikiLog"라고 부릅니다. 로그는 카메라가 직접 기록하며, 이 프로그램은 읽기만 합니다. 형식에 대한 공개된 설명은 알려져 있지 않습니다.
usage-log-info-read = 읽기만으로는 아무것도 바뀌지 않습니다. 연속으로 여러 번 다운로드한 결과가 바이트 단위까지 동일했습니다.
usage-log-info-flush-heading = 최근 항목이 없는 이유
usage-log-info-flush = 새 항목이 바로 나타나지 않는 경우가 많습니다. 카메라는 전원을 끌 때만 항목을 저장할 수 있으며, 때로는 여러 번의 전원 켜기/끄기에 걸쳐 나누어 저장합니다. 카메라를 껐다 켜고 다시 다운로드하십시오. 새 항목이 더 나타나지 않을 때까지 반복하십시오.
usage-log-info-content-heading = 알 수 있는 내용
usage-log-info-shutter = 셔터 작동: 일부 이벤트는 각각 한 번의 노출을 나타냅니다. 하지만 이를 세어도 셔터 횟수가 되지는 않습니다. 로그는 크기가 고정되어 있어 카메라의 전체 사용 기간을 포함하지 않을 수 있기 때문입니다. 카메라는 총 횟수를 다른 곳에 보관하고 저장하는 사진에 기록하며, 이 값은 "셔터 횟수" 섹션에서 읽을 수 있습니다.
usage-log-info-clock = 타임스탬프는 카메라 시계만큼만 정확합니다. 배터리를 교체한 뒤 시계가 초기화되어 카메라의 초기 날짜가 붙은 항목이 생길 수 있습니다.
usage-log-info-unknown = 이벤트 번호에 대한 공개된 설명은 알려져 있지 않습니다. 프로그램은 번호를 해석하지 않고 그대로 표시합니다.
usage-log-info-privacy-heading = 개인정보
usage-log-info-privacy = 이 파일에는 삽입했던 메모리 카드의 일련번호가 들어 있습니다. 함부로 공유하거나 게시하지 마십시오.
btn-usage-log = 로그 저장
usagelog-size = 로그 영역
usagelog-cursor = 쓰기 위치
usagelog-spare = 두 번째 영역
usagelog-count = 항목
usagelog-events = 번호별 이벤트
usagelog-entries = 순서별 항목
msg-link-copied = 링크를 클립보드에 복사했습니다.

# 백엔드 오류 메시지이며, 백엔드가 보고하는 고정 코드를 키로 사용합니다.
# 메시지의 가변 부분(경로, 상태 바이트, 장치 오류)은 프런트엔드가 덧붙이므로,
# 이 메시지들은 그것 없이도 완전한 문장으로 읽혀야 합니다.
# `err-operation`은 일부러 두지 않았습니다. 이 코드는 백엔드가 이미 번역한
# 메시지를 나타내며, 그 메시지는 그대로 표시해야 합니다.
err-usb-no-device = 카메라를 찾을 수 없습니다. 카메라를 연결하고 전원을 켜십시오. 카메라 화면에 계속 연결 중이라고 표시되면 컴퓨터가 카메라를 인식하지 못하는 것입니다. 충전 전용 케이블이 아닌 데이터 케이블을 사용하고, 허브나 어댑터 없이 카메라를 직접 연결한 뒤 전원을 껐다 다시 켜십시오. 그래도 해결되지 않으면 배터리를 잠시 분리하십시오. 데이터 케이블로 연결하면 전원이 꺼진 카메라도 감지되어 준비되지 않음으로 보고됩니다. 카메라 전원이 꺼진 상태에서도 이 메시지가 나타나면 케이블이 데이터를 전송하지 않는 것일 수 있습니다.
err-usb-camera-not-ready = 카메라가 연결되어 있지만 준비되지 않았습니다. 전원이 꺼져 있거나, 화면에서 USB 모드 선택을 기다리고 있습니다. 전원을 켜거나 카메라에서 “대용량 저장장치(MSC)”를 선택하십시오.
camera-choose-usb-mode = 카메라가 USB 모드 선택을 기다리고 있습니다. 카메라에서 “대용량 저장장치(MSC)”를 선택하십시오.
err-usb-too-many-devices = 카메라가 두 대 이상 연결되어 있습니다. 작업할 카메라 한 대만 남기고 모두 분리하십시오.
err-usb-transfer = USB 전송에 실패했습니다. 다른 케이블이나 포트를 사용해 보고, USB 허브는 피하십시오.
err-usb-permission = 프로그램에 USB로 카메라를 열 수 있는 권한이 없습니다. Linux에서는 Sony 공급업체 ID에 대한 udev 규칙으로 이 권한을 한 번 부여합니다. 소스 트리에 있는 scripts/linux-prepare.sh 스크립트가 이 규칙을 설치합니다. 그런 다음 카메라를 뽑았다가 다시 꽂으십시오.
err-usb-busy = 다른 프로그램이 카메라를 사용 중이어서 열 수 없습니다. 카메라에 접근할 수 있는 다른 프로그램을 닫으십시오. Linux에서는 데스크톱의 미디어 모니터가 카메라를 점유할 수 있으며, scripts/linux-prepare.sh가 이를 중지합니다. macOS에서는 카메라의 USB 연결을 MTP로 설정하십시오. 대용량 저장장치 모드에서는 시스템 자체 디스크 드라이버가 카메라를 점유합니다.
err-usb-needs-root = macOS는 관리자 권한으로 실행되는 프로그램에만 카메라의 대용량 저장장치 연결을 넘겨줍니다. 명령줄 프로그램을 sudo로 시작하십시오. 예: sudo pmca-console tweaks. 카메라 정보 읽기는 카메라의 USB 연결을 MTP로 설정하면 관리자 권한 없이도 가능합니다.
macos-admin-prompt = PMCA-RE RELOADED가 macOS로부터 카메라의 대용량 저장장치 연결을 넘겨받으려면 관리자 권한이 필요합니다. 작업하는 동안 카메라의 카드가 Finder에서 사라집니다. 카드의 파일이 열려 있으면 먼저 Finder에서 꺼내십시오.
macos-admin-declined = 관리자 권한이 부여되지 않아 카메라에 접근하지 않았습니다. macOS에서는 이 기능에 관리자 권한이 필요합니다.
macos-admin-failed = 프로그램이 관리자 권한으로 도우미를 시작할 수 없습니다.
err-usb-stall = 카메라가 USB 전송을 중단했습니다.
err-usb-protocol = 카메라가 예기치 않은 응답을 보냈습니다.
err-usb-msc-status = 카메라가 저장장치 명령을 거부했습니다.
cli-error-prefix = 오류
cli-hint-mass-storage = 대용량 저장장치 모드로 전환: pmca-console mass-storage
err-usb-driver = Windows가 이 카메라에 직접 액세스를 허용하지 않는 드라이버를 할당했습니다. 이 모드에는 WinUSB 드라이버가 필요하며, 애플리케이션이 직접 할당할 수 있지만 관리자 권한으로 실행 중일 때만 가능합니다.
err-needs-elevation = 드라이버를 할당하거나 해제하려면 관리자 권한이 필요한데, Windows 확인 메시지가 거부되었습니다. “드라이버 도우미 시작”으로 다시 시도하십시오. 카메라에서는 아무것도 변경할 필요가 없습니다.
err-usb-no-mass-storage-volume = 카메라가 대용량 저장장치 모드로 연결되어 있지만 드라이브를 제공하지 않습니다. 카메라를 깨우고, 화면에서 USB 모드를 묻는 경우 확인한 뒤, 메모리 카드가 삽입되어 있는지 확인하십시오. 드라이버는 변경하지 마십시오. 이 모드에는 표준 Windows 드라이버가 필요합니다.
err-usb-msc-sense = 카메라가 저장장치 오류를 보고했습니다.
err-usb-ptp-response = 카메라가 명령을 거부했습니다.
err-usb-unsupported = 이 카메라 모델은 해당 기능을 지원하지 않습니다.
err-usb-updater-status = 카메라의 펌웨어 업데이터가 작업을 거부했습니다.
err-usb-senser = 서비스 모드 명령이 실패했습니다.
err-market-core = 카메라 데이터 파일을 읽을 수 없습니다.
err-market-json = 받은 데이터의 형식이 잘못되었습니다.
err-market-missing-field = 응답에 예상한 필드가 없습니다.
err-market-no-apk = 설치할 앱을 선택하지 않았습니다.
err-market-http = 네트워크 요청이 실패했습니다.
err-market-io = 로컬 설치 서버에 오류가 발생했습니다.
err-market-tls = 카메라와의 암호화된 연결을 설정할 수 없습니다.
err-apk = 앱 파일을 읽을 수 없습니다.
err-io = 파일을 읽거나 쓸 수 없습니다.
err-json = 카메라가 잘못된 형식의 데이터를 보냈습니다.
err-unknown-locale = 이 언어는 사용할 수 없습니다.
err-catalog-encoding = 번역 파일이 UTF-8 텍스트로 저장되어 있지 않습니다. UTF-8 인코딩으로 다시 저장하십시오.
err-internal = 내부 오류가 발생했습니다. 애플리케이션의 버그입니다.

## 기능 분석
nav-analysis = 분석
analysis-title = 기능 분석
analysis-experimental = 실험적 기능입니다. 이 기능은 새로 추가되었으며 소수의 카메라에서만 시험되었습니다.
analysis-own-hardware = 본인 소유의 카메라에서만 사용하십시오. 서비스 모드 자체가 Sony가 의도하지 않은 개입이며, 책임은 사용자에게 있습니다.
analysis-explanation-toggle = 설명: 목적, 절차, 위험
analysis-purpose-heading = 무엇을 위한 기능인가요?
analysis-purpose = 기능 분석은 카메라가 자체 메뉴 설정 하나를 설정 메모리(백업)의 어느 값에 저장하는지 기록합니다. 결과는 이 카메라의 설정 메모리를 설명하며, 다른 카메라와 비교할 수는 있지만 카메라가 제공하는 기능을 바꾸지는 않습니다.
analysis-how-heading = 어떻게 작동하나요?
analysis-how = 입력한 메뉴 값마다 프로그램이 전체 설정 메모리를 한 번씩 읽으며, 매번 별도의 짧은 서비스 모드 세션을 사용합니다. 그 사이에 케이블을 뽑고 카메라 메뉴에서 설정을 변경합니다. 카메라는 켜 둔 상태로 둡니다. 마지막으로 첫 번째 값을 한 번 더 읽습니다. 메뉴를 따라 바뀌고 마지막에 시작 값으로 돌아온 값이 후보입니다. 스스로 바뀌는 값(카운터, 시계)은 제외합니다.
analysis-requirements-heading = 요구 사항
analysis-requirements = 카메라가 USB로 연결되어 있고 USB 모드가 “자동”으로 설정되어 있어야 하며, 각 세션이 끝난 뒤 카메라에서 “대용량 저장장치(MSC)”를 선택합니다. Windows에서는 서비스 모드를 위해 잠시 드라이버를 할당하므로 프로그램에 관리자 권한이 필요합니다. 배터리를 충전해 두십시오. 각 세션은 약 1분이 걸립니다.
analysis-read-heading = 읽기 단계
analysis-read = 스냅샷은 읽기만 합니다. 카메라 설정은 아무것도 바뀌지 않습니다. 위험은 낮지만 없지는 않습니다. 서비스 모드로 전환하는 것 자체가 Sony가 의도하지 않은 개입이기 때문입니다.
analysis-write-heading = 쓰기 단계 (쓰기 테스트)
analysis-write = 명시적으로 켠 경우에만 실행됩니다. 프로그램이 찾은 값을 카메라에 기록하면, 재시작 후 메뉴에서 그 값이 적용되는지 확인하고, 그다음 원래 값을 다시 기록합니다. 카메라가 직접 저장한 값만 기록하며, 일치하는 후보가 정확히 하나일 때만 기록합니다. 이전 값은 미리 저장됩니다. 그렇더라도 이 단계는 카메라를 변경하며, 오류가 발생하면 서비스 센터에서만 수리할 수 있는 상태가 될 수 있습니다.
analysis-legal-heading = 법적 사항
analysis-legal = 이 소프트웨어는 어떠한 보증도 없이 제공됩니다(MIT 라이선스). 카메라에 대한 개입은 보증에 영향을 줄 수 있습니다. 본인 소유의 카메라에서만 이 기능을 사용하십시오. 분석 보고서에는 일련번호가 없으며, 찾은 후보 외에는 설정 메모리의 어떤 내용도 포함되지 않습니다. 보고서는 어디에도 자동으로 전송되지 않습니다.
analysis-start-heading = 새 분석
analysis-setting-label = 카메라에 표시되는 이름 그대로의 메뉴 설정
analysis-values-label = 쉼표로 구분한 메뉴 값
analysis-values-hint = 첫 번째 값은 현재 설정된 값입니다. 2~8개의 값을 입력하며, 값마다 서비스 모드 세션이 한 번씩 필요합니다.
analysis-write-option = 그런 다음 쓰기 테스트 실행 (카메라를 변경함)
analysis-write-warning = 쓰기 테스트는 카메라를 변경합니다. 설정 메모리에 값을 기록한 뒤 원래 값을 다시 기록합니다. 본인 소유의 카메라에서만, 사용자 책임하에 실행하십시오.
analysis-write-consent = 본인 소유의 카메라이며 위험을 감수합니다.
btn-analysis-write-enable = 쓰기 테스트 활성화
btn-analysis-start = 분석 시작
btn-analysis-snapshot = 백업 읽기
btn-analysis-new = 새 분석
analysis-discard-confirm = 진행 중인 분석을 버리시겠습니까? 지금까지의 보고서는 저장된 상태로 유지됩니다.
analysis-access-read = 읽기
analysis-access-write = 쓰기
analysis-access-user = 수동
analysis-access-local = PC
analysis-step-snapshot = 백업 읽기, 설정은 “{ $value }” 상태
analysis-step-change = 카메라 메뉴에서 “{ $value }”(으)로 변경
analysis-step-evaluate = 스냅샷 비교
analysis-step-write = “{ $value }”에 해당하는 값을 카메라에 기록
analysis-step-check = 카메라를 껐다 켜고 메뉴 확인 (예상: “{ $value }”)
analysis-step-restore = “{ $value }”의 원래 값을 다시 기록
analysis-step-report = 분석 보고서 저장
analysis-do-prepare = 준비: 1. 카메라 메뉴에서 “{ $setting }”을(를) “{ $value }”(으)로 설정하고, 카메라가 설정을 저장하도록 MENU 버튼으로 메뉴를 나갑니다. 2. 카메라를 켜 둔 상태로 둡니다. 3. 케이블을 연결하고 카메라에서 “대용량 저장장치(MSC)”를 선택합니다.
analysis-do-change = 사용자 차례: 1. 케이블을 뽑습니다. 2. 카메라는 켜 둔 상태로 둡니다. 3. 카메라 메뉴에서 “{ $setting }”을(를) “{ $value }”(으)로 설정하고, 카메라가 설정을 저장하도록 MENU 버튼으로 메뉴를 나갑니다. 4. 케이블을 다시 연결하고 “대용량 저장장치(MSC)”를 선택합니다.
analysis-do-check = 사용자 차례: 1. 케이블을 뽑습니다. 2. 카메라를 껐다 다시 켭니다. 3. 메뉴에서 “{ $setting }”에 무엇이 표시되는지 확인합니다.
analysis-do-reconnect = 케이블을 다시 연결하고 카메라에서 “대용량 저장장치(MSC)”를 선택하십시오.
analysis-reading = 스냅샷 { $index }/{ $total }: 읽는 동안 케이블을 절대 뽑지 마십시오.
analysis-cli-continue = 계속하려면 Enter, 취소하려면 q를 누르십시오:
analysis-confirm-word = 예
analysis-cli-confirm = 확인하려면 “{ $word }”을(를) 입력하십시오:
analysis-write-declined = 확인되지 않았습니다. 아무것도 기록하지 않았습니다.
analysis-aborted = 취소되었습니다. 아무것도 기록하지 않았습니다.
analysis-snapshot-done = 스냅샷을 읽었습니다. 카메라가 서비스 모드를 종료했습니다.
analysis-result = 값 { $compared }개 비교: 후보 { $count }개, 스스로 바뀐 값 { $running }개.
analysis-candidate-consistent = 메뉴와 명확하게 일치함
analysis-candidate-inconsistent = 명확하게 일치하지 않음
analysis-write-preview-heading = 기록할 내용
analysis-write-preview = 값 { $id }({ $size }바이트): { $old }(“{ $old-label }”)에서 { $new }(“{ $new-label }”)(으)로. 카메라는 “{ $new-label }”이(가) 설정되어 있을 때 새 값을 직접 저장했습니다.
btn-analysis-write = 기록
btn-analysis-skip-write = 쓰기 테스트 없이 완료
analysis-written = 값을 기록하고 다시 읽은 뒤 영구 저장했습니다.
analysis-menu-question = 지금 메뉴에 무엇이 표시됩니까?
analysis-menu-other = 다른 값
btn-analysis-menu-confirm = 확인
btn-analysis-restore = 원래 값 다시 기록
analysis-restored = 원래 값을 다시 기록하고 영구 저장했습니다.
analysis-survived = 재시작 후에도 기록한 값이 저장되어 있었습니다.
analysis-not-survived = 재시작 후 기록한 값이 더 이상 저장되어 있지 않았습니다.
analysis-report-saved = 분석 보고서를 저장했습니다: { $path }
analysis-report-hint = 보고서에는 일련번호와 백업이 포함되어 있지 않습니다. 결과를 확인하고 문서화할 수 있도록 다른 사람에게 전달해도 됩니다.
btn-analysis-export = 보고서 내보내기…
file-filter-analysis = 분석 보고서
analysis-pending = 완료되지 않은 쓰기 테스트가 있습니다. 원래 값은 저장되어 있으며 다시 기록할 수 있습니다.
analysis-pending-none = 이 카메라에 대해 저장된 미완료 쓰기 테스트가 없습니다.
analysis-invalid-setting = 카메라에 표시되는 이름 그대로 메뉴 설정을 입력하십시오.
analysis-invalid-values = 서로 다른 메뉴 값을 2개에서 { $max }개까지 입력하십시오. 첫 번째 값은 현재 설정된 값입니다.
analysis-already-complete = 모든 스냅샷을 이미 읽었습니다.
analysis-other-camera = 첫 번째 스냅샷과 같은 카메라가 아닙니다. 분석을 중지했습니다.
analysis-write-not-allowed = 이 상태에서는 쓰기 테스트를 할 수 없습니다.
analysis-camera-changed = 카메라에 현재 첫 번째 스냅샷과 다른 값이 들어 있습니다. 아무것도 기록하지 않았습니다.
analysis-write-refused = 카메라가 값을 거부했습니다. 모든 것이 이전과 같습니다. ({ $detail })
analysis-refuse-none = 쓰기 테스트를 할 수 없습니다: 메뉴를 따라 바뀐 값이 없습니다.
analysis-refuse-ambiguous = 쓰기 테스트를 할 수 없습니다: 메뉴를 따라 바뀐 값이 { $count }개이며, 어느 것도 확실하게 대응시킬 수 없습니다.
analysis-refuse-inconsistent = 쓰기 테스트를 할 수 없습니다: 값이 메뉴 값과 명확하게 일치하지 않습니다.
analysis-refuse-size = 쓰기 테스트를 할 수 없습니다: 값의 크기가 { $size }바이트입니다. 작은 값만 기록합니다.
analysis-refuse-read-only = 쓰기 테스트를 할 수 없습니다: 카메라가 이 값을 읽기 전용으로 표시합니다.
analysis-refuse-protected = 쓰기 테스트를 할 수 없습니다: 카메라가 이 값을 보호됨으로 표시합니다.
settings-analysis-heading = 기능 분석
settings-analysis-purpose = 기능 분석은 카메라가 자체 메뉴 설정 하나를 설정 메모리의 어느 값에 저장하는지 기록합니다. 자신의 모델의 설정 메모리를 이해하고 결과를 공유하려는 사용자를 위한 기능입니다.
settings-analysis-why-off = 일반적인 사용에는 필요하지 않고, 카메라를 여러 번 서비스 모드로 전환하며, 요청 시 카메라에 기록도 하기 때문에 꺼져 있습니다. 실험적 기능이며 소수의 모델에서만 시험되었습니다.
settings-analysis-toggle = 기능 분석 켜기 (메뉴의 “분석” 항목)
settings-analysis-confirm = 기능 분석을 켜시겠습니까? 카메라를 여러 번 서비스 모드로 전환하며, 요청 시 카메라에 기록할 수 있습니다. 본인 소유의 카메라에서만 사용하십시오.
btn-analysis-enable = 켜기
settings-analysis-on = 기능 분석이 켜져 있습니다.
settings-analysis-off = 기능 분석이 꺼져 있습니다.
analysis-needs-admin = 각 스냅샷은 서비스 모드를 위해 관리자 권한이 필요합니다. Windows는 프로그램 세션마다 한 번 권한을 묻습니다. 지금 드라이버 도우미를 시작하거나, 첫 번째 스냅샷에서 요청할 때까지 기다릴 수 있습니다.
analysis-changed-hint = 확인하십시오: 후보를 찾지 못했습니다. 다음 값들은 바뀌었지만 시작 값으로 돌아오지 않았습니다. 그중 하나가 두 값 사이에서 한 번만 바뀌었다면, 첫 번째 스냅샷 때 카메라가 첫 번째 메뉴 값으로 설정되어 있지 않았을 가능성이 높습니다. 이 경우 시작 값을 올바르게 설정한 뒤 분석을 다시 실행하십시오.
analysis-changed-info = 정보: 분석 중에 스스로 바뀐 값(예: 카운터, 시계)은 무시됩니다. 따로 할 일은 없습니다.
elevation-admin = 관리자
elevation-standard = 표준 권한
elevation-admin-hint = 프로그램이 관리자 권한으로 실행 중이며 서비스 모드에 필요한 드라이버를 할당할 수 있습니다.
elevation-standard-hint = 프로그램이 표준 권한으로 실행 중입니다. 서비스 모드, 트윅, 기능 분석을 위해 필요할 때 관리자 권한으로 드라이버 도우미를 시작합니다. Windows는 세션마다 한 번 묻습니다. 지금 시작하려면 클릭하십시오.
driver-helper-declined = 관리자 권한 요청이 거부되었습니다. 관리자 권한이 없으면 서비스 모드용 드라이버를 할당할 수 없습니다. 아무것도 변경하지 않았습니다.
btn-remove-foreign-drivers = 이전 드라이버 제거
btn-not-now = 나중에
btn-ok = 확인
foreign-drivers-found = Windows에 Zadig 또는 유사한 도구가 Sony 장치용으로 설치한 드라이버가 아직 남아 있습니다: { $packages }. 이 프로그램이 자체 드라이버를 할당하기 전에 Windows가 카메라를 이 드라이버에 넘겨주므로, 서비스 모드가 실패하고 카메라가 드라이브로 나타나지 않을 수 있습니다. 지금 제거하시겠습니까? 그러면 카메라는 Windows 기본 제공 드라이버를 사용합니다. 제거하려면 관리자 권한이 한 번 필요합니다.
foreign-drivers-removed = 이전 드라이버를 제거했습니다. 카메라를 뽑았다가 다시 꽂으십시오.
foreign-drivers-remaining = 다음 드라이버를 제거할 수 없습니다: { $packages }. Windows를 다시 시작한 뒤 다시 시도하거나, 장치 관리자에서 장치를 드라이버와 함께 제거하십시오.
foreign-drivers-hint = Zadig 또는 유사한 도구가 Sony 장치용으로 설치한 드라이버가 아직 있습니다({ $packages }). 이 프로그램이 자체 드라이버를 할당하기 전에 Windows가 카메라를 이 드라이버에 넘겨주므로 서비스 모드가 실패할 수 있습니다. 다음 명령으로 제거하십시오: pmca-console driver zadig --remove
driver-zadig-none = Sony 장치용으로 Zadig 또는 유사한 도구가 설치한 드라이버가 없습니다.
driver-zadig-entry = { $package } ({ $provider }): { $devices }
driver-zadig-remove-hint = 다음 명령으로 제거하십시오: pmca-console driver zadig --remove
driver-zadig-removed = 이전 드라이버 패키지 { $count }개를 제거했습니다. 카메라를 뽑았다가 다시 꽂으십시오.
elevation-helper = 드라이버 도우미 활성
elevation-helper-hint = 관리자 권한이 있는 드라이버 도우미가 서비스 모드용 드라이버를 할당합니다. 도우미에는 창이 없으며 프로그램과 함께 종료됩니다. 추가 확인 메시지는 표시되지 않습니다.
elevation-helper-confirm = 지금 드라이버 도우미를 시작하시겠습니까? Windows가 관리자 권한을 한 번 묻고, 프로그램이 종료될 때까지 다시 묻지 않습니다.
analysis-cross-heading = 다른 카메라에서 교차 확인
analysis-cross-intro = 선택 사항: 다른 카메라에도 찾은 값이 있는지, 그리고 그 카메라에서는 무엇이 설정되어 있는지 확인합니다. 읽기만 합니다. 번호가 같다고 해서 반드시 같은 기능을 뜻하는 것은 아니며, 다른 카메라의 메뉴를 살펴봐야만 이를 확인할 수 있습니다. 결과는 같은 분석 보고서에 기록됩니다.
analysis-cross-do = 1. 분석한 카메라를 분리합니다. 2. 다른 카메라를 켜고 USB 모드를 “자동”으로 설정한 뒤 케이블을 연결하고 “대용량 저장장치(MSC)”를 선택합니다.
btn-analysis-cross = 다른 카메라 확인
analysis-cross-present = 있음, 값 { $value }
analysis-cross-match = “{ $label }”과(와) 같음
analysis-cross-nomatch = 측정한 메뉴 값 중 어느 것과도 같지 않음
analysis-cross-absent = 없음
analysis-cross-same-camera = 분석한 카메라 자체입니다. 다른 카메라를 연결하십시오.
analysis-cross-not-now = 교차 확인은 모든 스냅샷을 읽었고 복원을 기다리는 기록된 값이 없을 때만 가능합니다.
analysis-cross-none = 교차 확인할 후보가 없습니다.
analysis-wrong-camera = 연결된 카메라는 분석한 카메라가 아닙니다. { $model }을(를) 연결하십시오. 아무것도 기록하지 않았습니다.
keys-fetching = 소스를 다운로드하는 중…
keys-scanning = 키 자료를 검색하는 중…
keys-complete = 모든 키 자료가 있습니다.
keys-incomplete = 일부 키 자료가 아직 없습니다.
keys-present = 있음
keys-missing = 없음
keys-class-senser = 서비스 모드 키 (서비스 셸, 트윅, 분석)
keys-class-cic = XPD 체크섬 키 (앱 설치)
keys-class-spk = SPK 키 자료 (앱 설치, apk2spk)
keys-class-fdat = FDAT 키 자료 (업데이터 셸 펌웨어)
plugin-heading = 플러그인
plugin-intro = 플러그인은 언어, 카메라 설정용 스위치, 호환성 정보, 도구, 진단 기능으로 이 프로그램을 확장합니다. 플러그인은 사용자가 직접 선택한 파일이나 사용자가 추가한 소스에서 가져오며, 이 프로그램은 플러그인을 검사하지 않습니다. 플러그인 코드는 사용자의 파일, 네트워크, 키 자료에 접근할 수 없는 샌드박스에서 실행됩니다. 작성자를 신뢰할 수 있는 플러그인만 설치하십시오. 변경 사항은 다음 시작 시 적용됩니다.
plugin-none = 설치된 플러그인이 없습니다.
plugin-inactive = (다시 시작한 후 활성화됨)
plugin-author = 작성자
plugin-restart = 변경 사항을 적용하려면 프로그램을 다시 시작하십시오.
btn-plugin-install = 플러그인 설치…
plugin-install-confirm = 이 플러그인은 다음을 할 수 있습니다:
btn-confirm-plugin-install = 플러그인 설치
plugin-installed = 플러그인을 설치했습니다.
plugin-remove-confirm = 플러그인 “{ $name }”을(를) 제거하시겠습니까?
plugin-removed = 플러그인 { $id }을(를) 제거했습니다.
plugin-perm-languages = 프로그램에 언어 추가
plugin-perm-camera-settings = 카메라 설정 변경 (플러그인이 명시한 두 값 사이에서만, 사용자가 확인한 후에만)
plugin-perm-compatibility = 호환성 개요에 항목 추가
plugin-perm-code = 샌드박스에서 자체 코드 실행 (도구 또는 진단)
plugin-perm-host-os = 사용 중인 운영 체제 확인
plugin-perm-host-cameras = 연결된 Sony 카메라와 해당 모드 확인
plugin-invalid = 사용할 수 있는 플러그인이 아닙니다: { $reason }
plugin-unknown-field = 이 플러그인은 이 버전의 프로그램이 알지 못하는 기능({ $field })을 사용합니다. 더 새로운 버전이나 다른 에디션의 프로그램용으로 만들어졌습니다.
plugin-no-folder = 사용자 데이터 폴더를 확인할 수 없어 플러그인을 설치할 수 없습니다.
plugin-not-installed = ID가 { $id }인 플러그인이 설치되어 있지 않습니다.
plugin-failed = 플러그인이 실패했습니다: { $reason }
plugin-unknown-entry = { $id }을(를) 제공하는 로드된 플러그인이 없습니다.
plugin-run-done = 플러그인이 완료되었습니다.
plugin-run-warning = 플러그인이 참고 사항과 함께 완료되었습니다.
plugin-run-error = 플러그인이 오류를 보고했습니다.
plugin-tools-heading = 플러그인 도구
plugin-tools-hint = 설치된 플러그인에서 제공하는 도구이며 샌드박스에서 실행됩니다. 플러그인은 사용자가 넘겨준 것과 선언한 것만 받으며, 카메라를 변경할 수 없습니다.
plugin-diagnostics-heading = 플러그인 진단
plugin-compat-heading = 플러그인 제공
plugin-col-note = 참고
file-filter-plugin = 플러그인 파일
keys-not-set-up = 이 작업에 필요한 키 자료가 설정되어 있지 않습니다.
err-missing-keys = 이 작업에 필요한 키 자료가 설정되어 있지 않습니다.
keys-heading = 필요한 키 자료
keys-intro = 서비스 모드 키, XPD 체크섬 키, SPK 키 자료는 이 프로그램에 포함되어 있지 않습니다. 보유한 소스(저장소, 압축 파일 또는 파일)의 주소를 입력하면 프로그램이 인식할 수 있는 부분을 찾아 가져옵니다. 아무것도 실행하지 않으며, 자료는 이 컴퓨터에만 보관됩니다.
keys-needs-toggle = 어떤 기능에 어떤 키 자료가 필요합니까?
keys-needs-info = 카메라 정보 (MTP, 대용량 저장장치, 업데이터 모드)
keys-needs-info-keys = 필요 없습니다.
keys-needs-install = 앱 설치 및 '카메라 확인'
keys-needs-install-keys = XPD 체크섬 키. 앱을 설치하려면 SPK 키 자료도 필요합니다.
keys-needs-service = 서비스 모드: 서비스 셸, 트윅, 분석
keys-needs-service-keys = 서비스 모드 키.
keys-needs-updater = 업데이터 모드: 업데이터 셸 및 앱 백업
keys-needs-updater-keys = FDAT 키 자료.
keys-law-notice = 거주 지역에 적용되는 법률을 준수하고, 본인 소유의 카메라와 데이터로만 작업하십시오.
keys-url-label = 소스 주소 (한 줄에 하나씩)
keys-url-required = 소스 주소를 하나 이상 입력하십시오.
btn-keys-import = 가져오기
btn-import-keys = 키 자료 설정
