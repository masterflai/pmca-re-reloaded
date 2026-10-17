# Localización al español de PMCA-RE RELOADED.
# Traduce únicamente la parte derecha del signo =. No cambies nunca los ID.

app-title = PMCA-RE RELOADED – Herramientas para cámaras

# Navegación / pestañas
nav-camera-info = Información de la cámara
nav-install-app = Aplicaciones
nav-tweaks = Ajustes
nav-network = Red
nav-firmware = Firmware
nav-updater = Consola del actualizador
nav-service = Consola de servicio
nav-tools = Herramientas de archivos
nav-settings = Configuración
nav-license = Licencia

# Página de bienvenida: la página con la que se abre la ventana.
nav-welcome = Bienvenida
welcome-intro-heading = Qué hace este programa
welcome-intro-body = Este programa ayuda a conservar las funciones de su cámara Sony y a adaptarla a sus necesidades. Cuando Sony cerró la tienda de PlayMemories Camera Apps, dejó de haber forma de hacer copias de seguridad de las aplicaciones compradas o ya instaladas, o de volver a instalarlas. Restablecer la cámara borra las aplicaciones instaladas de forma irrecuperable. Este programa permite hacer copias de seguridad de las aplicaciones existentes y volver a instalarlas, y además puede ofrecer aplicaciones de la comunidad de libre acceso. También puede instalar sus propias aplicaciones (archivos *.apk). Ante problemas, el programa lee la información de la cámara y sus registros internos para analizarlos. Los ajustes que la cámara ya tiene pero que su menú no ofrece se pueden cambiar y deshacer de nuevo. El programa se comunica con la cámara directamente por el cable USB. Los modelos admitidos se enumeran en la página Información de la cámara.
welcome-start-heading = El primer paso
welcome-start-body = Encienda primero la cámara y, con el cable aún desconectado, ponga «Conexión USB» en su menú en Almacenamiento masivo o MTP. «Auto» suele funcionar también, porque entonces la cámara se decide por uno de los dos por sí misma. El orden importa: una vez conectado el cable, ya no se puede acceder al menú de la cámara. Solo entonces conecte la cámara por USB, abra Información de la cámara y pulse «Obtener información de la cámara». En cuanto aparezcan el modelo y la versión del firmware, la conexión es correcta y todas las demás funciones de este programa están disponibles.
welcome-start-driver = Si en su lugar aparece un aviso sobre el controlador, atienda primero a ese. En Windows un controlador del sistema se apropia de la cámara y hay que sustituirlo antes de que sea posible el acceso USB directo que estas herramientas necesitan. El programa se encarga de ello por sí mismo, pero asignar un controlador es un derecho que Windows solo concede a un administrador. En ese caso, el programa le ofrece la posibilidad de reiniciarlo como administrador, y Windows le pedirá que confirme la operación. La asignación se retira de nuevo al terminar la sesión. En Linux y macOS no es necesario.
welcome-start-driver-linux = Si en su lugar aparece un aviso sobre el controlador, atienda primero a ese. En Linux no hay que sustituir nada: la cámara se alcanza directamente. Lo que hace falta una sola vez es el derecho a abrir el dispositivo —una regla udev para el identificador de fabricante de Sony— y, para el modo de almacenamiento masivo, liberar el controlador del núcleo usb-storage. Ambas cosas se preparan fuera de este programa con derechos de root; el script scripts/linux-prepare.sh del código fuente hace las dos. Esta ventana nunca pide por sí misma tales derechos.
welcome-start-driver-macos = Si en su lugar aparece un aviso sobre el controlador, atienda primero a ese. En macOS no hay que instalar ni sustituir nada, y esta ventana nunca pide derechos de administrador. Configure la conexión USB de la cámara en MTP: en el modo de almacenamiento masivo la retiene el controlador de disco del sistema y no se puede acceder a ella.
welcome-tweaks-heading = Ajustes ocultos e idiomas
welcome-tweaks-body = Los ajustes ocultos son opciones que la cámara ya tiene pero no ofrece en su menú: el límite de tiempo de grabación, el límite de grabación en 4K, la elección entre PAL y NTSC, el instalador de aplicaciones por USB — y los idiomas del menú.
welcome-tweaks-languages-heading = Los idiomas de la cámara
welcome-tweaks-languages-body = Las cámaras Sony solo muestran los idiomas de menú habituales en su región de venta. Quien vive en el extranjero a menudo no encuentra su propio idioma y tiene que arreglárselas con uno ajeno. Sin embargo, el firmware ya contiene más idiomas. Este programa los lleva al menú para que pueda manejar su cámara en un idioma familiar.
welcome-tweaks-ui-heading = El idioma de este programa
welcome-tweaks-ui-body = Aparte de todo eso está el selector de la parte superior. Solo cambia el idioma de este programa y nunca toca la cámara.
welcome-tweaks-safety = Cada ajuste se escribe en la memoria de respaldo de la cámara, y todos pueden deshacerse con el mismo interruptor. Una escritura interrumpida puede dañar la cámara de forma permanente: no desconecte el cable ni retire la batería mientras haya una en curso.
welcome-privacy-heading = Sus datos
welcome-privacy-where = La página «Licencia» vuelve a exponer todo esto por completo, junto con las licencias de todas las bibliotecas con las que está construido este programa.
welcome-where-heading = Dónde encontrar cada cosa
welcome-where-info = Modelo, número de serie y versión del firmware. Es también la prueba de conexión.
welcome-where-install = Aplicaciones de la lista de la comunidad, o un archivo .apk propio.
welcome-where-tweaks = Los ajustes que el menú de la cámara no alcanza, entre ellos los idiomas.
welcome-where-network = Datos de asistencia GPS, y los puntos de acceso y servicios usados para la retransmisión en directo.
welcome-where-firmware = Escribir una imagen de firmware de Sony en la cámara.
welcome-where-tools = Desempaquetar e inspeccionar archivos de firmware y de paquete. No hace falta cámara.
welcome-where-shells = Acceso directo a la memoria y al sistema de archivos de la cámara. Antes hay que mostrarlo en Configuración.
welcome-where-settings = Tema, modo avanzado y edición de la traducción de este programa.
welcome-where-license = De dónde procede esta adaptación, sobre el trabajo de quién se apoya y bajo qué condiciones se publica.
welcome-where-hint = Ninguna página actúa sobre la cámara por su cuenta. No se lee ni se escribe nada hasta que pulse un botón que lo anuncie.

# Botones
btn-get-info = Obtener información de la cámara
btn-refresh = Actualizar
btn-load-apps = Cargar aplicaciones disponibles
btn-list-tweaks = Mostrar ajustes disponibles
btn-install = Instalar la aplicación seleccionada
btn-install-file = Instalar archivo .apk…
btn-apply = Aplicar
btn-save = Guardar en la cámara
btn-add = Añadir
btn-remove = Quitar
btn-reveal = Mostrar
btn-update-gps = Transferir los datos de asistencia GPS al dispositivo
btn-cancel = Cancelar
btn-browse = Examinar…
btn-switch-msc = Cambiar al modo de almacenamiento masivo
btn-start-driver-helper = Iniciar ayudante de controladores

# Filtros de tipo de archivo de los diálogos
file-filter-apk = Aplicación Android (*.apk)
file-filter-firmware = Imagen de firmware Sony (*.dat)
file-filter-usage-log = Registro de uso (*.bin)
file-filter-catalog = Archivo de traducción (*.ftl)
file-filter-spk = Paquete de aplicación Sony (*.spk)
file-filter-any = Todos los archivos
file-filter-gps = Datos de asistencia GPS (*.dat)

# Estado / mensajes
msg-connecting = Conectando…
msg-usage-log-written = Registro de uso escrito en { $path }
msg-apps-found = Se encontraron { $count } aplicaciones
msg-switch-mode = Cambia la cámara al modo MTP o de almacenamiento masivo.
msg-no-tweaks = No hay ajustes disponibles para esta cámara.
msg-done = Listo.

# Cambio de modo USB de la cámara (MTP → almacenamiento masivo). Se ofrece bajo
# un error de «modo incorrecto»; no existe comando USB para el sentido contrario.
mode-switching = Pidiendo a la cámara que cambie al modo de almacenamiento masivo…
mode-switch-wait = Esperando a que la cámara vuelva a conectarse…
camera-confirm-hint = Los mensajes de la cámara (p. ej., sobre www.sony.net/pm) se pueden cerrar; no interfieren en el proceso.
mode-switch-done = La cámara está ahora en modo de almacenamiento masivo.
mode-switch-timeout = La cámara no reapareció en modo de almacenamiento masivo. Apágala y enciéndela de nuevo e inténtalo otra vez.
mode-switch-impossible = Este modo no se puede cambiar por USB. Apaga y enciende la cámara.
mode-switch-hint = Solo una cámara en modo MTP se puede cambiar por USB. El cambio es temporal: el modo USB seleccionado en el menú de la propia cámara no varía, así que apagarla y encenderla —o desconectar y volver a conectar el cable— restaura el modo anterior. No existe ningún comando USB para el camino de vuelta.

# Campos de información de la cámara
info-model = Modelo
info-product-code = Código de producto
info-serial = Número de serie
info-firmware = Versión del firmware
info-lens = Objetivo
info-region = Región
info-mode = Modo
mode-mass-storage = Almacenamiento masivo
mode-mtp = MTP
mode-updater = Modo actualizador
mode-app-install = Modo de instalación de aplicaciones
mode-service = Modo de servicio
mode-unknown = Desconocido
info-android-version = Versión de la plataforma Android
info-photographer = Fotógrafo
info-copyright = Copyright
info-photographer-1 = Fotógrafo (entrada 1)
info-photographer-2 = Fotógrafo (entrada 2)
info-copyright-1 = Copyright (entrada 1)
info-copyright-2 = Copyright (entrada 2)
diagnose-services-heading = ¿No se puede acceder a la cámara?
diagnose-services-hint = Algunos servicios de Windows pueden acaparar una cámara en modo MTP, de modo que esta aplicación no puede acceder a ella. La comprobación solo pregunta a Windows qué servicios están en ejecución; no cambia nada.
btn-diagnose-services = Comprobar servicios de Windows
diagnose-services-none = No se está ejecutando ningún servicio conflictivo conocido.
diagnose-services-found = Estos servicios en ejecución pueden interferir con el acceso a la cámara:
diagnose-services-advice = Detenga el servicio en Servicios (services.msc) o con el siguiente comando como administrador y vuelva a conectar la cámara:

# Actualizador de firmware / consola
updater-model = Modelo de cámara
updater-firmware-current = Versión de firmware actual
updater-guard-ok = La imagen de firmware coincide con esta cámara.
updater-no-fdat = No hay firmware de actualizador incluido para el modelo { $model }.
updater-model-required = La cámara ya está en modo actualizador y ahí no puede informar de su modelo. Apaga la cámara y vuelve a encenderla para que arranque en modo normal, y vuelve a intentarlo. También puedes indicar un archivo de carga útil (--fdat-file) o, en la consola, el modelo (--model <MODELO>).
updater-already-updater = La cámara ya está en modo actualizador.
updater-flash-required = No se modificará la cámara sin confirmación. Vuelve a ejecutarlo con --flash para cambiar al modo actualizador y escribir la carga útil.
updater-flashing = Escribiendo la carga útil del actualizador en la cámara…
updater-switch-wait = Sigue las instrucciones de la pantalla de la cámara. Esperando a que la cámara pase al modo actualizador…
updater-switch-timeout = Se agotó el tiempo de espera de la cámara. Ejecuta el comando de nuevo cuando se haya reconectado.
updater-shell-ready = La carga útil del actualizador está en ejecución. Escribe «help» para ver los comandos y «exit» para salir.
updater-shell-done = Sesión de consola finalizada.
updater-session-panic = La sesión del actualizador terminó de forma inesperada. Apague y encienda la cámara antes de volver a intentarlo.

# Lo que responde la consola del actualizador. Los fallos al analizar una línea
# están redactados una sola vez, en las claves service-*: ambas consolas
# comparten el analizador, así que una ruta entre comillas y un valor
# hexadecimal erróneo fallan igual en las dos.
updater-backup-synced = Copia de seguridad escrita en la memoria flash.
updater-backup-locked = Los ajustes protegidos de la copia de seguridad quedan bloqueados.
updater-backup-writable = Los ajustes protegidos de la copia de seguridad ya admiten escritura.
updater-shell-console-only = La consola de root necesita un terminal y solo está disponible desde la línea de comandos: ejecute pmca-console updatershell --flash y escriba allí «shell».

# La página de la consola del actualizador.
updater-warning = Atención: para llegar a esta consola la cámara se reinicia en modo actualizador y se escribe en ella una carga útil: un pequeño programa de este proyecto que la cámara acepta como una actualización de firmware y que después responde a las órdenes de esta consola. Esa carga útil desaparece al apagar la cámara, pero sus órdenes escriben en la memoria de ajustes permanente de la cámara, y una escritura interrumpida puede dañarla de forma irreparable. Además, en el proceso se restablecen todos los ajustes de la cámara, a diferencia de lo que ocurre con la consola de servicio.
updater-console-heading = Ejecutar la consola del actualizador desde la línea de comandos
updater-driver-hint = La consola del actualizador necesita que la cámara siga en el controlador de almacenamiento masivo propio de Windows, USBSTOR, porque la carga útil se transmite por él. Es justo lo contrario de lo que exige la consola de servicio. La aplicación asigna y libera el controlador WinUSB por sí misma, así que no hay nada que preparar a mano; eso sí, un controlador asignado por otra herramienta hay que deshacerlo antes. Una cámara en modo actualizador se presenta con su propio identificador de producto y tampoco debe reasignarse.
updater-fdat-label = Archivo de carga útil (opcional)
updater-confirm = La cámara se reiniciará en modo actualizador y se escribirá en ella una carga útil. ¿Continuar?
updater-cancelled = Cancelado. La cámara no se ha modificado.

btn-shell-primer = Lo que conviene saber de este shell
btn-shell-primer-hide = Ocultar la explicación
shell-primer-compare-heading = Dos shells, dos protocolos
shell-primer-compare = Ambas páginas ofrecen una línea de órdenes hacia la cámara, pero no son el mismo shell. El shell de servicio habla el protocolo de servicio de fábrica de Sony, al que la cámara responde por sí sola una vez autenticada; para llegar ahí no se escribe nada en la cámara. El shell de actualización solo existe porque este proyecto escribe un programa propio en la cámara: esta reinicia en modo actualización, acepta la carga útil como si fuera un firmware y es esa carga útil la que responde después a las órdenes de más abajo. Solo uno de los dos puede estar abierto a la vez.
shell-primer-payload = «Carga útil» (en inglés, «payload») designa el contenido propiamente dicho que transporta una vía de envío. En el shell de actualización es un pequeño programa de este proyecto: la cámara lo acepta por la misma vía que una actualización de firmware, pero solo lo conserva hasta que se apaga. El shell de servicio no necesita carga útil, porque la cámara responde a sus órdenes con su propio firmware. En los archivos de firmware el término significa lo mismo: la imagen de firmware propiamente dicha dentro del contenedor del archivo .dat.
shell-primer-when-service = El shell de servicio sirve para todo lo que lea o modifique los ajustes guardados de la cámara mientras esta sigue ejecutando su propio firmware: leer el bloque HASP, guardar una copia de seguridad de la zona de respaldo, cambiar entradas sueltas, sacar archivos de la cámara. Es también lo que usa por debajo la página Ajustes; todo lo de allí puede hacerse aquí a mano.
shell-primer-when-updater = El shell de actualización es la opción cuando no hay otra forma de llegar a la cámara, o cuando hace falta un shell sobre el sistema en marcha en vez de órdenes sueltas. Es la única vía que ofrece un shell root de verdad y sigue funcionando en una cámara que rechaza el modo de servicio. Cuesta un reinicio, una escritura y los ajustes de la cámara, así que es la segunda opción, no la primera.
shell-primer-driver-heading = Controladores: cada uno exige lo contrario
shell-primer-driver = El shell de servicio habla USB en bruto y necesita que en Windows la cámara esté asociada a WinUSB. La aplicación asigna ese controlador por sí misma al entrar en modo de servicio y lo libera al terminar la sesión; no interviene ninguna herramienta aparte. El shell de actualización llega a través de USBSTOR, el propio controlador de almacenamiento masivo de Windows, y deja de funcionar mientras la cámara esté asociada a WinUSB; por eso liberar la asignación forma parte de cerrar una sesión de servicio. Asignar un controlador requiere permisos de administrador. En Linux no hace falta cambiar ningún controlador; solo tener acceso al nodo del dispositivo.
shell-primer-risk-heading = Qué puede salir mal
shell-primer-risk = Las órdenes marcadas más abajo escriben en la memoria de ajustes permanente de la cámara. Una escritura interrumpida puede dejar la cámara sin arrancar y nada de esto se deshace: precisamente para eso existe bk d, guarde una copia de la zona de respaldo antes de cambiar nada. La carga útil del actualizador no se queda en la cámara: apagarla la borra. El paso por el modo actualizador, sin embargo, restablece todos los ajustes de la cámara.
shell-primer-commands-heading = Órdenes de este shell
shell-primer-writes = Las órdenes marcadas con (WRITES) modifican la cámara. Todas las demás solo leen.
shell-cmd-hasp = Lee el bloque HASP, la zona de identidad protegida de la cámara.
shell-cmd-pull = Trae un archivo de la cámara. Sin nombre local se conserva el nombre remoto.
shell-cmd-push = Copia un archivo local a la cámara sustituyendo lo que haya allí. (WRITES)
shell-cmd-rm = Borra un archivo de la cámara. (WRITES)
shell-cmd-rmem = Escribe una región de la memoria del dispositivo en un archivo local. Desplazamiento y longitud son hexadecimales.
shell-cmd-bk-r = Lee una entrada de la zona de respaldo y la muestra en volcado hexadecimal.
shell-cmd-bk-d = Guarda toda la zona de respaldo en un archivo. Hágalo antes de cambiar nada: es el único camino de vuelta.
shell-cmd-bk-dump-txt = Guarda toda la zona de respaldo como <file>.bin y un listado legible como <file>.txt.
shell-cmd-bk-stat = Muestra el bloque de estado del respaldo de la cámara, incluida la región.
shell-cmd-bk-w = Escribe una entrada de la zona de respaldo, dada como bytes hexadecimales separados por espacios. (WRITES)
shell-cmd-bk-s-sub = Fija en la memoria flash la zona de respaldo de un subsistema para que el cambio sobreviva a un reinicio. (WRITES)
shell-cmd-bk-s = Fija la zona de respaldo en la memoria flash para que el cambio sobreviva a un reinicio. (WRITES)
shell-cmd-bk-lock = Activa o desactiva la protección contra escritura de las entradas de respaldo protegidas; algunos ajustes se niegan a cambiar mientras está activa. (WRITES)
shell-cmd-term = Enciende o apaga el terminal de depuración de la cámara. (WRITES)
shell-cmd-info = Lee el bloque de estado del respaldo, que dice si la zona se puede escribir siquiera.
shell-cmd-shell = Abre un shell root en la cámara. Necesita un terminal, así que solo está disponible en la consola; shell tcp <port> lo ofrece en 127.0.0.1 para otro cliente. (WRITES)
shell-cmd-help = Enumera las órdenes que acepta este shell.
shell-cmd-exit = Termina la sesión y sale del shell.
shell-primer-diagnostics-heading = Diagnóstico y reparación
shell-cmd-bk-trace = Registra en un archivo, paquete a paquete, cómo envía la cámara toda la zona de respaldo por USB. Sirve para examinar respaldos grandes; puede dejar la cámara bloqueada a mitad de la respuesta. En ese caso, desconecte el cable y apague y encienda la cámara, retirando la batería si es necesario.
shell-cmd-bk-rewrite = Lee la zona de respaldo, la vuelve a escribir sin cambios y la lee de nuevo. Muestra si la cámara acepta una imagen completa sin cambiar ningún ajuste. (WRITES)
shell-cmd-bk-restore-attrs = Restablece los atributos de todas las entradas de respaldo, como la protección contra escritura, según una copia guardada con bk d o bk dump-txt. Los valores no cambian. Use solo una copia de la misma cámara. (WRITES)
shell-primer-diagnostics-env = Tres variables de entorno afectan tanto a la consola como a la interfaz gráfica si se definen antes de iniciarlas, por ejemplo en PowerShell con $env:PMCA_TWEAKS_DIRECT = "1". PMCA_TWEAKS_DIRECT hace que los tweaks escriban valores individuales aunque el respaldo ocupe menos de 1 MB. PMCA_TWEAKS_IMAGE hace que escriban la imagen completa aunque ocupe más de 1 MB; si ambas están definidas, prevalece PMCA_TWEAKS_DIRECT. PMCA_TLS_DUMP=<archivo> guarda el primer mensaje TLS de la cámara (ClientHello) durante la instalación de una app, para examinar una conexión fallida.

# Instalación de aplicaciones
install-switching = Cambiando al modo de instalación de aplicaciones…
install-disabled = La cámara rechazó entrar en el modo de instalación de aplicaciones. Suele deberse al ajuste «Activar el instalador de aplicaciones por USB»: hay que activarlo en «Ajustes».
install-stays-app-mode = La cámara permanece en el modo de instalación de aplicaciones, en el que se pueden instalar más aplicaciones. Para cualquier otra función, desconecta el cable USB y apaga y vuelve a encender la cámara.
err-usb-app-install-mode = La cámara sigue en el modo de instalación de aplicaciones, en el que solo acepta instalaciones de aplicaciones. Desconecta el cable USB y luego apágala y vuélvela a encender: si se reinicia conectada, vuelve directamente a este modo.
install-switch-wait = Esperando a que la cámara se reconecte en modo de instalación de aplicaciones…
install-switch-timeout = Se agotó el tiempo de espera para que la cámara entrara en el modo de instalación de aplicaciones. Ejecuta el comando de nuevo cuando se haya reconectado.
install-bad-mode = La cámara no se puede usar en este modo. Cámbiala al modo MTP o de almacenamiento masivo.
install-starting = Iniciando la instalación…
install-error = Error de comunicación durante la instalación
install-server-failed = El servidor de instalación local no aceptó la conexión de la cámara:
install-done = Tarea completada correctamente.

# Análisis del APK
apk-package = Paquete:
apk-unreadable = Aviso: no se pudo analizar el archivo .apk.
apk-incompatible = Aviso: esta aplicación declara un nivel mínimo de SDK que quizá la cámara no ofrezca.
apk-unsigned = Aviso: el archivo .apk no tiene firma.

# Tienda de aplicaciones de la comunidad
store-loading = Cargando la lista de aplicaciones…
store-resolving = Buscando la última versión publicada…
store-downloading = Descargando
store-unknown-app = Aplicación desconocida
store-no-release = Esta aplicación no tiene ninguna versión descargable
store-not-installable = sin versión
msg-no-apps = No se encontraron aplicaciones.
store-heading = Tienda de aplicaciones de la comunidad
store-discontinued = Sony cerró la tienda oficial PlayMemories Camera Apps; sus servidores ya no responden, así que allí no se pueden comprar ni descargar aplicaciones.
store-use-own-apk = Aún puedes instalar tu propio .apk: elige el archivo abajo o arrástralo a esta ventana. La tienda de la comunidad, más abajo, también sigue funcionando.
btn-store-sync = Comprobar la cámara
btn-install-selected = Instalar seleccionadas
btn-store-stop = Detener tras esta aplicación
btn-store-stopping = Se detendrá tras esta aplicación…
btn-store-retry = Reintentar las fallidas
btn-continue = Continuar
btn-reinstall = Reinstalar
btn-skip = Omitir
btn-confirm-updater = Reiniciar y grabar
btn-confirm-tweaks = Grabar ajustes
btn-confirm-languages = Activar idiomas
btn-confirm-wifi = Sustituir puntos de acceso
btn-confirm-stream = Sobrescribir ajustes
btn-confirm-show-shells = Mostrar consolas
btn-confirm-discard = Descartar traducción
store-select-all = Seleccionar todas las aplicaciones
store-col-app = Aplicación
store-col-version = Versión
store-col-author = Autor
store-col-camera = Versión en la cámara
store-col-status = Estado
store-camera-absent = no instalada
store-version-unknown = desconocida
store-batch-query = Leyendo las aplicaciones instaladas en la cámara…
store-sync-done = Cámara comprobada. Aplicaciones de la lista instaladas: { $count }
store-batch-downloading = Descargando la aplicación { $current } de { $total }…
store-batch-installing = Aplicación { $current } de { $total }: instalando { $name }…
store-reinstall-confirm = { $name } ya está instalada en la cámara. Versión instalada: { $installed }. Versión en la tienda de aplicaciones: { $store }. ¿Desea instalarla de nuevo?
store-state-queued = en espera
store-state-installed = instalada
store-state-failed = con error
store-state-skipped = omitida
store-state-not-run = no ejecutada
store-batch-summary = Instaladas: { $done } · con error: { $failed } · omitidas: { $skipped } · no ejecutadas: { $notrun }

# Modo de servicio (senser)
usb-mode-advice = Para los ajustes, las copias de seguridad y el shell de servicio, ponga el modo de conexión USB de la cámara en «Auto», no fijo en almacenamiento masivo. Cuando la cámara pregunte por el modo tras una sesión, elija «Almacenamiento masivo (MSC)». Con el almacenamiento masivo fijo, la cámara rechaza a menudo el siguiente cambio al modo de servicio.
service-switch-refused = La cámara no completó el cambio al modo de servicio ni volvió a conectarse por USB; normalmente muestra entonces «Compruebe el dispositivo conectado». Desconecte el cable USB, vuelva a conectarlo e inténtelo de nuevo.
service-switching = Cambiando al modo de servicio…
service-switch-wait = Esperando a que la cámara se reconecte en modo de servicio…
service-switch-timeout = La cámara no apareció en modo de servicio. Se vuelve a enumerar como un dispositivo USB independiente (id. de producto 0336), que necesita su propia asignación del controlador WinUSB; busca en el administrador de dispositivos un dispositivo Sony sin controlador. Apaga y enciende la cámara para devolverla al modo normal.
service-authenticating = Autenticando…
# Se muestra cuando la cámara sigue en modo de servicio por una ejecución
# anterior, y cuando eso no se ha podido deshacer.
service-stranded-reset = La cámara sigue en modo de servicio por una ejecución anterior. Primero se finalizará esa sesión…
service-stranded-failed = La cámara se ha quedado bloqueada en modo de servicio por una ejecución anterior y no ha respondido a la petición de salir de él. Apáguela y vuelva a encenderla, y luego inténtelo de nuevo.
service-bad-mode = La cámara no se puede usar en este modo. Cámbiala al modo de almacenamiento masivo.
service-driver-required = Se denegó el acceso USB directo y no se pudo asignar el controlador WinUSB. Asignar un controlador requiere permisos de administrador.
service-warning = Aviso: el modo de servicio concede acceso directo a la memoria, el sistema de archivos y los datos de calibración de la cámara. Los comandos de escritura pueden dañarla de forma permanente.
service-shell-ready = Consola de servicio lista. Escribe «help» para ver los comandos y «exit» para salir.
service-shell-done = Sesión de consola de servicio finalizada.
# Se informa al iniciar cuando una ejecución anterior terminó antes de poder
# liberar el controlador que había asignado.
driver-swept = Se han eliminado { $count } asignación(es) de controlador que dejó una ejecución anterior.
# Se informa al inicio cuando esa misma ejecución interrumpida también dejó la
# cámara en modo de servicio, donde rechazaría la siguiente sesión.
driver-swept-camera = Una cámara que una ejecución anterior dejó en modo de servicio ha vuelto a su modo normal.
driver-release-replug = Windows está esperando a la cámara mientras libera el controlador. Desconecte el cable USB y vuelva a conectarlo para que la limpieza pueda terminar.
# Resultados del comando `driver`.
driver-bound = { $id } ya utiliza WinUSB (paquete de controladores { $package }).
driver-released = Se han liberado { $count } asignación(es) de controlador; la cámara vuelve a usar los controladores incluidos en Windows.
driver-needs-elevation = Asignar un controlador requiere permisos de administrador. Ejecute este comando en una consola iniciada como administrador.
driver-leftover-needs-elevation = Una ejecución anterior dejó un controlador asignado a la cámara. Mientras exista, la cámara pide comprobar el dispositivo conectado y su almacenamiento masivo no funciona. Liberarlo requiere derechos de administrador; confirme la solicitud de Windows.
driver-binding = Asignando el controlador WinUSB a la cámara…
driver-bind-wait = Esperando a que la cámara vuelva a conectarse con el nuevo controlador…
driver-bind-timeout = La cámara no volvió a aparecer tras asignar el controlador. Desconéctela, vuelva a conectarla e inténtelo de nuevo.
driver-bind-failed = No se pudo asignar el controlador WinUSB: { $detail }
# Respuestas de la consola de servicio. Proceden del backend, de modo que la
# consola y la ventana emplean las mismas palabras.
service-usage = Uso: { $usage }
service-bad-hex = No es un valor hexadecimal: { $value }
service-unknown-command = Comando desconocido: { $command }. Escriba 'help' para ver la lista.
service-unterminated-quote = La línea termina dentro de un argumento entre comillas.
service-help = Lista de comandos:
service-read-bytes = Se leyeron { $count } bytes.
service-pulled = Se copió { $remote } de la cámara a { $local }.
service-pushed = Se copió { $local } a { $remote } en la cámara.
service-removed = Se eliminó { $remote } de la cámara.
service-dumped = Se volcaron { $size } bytes desde { $offset } a { $local }.
service-backup-saved = Imagen de copia de seguridad ({ $count } bytes) guardada en { $local }.
service-backup-text-saved = Imagen de copia de seguridad ({ $count } bytes) guardada en { $bin } y listado en { $txt }.
service-backup-status = Estado del respaldo, región: { $region }
service-backup-rewritten = Imagen ({ $count } bytes) reescrita sin cambios y leída de nuevo; no se ha guardado de forma permanente.
service-backup-attrs-restored = Atributos de { $count } entradas restablecidos según { $local }.
service-backup-written = Se escribió la entrada de copia de seguridad { $id }.
service-backup-persisted = Copia de seguridad del subsistema { $id } guardada en la memoria flash.
service-terminal-on = El terminal de depuración está ahora activado.
service-terminal-off = El terminal de depuración está ahora desactivado.
btn-service-terminal-open = Abrir consola serie
btn-service-terminal-close = Cerrar consola serie
service-terminal-heading = Consola serie
service-terminal-hint = En las cámaras Linux, el terminal de depuración es un shell de root por USB. Ábrelo y escribe comandos como en un terminal.
service-terminal-label = Consola
service-terminal-interactive = Consola serie abierta. Escribe comandos; escribe «exit» para volver al shell de servicio.

# La página de servicio sin el modo avanzado: cómo llegar a la consola de
# servicio desde la línea de comandos y qué hay que preparar antes.
service-console-heading = Ejecutar la consola de servicio en la línea de comandos
service-console-hint = Abra un terminal en la carpeta que contiene pmca-console y ejecute este comando:
service-driver-heading = Antes de empezar
service-driver-hint = El modo de servicio necesita acceso USB directo, que Windows no concede a través del controlador de almacenamiento masivo. La aplicación asigna el controlador WinUSB por sí misma — dos veces, una mientras la cámara está en modo de almacenamiento masivo y otra después de que reaparezca en modo de servicio con un segundo identificador de producto — y libera ambas asignaciones al terminar la sesión. Asignar un controlador requiere permisos de administrador, así que inicie la consola como administrador; no hace falta ninguna otra herramienta.
driver-note-linux = Aquí no hace falta instalar nada: libusb llega a la cámara directamente, sin desplazar ningún controlador del núcleo. Solo se necesitan permiso para abrir el dispositivo y un escritorio que no lo tenga ya ocupado. Instale una regla udev para el identificador de fabricante de Sony 054c y desvincule el controlador usb-storage antes de usar el modo de almacenamiento masivo. El script scripts/linux-prepare.sh hace ambas cosas y además detiene los monitores de volúmenes de gvfs.
driver-note-macos = Para el acceso USB directo no hace falta instalar nada. Configure la conexión USB de la cámara en MTP: en el modo de almacenamiento masivo la retiene el controlador de disco del sistema y el programa no puede acceder a ella. En el modo MTP, el programa detiene el servicio de cámaras de macOS ptpcamerad si ocupa la cámara; el sistema lo vuelve a iniciar cuando hace falta. Esta plataforma es la menos probada de las tres; cuente con informar de un problema antes que con encontrarlo ya descrito.
service-advanced-hint = La consola también puede mostrarse aquí, en Ajustes, Modo avanzado.
btn-copy-command = Copiar comando
msg-command-copied = Comando copiado al portapapeles.

# La página de servicio con el modo avanzado activado.
btn-service-open = Conectar
btn-service-close = Desconectar
btn-service-run = Ejecutar
btn-service-browse = Insertar ruta de archivo…
shell-path-quote = No se puede insertar esta ruta: contiene comillas dobles, el carácter con el que la línea de órdenes delimita el principio y el final de un argumento. Cambie el nombre del archivo o muévalo a una ruta sin ese carácter.
service-command-label = Comando
service-not-connected = Sin conexión.

# Cadenas exclusivas de la interfaz gráfica
install-apk-path = Ruta a un archivo .apk local (opcional)
install-drop-hint = Arrastra aquí un archivo .apk

# Descripciones de los ajustes
tweak-rec-limit = Grabar vídeo sin límite de tiempo
tweak-rec-limit-4k = Grabar vídeo 4K sin límite de tiempo
tweak-language = Mostrar todos los idiomas del menú
tweak-pal-ntsc = Activar el selector PAL / NTSC y el aviso
tweak-pal-ntsc-mode = Sistema de vídeo (PAL / NTSC)
tweak-pal-ntsc-pal = PAL
tweak-pal-ntsc-ntsc = NTSC
tweak-pal-ntsc-note = Tras cambiar el sistema de vídeo, la cámara exige formatear la tarjeta de memoria. Haga antes una copia de todo: fotos, vídeos, LUT. Lo más sencillo es sacar la tarjeta antes de cambiar.
tweaks-set-malformed = Se esperaba --set ID=VALOR, se recibió: { $arg }
tweaks-set-unknown-value = Valor desconocido «{ $value }». Se esperaba uno de: { $options }
tweaks-set-not-a-choice = { $id } es un interruptor, no una elección. Use --enable o --disable.
tweaks-needs-set = { $id } es una elección, no un interruptor. Use --set { $id }=VALOR con uno de: { $options }
tweak-usb-installer = Activar el instalador de aplicaciones por USB

# Proceso de ajustes
tweaks-warning = Aviso: aplicar ajustes reescribe la memoria de copia de seguridad de la cámara. Una escritura interrumpida puede dañarla de forma permanente.
tweaks-reading = Leyendo la configuración de la cámara…
tweaks-writing = Escribiendo la configuración de la cámara…
tweaks-done = Configuración de la cámara actualizada.
tweaks-write-failed = La cámara no guardó la nueva configuración tal como se envió. Es posible que una parte se haya escrito igualmente: vuelva a leer la configuración y deshaga los cambios si es necesario. Una copia de seguridad hecha antes del cambio es el camino seguro de vuelta.
tweaks-confirm-required = No se escribirá sin confirmación. Vuelve a ejecutarlo con --confirm cuando hayas leído el aviso anterior.
tweaks-state-enabled = activado
tweaks-state-disabled = desactivado
tweaks-full-read-failed = La cámara no entregó su copia de seguridad completa ({ $detail }). Después suele dejar de responder: apáguela, quite la batería unos segundos, vuelva a colocarla y conéctela de nuevo.
tweaks-direct-mode = La copia de seguridad de esta cámara supera 1 MB, así que sus ajustes se leen y escriben uno a uno. Los valores anteriores se guardan antes de cada cambio y pueden restaurarse con «Deshacer cambios».
tweaks-direct-forced = PMCA_TWEAKS_DIRECT fuerza la vía de valores individuales, así que los ajustes se leen y escriben uno a uno. Los valores anteriores se guardan antes de cada cambio y pueden restaurarse con «Deshacer cambios».
tweaks-direct-refused = La cámara rechazó el cambio ({ $detail }). Los valores ya escritos se restauraron; no se cambió nada. Algunos modelos, sobre todo japoneses, solo permiten este ajuste mediante la copia de seguridad completa.
tweaks-direct-revert-failed = La cámara rechazó el cambio ({ $detail }) y no todos los valores ya escritos pudieron restaurarse. Los valores anteriores están guardados: pruebe «Deshacer cambios».
tweaks-direct-restart = Ajustes de la cámara actualizados. Apague y vuelva a encender la cámara para que el cambio surta efecto.
tweaks-rollback-unsaved = No se pudieron guardar los valores actuales para deshacer; no se cambió nada.
tweaks-image-saved = Se guardó la imagen de la copia de seguridad anterior al cambio: { $path }
tweaks-whole-image-hint = Para escribir el cambio mediante la imagen completa de la copia de seguridad, vuelva a ejecutar el mismo comando con --whole-image. Antes se guarda la imagen anterior al cambio.
tweaks-whole-image-confirm = ¿Escribir el cambio mediante la imagen completa de la copia de seguridad? La cámara lo rechazó como valor individual. Se vuelve a escribir la imagen completa con el cambio; antes se guarda en este equipo la imagen anterior al cambio.
btn-whole-image = Escribir copia completa
tweaks-undo-none = No hay valores guardados para esta cámara.
tweaks-undo-confirm = ¿Escribir de nuevo en la cámara los valores guardados? Esto deshace todos los cambios escritos ajuste por ajuste desde que se guardaron.
tweaks-undo-done = Los valores guardados se escribieron de nuevo. Apague y vuelva a encender la cámara.
tweaks-undo-available = Los valores de antes del primer cambio están guardados y pueden restaurarse.
btn-undo-tweaks = Deshacer cambios
tweaks-identity-heading = Identidad de la cámara (solo lectura)
tweaks-caution-languages = Esta cámara no puede mostrar dos idiomas: probablemente chino (simplificado) y chino (tradicional). Al desplazarse hasta ellos, el menú de idioma se reinicia, por lo que se dejan desactivados por precaución.
tweaks-caution-accept = Activar estos dos idiomas de todos modos (bajo su responsabilidad)
tweaks-caution-confirm = Estos dos idiomas pueden hacer que se reinicie el menú de idioma de la cámara. No se daña nada y el cambio se puede deshacer, pero quizá tenga que buscar el ajuste desde el principio. ¿Activarlos igualmente?

# Configuración
settings-language = Idioma
settings-theme = Tema
settings-theme-system = Sistema
settings-theme-light = Claro
settings-theme-dark = Oscuro

# Modo avanzado. Una protección frente a un descuido, no frente a un atacante:
# todo lo que muestra sigue estando accesible desde la línea de comandos.
settings-advanced-heading = Modo avanzado
settings-advanced-hint = El modo avanzado muestra la consola de servicio y la consola del actualizador en las páginas del mismo nombre. El modo de servicio da acceso directo a la memoria de la cámara, a su sistema de archivos y a sus datos de calibración; un comando mal escrito puede dejar la cámara inservible. Déjelo desactivado mientras no sepa qué va a escribir.
settings-advanced-toggle = Mostrar las consolas de servicio y del actualizador
settings-advanced-on = El modo avanzado está activado. Las páginas Consola de servicio y Consola del actualizador ofrecen ahora las consolas en sí.
settings-advanced-off = El modo avanzado está desactivado. Las páginas Consola de servicio y Consola del actualizador explican en su lugar cómo usar la línea de comandos.
settings-advanced-confirm = Las consolas de servicio y del actualizador pueden dañar la cámara de forma permanente. ¿Mostrar ambas consolas?

# Edición de una traducción. Los archivos de idioma se pueden descargar,
# modificar en cualquier editor de texto y volver a subir; un archivo subido
# se comprueba antes de usarse.
settings-catalog-heading = Editar traducción
settings-catalog-hint = Descargue un archivo de idioma, modifíquelo en el editor de texto que prefiera y vuelva a subirlo. Solo se traduce el texto a la derecha del signo igual, y las entradas que omita conservan su texto original. El idioma elegido aquí no cambia el idioma de la aplicación.
settings-catalog-locale = Archivo de idioma
settings-catalog-builtin = Se está usando la traducción incluida.
settings-catalog-custom = Se está usando su propia traducción:
btn-catalog-export = Descargar…
btn-catalog-import = Subir…
btn-catalog-reset = Restaurar la incluida
catalog-exported = Escrito en
catalog-imported = Traducción aplicada.
catalog-reset-done = Se ha restaurado la traducción incluida.
catalog-reset-none = No había ninguna traducción propia que restaurar.
catalog-reset-confirm = ¿Descartar su propia traducción para este idioma?
catalog-rejected = El archivo no se ha aplicado. Corrija estas líneas:
catalog-count-accepted = Entradas aplicadas
catalog-count-missing = Entradas conservadas de la traducción incluida
catalog-count-untranslated = Entradas idénticas al original en inglés
catalog-line = Línea
catalog-problem-syntax = no es una entrada con la forma «id = texto»
catalog-problem-continuation = continúa en la línea siguiente, lo que no es compatible
catalog-problem-duplicate = definida más de una vez
catalog-problem-unknown-key = no es una entrada que use esta aplicación
catalog-problem-empty = sin texto
catalog-problem-placeholder = marcador de posición modificado, se esperaba:

# Origen, agradecimientos y licencia. Se muestran en la página Licencia y con
# `pmca-console license`. El texto de la licencia no se traduce: es un
# documento legal y se reproduce literalmente.
about-thanks-heading = Agradecimientos
about-thanks-body-1 = Esta herramienta existe gracias a ma1co, que averiguó cómo se comunican las cámaras de Sony por USB y publicó Sony-PMCA-RE en 2015, y gracias a todas las personas que contribuyeron a ese proyecto y a los proyectos OpenMemories en los años siguientes.
about-thanks-body-2 = La detección de la cámara, el protocolo de comandos externos, el procedimiento del actualizador de firmware y el modo de servicio son obra suya, y esta adaptación continúa ese trabajo. Gracias. Sobre esa base, PMCA-RE RELOADED añade muchas funciones nuevas aportadas por masterflai. La principal es el cambio automático del controlador del dispositivo en Windows, ahora integrado: ya no hace falta asignar a mano otro controlador a la cámara con una herramienta aparte, lo que hace el proceso más cómodo, más rápido y menos arriesgado.
about-contributors-heading = Colaboradores
about-contributors-body = Además de ma1co, estas personas contribuyeron a Sony-PMCA-RE o publicaron en sus propias copias cambios que se incorporaron a esta adaptación:
about-purpose-heading = Por qué existe esta adaptación
about-purpose-body-1 = La aplicación original ya no se desarrolla. Se escribió para Python 2, que llegó al final de su vida útil en 2020, y algunas de sus partes dependen de servicios que Sony ha cerrado desde entonces, por lo que ya no funciona sin restricciones en los sistemas actuales.
about-purpose-body-2 = El propósito de esta adaptación es la preservación, no el reemplazo, y en igual medida un uso sin complicaciones hoy: mantener estas cámaras utilizables y que el programa funcione sin dificultades tanto en los ordenadores actuales como con modelos de cámara más recientes. Todo lo que podía hacer la original debería seguir siendo posible aquí.
about-origin-heading = Cómo se hizo
about-origin-body-1 = La adaptación de Python a Rust se llevó a cabo con ayuda de la IA.
about-origin-body-2 = Lo que eso no significa merece decirse con la misma claridad: el comportamiento está cubierto por un conjunto de pruebas automatizadas y por un plan de pruebas de hardware. Sobre todo la versión para Windows se ha comprobado a fondo, porque los controladores de dispositivo la convierten en la plataforma más compleja; macOS y Linux, en cambio, no han podido probarse por completo. Trata esto como software experimental.
about-license-heading = Licencia
about-license-body = Esta adaptación se publica bajo la licencia MIT, la misma licencia que el proyecto original. El aviso de copyright original se mantiene sin cambios, tal como exige esa licencia.
about-license-full = Texto de la licencia
about-third-party-heading = Licencias de las bibliotecas utilizadas
about-third-party-body = Este programa está enlazado estáticamente: el código de cada biblioteca que utiliza se encuentra dentro del ejecutable, y sus licencias exigen que los avisos de copyright lo acompañen. Se reproducen íntegramente más abajo, una entrada por paquete.
about-third-party-toggle = Mostrar todos los avisos de terceros
about-privacy-heading = Datos
about-privacy-body-1 = Este programa se ejecuta en su propio ordenador. No envía datos de uso, no hay telemetría ni se exige ninguna cuenta. Nada de lo que haga aquí se comunica a ningún sitio.
about-privacy-body-2 = Dos funciones sí establecen conexiones por sí solas, y la otra parte conoce entonces su dirección IP: los datos de asistencia GPS se descargan de Sony, y la lista de aplicaciones de GitHub. Sin esas funciones el programa permanece completamente sin conexión.
about-privacy-body-3 = Lo que se lee de la cámara —incluidos su número de serie y las contraseñas de Wi-Fi guardadas— no sale de su ordenador. Si lo escribe en un archivo, tenga cuidado a quién le entrega ese archivo.
about-privacy-body-4 = Detrás de este programa no se explota ningún servicio y ningún dato llega a sus autores, así que aquí no hay nada que solicitar ni que hacer borrar. Lo que los dos servicios mencionados hagan con las direcciones que ven se rige por sus propias declaraciones de privacidad, no por esta.
about-legal-heading = Avisos legales
about-legal-trademark = Este es un proyecto independiente. No está hecho por Sony, ni cuenta con su respaldo, ni guarda relación alguna con ella. «Sony», «PlayMemories» y los nombres de los modelos de cámara son marcas de Sony Group Corporation y aquí se mencionan únicamente para indicar con qué cámaras habla esta herramienta.
about-legal-own-device = Este programa está pensado para usarse con cámaras de su propiedad.
about-legal-warranty = Lo que hace esta herramienta queda fuera de lo que Sony prevé para estas cámaras y puede poner fin a su garantía. El software se ofrece sin garantía propia de ningún tipo y su uso corre por su cuenta y riesgo. Si la ingeniería inversa de un aparato de su propiedad está permitida, y hasta qué punto, depende del país.
about-links-heading = Proyectos
about-link-original = Sony-PMCA-RE — el proyecto original
about-link-fwtool = fwtool.py — las herramientas de firmware en las que se basa la generación de imágenes de actualización
about-link-tweak = OpenMemories: Tweak — la aplicación recomendada para las cámaras que la admiten
about-link-applist = OpenMemories: App list — más aplicaciones
about-link-platform = OpenMemories: Platform — el cuerpo del actualizador incluido
about-link-devices = Modelos de cámara compatibles
about-link-nexhack = Nex-Hack — la investigación comunitaria en la que se basa este proyecto
about-link-zve10 = zv-e10-re — investigación sobre las interfaces y la memoria de ajustes de la ZV-E10
about-version = Versión
btn-copy-url = Copiar dirección
msg-url-copied = Dirección copiada al portapapeles.

# Aviso de seguridad
notice-experimental = Este software es experimental. Úsalo bajo tu propia responsabilidad.

# Ajustes de red (GPS / Wi-Fi / emisión en directo)
gps-downloading = Descargando los datos de asistencia GPS…
gps-writing = Escribiendo los datos de asistencia GPS en la cámara…
gps-bad-size = Los datos de asistencia GPS tienen un tamaño inesperado y no se escribieron.
wifi-read = Leyendo los puntos de acceso de transmisión en directo de la cámara…
wifi-written = Puntos de acceso de transmisión en directo escritos.
stream-read = Leyendo la configuración de emisión en directo de la cámara…
stream-written = Configuración de emisión en directo escrita.
net-secret-warning = Aviso: este archivo contiene contraseñas Wi-Fi y tokens de acceso en texto claro.
wifi-secret-warning = La cámara guarda estas contraseñas sin cifrar y aquí se muestran tal como están guardadas. No salen de este ordenador; si las guarda en un archivo, tenga cuidado a quién se lo entrega.
gps-heading = Datos de asistencia GPS
gps-hint = Arrastra un archivo aquí, introduce una ruta o deja el campo vacío para obtener los datos actuales de Sony.
btn-gps-devices = Dispositivos compatibles
btn-gps-devices-hide = Ocultar dispositivos compatibles
gps-devices-note = Estos modelos tienen un módulo GPS integrado. En muchos de ellos solo lo lleva la variante cuyo nombre termina en V; las variantes sin V no tienen GPS.
gps-devices-compact = Cámaras compactas (DSC)
gps-devices-slt = Cámaras SLT (montura A)
gps-devices-action = Cámaras de acción
gps-devices-camcorder = Videocámaras (Handycam)
gps-col-variant = Variante GPS
gps-col-arch = Arquitectura
gps-col-module = Módulo GPS
gps-col-apps = Compatibilidad con apps
gps-yes = sí
gps-no = no
gps-unknown = desconocido
gps-devices-untested = Con GPS, compatibilidad sin probar
gps-devices-untested-note = Estos modelos tienen un módulo GPS integrado, pero no figuran en la lista de dispositivos de OpenMemories. No se ha probado si esta aplicación funciona con ellos, por lo que su arquitectura y su compatibilidad con apps son desconocidas.
wifi-heading = Puntos de acceso para la transmisión en directo
wifi-hint = Son los puntos de acceso que usa la función de transmisión en directo, no la configuración Wi-Fi de la cámara. Un cuerpo sin función de transmisión rechaza esta orden.
wifi-multi = Lista de varios puntos de acceso
wifi-none = No hay puntos de acceso guardados.
wifi-writing = Escribiendo los puntos de acceso de transmisión en directo en la cámara…
wifi-confirm = Esto sustituirá todos los puntos de acceso guardados en la cámara. ¿Continuar?
wifi-empty-refused = No se escribirá una lista vacía, ya que borraría todos los puntos de acceso guardados.
stream-heading = Emisión en directo
stream-writing = Escribiendo la configuración de emisión en directo en la cámara…
stream-confirm = Esto sobrescribirá la configuración de emisión en directo de la cámara. ¿Continuar?
stream-invalid-json = La configuración no es JSON válido:

# Actualización de firmware y tienda oficial
firmware-warning = Aviso: esto sobrescribe el firmware de la cámara. No interrumpas la actualización una vez iniciada: no desconectes el cable USB ni apagues la cámara. Usa solo un archivo de firmware destinado exactamente a este modelo de cámara y asegúrate de que la batería esté suficientemente cargada. De lo contrario, la cámara puede quedar inutilizada de forma permanente.
firmware-untested = Sin probar: la escritura de firmware se ha portado, pero nunca se ha ensayado en una cámara real. Una escritura fallida o interrumpida no se puede deshacer; considere todo lo de esta página como no comprobado.
firmware-confirm-required = No se grabará sin --confirm.
firmware-truncated = El archivo de firmware está truncado: la carga útil FDAT se extiende más allá del final del archivo.
firmware-updating = Actualizando el firmware:
firmware-invalid = El archivo no es una imagen de firmware Sony válida
firmware-dat-path = Archivo de firmware (.dat)
firmware-drop-hint = Arrastra aquí un archivo de firmware .dat
firmware-file-required = Elige primero un archivo de firmware.
firmware-check-ok = La cámara acepta esta imagen. No se ha escrito nada.
firmware-cancelled = Grabación cancelada. La cámara no se modificó.
firmware-current-version = Versión instalada
firmware-new-version = Versión de esta imagen
firmware-models-hint = Una cámara es compatible si la consola del actualizador puede acceder a ella.
btn-firmware-check = Comprobar imagen
btn-firmware-flash = Grabar firmware
firmware-own-device = Confirme que es consciente de estos riesgos y que solo actualiza su propio dispositivo.
btn-firmware-accept = Riesgos entendidos – grabar

# Herramientas de archivos: las operaciones que no necesitan cámara alguna.
tools-hint = Estas operaciones trabajan sobre archivos y nunca tocan la cámara, así que nada de esto puede dañarla. Son las mismas que pmca-console ofrece como apk2spk, spk2apk, print-backup, print-usagelog, guess-firmware y xpd-checksum.
tools-operation = Operación
tools-apk2spk = Empaquetar un .apk en un contenedor .spk
tools-spk2apk = Desempaquetar un .spk de vuelta a .apk
tools-print-backup = Mostrar el contenido de un Backup.bin
tools-print-usagelog = Mostrar el contenido de un Usagelog.bin
tools-guess-firmware = Localizar la carga útil dentro de un .dat de firmware
tools-xpd-checksum = Calcular la suma de comprobación CIC de un archivo
tools-input = Archivo de entrada
tools-file-required = Elija primero los archivos que necesita esta operación.
tools-apk-invalid = El archivo no se pudo empaquetar como .spk
tools-spk-invalid = El archivo no es un paquete .spk válido
tools-backup-invalid = El archivo no es una imagen Backup.bin válida
tools-usagelog-invalid = El archivo no es un registro de uso válido
tools-reset-value = valor de reinicio:
tools-fdat-found = Carga útil encontrada en el desplazamiento { $offset }, tamaño { $size }.
tools-checksum = Suma de comprobación
tools-written = Escrito en { $path }

# Enlaces a la documentación del proyecto
docs-compatibility = Compatibilidad de cámaras
docs-compatibility-hint = El proyecto original ofrecía un resumen de las cámaras probadas. Abarca sobre todo modelos antiguos y se encuentra en esta dirección:
devices-note = Datos tomados del resumen del proyecto original. El número junto a la compatibilidad con apps es la versión de la plataforma de aplicaciones de la cámara.
devices-more-note = Las siguientes cámaras no se han probado expresamente con este programa. Una marca en la última columna significa que la comunidad ha informado de su uso. Si además la consola de servicio está marcada con ✖, ese uso fracasó.
btn-more-models = Más dispositivos
btn-more-models-hide = Ocultar más dispositivos
check-updater-yes = Su dispositivo { $model } es compatible: según la lista de dispositivos, la consola del actualizador puede acceder a él.
check-updater-no = Su dispositivo { $model } no es compatible: según la lista de dispositivos, la consola del actualizador no puede acceder a él.
check-updater-unknown = La compatibilidad de su dispositivo { $model } es desconocida: no figura en ninguna lista de dispositivos de este programa, por lo que no se sabe si la consola del actualizador puede acceder a él.
check-apps-no = Su dispositivo { $model } no es compatible: según la lista de dispositivos, no admite apps.
check-apps-untested = La compatibilidad de su dispositivo { $model } es desconocida: figura en la lista de dispositivos, pero no está confirmado que admita apps.
check-apps-unknown = La compatibilidad de su dispositivo { $model } es desconocida: no figura en ninguna lista de dispositivos de este programa, por lo que no se sabe si admite apps.
check-gps-no = Su dispositivo { $model } no es compatible: no figura en la lista de dispositivos con receptor GPS.
check-gps-untested = La compatibilidad de su dispositivo { $model } es desconocida: según la lista de dispositivos tiene receptor GPS, pero la transferencia a este modelo no se ha probado.
check-gps-unknown = La compatibilidad de su dispositivo { $model } es desconocida: no figura en ninguna lista de dispositivos de este programa, por lo que no se sabe si tiene receptor GPS.
check-unidentified = No se pudo identificar el dispositivo conectado, por lo que su compatibilidad es desconocida.
check-proceed = ¿Desea continuar de todos modos?
check-cancelled = Operación cancelada. El dispositivo no se modificó.
devices-col-confirmed = Confirmado
devices-group-ilc = Cámaras de objetivos intercambiables (monturas E y A)
devices-group-remote = Mandos Live-View
btn-copy-link = Copiar enlace
shutter-heading = Contador de disparos
shutter-hint = La cámara cuenta cada disparo del obturador y escribe el valor actual en cada foto que guarda (ARW, JPEG o HEIF). Elige la foto más reciente para leer el valor actual. El archivo solo se lee, nunca se modifica.
shutter-source = La posición del contador en el archivo varía según el modelo. Las posiciones proceden de investigaciones documentadas públicamente (ExifTool) y no están confirmadas para todos los modelos.
shutter-drop-hint = Arrastra aquí una foto (ARW, JPEG, HEIF)
btn-shutter-read = Analizar foto
shutter-file-required = Elige primero una foto.
shutter-file = Archivo
shutter-taken = Fecha de captura
shutter-count = Disparos
shutter-type = Obturador
shutter-type-mechanical = Mecánico
shutter-type-electronic = Electrónico
shutter-electronic-hint = Esta foto se tomó con el obturador electrónico. En muchos modelos esas tomas no se cuentan; el valor muestra entonces el recuento del último disparo mecánico.
shutter-not-sony = La foto no se tomó con una cámara Sony. No se puede leer de ella un contador de disparos.
shutter-unknown-model = No se sabe dónde guarda este modelo de cámara el contador en el archivo, por lo que no se muestra ningún valor.
shutter-not-recorded = El archivo no contiene un contador legible. Ocurre con fotos guardadas o exportadas por programas de edición y, en algunos modelos, con tomas hechas con el obturador electrónico. Elige el archivo original de la tarjeta de memoria.
shutter-model-mismatch = La foto procede de un modelo de cámara distinto al de la cámara conectada.
shutter-invalid = El archivo no es una foto legible
file-filter-photo = Fotos (ARW, JPEG, HEIF)
usage-log-heading = Registro de uso
usage-log-hint = La cámara mantiene un registro interno en un formato no documentado. Se guarda tal cual; la cámara solo se lee.
usage-log-written = Escrito en
btn-usage-log-info = Acerca del registro de uso
btn-usage-log-info-hide = Ocultar la explicación
usage-log-info-what-heading = Qué es este registro
usage-log-info-what = Sony llama internamente a este registro «KikiLog». La cámara lo escribe ella misma; este programa solo lo lee. No se conoce ninguna descripción pública del formato.
usage-log-info-read = Leerlo no cambia nada: varias descargas seguidas fueron idénticas byte a byte.
usage-log-info-flush-heading = Por qué faltan entradas recientes
usage-log-info-flush = Las entradas nuevas a menudo no aparecen enseguida: la cámara puede guardarlas solo al apagarse, a veces repartidas en varios ciclos de encendido y apagado. Apaga y enciende la cámara, vuelve a descargar y repite hasta que no aparezcan entradas nuevas.
usage-log-info-content-heading = Qué se puede leer en él
usage-log-info-shutter = Disparos: algunos eventos corresponden cada uno a una toma. Sin embargo, su número no da el contador de disparos: el registro tiene un tamaño fijo y no tiene por qué abarcar toda la vida útil de la cámara. La cámara guarda el total en otro lugar y lo escribe en las fotos que guarda; puede leerse en la sección «Contador de disparos».
usage-log-info-clock = Las marcas de tiempo solo son tan fiables como el reloj de la cámara. Tras cambiar la batería, el reloj puede retroceder y generar entradas con la fecha inicial de la cámara.
usage-log-info-unknown = No se conoce ninguna descripción pública de los números de evento. El programa muestra los números sin interpretarlos.
usage-log-info-privacy-heading = Privacidad
usage-log-info-privacy = El archivo contiene los números de serie de las tarjetas de memoria insertadas. No lo comparta ni lo publique a la ligera.
btn-usage-log = Guardar registro
usagelog-size = Área del registro
usagelog-cursor = Posición de escritura
usagelog-spare = Segunda área
usagelog-count = Entradas
usagelog-events = Eventos por número
usagelog-entries = Entradas en orden
msg-link-copied = Enlace copiado al portapapeles.

# Mensajes de error del backend, indexados por el código estable que informa.
# La parte variable del mensaje (una ruta, un byte de estado, un error del
# dispositivo) la añade la interfaz, así que estas frases se leen completas sin ella.
# No existe `err-operation` a propósito: ese código marca un mensaje que el
# backend ya ha traducido y debe mostrarse literalmente.
err-usb-no-device = No se encontró ninguna cámara. Conecta una cámara y enciéndela.
err-usb-camera-not-ready = La cámara está conectada pero no lista: está apagada o espera en su pantalla que se elija el modo USB. Enciéndela o elige «Almacenamiento masivo (MSC)» en la cámara.
camera-choose-usb-mode = La cámara espera que se elija el modo USB. Elige «Almacenamiento masivo (MSC)» en la cámara.
err-usb-too-many-devices = Hay más de una cámara conectada. Desconecta todas salvo la que quieras usar.
err-usb-transfer = Falló la transferencia USB. Prueba otro cable u otro puerto y evita los concentradores USB.
err-usb-permission = El programa no tiene permiso para abrir la cámara por USB. En Linux este derecho se concede una sola vez mediante una regla udev para el identificador de fabricante de Sony; el script scripts/linux-prepare.sh del código fuente la instala. Después, desconecta la cámara y vuelve a conectarla.
err-usb-busy = Otro programa está usando la cámara, por lo que no se pudo abrir. Cierra otros programas que puedan acceder a ella; en Linux el monitor de medios del escritorio puede retenerla, y scripts/linux-prepare.sh lo detiene. En macOS, configura la conexión USB de la cámara en MTP: en el modo de almacenamiento masivo la retiene el controlador de disco del propio sistema.
err-usb-stall = La cámara canceló la transferencia USB.
err-usb-protocol = La cámara envió una respuesta inesperada.
err-usb-msc-status = La cámara rechazó un comando de almacenamiento.
cli-error-prefix = error
err-usb-driver = Windows ha asignado a esta cámara un controlador que no permite el acceso directo. Este modo necesita el controlador WinUSB, que la aplicación puede asignar por sí misma, pero solo con permisos de administrador.
err-needs-elevation = Asignar o liberar un controlador requiere derechos de administrador, y la solicitud de Windows fue rechazada. Vuelva a intentarlo con «Iniciar ayudante de controladores»; no hay que cambiar nada en la cámara.
err-usb-no-mass-storage-volume = La cámara está conectada en modo de almacenamiento masivo pero no ofrece ninguna unidad. Actívela, confirme el modo USB en su pantalla si lo pide y compruebe que hay una tarjeta de memoria insertada. No cambie el controlador: este modo necesita precisamente el estándar de Windows.
err-usb-msc-sense = La cámara informó de un error de almacenamiento.
err-usb-ptp-response = La cámara rechazó el comando.
err-usb-unsupported = Este modelo de cámara no admite esa función.
err-usb-updater-status = El actualizador de firmware de la cámara rechazó la operación.
err-usb-senser = Falló un comando del modo de servicio.
err-market-core = No se pudo leer un archivo de datos de la cámara.
err-market-json = Los datos recibidos tienen un formato incorrecto.
err-market-missing-field = Falta un campo esperado en la respuesta.
err-market-no-apk = No se ha seleccionado ninguna aplicación para instalar.
err-market-http = Falló la petición de red.
err-market-io = Falló el servidor de instalación local.
err-market-tls = No se pudo establecer la conexión cifrada con la cámara.
err-apk = No se pudo leer el archivo de la aplicación.
err-io = No se pudo leer o escribir un archivo.
err-json = La cámara envió datos con un formato incorrecto.
err-unknown-locale = Este idioma no está disponible.
err-catalog-encoding = El archivo de traducción no está guardado como texto UTF-8. Guárdelo de nuevo con codificación UTF-8.
err-internal = Se produjo un error interno. Se trata de un fallo de la aplicación.

## Function analysis
nav-analysis = Análisis
analysis-title = Análisis de funciones
analysis-experimental = Experimental. Esta función es nueva y solo se ha probado en pocas cámaras.
analysis-own-hardware = Úsela solo en su propia cámara. El modo de servicio ya es una intervención que Sony no prevé; la responsabilidad es suya.
analysis-explanation-toggle = Explicación: finalidad, procedimiento, riesgos
analysis-purpose-heading = ¿Para qué sirve?
analysis-purpose = El análisis de funciones documenta en qué valor de su memoria de ajustes (copia de seguridad) guarda la cámara uno de sus propios ajustes de menú. El resultado describe la memoria de ajustes de esta cámara; puede compararse con otras cámaras, pero no cambia lo que ofrece una cámara.
analysis-how-heading = ¿Cómo funciona?
analysis-how = Por cada valor de menú que indique, el programa lee una vez la memoria de ajustes completa, cada vez en una breve sesión propia del modo de servicio. Entre lecturas, desconecte el cable y cambie el ajuste en el menú de la cámara; la cámara permanece encendida. Al final se vuelve a leer el primer valor. Los valores que siguen al menú y al final vuelven a su valor inicial son candidatos. Los valores que cambian por sí solos (contadores, reloj) se descartan.
analysis-requirements-heading = Requisitos
analysis-requirements = La cámara está conectada por USB, su modo USB está en «Auto» y tras cada sesión elige «Almacenamiento masivo (MSC)» en la cámara. En Windows el programa necesita derechos de administrador, porque asigna brevemente un controlador para el modo de servicio. Cargue la batería; cada sesión dura aproximadamente un minuto.
analysis-read-heading = Pasos de lectura
analysis-read = Las capturas solo leen. Nada cambia en los ajustes de la cámara. El riesgo es bajo, pero no nulo: cambiar al modo de servicio ya es una intervención que Sony no prevé.
analysis-write-heading = Paso de escritura (prueba de escritura)
analysis-write = Solo si lo activa expresamente: el programa escribe en la cámara el valor encontrado, usted comprueba en el menú tras un reinicio si surte efecto y después se vuelve a escribir el valor original. Solo se escribe un valor que la propia cámara guardó y solo si se encontró exactamente un candidato adecuado. El valor anterior se guarda antes. Aun así, este paso modifica la cámara; un fallo puede dejarla en un estado que solo el servicio técnico puede reparar.
analysis-legal-heading = Aspectos legales
analysis-legal = El software se proporciona sin ninguna garantía (licencia MIT). Las intervenciones en la cámara pueden afectar a su garantía. Use la función solo en cámaras de su propiedad. El informe de análisis no contiene número de serie ni nada de la memoria de ajustes salvo los candidatos encontrados; nunca se envía automáticamente.
analysis-start-heading = Nuevo análisis
analysis-setting-label = Ajuste de menú, tal como lo llama la cámara
analysis-values-label = Valores de menú, separados por comas
analysis-values-hint = El primer valor es el que está ajustado ahora. De dos a ocho valores; cada uno cuesta una sesión del modo de servicio.
analysis-write-option = Después, realizar la prueba de escritura (modifica la cámara)
analysis-write-warning = La prueba de escritura modifica la cámara: escribe un valor en su memoria de ajustes y después vuelve a escribir el valor original. Solo en su propia cámara, bajo su propio riesgo.
analysis-write-consent = La cámara es mía y asumo el riesgo.
btn-analysis-write-enable = Activar prueba de escritura
btn-analysis-start = Iniciar análisis
btn-analysis-snapshot = Leer copia de seguridad
btn-analysis-new = Nuevo análisis
analysis-discard-confirm = ¿Descartar el análisis en curso? El informe hasta ahora sigue guardado.
analysis-access-read = LECTURA
analysis-access-write = ESCRITURA
analysis-access-user = MANUAL
analysis-access-local = PC
analysis-step-snapshot = Leer la copia de seguridad, el ajuste está en «{ $value }»
analysis-step-change = Cambiar a «{ $value }» en el menú de la cámara
analysis-step-evaluate = Comparar las capturas
analysis-step-write = Escribir en la cámara el valor de «{ $value }»
analysis-step-check = Apagar y encender la cámara, leer el menú (esperado: «{ $value }»)
analysis-step-restore = Volver a escribir el valor original de «{ $value }»
analysis-step-report = Guardar el informe de análisis
analysis-do-prepare = PREPARACIÓN: 1. En el menú de la cámara, ponga «{ $setting }» en «{ $value }» y salga del menú con el botón MENU para que la cámara guarde el ajuste. 2. Deje la cámara encendida. 3. Conecte el cable y elija «Almacenamiento masivo (MSC)» en la cámara.
analysis-do-change = SU TURNO: 1. Desconecte el cable. 2. La cámara sigue encendida. 3. En el menú de la cámara, ponga «{ $setting }» en «{ $value }» y salga del menú con el botón MENU para que la cámara guarde el ajuste. 4. Vuelva a conectar el cable y elija «Almacenamiento masivo (MSC)».
analysis-do-check = SU TURNO: 1. Desconecte el cable. 2. Apague y vuelva a encender la cámara. 3. Mire qué muestra el menú en «{ $setting }».
analysis-do-reconnect = Vuelva a conectar el cable y elija «Almacenamiento masivo (MSC)» en la cámara.
analysis-reading = Captura { $index } de { $total }: NO desconecte el cable mientras se lee.
analysis-cli-continue = Pulse Intro para continuar, q para cancelar:
analysis-confirm-word = si
analysis-cli-confirm = Escriba «{ $word }» para confirmar:
analysis-write-declined = No confirmado. No se ha escrito nada.
analysis-aborted = Cancelado. No se ha escrito nada.
analysis-snapshot-done = Captura leída. La cámara ha salido del modo de servicio.
analysis-result = { $compared } valores comparados: { $count } candidatos, { $running } valores cambiaron por sí solos.
analysis-candidate-consistent = coincide claramente con el menú
analysis-candidate-inconsistent = no coincide claramente
analysis-write-preview-heading = Esto se escribirá
analysis-write-preview = Valor { $id } ({ $size } bytes): de { $old } («{ $old-label }») a { $new } («{ $new-label }»). La propia cámara guardó el nuevo valor cuando estaba ajustado «{ $new-label }».
btn-analysis-write = Escribir
btn-analysis-skip-write = Terminar sin prueba de escritura
analysis-written = Valor escrito, releído y guardado de forma permanente.
analysis-menu-question = ¿Qué muestra ahora el menú?
analysis-menu-other = otra cosa
btn-analysis-menu-confirm = Aceptar
btn-analysis-restore = Volver a escribir el valor original
analysis-restored = Valor original vuelto a escribir y guardado de forma permanente.
analysis-survived = El valor escrito seguía guardado tras el reinicio.
analysis-not-survived = El valor escrito ya no estaba guardado tras el reinicio.
analysis-report-saved = Informe de análisis guardado: { $path }
analysis-report-hint = El informe no contiene número de serie ni copia de seguridad. Puede compartirlo para que el hallazgo se compruebe y documente.
btn-analysis-export = Exportar informe…
file-filter-analysis = Informe de análisis
analysis-pending = Una prueba de escritura no se completó. El valor original está guardado y puede volver a escribirse.
analysis-pending-none = No hay ninguna prueba de escritura pendiente guardada para esta cámara.
analysis-invalid-setting = Indique el ajuste de menú tal como lo llama la cámara.
analysis-invalid-values = Indique de dos a { $max } valores de menú distintos. El primero es el que está ajustado ahora.
analysis-already-complete = Todas las capturas ya se han leído.
analysis-other-camera = No es la misma cámara que en la primera captura. El análisis se ha detenido.
analysis-write-not-allowed = La prueba de escritura no es posible en este estado.
analysis-camera-changed = La cámara contiene ahora un valor distinto al de la primera captura. No se ha escrito nada.
analysis-write-refused = La cámara rechazó el valor; todo sigue como antes. ({ $detail })
analysis-refuse-none = Prueba de escritura no posible: ningún valor siguió al menú.
analysis-refuse-ambiguous = Prueba de escritura no posible: { $count } valores siguieron al menú, ninguno puede asignarse con seguridad.
analysis-refuse-inconsistent = Prueba de escritura no posible: el valor no coincide claramente con los valores del menú.
analysis-refuse-size = Prueba de escritura no posible: el valor ocupa { $size } bytes; solo se escriben valores pequeños.
analysis-refuse-read-only = Prueba de escritura no posible: la cámara marca el valor como de solo lectura.
analysis-refuse-protected = Prueba de escritura no posible: la cámara marca el valor como protegido.
settings-analysis-heading = Análisis de funciones
settings-analysis-purpose = El análisis de funciones documenta en qué valor de su memoria de ajustes guarda la cámara uno de sus propios ajustes de menú. Está pensado para quienes quieren entender la memoria de ajustes de su modelo y compartir los resultados.
settings-analysis-why-off = Está desactivado porque el uso normal no lo necesita, porque pone la cámara varias veces en modo de servicio y porque, si se solicita, también escribe en la cámara. Es experimental y solo se ha probado en pocos modelos.
settings-analysis-toggle = Activar el análisis de funciones (entrada «Análisis» en el menú)
settings-analysis-confirm = ¿Activar el análisis de funciones? Pone la cámara varias veces en modo de servicio y, si se solicita, puede escribir en ella. Úselo solo en su propia cámara.
btn-analysis-enable = Activar
settings-analysis-on = El análisis de funciones está activado.
settings-analysis-off = El análisis de funciones está desactivado.
analysis-needs-admin = Cada captura necesita derechos de administrador para el modo de servicio. Windows los pide una vez por sesión del programa; puede iniciar ahora el ayudante de controladores o esperar a que lo pida la primera captura.
analysis-changed-hint = Compruebe: no se encontró ningún candidato. Los siguientes valores cambiaron pero no volvieron a su valor inicial. Si uno de ellos cambia una sola vez entre dos valores, probablemente la cámara no estaba en el primer valor de menú en la primera captura. En ese caso repita el análisis con el valor inicial bien ajustado.
analysis-changed-info = Info: los valores que cambiaron por sí solos durante el análisis (p. ej. contadores, reloj) se ignoran. No tiene que hacer nada.
elevation-admin = Administrador
elevation-standard = Derechos estándar
elevation-admin-hint = El programa se ejecuta con derechos de administrador y puede asignar el controlador que necesita el modo de servicio.
elevation-standard-hint = El programa se ejecuta con derechos estándar. Para el modo de servicio, los ajustes y el análisis de funciones inicia cuando hace falta un ayudante de controladores con derechos de administrador; Windows pregunta una vez por sesión. Haga clic para iniciarlo.
driver-helper-declined = Se rechazó la solicitud de derechos de administrador. Sin ellos no se puede asignar el controlador para el modo de servicio; no se ha cambiado nada.
btn-remove-foreign-drivers = Eliminar controladores antiguos
btn-not-now = Ahora no
btn-ok = Aceptar
foreign-drivers-found = Windows aún conserva controladores que Zadig u otra herramienta similar instaló para dispositivos Sony: { $packages }. Windows les asigna la cámara antes de que este programa pueda asignar su propio controlador, por lo que el modo de servicio falla y es posible que la cámara no aparezca como unidad. ¿Eliminarlos ahora? Después la cámara usará los controladores incluidos en Windows. Para eliminarlos se necesitan una vez permisos de administrador.
foreign-drivers-removed = Se han eliminado los controladores antiguos. Desconecte la cámara y vuelva a conectarla.
foreign-drivers-remaining = No se pudieron eliminar estos controladores: { $packages }. Reinicie Windows e inténtelo de nuevo, o desinstale el dispositivo junto con su controlador en el Administrador de dispositivos.
foreign-drivers-hint = Siguen instalados controladores que Zadig u otra herramienta similar instaló para dispositivos Sony ({ $packages }). Windows les asigna la cámara antes de que este programa pueda asignar su propio controlador, lo que puede hacer fallar el modo de servicio. Elimínelos con: pmca-console driver zadig --remove
driver-zadig-none = No hay controladores de Zadig ni de herramientas similares instalados para dispositivos Sony.
driver-zadig-entry = { $package } ({ $provider }): { $devices }
driver-zadig-remove-hint = Elimínelos con: pmca-console driver zadig --remove
driver-zadig-removed = Se han eliminado { $count } paquete(s) de controlador antiguo(s). Desconecte la cámara y vuelva a conectarla.
elevation-helper = Ayudante de controladores activo
elevation-helper-hint = Un ayudante con derechos de administrador asigna el controlador para el modo de servicio. No tiene ventana y termina con el programa; no habrá más solicitudes.
elevation-helper-confirm = ¿Iniciar ahora el ayudante de controladores? Windows pide derechos de administrador una vez y no vuelve a hacerlo hasta que se cierre el programa.
analysis-cross-heading = Comprobación en otra cámara
analysis-cross-intro = Opcional: comprueba si otra cámara también tiene el valor encontrado y qué está ajustado allí. Solo lee. El mismo número no significa necesariamente la misma función; solo un vistazo al menú de la otra cámara lo confirma. El resultado se añade al mismo informe de análisis.
analysis-cross-do = 1. Desconecte la cámara analizada. 2. Encienda la otra cámara, ponga su modo USB en «Auto», conecte el cable y elija «Almacenamiento masivo (MSC)».
btn-analysis-cross = Comprobar otra cámara
analysis-cross-present = presente, valor { $value }
analysis-cross-match = corresponde a «{ $label }»
analysis-cross-nomatch = no corresponde a ningún valor de menú medido
analysis-cross-absent = no presente
analysis-cross-same-camera = Esta es la propia cámara analizada. Conecte la otra cámara.
analysis-cross-not-now = La comprobación solo es posible cuando se han leído todas las capturas y no queda ningún valor escrito por restaurar.
analysis-cross-none = No hay ningún candidato que comprobar.
analysis-wrong-camera = La cámara conectada no es la analizada. Conecte la { $model }; no se ha escrito nada.
keys-fetching = Descargando la fuente…
keys-scanning = Buscando el material de claves…
keys-complete = Todo el material de claves está presente.
keys-incomplete = Todavía falta parte del material de claves.
keys-present = presente
keys-missing = ausente
keys-class-senser = Claves del modo de servicio (consola de servicio, ajustes, análisis)
keys-class-cic = Clave de suma de comprobación XPD (instalación de apps)
keys-class-spk = Material de claves SPK (instalación de apps, apk2spk)
keys-class-fdat = Material de claves FDAT (firmware de updater-shell)
plugin-heading = Complementos
plugin-intro = Los complementos amplían este programa con idiomas, interruptores para ajustes de la cámara, datos de compatibilidad, herramientas y diagnósticos. Solo proceden de archivos que usted elige: no existe ninguna tienda de complementos. Su código se ejecuta en un entorno aislado, sin acceso a sus archivos, a la red ni al material de claves. Instale solo complementos cuyo autor le merezca confianza; los cambios surten efecto en el siguiente inicio.
plugin-none = No hay complementos instalados.
plugin-inactive = (activo tras reiniciar)
plugin-author = Autor
plugin-restart = Reinicie el programa para que el cambio surta efecto.
btn-plugin-install = Instalar complemento…
plugin-install-confirm = Este complemento puede:
btn-confirm-plugin-install = Instalar complemento
plugin-installed = El complemento se ha instalado.
plugin-remove-confirm = ¿Quitar el complemento «{ $name }»?
plugin-removed = Se ha quitado el complemento { $id }.
plugin-perm-languages = añadir idiomas al programa
plugin-perm-camera-settings = cambiar ajustes de la cámara (solo entre los dos valores que indica y únicamente tras su confirmación)
plugin-perm-compatibility = añadir entradas al resumen de compatibilidad
plugin-perm-code = ejecutar código propio en el entorno aislado (herramientas o diagnósticos)
plugin-perm-host-os = consultar qué sistema operativo es este
plugin-perm-host-cameras = ver qué cámaras Sony están conectadas y en qué modo
plugin-invalid = Esto no es un complemento utilizable: { $reason }
plugin-no-folder = No se pudo determinar la carpeta de datos del usuario, por lo que no se pueden instalar complementos.
plugin-not-installed = No hay ningún complemento instalado con el identificador { $id }.
plugin-failed = El complemento ha fallado: { $reason }
plugin-unknown-entry = Ningún complemento cargado ofrece { $id }.
plugin-run-done = El complemento ha terminado.
plugin-run-warning = El complemento ha terminado con observaciones.
plugin-run-error = El complemento ha informado de un error.
plugin-tools-heading = Herramientas de complementos
plugin-tools-hint = Proceden de complementos instalados y se ejecutan en un entorno aislado. Un complemento solo recibe lo que usted le entrega y lo que ha declarado, y no puede modificar la cámara.
plugin-diagnostics-heading = Diagnósticos de complementos
plugin-compat-heading = De un complemento
plugin-col-note = Nota
file-filter-plugin = Archivo de complemento
keys-not-set-up = El material de claves que necesita esta operación no está configurado.
err-missing-keys = El material de claves que necesita esta operación no está configurado.
keys-heading = Se necesita material de claves
keys-intro = Las claves del modo de servicio, la clave de suma de comprobación XPD y el material de claves SPK no forman parte de este programa. Indique la dirección de una fuente que tenga — un repositorio, un archivo comprimido o un fichero — y el programa encontrará e importará las partes que reconozca. No se ejecuta nada y el material se guarda solo en este ordenador.
keys-law-notice = Respete las leyes vigentes donde se encuentre y trabaje solo con cámaras y datos que sean de su propiedad.
keys-url-label = Dirección de la fuente (una por línea)
keys-url-required = Indique al menos una dirección de fuente.
btn-keys-import = Importar
btn-import-keys = Configurar el material de claves
