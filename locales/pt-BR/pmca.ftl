# Localização em português (Brasil) do PMCA-RE RELOADED.
# Traduza apenas o texto à direita do sinal =. Nunca altere os IDs.

app-title = PMCA-RE RELOADED – Kit de ferramentas para câmeras

# Navegação / abas
nav-camera-info = Informações da câmera
nav-install-app = Aplicativos
nav-tweaks = Ajustes
nav-network = Rede
nav-firmware = Firmware
nav-updater = Console do atualizador
nav-service = Console de serviço
nav-tools = Ferramentas de arquivo
nav-settings = Configurações
nav-license = Licença

# Página de boas-vindas: a página com que a janela abre.
nav-welcome = Boas-vindas
welcome-intro-heading = O que este programa faz
welcome-intro-body = Este programa ajuda a preservar as funções da sua câmera Sony e a adaptá-la às suas necessidades. Depois que a Sony encerrou a loja dos PlayMemories Camera Apps, não havia como fazer backup de aplicativos comprados ou já instalados, nem reinstalá-los. Redefinir a câmera apaga os aplicativos instalados de forma irrecuperável. Este programa permite fazer backup dos aplicativos existentes e reinstalá-los, além de poder disponibilizar aplicativos da comunidade livremente disponíveis. Você também pode instalar seus próprios aplicativos (arquivos *.apk). Em caso de problemas, o programa lê as informações da câmera e os registros internos para análise. As configurações que a câmera já possui, mas que seu menu não oferece, podem ser alteradas e desfeitas novamente. O programa fala com a câmera diretamente pelo cabo USB. Os modelos suportados estão listados na página Informações da câmera.
welcome-start-heading = O primeiro passo
welcome-start-body = Ligue primeiro a câmera e, com o cabo ainda fora, defina “Conexão USB” no menu dela como Armazenamento em massa ou MTP. “Auto” costuma funcionar também, porque a câmera então escolhe um dos dois sozinha. A ordem importa: depois que o cabo estiver conectado, o menu da câmera deixa de ser acessível. Só então conecte a câmera por USB, abra Informações da câmera e pressione “Obter informações da câmera”. Assim que o modelo e a versão do firmware aparecerem, a conexão está correta e todas as demais funções deste programa ficam disponíveis.
welcome-start-driver = Se em vez disso aparecer um aviso sobre o driver, siga esse primeiro. No Windows um driver do sistema toma a câmera para si e precisa ser substituído antes que o acesso USB direto de que estas ferramentas precisam seja possível. O programa faz isso sozinho, mas atribuir um driver é um direito que o Windows concede apenas a um administrador. Nesse caso, o programa oferece a você a possibilidade de reiniciá-lo como administrador, e o Windows pedirá que você confirme a operação. A atribuição é desfeita ao fim da sessão. No Linux e no macOS isso não é necessário.
welcome-start-driver-linux = Se em vez disso aparecer um aviso sobre o driver, siga esse primeiro. No Linux nada precisa ser substituído: a câmera é alcançada diretamente. O que é preciso uma única vez é o direito de abrir o dispositivo — uma regra udev para o identificador de fabricante da Sony — e, para o modo de armazenamento em massa, liberar o driver de núcleo usb-storage. Ambos se preparam fora deste programa com direitos de root; o script scripts/linux-prepare.sh no código-fonte faz os dois. Esta janela nunca pede tais direitos por si mesma.
welcome-start-driver-macos = Se aparecer um aviso de driver no lugar, siga-o primeiro. No macOS nada precisa ser instalado ou substituído. Defina a conexão USB da câmera como MTP. Para os ajustes, o shell de serviço, a análise de funções, as atualizações de firmware e o console do atualizador, esta janela pede uma vez sua senha de administrador: eles passam pela conexão de armazenamento em massa da câmera, que o macOS só entrega a um programa assim.
welcome-tweaks-heading = Ajustes ocultos e idiomas
welcome-tweaks-body = Ajustes ocultos são configurações que a câmera já possui mas não oferece em seu menu: o limite de tempo de gravação, o limite de gravação em 4K, a escolha entre PAL e NTSC, o instalador de aplicativos por USB — e os idiomas do menu.
welcome-tweaks-languages-heading = Os idiomas da câmera
welcome-tweaks-languages-body = As câmeras Sony mostram apenas os idiomas de menu comuns em sua região de venda. Quem mora no exterior, por isso, muitas vezes não encontra o próprio idioma e precisa se virar com um idioma estrangeiro. No entanto, outros idiomas já estão no firmware. Este programa os traz para o menu, para que você possa operar sua câmera em um idioma familiar.
welcome-tweaks-ui-heading = O idioma deste programa
welcome-tweaks-ui-body = Separado de tudo isso está o seletor na parte superior. Ele muda apenas o idioma deste programa e nunca toca na câmera.
welcome-tweaks-safety = Cada ajuste é gravado na memória de backup da câmera, e todos podem ser desfeitos com o mesmo interruptor. Uma gravação interrompida pode danificar a câmera de forma permanente: não desconecte o cabo nem retire a bateria enquanto uma estiver em andamento.
welcome-privacy-heading = Seus dados
welcome-privacy-where = A página “Licença” expõe tudo isso novamente por inteiro, junto com as licenças de todas as bibliotecas com que este programa é construído.
welcome-where-heading = Onde encontrar o quê
welcome-where-info = Modelo, número de série e versão do firmware. É também o teste de conexão.
welcome-where-install = Aplicativos da lista da comunidade, ou um arquivo .apk seu.
welcome-where-tweaks = As configurações que o menu da câmera não alcança, entre elas os idiomas.
welcome-where-network = Dados de assistência GPS, e os pontos de acesso e serviços usados para transmissão ao vivo.
welcome-where-firmware = Gravar uma imagem de firmware da Sony na câmera.
welcome-where-tools = Desempacotar e inspecionar arquivos de firmware e de pacote. Não precisa de câmera.
welcome-where-shells = Acesso direto à memória e ao sistema de arquivos da câmera. Antes é preciso exibi-lo em Configurações.
welcome-where-settings = Tema, modo avançado e edição da tradução deste programa.
welcome-where-license = De onde vem esta portação, sobre o trabalho de quem ela se apóia e sob quais termos é publicada.
welcome-where-hint = Nenhuma página age sobre a câmera por conta própria. Nada é lido ou gravado até que você pressione um botão que anuncie isso.

# Botões
btn-get-info = Obter informações da câmera
btn-refresh = Atualizar
btn-load-apps = Carregar aplicativos disponíveis
btn-list-tweaks = Mostrar ajustes disponíveis
btn-install = Instalar o aplicativo selecionado
btn-install-file = Instalar arquivo .apk…
btn-apply = Aplicar
btn-save = Salvar na câmera
btn-add = Adicionar
btn-remove = Remover
btn-reveal = Mostrar
btn-update-gps = Transferir os dados de assistência GPS para o dispositivo
btn-cancel = Cancelar
btn-browse = Procurar…
btn-switch-msc = Mudar para o modo de armazenamento em massa
btn-start-driver-helper = Iniciar auxiliar de driver

# Filtros de tipo de arquivo das caixas de diálogo
file-filter-apk = Aplicativo Android (*.apk)
file-filter-firmware = Imagem de firmware Sony (*.dat)
file-filter-usage-log = Registro de uso (*.bin)
file-filter-catalog = Arquivo de tradução (*.ftl)
file-filter-spk = Pacote de aplicativo Sony (*.spk)
file-filter-any = Todos os arquivos
file-filter-gps = Dados de assistência GPS (*.dat)

# Status / mensagens
msg-connecting = Conectando…
msg-usage-log-written = Registro de uso gravado em { $path }
msg-apps-found = { $count } aplicativos encontrados
msg-switch-mode = Mude a câmera para o modo MTP ou de armazenamento em massa.
msg-no-tweaks = Não há ajustes disponíveis para esta câmera.
msg-done = Pronto.

# Troca de modo USB da câmera (MTP → armazenamento em massa). Oferecida abaixo
# de um erro de "modo incorreto"; não existe comando USB para o sentido oposto.
mode-switching = Pedindo à câmera para mudar para o modo de armazenamento em massa…
mode-switch-wait = Aguardando a câmera se reconectar…
camera-confirm-hint = Mensagens na câmera (p. ex., sobre www.sony.net/pm) podem ser fechadas; elas não atrapalham o processo.
mode-switch-done = A câmera está agora no modo de armazenamento em massa.
mode-switch-timeout = A câmera não reapareceu no modo de armazenamento em massa. Desligue-a e ligue-a novamente e tente outra vez.
mode-switch-impossible = Este modo não pode ser alterado por USB. Desligue e ligue a câmera novamente.
mode-switch-hint = Somente uma câmera no modo MTP pode ser alternada por USB. A mudança é temporária: o modo USB escolhido no próprio menu da câmera permanece inalterado, portanto desligar e ligar a câmera — ou desconectar e reconectar o cabo — restaura o modo anterior. Não existe comando USB para o caminho de volta.

# Campos de informações da câmera
info-model = Modelo
info-product-code = Código do produto
info-serial = Número de série
info-firmware = Versão do firmware
info-lens = Lente
info-region = Região
info-mode = Modo
mode-mass-storage = Armazenamento em massa
mode-mtp = MTP
mode-updater = Modo atualizador
mode-app-install = Modo de instalação de aplicativos
mode-service = Modo de serviço
mode-unknown = Desconhecido
info-android-version = Versão da plataforma Android
info-photographer = Fotógrafo
info-copyright = Copyright
info-photographer-1 = Fotógrafo (entrada 1)
info-photographer-2 = Fotógrafo (entrada 2)
info-copyright-1 = Copyright (entrada 1)
info-copyright-2 = Copyright (entrada 2)
diagnose-services-heading = A câmera não é alcançada?
diagnose-services-hint = Alguns serviços do Windows podem ocupar uma câmera no modo MTP, de modo que este aplicativo não consegue alcançá-la. A verificação apenas pergunta ao Windows quais serviços estão em execução; ela não altera nada.
btn-diagnose-services = Verificar serviços do Windows
diagnose-services-none = Nenhum serviço conflitante conhecido está em execução.
diagnose-services-found = Estes serviços em execução podem interferir no acesso à câmera:
diagnose-services-advice = Pare o serviço em Serviços (services.msc) ou com o comando a seguir como administrador e depois reconecte a câmera:

# Atualizador de firmware / console
updater-model = Modelo da câmera
updater-firmware-current = Versão atual do firmware
updater-guard-ok = A imagem de firmware corresponde a esta câmera.
updater-no-fdat = Não há firmware de atualizador incluído para o modelo { $model }.
updater-model-required = A câmera já está no modo atualizador e ali não consegue informar seu modelo. Desligue a câmera e ligue-a novamente para que ela inicie no modo normal e tente de novo. Como alternativa, indique um arquivo de carga útil (--fdat-file) ou, no console, o modelo (--model <MODELO>).
updater-already-updater = A câmera já está no modo atualizador.
updater-flash-required = A câmera não será modificada sem confirmação. Execute novamente com --flash para mudar para o modo atualizador e gravar a carga útil.
updater-flashing = Gravando a carga útil do atualizador na câmera…
updater-switch-wait = Siga as instruções na tela da câmera. Aguardando a câmera entrar no modo atualizador…
updater-switch-timeout = Tempo de espera pela câmera esgotado. Execute o comando novamente depois que ela se reconectar.
updater-shell-ready = A carga útil do atualizador está em execução. Digite "help" para ver os comandos e "exit" para sair.
updater-shell-done = Sessão do console encerrada.
updater-session-panic = A sessão do atualizador terminou de forma inesperada. Desligue e ligue a câmera antes de tentar de novo.

# O que o console do atualizador responde. As falhas ao interpretar uma linha
# estão escritas uma única vez, nas chaves service-*: os dois consoles usam o
# mesmo interpretador, então um caminho entre aspas e um valor hexadecimal
# errado falham igual nos dois.
updater-backup-synced = Backup gravado na memória flash.
updater-backup-locked = As configurações protegidas do backup estão bloqueadas agora.
updater-backup-writable = As configurações protegidas do backup agora aceitam gravação.
updater-shell-console-only = O console root precisa de um terminal e só existe na linha de comando: execute pmca-console updatershell --flash e digite “shell” ali.

# A página do console do atualizador.
updater-warning = Atenção: para chegar a este console a câmera reinicia em modo atualizador e uma carga útil é gravada nela — um pequeno programa deste projeto que a câmera aceita como uma atualização de firmware e que depois responde aos comandos deste console. Essa carga útil desaparece assim que a câmera é desligada, mas os comandos que ela oferece escrevem na memória de configurações permanente da câmera, e uma gravação interrompida pode danificá-la para sempre. Além disso, todas as configurações da câmera são redefinidas no processo — ao contrário do console de serviço.
updater-console-heading = Executar o console do atualizador na linha de comando
updater-driver-hint = O console do atualizador precisa que a câmera fique no driver de armazenamento em massa do próprio Windows, o USBSTOR, porque é por ele que a carga útil é transmitida. É o oposto do que o console de serviço exige. O aplicativo atribui e libera o driver WinUSB sozinho, então não há nada a preparar à mão — mas um driver atribuído por outra ferramenta precisa ser desfeito antes. Uma câmera em modo atualizador se apresenta com um identificador de produto próprio e também não deve ser reassociada.
updater-fdat-label = Arquivo de carga útil (opcional)
updater-confirm = A câmera vai reiniciar em modo atualizador e uma carga útil será gravada nela. Continuar?
updater-cancelled = Cancelado. A câmera não foi alterada.

btn-shell-primer = O que saber sobre este shell
btn-shell-primer-hide = Ocultar a explicação
shell-primer-compare-heading = Dois shells, dois protocolos
shell-primer-compare = As duas páginas oferecem uma linha de comando para a câmera, mas não são o mesmo shell. O shell de serviço fala o protocolo de serviço de fábrica da própria Sony, ao qual a câmera responde sozinha depois da autenticação; nada é gravado na câmera para chegar até lá. O shell do atualizador só existe porque este projeto grava um programa próprio na câmera: ela reinicia em modo atualizador, aceita a carga útil como se fosse um firmware, e é essa carga útil que responde aos comandos listados abaixo. Só um dos dois pode estar aberto por vez.
shell-primer-payload = “Carga útil” (em inglês, “payload”) designa o conteúdo propriamente dito que uma via de transporte leva. No shell do atualizador, é um pequeno programa deste projeto: a câmera o aceita pelo mesmo caminho de uma atualização de firmware, mas só o mantém até ser desligada. O shell de serviço não precisa de carga útil, porque a câmera responde aos seus comandos com o próprio firmware. Nos arquivos de firmware, o termo significa o mesmo: a imagem de firmware propriamente dita dentro do contêiner do arquivo .dat.
shell-primer-when-service = O shell de serviço serve para tudo que lê ou altera os ajustes guardados da câmera enquanto ela continua rodando o próprio firmware: ler o bloco HASP, salvar uma cópia de recuperação da área de backup, mudar entradas isoladas, tirar arquivos da câmera. É também o que a página Ajustes usa por baixo; tudo o que se faz lá pode ser feito aqui à mão.
shell-primer-when-updater = O shell do atualizador é a escolha quando não há outro jeito de alcançar a câmera, ou quando você precisa de um shell no sistema em funcionamento em vez de comandos isolados. É o único caminho que dá um shell root de verdade e ainda funciona numa câmera que recusa o modo de serviço. Custa uma reinicialização, uma gravação e as configurações da câmera, então é a segunda opção, não a primeira.
shell-primer-driver-heading = Drivers: cada um exige o contrário
shell-primer-driver = O shell de serviço fala USB bruto e exige que no Windows a câmera esteja ligada ao WinUSB. O aplicativo atribui esse driver sozinho ao entrar no modo de serviço e o libera ao fim da sessão; nenhuma ferramenta separada participa. O shell do atualizador chega pelo USBSTOR, o driver de armazenamento em massa do próprio Windows, e para de funcionar enquanto a câmera estiver ligada ao WinUSB — por isso liberar a atribuição faz parte de encerrar uma sessão de serviço. Atribuir um driver exige direitos de administrador. No Linux nenhuma troca de driver é necessária; só o acesso ao nó do dispositivo.
shell-primer-risk-heading = O que pode dar errado
shell-primer-risk = Os comandos marcados abaixo gravam na memória de ajustes permanente da câmera. Uma gravação interrompida pode deixar a câmera sem ligar, e nada disso se desfaz — é exatamente para isso que existe o bk d: salve uma cópia da área de backup antes de mudar qualquer coisa. A carga útil do atualizador em si não fica na câmera: desligá-la a descarta. A passagem pelo modo atualizador, porém, redefine todas as configurações da câmera.
shell-primer-commands-heading = Comandos deste shell
shell-primer-writes = Comandos marcados com (WRITES) alteram a câmera. Todos os outros apenas leem.
shell-cmd-hasp = Lê o bloco HASP, a área de identidade protegida da câmera.
shell-cmd-pull = Traz um arquivo da câmera. Sem nome local, o nome remoto é mantido.
shell-cmd-push = Copia um arquivo local para a câmera, substituindo o que estiver lá. (WRITES)
shell-cmd-rm = Apaga um arquivo na câmera. (WRITES)
shell-cmd-rmem = Grava uma região da memória do dispositivo em um arquivo local. Deslocamento e comprimento são hexadecimais.
shell-cmd-bk-r = Lê uma entrada da área de backup e a mostra como despejo hexadecimal.
shell-cmd-bk-d = Salva toda a área de backup em um arquivo. Faça isso antes de mudar qualquer coisa: é o único caminho de volta.
shell-cmd-bk-dump-txt = Salva toda a área de backup como <file>.bin e uma listagem legível como <file>.txt.
shell-cmd-bk-stat = Mostra o bloco de status do backup da câmera, incluindo a região.
shell-cmd-bk-w = Grava uma entrada da área de backup, dada como bytes hexadecimais separados por espaços. (WRITES)
shell-cmd-bk-s-sub = Fixa na memória flash a área de backup de um subsistema, para que a mudança sobreviva a uma reinicialização. (WRITES)
shell-cmd-bk-s = Fixa a área de backup na memória flash, para que a mudança sobreviva a uma reinicialização. (WRITES)
shell-cmd-bk-lock = Ativa ou desativa a proteção contra gravação das entradas de backup protegidas; alguns ajustes se recusam a mudar enquanto ela está ativa. (WRITES)
shell-cmd-term = Liga ou desliga o terminal de depuração da câmera. (WRITES)
shell-cmd-info = Lê o bloco de status do backup, que diz se a área pode sequer ser gravada.
shell-cmd-shell = Abre um shell root na câmera. Precisa de um terminal, então só está disponível no console; shell tcp <port> o oferece em 127.0.0.1 para outro cliente. (WRITES)
shell-cmd-help = Lista os comandos que este shell aceita.
shell-cmd-exit = Encerra a sessão e sai do shell.
shell-primer-diagnostics-heading = Diagnóstico e reparo
shell-cmd-bk-trace = Grava em um arquivo, pacote por pacote, como a câmera envia toda a área de backup por USB. Serve para examinar backups grandes; pode deixar a câmera travada no meio da resposta. Nesse caso, desconecte o cabo e desligue e ligue a câmera, retirando a bateria se necessário.
shell-cmd-bk-rewrite = Lê a área de backup, grava-a de volta sem alterações e a lê novamente. Mostra se a câmera aceita uma imagem completa sem alterar nenhum ajuste. (WRITES)
shell-cmd-bk-restore-attrs = Restaura os atributos de todas as entradas de backup, como a proteção contra gravação, conforme uma cópia salva com bk d ou bk dump-txt. Os valores não mudam. Use apenas uma cópia da mesma câmera. (WRITES)
shell-primer-diagnostics-env = Três variáveis de ambiente afetam o console e a interface gráfica quando definidas antes de iniciar, por exemplo no PowerShell com $env:PMCA_TWEAKS_DIRECT = "1". PMCA_TWEAKS_DIRECT faz os tweaks gravarem valores individuais mesmo quando o backup tem menos de 1 MB. PMCA_TWEAKS_IMAGE faz com que gravem a imagem completa mesmo quando ela tem mais de 1 MB; se ambas estiverem definidas, vale PMCA_TWEAKS_DIRECT. PMCA_TLS_DUMP=<arquivo> salva a primeira mensagem TLS da câmera (ClientHello) durante a instalação de um app, para examinar uma conexão que falhou.

# Instalação de aplicativos
install-switching = Mudando para o modo de instalação de aplicativos…
install-disabled = A câmera recusou entrar no modo de instalação de aplicativos. Normalmente a causa é a configuração "Ativar instalador de aplicativos por USB": ela precisa estar ligada em "Ajustes".
install-stays-app-mode = A câmera permanece no modo de instalação de aplicativos, no qual outros aplicativos podem ser instalados. Para todas as outras funções, desconecte o cabo USB e desligue e ligue a câmera novamente.
err-usb-app-install-mode = A câmera ainda está no modo de instalação de aplicativos, no qual só aceita instalações de aplicativos. Desconecte o cabo USB e depois desligue e ligue a câmera novamente — se reiniciada ainda conectada, ela volta direto para esse modo.
install-switch-wait = Aguardando a câmera se reconectar no modo de instalação de aplicativos…
install-switch-timeout = Tempo esgotado aguardando a câmera entrar no modo de instalação de aplicativos. Execute o comando novamente depois que ela se reconectar.
install-bad-mode = A câmera não pode ser usada neste modo. Mude-a para o modo MTP ou de armazenamento em massa.
install-starting = Iniciando a instalação…
install-error = Erro de comunicação durante a instalação
install-server-failed = O servidor de instalação local não aceitou a conexão da câmera:
install-done = Tarefa concluída com sucesso.

# Análise do APK
apk-package = Pacote:
apk-unreadable = Aviso: não foi possível analisar o arquivo .apk.
apk-incompatible = Aviso: este aplicativo declara um nível mínimo de SDK que a câmera talvez não ofereça.
apk-unsigned = Aviso: o arquivo .apk não possui assinatura.

# Loja de aplicativos da comunidade
store-loading = Carregando a lista de aplicativos…
store-resolving = Procurando a versão mais recente…
store-downloading = Baixando
store-unknown-app = Aplicativo desconhecido
store-no-release = Não há versão disponível para download deste aplicativo
store-not-installable = sem versão
msg-no-apps = Nenhum aplicativo encontrado.
store-heading = Loja de aplicativos da comunidade
store-discontinued = A Sony encerrou a loja oficial PlayMemories Camera Apps; seus servidores não respondem mais, então não é mais possível comprar nem baixar aplicativos dela.
store-use-own-apk = Você ainda pode instalar seu próprio .apk: escolha o arquivo abaixo ou arraste-o para esta janela. A loja da comunidade, mais abaixo, também continua funcionando.
btn-store-sync = Verificar a câmera
btn-install-selected = Instalar selecionados
btn-store-stop = Parar após este aplicativo
btn-store-stopping = Parando após este aplicativo…
btn-store-retry = Tentar novamente os que falharam
btn-continue = Continuar
btn-reinstall = Instalar novamente
btn-skip = Pular
btn-confirm-updater = Reiniciar e gravar
btn-confirm-tweaks = Gravar ajustes
btn-confirm-languages = Ativar idiomas
btn-confirm-wifi = Substituir pontos de acesso
btn-confirm-stream = Substituir configurações
btn-confirm-show-shells = Exibir consoles
btn-confirm-discard = Descartar tradução
store-select-all = Selecionar todos os aplicativos
store-col-app = Aplicativo
store-col-version = Versão
store-col-author = Autor
store-col-camera = Versão na câmera
store-col-status = Status
store-camera-absent = não instalado
store-version-unknown = desconhecida
store-batch-query = Lendo os aplicativos instalados na câmera…
store-sync-done = Câmera verificada. Aplicativos da lista instalados: { $count }
store-batch-downloading = Baixando o aplicativo { $current } de { $total }…
store-batch-installing = Aplicativo { $current } de { $total }: instalando { $name }…
store-reinstall-confirm = { $name } já está instalado na câmera. Versão instalada: { $installed }. Versão na loja de aplicativos: { $store }. Deseja instalá-lo novamente?
store-state-queued = aguardando
store-state-installed = instalado
store-state-failed = falhou
store-state-skipped = ignorado
store-state-not-run = não executado
store-batch-summary = Instalados: { $done } · com falha: { $failed } · ignorados: { $skipped } · não executados: { $notrun }

# Modo de serviço (senser)
usb-mode-advice = Para ajustes, backups e o shell de serviço, defina o modo de conexão USB da câmera como “Auto”, e não fixo em armazenamento em massa. Quando a câmera perguntar o modo após uma sessão, escolha “Armazenamento em massa (MSC)”. Com o armazenamento em massa fixo, a câmera frequentemente recusa a próxima mudança para o modo de serviço.
service-switch-refused = A câmera não concluiu a mudança para o modo de serviço nem se reconectou via USB; nesse caso ela costuma mostrar “Verifique o dispositivo conectado”. Desconecte o cabo USB, conecte-o novamente e tente outra vez.
service-switching = Mudando para o modo de serviço…
service-switch-wait = Aguardando a câmera se reconectar no modo de serviço…
service-switch-timeout = A câmera não apareceu no modo de serviço. Ela é reconhecida como um dispositivo USB separado (id de produto 0336), que precisa da própria atribuição do driver WinUSB; verifique no gerenciador de dispositivos se há um dispositivo Sony sem driver. Desligue e ligue a câmera para retorná-la ao modo normal.
service-authenticating = Autenticando…
# Exibido quando a câmera ainda está em modo de serviço por causa de uma
# execução anterior, e quando isso não pôde ser desfeito.
service-stranded-reset = A câmera ainda está em modo de serviço de uma execução anterior. Encerrando essa sessão primeiro…
service-stranded-failed = A câmera está presa em modo de serviço por causa de uma execução anterior e não respondeu ao pedido para sair dele. Desligue e ligue a câmera novamente e tente outra vez.
service-bad-mode = A câmera não pode ser usada neste modo. Mude-a para o modo de armazenamento em massa.
service-driver-required = O acesso USB direto foi negado e o driver WinUSB não pôde ser atribuído. Atribuir um driver exige direitos de administrador.
service-warning = Aviso: o modo de serviço concede acesso direto à memória, ao sistema de arquivos e aos dados de calibração da câmera. Comandos de gravação podem danificá-la permanentemente.
service-shell-ready = Console de serviço pronto. Digite "help" para ver os comandos e "exit" para sair.
service-shell-done = Sessão do console de serviço encerrada.
# Informado na inicialização quando uma execução anterior foi interrompida
# antes de liberar o driver que havia atribuído.
driver-swept = Foram removidas { $count } atribuição(ões) de driver deixadas por uma execução anterior.
# Informado na inicialização quando essa mesma execução interrompida também
# deixou a câmera em modo de serviço, onde ela recusaria a próxima sessão.
driver-swept-camera = Uma câmera deixada em modo de serviço por uma execução anterior foi devolvida ao seu modo normal.
driver-release-replug = O Windows está aguardando a câmera enquanto libera o driver. Desconecte o cabo USB e conecte-o novamente para que a limpeza possa terminar.
# Resultados do comando `driver`.
driver-bound = { $id } agora usa o WinUSB (pacote de drivers { $package }).
driver-released = { $count } atribuição(ões) de driver liberada(s); a câmera voltou a usar os drivers que acompanham o Windows.
driver-needs-elevation = Atribuir um driver exige direitos de administrador. Execute este comando em um console iniciado como administrador.
driver-leftover-needs-elevation = Uma execução anterior deixou um driver atribuído à câmera. Enquanto ele existir, a câmera pede para verificar o dispositivo conectado e o armazenamento em massa não funciona. Liberá-lo requer direitos de administrador; confirme a solicitação do Windows.
driver-binding = Atribuindo o driver WinUSB à câmera…
driver-bind-wait = Aguardando a câmera reconectar com o novo driver…
driver-bind-timeout = A câmera não reapareceu após a atribuição do driver. Desconecte-a, conecte-a novamente e tente de novo.
driver-bind-failed = Não foi possível atribuir o driver WinUSB: { $detail }
# Respostas do console de serviço. Elas vêm do backend, para que o console e a
# janela usem as mesmas palavras.
service-usage = Uso: { $usage }
service-bad-hex = Não é um valor hexadecimal: { $value }
service-unknown-command = Comando desconhecido: { $command }. Digite 'help' para ver a lista.
service-unterminated-quote = A linha termina dentro de um argumento entre aspas.
service-help = Lista de comandos:
service-read-bytes = { $count } bytes lidos.
service-pulled = { $remote } copiado da câmera para { $local }.
service-pushed = { $local } copiado para { $remote } na câmera.
service-removed = { $remote } excluído da câmera.
service-dumped = { $size } bytes extraídos de { $offset } para { $local }.
service-backup-saved = Imagem de backup ({ $count } bytes) salva em { $local }.
service-backup-text-saved = Imagem de backup ({ $count } bytes) salva em { $bin } e listagem em { $txt }.
service-backup-status = Status do backup, região: { $region }
service-backup-rewritten = Imagem ({ $count } bytes) regravada sem alterações e lida novamente; não foi salva de forma permanente.
service-backup-attrs-restored = Atributos de { $count } entradas restaurados conforme { $local }.
service-backup-written = Entrada de backup { $id } gravada.
service-backup-persisted = Backup do subsistema { $id } gravado na memória flash.
service-terminal-on = O terminal de depuração está ativado.
service-terminal-off = O terminal de depuração está desativado.
btn-service-terminal-open = Abrir console serial
btn-service-terminal-close = Fechar console serial
service-terminal-heading = Console serial
service-terminal-hint = Em câmeras Linux, o terminal de depuração é um shell root por USB. Abra-o e digite comandos como em um terminal.
service-terminal-label = Console
service-terminal-interactive = Console serial aberto. Digite comandos; digite "exit" para voltar ao shell de serviço.

# A página de serviço sem o modo avançado: como chegar ao console de serviço
# pela linha de comando e o que precisa estar preparado.
service-console-heading = Executar o console de serviço na linha de comando
service-console-hint = Abra um terminal na pasta que contém o pmca-console e execute este comando:
service-driver-heading = Antes de começar
service-driver-hint = O modo de serviço precisa de acesso USB direto, que o Windows não concede pelo driver de armazenamento em massa. O aplicativo atribui o driver WinUSB sozinho — duas vezes, uma enquanto a câmera está no modo de armazenamento em massa e outra depois que ela reaparecer no modo de serviço com um segundo identificador de produto — e libera as duas atribuições ao fim da sessão. Atribuir um driver exige direitos de administrador, então inicie o console como administrador; nenhuma outra ferramenta é necessária.
driver-note-linux = Aqui não é preciso instalar nada: a libusb alcança a câmera diretamente, sem deslocar nenhum driver do núcleo. Só são necessários a permissão para abrir o dispositivo e uma área de trabalho que não o esteja segurando. Instale uma regra udev para o identificador de fabricante da Sony 054c e desvincule o driver usb-storage antes de usar o modo de armazenamento em massa. O script scripts/linux-prepare.sh faz as duas coisas e também encerra os monitores de volumes do gvfs.
driver-note-macos = Nada precisa ser instalado para o acesso USB direto. Defina a conexão USB da câmera como MTP. No modo MTP o programa encerra o serviço de câmera do macOS ptpcamerad se ele ocupar a câmera; o sistema o reinicia quando necessário. Os ajustes, o shell de serviço, a análise de funções, as atualizações de firmware e o console do atualizador passam pela conexão de armazenamento em massa, que o macOS só entrega a um programa com direitos de administrador: a janela pede sua senha uma vez por sessão, a linha de comando precisa de sudo. Esta plataforma é a menos testada das três; conte mais com relatar um problema do que encontrá-lo já descrito.
service-advanced-hint = O console também pode ser exibido aqui, em Configurações, Modo avançado.
btn-copy-command = Copiar comando
msg-command-copied = Comando copiado para a área de transferência.

# A página de serviço com o modo avançado ativado.
btn-service-open = Conectar
btn-service-close = Desconectar
btn-service-run = Executar
btn-service-browse = Inserir caminho do arquivo…
shell-path-quote = Não é possível inserir este caminho: ele contém aspas duplas, o carácter com que a linha de comandos delimita o início e o fim de um argumento. Renomeie o ficheiro ou mova-o para um caminho sem esse carácter.
service-command-label = Comando
service-not-connected = Não conectado.

# Textos exclusivos da interface gráfica
install-apk-path = Caminho para um arquivo .apk local (opcional)
install-drop-hint = Solte um arquivo .apk aqui

# Descrições dos ajustes
tweak-rec-limit = Gravar vídeo sem limite de tempo
tweak-rec-limit-4k = Gravar vídeo 4K sem limite de tempo
tweak-language = Exibir todos os idiomas do menu
tweak-language-note = A câmera mostra apenas os idiomas que o firmware dela conhece, então podem aparecer menos do que os ativados aqui. Uma DSC-RX1RM2, por exemplo, mostra 25 dos 35.
tweak-pal-ntsc = Ativar o seletor PAL / NTSC e o aviso
tweak-pal-ntsc-mode = Sistema de vídeo (PAL / NTSC)
tweak-pal-ntsc-pal = PAL
tweak-pal-ntsc-ntsc = NTSC
tweak-pal-ntsc-note = Depois de mudar o sistema de vídeo, a câmera exige a formatação do cartão de memória. Faça antes uma cópia de tudo: fotos, vídeos, LUTs. O mais simples é retirar o cartão antes de trocar.
tweaks-set-malformed = Esperado --set ID=VALOR, recebido: { $arg }
tweaks-set-unknown-value = Valor desconhecido "{ $value }". Esperado um de: { $options }
tweaks-set-not-a-choice = { $id } é um interruptor, não uma escolha. Use --enable ou --disable.
tweaks-needs-set = { $id } é uma escolha, não um interruptor. Use --set { $id }=VALOR com um de: { $options }
tweak-usb-installer = Ativar o instalador de aplicativos por USB

# Fluxo dos ajustes
tweaks-warning = Aviso: aplicar ajustes regrava a memória de backup da câmera. Uma gravação interrompida pode danificá-la permanentemente.
tweaks-reading = Lendo as configurações da câmera…
tweaks-writing = Gravando as configurações da câmera…
tweaks-done = Configurações da câmera atualizadas.
tweaks-write-failed = A câmera não salvou as novas configurações como foram enviadas. Parte delas pode ter sido gravada mesmo assim: leia as configurações novamente e desfaça-as se necessário. Um backup feito antes da alteração é o caminho seguro de volta.
tweaks-confirm-required = Nada será gravado sem confirmação. Execute novamente com --confirm depois de ler o aviso acima.
tweaks-state-enabled = ativado
tweaks-state-disabled = desativado
tweaks-full-read-failed = A câmera não entregou o backup completo ({ $detail }). Depois disso ela costuma parar de responder: desligue-a, retire a bateria por alguns segundos, recoloque-a e conecte novamente.
tweaks-direct-mode = O backup desta câmera é maior que 1 MB, por isso as configurações são lidas e gravadas uma a uma. Os valores antigos são salvos antes de cada alteração e podem ser restaurados com “Desfazer alterações”.
tweaks-direct-forced = PMCA_TWEAKS_DIRECT força a via de valores individuais, por isso as configurações são lidas e gravadas uma a uma. Os valores antigos são salvos antes de cada alteração e podem ser restaurados com “Desfazer alterações”.
tweaks-direct-refused = A câmera recusou a alteração ({ $detail }). Os valores já gravados foram restaurados; nada foi alterado. Alguns modelos, sobretudo japoneses, só permitem esta configuração pelo backup completo.
tweaks-direct-revert-failed = A câmera recusou a alteração ({ $detail }) e nem todos os valores já gravados puderam ser restaurados. Os valores antigos estão salvos: tente “Desfazer alterações”.
tweaks-direct-restart = Configurações da câmera atualizadas. Desligue e ligue a câmera novamente para que a alteração tenha efeito.
tweaks-rollback-unsaved = Não foi possível salvar os valores atuais para desfazer; nada foi alterado.
tweaks-image-saved = A imagem do backup anterior à alteração foi salva: { $path }
tweaks-whole-image-hint = Para gravar a alteração pela imagem completa do backup, execute o mesmo comando novamente com --whole-image. A imagem anterior à alteração é salva antes.
tweaks-whole-image-confirm = Gravar a alteração pela imagem completa do backup? A câmera a recusou como valor individual. A imagem inteira é gravada de volta com a alteração; antes, a imagem anterior à alteração é salva neste computador.
btn-whole-image = Gravar backup completo
tweaks-undo-none = Não há valores salvos para esta câmera.
tweaks-undo-confirm = Gravar de volta na câmera os valores salvos? Isso desfaz todas as alterações gravadas uma a uma desde que foram salvos.
tweaks-undo-done = Os valores salvos foram gravados de volta. Desligue e ligue a câmera novamente.
tweaks-undo-available = Os valores de antes da primeira alteração estão salvos e podem ser restaurados.
btn-undo-tweaks = Desfazer alterações
tweaks-identity-heading = Identidade da câmera (somente leitura)
tweaks-caution-languages = Esta câmera não consegue exibir dois idiomas: provavelmente chinês (simplificado) e chinês (tradicional). Ao rolar até eles, o menu de idiomas reinicia, por isso permanecem desativados por precaução.
tweaks-caution-accept = Ativar esses dois idiomas mesmo assim (por sua conta e risco)
tweaks-caution-confirm = Esses dois idiomas podem fazer o menu de idiomas da câmera reiniciar. Nada é danificado e a alteração pode ser desfeita, mas talvez você precise procurar a configuração desde o início. Ativar mesmo assim?

# Configurações
settings-language = Idioma
settings-theme = Tema
settings-theme-system = Sistema
settings-theme-light = Claro
settings-theme-dark = Escuro

# Modo avançado. Uma proteção contra um descuido, não contra um invasor: tudo
# o que ele libera continua acessível pela linha de comando.
settings-advanced-heading = Modo avançado
settings-advanced-hint = O modo avançado exibe o console de serviço e o console do atualizador nas páginas de mesmo nome. O modo de serviço dá acesso direto à memória da câmera, ao seu sistema de arquivos e aos seus dados de calibração; um comando digitado errado pode inutilizar a câmera. Deixe-o desligado enquanto não souber o que vai escrever.
settings-advanced-toggle = Exibir os consoles de serviço e do atualizador
settings-advanced-on = O modo avançado está ligado. As páginas Console de serviço e Console do atualizador agora oferecem os próprios consoles.
settings-advanced-off = O modo avançado está desligado. As páginas Console de serviço e Console do atualizador explicam, em vez disso, o caminho pela linha de comando.
settings-advanced-confirm = Os consoles de serviço e do atualizador podem danificar a câmera de forma permanente. Exibir os dois consoles?
settings-update-heading = Aviso de atualização
settings-update-purpose = Quando ativado, o programa pergunta ao GitHub a cada inicialização se uma versão mais nova foi publicada e, nesse caso, avisa no topo da janela. O GitHub fica sabendo o seu endereço IP; nada mais é enviado. Versões de pré-lançamento não são anunciadas.
settings-update-toggle = Avisar ao iniciar quando houver uma nova versão
settings-update-on = O aviso de atualização está ativado. A partir da próxima inicialização, uma nova versão é anunciada no topo da janela.
settings-update-off = O aviso de atualização está desativado. O programa não procura novas versões.
update-available = Uma nova versão está disponível
update-current = Versão instalada
update-latest = Nova versão
btn-update-dismiss = Ocultar

# Edição de uma tradução. Os arquivos de idioma podem ser baixados, alterados
# em qualquer editor de texto e enviados de volta; um arquivo enviado é
# verificado antes de ser usado.
settings-catalog-heading = Editar tradução
settings-catalog-hint = Baixe um arquivo de idioma, altere-o no editor de texto de sua preferência e envie-o de volta. Somente o texto à direita do sinal de igual é traduzido, e as entradas omitidas mantêm o texto original. O idioma escolhido aqui não altera o idioma do aplicativo.
settings-catalog-locale = Arquivo de idioma
settings-catalog-builtin = A tradução incluída está em uso.
settings-catalog-custom = Sua própria tradução está em uso:
btn-catalog-export = Baixar…
btn-catalog-import = Enviar…
btn-catalog-reset = Restaurar a incluída
catalog-exported = Gravado em
catalog-imported = Tradução aplicada.
catalog-reset-done = A tradução incluída foi restaurada.
catalog-reset-none = Não havia tradução própria a restaurar.
catalog-reset-confirm = Descartar sua própria tradução para este idioma?
catalog-rejected = O arquivo não foi aplicado. Corrija estas linhas:
catalog-count-accepted = Entradas aplicadas
catalog-count-missing = Entradas mantidas da tradução incluída
catalog-count-untranslated = Entradas idênticas ao original em inglês
catalog-line = Linha
catalog-problem-syntax = não é uma entrada no formato “id = texto”
catalog-problem-continuation = continua na linha seguinte, o que não é suportado
catalog-problem-duplicate = definida mais de uma vez
catalog-problem-unknown-key = não é uma entrada usada por este aplicativo
catalog-problem-empty = sem texto
catalog-problem-placeholder = espaço reservado alterado, esperado:

# Origem, agradecimentos e licença. Exibidos na página Licença e pelo comando
# `pmca-console license`. O texto da licença em si não é traduzido: é um
# documento jurídico e é reproduzido literalmente.
about-thanks-heading = Agradecimentos
about-thanks-body-1 = Esta ferramenta existe graças a ma1co, que descobriu como as câmeras da Sony se comunicam por USB e publicou o Sony-PMCA-RE em 2015, e graças a todas as pessoas que contribuíram com ele e com os projetos OpenMemories nos anos seguintes.
about-thanks-body-2 = A detecção da câmera, o protocolo de comandos externos, o procedimento do atualizador de firmware e o modo de serviço são todos trabalho deles, e esta portabilidade dá continuidade a esse trabalho. Obrigado. Sobre essa base, o PMCA-RE RELOADED acrescenta muitos recursos novos, contribuídos por masterflai. O principal é a troca automática do driver do dispositivo no Windows, agora integrada: não é mais preciso atribuir manualmente outro driver à câmera com uma ferramenta separada, o que torna o processo mais cômodo, mais rápido e menos arriscado.
about-contributors-heading = Colaboradores
about-contributors-body = Além de ma1co, estas pessoas contribuíram com o Sony-PMCA-RE ou publicaram em suas próprias cópias alterações que foram incorporadas a esta portabilidade:
about-purpose-heading = Por que esta portabilidade existe
about-purpose-body-1 = O aplicativo original não está mais em desenvolvimento. Ele foi escrito para Python 2, que chegou ao fim da vida útil em 2020, e partes dele dependem de serviços que a Sony já desativou, de modo que ele não funciona mais sem restrições nos sistemas atuais.
about-purpose-body-2 = O objetivo desta portabilidade é a preservação, e não a substituição, e na mesma medida um uso sem complicações hoje: manter estas câmeras utilizáveis e fazer o programa funcionar sem dificuldades tanto nos computadores atuais quanto com modelos de câmera mais novos. Tudo o que o original conseguia fazer deve continuar sendo possível aqui.
about-origin-heading = Como ela foi feita
about-origin-body-1 = A portabilidade de Python para Rust foi realizada com a ajuda de IA.
about-origin-body-2 = O que isso não significa merece ser dito com a mesma clareza: o comportamento é coberto por um conjunto de testes automatizados e por um plano de testes de hardware. Sobretudo a versão para Windows foi verificada a fundo, porque os drivers de dispositivo fazem dela a plataforma mais complexa; macOS e Linux, por outro lado, não puderam ser testados por completo. Trate isto como software experimental.
about-license-heading = Licença
about-license-body = Esta portabilidade é publicada sob a licença MIT, a mesma licença do projeto original. O aviso de copyright original é mantido sem alterações, como essa licença exige.
about-license-full = Texto da licença
about-third-party-heading = Licenças das bibliotecas utilizadas
about-third-party-body = Este programa é ligado estaticamente: o código de cada biblioteca que ele usa está dentro do executável, e as licenças dessas bibliotecas exigem que os avisos de direitos autorais o acompanhem. Eles são reproduzidos abaixo na íntegra, uma entrada por pacote.
about-third-party-toggle = Mostrar todos os avisos de terceiros
about-privacy-heading = Dados
about-privacy-body-1 = Este programa é executado no seu próprio computador. Ele não envia dados de uso, não há telemetria e não é exigida nenhuma conta. Nada do que você faz aqui é comunicado a lugar algum.
about-privacy-body-2 = Duas funções, porém, estabelecem conexões por conta própria, e a outra ponta fica sabendo o seu endereço IP: os dados de assistência de GPS são baixados da Sony, e a lista de aplicativos do GitHub. Sem essas funções o programa permanece inteiramente off-line. Se você ativar o aviso de atualização nas configurações, o programa também pergunta ao GitHub pela versão mais recente a cada inicialização.
about-privacy-body-3 = O que é lido da câmera — inclusive o número de série e as senhas de Wi-Fi armazenadas — não sai do seu computador. Se você gravar isso em um arquivo, preste atenção a quem repassa esse arquivo.
about-privacy-body-4 = Não há serviço algum operado por trás deste programa e nenhum dado chega aos seus autores, portanto aqui não há nada a solicitar nem a mandar apagar. O que os dois serviços citados fazem com os endereços que veem segue as declarações de privacidade deles, não esta.
about-legal-heading = Avisos legais
about-legal-trademark = Este é um projeto independente. Não foi feito pela Sony, não conta com seu aval e não tem qualquer vínculo com ela. «Sony», «PlayMemories» e os nomes dos modelos de câmera são marcas da Sony Group Corporation, mencionadas aqui apenas para indicar com quais câmeras esta ferramenta conversa.
about-legal-own-device = Este programa se destina ao uso com câmeras de sua propriedade.
about-legal-warranty = O que esta ferramenta faz está fora do que a Sony prevê para estas câmeras e pode encerrar sua garantia. O próprio software é fornecido sem garantia alguma, e o uso é por sua conta e risco. Se a engenharia reversa de um aparelho de sua propriedade é permitida, e em que medida, depende do país.
about-links-heading = Projetos
about-link-original = Sony-PMCA-RE — o projeto original
about-link-fwtool = fwtool.py — as ferramentas de firmware nas quais se baseia a geração das imagens de atualização
about-link-tweak = OpenMemories: Tweak — o aplicativo recomendado para as câmeras compatíveis
about-link-applist = OpenMemories: App list — mais aplicativos
about-link-platform = OpenMemories: Platform — o corpo do atualizador incluído
about-link-devices = Modelos de câmera compatíveis
about-link-nexhack = Nex-Hack — a pesquisa comunitária na qual este projeto se baseia
about-link-zve10 = zv-e10-re — pesquisa sobre as interfaces e a memória de configurações da ZV-E10
about-version = Versão
btn-copy-url = Copiar endereço
msg-url-copied = Endereço copiado para a área de transferência.

# Aviso de segurança
notice-experimental = Este é um software experimental. Use por sua conta e risco.

# Configurações de rede (GPS / Wi-Fi / transmissão ao vivo)
gps-downloading = Baixando os dados de assistência de GPS…
gps-writing = Gravando os dados de assistência de GPS na câmera…
gps-bad-size = Os dados de assistência de GPS têm um tamanho inesperado e não foram gravados.
wifi-read = Lendo os pontos de acesso de transmissão ao vivo da câmera…
wifi-written = Pontos de acesso de transmissão ao vivo gravados.
stream-read = Lendo as configurações de transmissão ao vivo da câmera…
stream-written = Configurações de transmissão ao vivo gravadas.
net-secret-warning = Aviso: este arquivo contém senhas de Wi-Fi e tokens de acesso em texto puro.
wifi-secret-warning = A câmera guarda essas senhas sem criptografia e elas são exibidas aqui exatamente como estão guardadas. Elas ficam neste computador; se você as salvar em um arquivo, preste atenção a quem o repassa.
gps-heading = Dados de assistência de GPS
gps-hint = Arraste um arquivo para cá, informe um caminho ou deixe o campo vazio para obter os dados atuais da Sony.
btn-gps-devices = Dispositivos compatíveis
btn-gps-devices-hide = Ocultar dispositivos compatíveis
gps-devices-note = Estes modelos têm um módulo GPS integrado. Em muitos deles, apenas a variante cujo nome termina em V o possui; as variantes sem V não têm GPS.
gps-devices-compact = Câmeras compactas (DSC)
gps-devices-slt = Câmeras SLT (montagem A)
gps-devices-action = Câmeras de ação
gps-devices-camcorder = Filmadoras (Handycam)
gps-col-variant = Variante GPS
gps-col-arch = Arquitetura
gps-col-module = Módulo GPS
gps-col-apps = Suporte a apps
gps-yes = sim
gps-no = não
gps-unknown = desconhecido
gps-devices-untested = Com GPS, suporte não testado
gps-devices-untested-note = Estes modelos têm um módulo GPS integrado, mas não constam da lista de dispositivos do OpenMemories. Não foi testado se este aplicativo funciona com eles; por isso, a arquitetura e o suporte a apps são desconhecidos.
wifi-heading = Pontos de acesso para transmissão ao vivo
wifi-hint = São os pontos de acesso usados pelo recurso de transmissão ao vivo, não as configurações de Wi-Fi da própria câmera. Um corpo sem recurso de transmissão recusa este comando.
wifi-multi = Lista de múltiplos pontos de acesso
wifi-none = Nenhum ponto de acesso armazenado.
wifi-writing = Gravando os pontos de acesso de transmissão ao vivo na câmera…
wifi-confirm = Isto substitui todos os pontos de acesso armazenados na câmera. Continuar?
wifi-empty-refused = Não será gravada uma lista vazia, pois isso apagaria todos os pontos de acesso armazenados.
stream-heading = Transmissão ao vivo
stream-writing = Gravando as configurações de transmissão ao vivo na câmera…
stream-confirm = Isto sobrescreve as configurações de transmissão ao vivo da câmera. Continuar?
stream-invalid-json = As configurações não são um JSON válido:

# Atualização de firmware e loja oficial
firmware-warning = Aviso: isto sobrescreve o firmware da câmera. Não interrompa a atualização depois de iniciada: não desconecte o cabo USB nem desligue a câmera. Use apenas um arquivo de firmware destinado exatamente a este modelo de câmera e verifique se a bateria está suficientemente carregada. Caso contrário, a câmera pode ficar permanentemente inutilizada.
firmware-untested = Não testado: a gravação do firmware foi portada, mas nunca foi experimentada em uma câmera real. Uma gravação malsucedida ou interrompida não pode ser desfeita; portanto, considere tudo nesta página como não comprovado.
firmware-confirm-required = A gravação não será feita sem --confirm.
firmware-truncated = O arquivo de firmware está truncado: a carga útil FDAT ultrapassa o fim do arquivo.
firmware-updating = Atualizando o firmware:
firmware-invalid = O arquivo não é uma imagem de firmware Sony válida
firmware-dat-path = Arquivo de firmware (.dat)
firmware-drop-hint = Solte um arquivo de firmware .dat aqui
firmware-file-required = Escolha primeiro um arquivo de firmware.
firmware-check-ok = A câmera aceita esta imagem. Nada foi gravado.
firmware-cancelled = Gravação cancelada. A câmera não foi modificada.
firmware-current-version = Versão instalada
firmware-new-version = Versão nesta imagem
firmware-models-hint = Uma câmera é compatível se o console do atualizador consegue acessá-la.
btn-firmware-check = Verificar imagem
btn-firmware-flash = Gravar firmware
firmware-own-device = Confirme que você está ciente desses riscos e que atualiza somente o seu próprio dispositivo.
btn-firmware-accept = Riscos entendidos – gravar

# Ferramentas de arquivo: as operações que não precisam de câmera alguma.
tools-hint = Estas operações trabalham em arquivos e nunca tocam na câmera, então nada aqui pode danificá-la. São as mesmas que o pmca-console oferece como apk2spk, spk2apk, print-backup, print-usagelog, guess-firmware e xpd-checksum.
tools-operation = Operação
tools-apk2spk = Empacotar um .apk em um contêiner .spk
tools-spk2apk = Desempacotar um .spk de volta para .apk
tools-print-backup = Mostrar o conteúdo de um Backup.bin
tools-print-usagelog = Mostrar o conteúdo de um Usagelog.bin
tools-guess-firmware = Encontrar a carga útil dentro de um .dat de firmware
tools-xpd-checksum = Calcular a soma de verificação CIC de um arquivo
tools-input = Arquivo de entrada
tools-file-required = Escolha primeiro os arquivos de que esta operação precisa.
tools-apk-invalid = O arquivo não pôde ser empacotado como .spk
tools-spk-invalid = O arquivo não é um pacote .spk válido
tools-backup-invalid = O arquivo não é uma imagem Backup.bin válida
tools-usagelog-invalid = O arquivo não é um registro de uso válido
tools-reset-value = valor de reposição:
tools-fdat-found = Carga útil encontrada no deslocamento { $offset }, tamanho { $size }.
tools-checksum = Soma de verificação
tools-written = Gravado em { $path }

# Links para a documentação do projeto
docs-compatibility = Compatibilidade de câmeras
docs-compatibility-hint = O projeto original disponibilizou uma visão geral das câmeras testadas. Ela abrange principalmente modelos mais antigos e está neste endereço:
devices-note = Dados extraídos da visão geral do projeto original. O número ao lado do suporte a apps é a versão da plataforma de aplicativos da câmera.
devices-more-note = As câmeras a seguir não foram testadas explicitamente com este programa. Uma marca na última coluna significa que a comunidade relatou o uso delas. Se o console de serviço também estiver marcado com ✖, esse uso falhou.
btn-more-models = Mais dispositivos
btn-more-models-hide = Ocultar mais dispositivos
check-updater-yes = Seu dispositivo { $model } é compatível: segundo a lista de dispositivos, o console do atualizador consegue acessá-lo.
check-updater-no = Seu dispositivo { $model } não é compatível: segundo a lista de dispositivos, o console do atualizador não consegue acessá-lo.
check-updater-unknown = A compatibilidade do seu dispositivo { $model } é desconhecida: ele não consta em nenhuma lista de dispositivos deste programa, então não se sabe se o console do atualizador consegue acessá-lo.
check-apps-no = Seu dispositivo { $model } não é compatível: segundo a lista de dispositivos, ele não oferece suporte a apps.
check-apps-untested = A compatibilidade do seu dispositivo { $model } é desconhecida: ele consta na lista de dispositivos, mas o suporte a apps não foi confirmado.
check-apps-unknown = A compatibilidade do seu dispositivo { $model } é desconhecida: ele não consta em nenhuma lista de dispositivos deste programa, então não se sabe se oferece suporte a apps.
check-gps-no = Seu dispositivo { $model } não é compatível: ele não consta na lista de dispositivos com receptor GPS.
check-gps-untested = A compatibilidade do seu dispositivo { $model } é desconhecida: segundo a lista de dispositivos ele tem receptor GPS, mas a transferência para este modelo não foi testada.
check-gps-unknown = A compatibilidade do seu dispositivo { $model } é desconhecida: ele não consta em nenhuma lista de dispositivos deste programa, então não se sabe se tem receptor GPS.
check-unidentified = Não foi possível identificar o dispositivo conectado, por isso a compatibilidade dele é desconhecida.
check-proceed = Deseja continuar mesmo assim?
check-cancelled = Operação cancelada. O dispositivo não foi modificado.
devices-col-confirmed = Confirmado
devices-group-ilc = Câmeras de lentes intercambiáveis (encaixes E e A)
devices-group-remote = Controles remotos Live-View
btn-copy-link = Copiar link
shutter-heading = Contador de disparos
shutter-hint = A câmera conta cada disparo do obturador e grava o valor atual em cada foto que salva (ARW, JPEG ou HEIF). Selecione a foto mais recente para ler o valor atual. O arquivo é apenas lido, nunca alterado.
shutter-source = A posição do contador no arquivo varia conforme o modelo. As posições vêm de pesquisas documentadas publicamente (ExifTool) e não estão confirmadas para todos os modelos.
shutter-drop-hint = Solte uma foto (ARW, JPEG, HEIF) aqui
btn-shutter-read = Analisar foto
shutter-file-required = Selecione uma foto primeiro.
shutter-file = Arquivo
shutter-taken = Data da captura
shutter-count = Disparos
shutter-type = Obturador
shutter-type-mechanical = Mecânico
shutter-type-electronic = Eletrônico
shutter-electronic-hint = Esta foto foi tirada com o obturador eletrônico. Em muitos modelos essas fotos não são contadas; o valor mostra então a contagem do último disparo mecânico.
shutter-not-sony = A foto não foi tirada com uma câmera Sony. Não é possível ler dela um contador de disparos.
shutter-unknown-model = Não se sabe onde este modelo de câmera grava o contador no arquivo, por isso nenhum valor é exibido.
shutter-not-recorded = O arquivo não contém um contador legível. Isso acontece com fotos salvas ou exportadas por programas de edição e, em alguns modelos, com fotos tiradas com o obturador eletrônico. Selecione o arquivo original do cartão de memória.
shutter-model-mismatch = A foto vem de um modelo de câmera diferente da câmera conectada.
shutter-invalid = O arquivo não é uma foto legível
file-filter-photo = Fotos (ARW, JPEG, HEIF)
usage-log-heading = Registro de uso
usage-log-hint = A câmera mantém um registro interno em um formato não documentado. Ele é salvo tal como está; a câmera é apenas lida.
usage-log-written = Gravado em
btn-usage-log-info = Sobre o registro de uso
btn-usage-log-info-hide = Ocultar a explicação
usage-log-info-what-heading = O que é este registro
usage-log-info-what = A Sony chama este registro internamente de "KikiLog". A câmera o grava por conta própria; este programa apenas o lê. Não se conhece nenhuma descrição pública do formato.
usage-log-info-read = A leitura não altera nada: vários downloads seguidos foram idênticos byte a byte.
usage-log-info-flush-heading = Por que faltam entradas recentes
usage-log-info-flush = Novas entradas muitas vezes não aparecem de imediato: a câmera pode gravá-las só ao ser desligada, às vezes distribuídas por vários ciclos de liga/desliga. Desligue e ligue a câmera, baixe novamente e repita até não surgirem novas entradas.
usage-log-info-content-heading = O que dá para ler nele
usage-log-info-shutter = Disparos: alguns eventos correspondem, cada um, a uma foto. O número deles, porém, não é o contador de disparos: o registro tem tamanho fixo e não cobre necessariamente toda a vida útil da câmera. A câmera guarda o total em outro lugar e o grava nas fotos que salva; ele pode ser lido na seção "Contador de disparos".
usage-log-info-clock = Os horários são tão confiáveis quanto o relógio da câmera. Após uma troca de bateria, o relógio pode voltar atrás e gerar entradas com a data inicial da câmera.
usage-log-info-unknown = Não se conhece nenhuma descrição pública dos números de evento. O programa mostra os números sem interpretá-los.
usage-log-info-privacy-heading = Privacidade
usage-log-info-privacy = O arquivo contém os números de série dos cartões de memória inseridos. Não o compartilhe nem publique sem pensar.
btn-usage-log = Salvar registro
usagelog-size = Área do registro
usagelog-cursor = Posição de escrita
usagelog-spare = Segunda área
usagelog-count = Entradas
usagelog-events = Eventos por número
usagelog-entries = Entradas em ordem
msg-link-copied = Link copiado para a área de transferência.

# Mensagens de falha do backend, indexadas pelo código estável informado.
# A parte variável da mensagem (um caminho, um byte de status, um erro do
# dispositivo) é acrescentada pela interface, então estas frases são completas sem ela.
# Não existe `err-operation` de propósito: esse código marca uma mensagem que o
# backend já traduziu e que deve ser exibida literalmente.
err-usb-no-device = Nenhuma câmera encontrada. Conecte uma câmera e ligue-a.
err-usb-camera-not-ready = A câmera está conectada, mas não está pronta: está desligada ou aguarda na tela a escolha do modo USB. Ligue-a ou escolha “Armazenamento em massa (MSC)” na câmera.
camera-choose-usb-mode = A câmera aguarda a escolha do modo USB. Escolha “Armazenamento em massa (MSC)” na câmera.
err-usb-too-many-devices = Há mais de uma câmera conectada. Desconecte todas, menos a que você quer usar.
err-usb-transfer = A transferência USB falhou. Tente outro cabo ou porta e evite hubs USB.
err-usb-permission = O programa não tem permissão para abrir a câmera por USB. No Linux esse direito é concedido uma única vez por uma regra udev para o identificador de fabricante da Sony; o script scripts/linux-prepare.sh no código-fonte a instala. Depois, desconecte a câmera e conecte-a novamente.
err-usb-busy = Outro programa está usando a câmera, por isso não foi possível abri-la. Feche outros programas que possam acessá-la; no Linux o monitor de mídia da área de trabalho pode retê-la, o que o scripts/linux-prepare.sh encerra. No macOS, defina a conexão USB da câmera como MTP: no modo de armazenamento em massa, o próprio driver de disco do sistema a retém.
err-usb-needs-root = O macOS só entrega a conexão de armazenamento em massa da câmera a um programa executado com direitos de administrador. Inicie o programa de linha de comando com sudo, por exemplo: sudo pmca-console tweaks. As informações da câmera também podem ser lidas sem isso, com a conexão USB da câmera definida como MTP.
macos-admin-prompt = O PMCA-RE RELOADED precisa de direitos de administrador para tirar do macOS a conexão de armazenamento em massa da câmera. O cartão some do Finder enquanto ele trabalha; ejete-o lá antes se houver arquivos abertos nele.
macos-admin-declined = Os direitos de administrador não foram concedidos, então a câmera não foi alterada. No macOS esta função precisa deles.
macos-admin-failed = O programa não conseguiu iniciar seu auxiliar com direitos de administrador.
err-usb-stall = A câmera abortou a transferência USB.
err-usb-protocol = A câmera enviou uma resposta inesperada.
err-usb-msc-status = A câmera recusou um comando de armazenamento.
cli-error-prefix = erro
err-usb-driver = O Windows atribuiu a esta câmera um driver que não permite acesso direto. Este modo precisa do driver WinUSB, que o próprio aplicativo pode atribuir — mas somente com direitos de administrador.
err-needs-elevation = Atribuir ou liberar um driver requer direitos de administrador, e a solicitação do Windows foi recusada. Tente novamente com “Iniciar auxiliar de driver”; nada precisa ser alterado na câmera.
err-usb-no-mass-storage-volume = A câmera está conectada em modo de armazenamento em massa, mas não oferece nenhuma unidade. Acorde-a, confirme o modo USB na tela dela se for solicitado e verifique se há um cartão de memória inserido. Não troque o driver: este modo precisa justamente do driver padrão do Windows.
err-usb-msc-sense = A câmera relatou um erro de armazenamento.
err-usb-ptp-response = A câmera rejeitou o comando.
err-usb-unsupported = Este modelo de câmera não oferece essa função.
err-usb-updater-status = O atualizador de firmware da câmera recusou a operação.
err-usb-senser = Um comando do modo de serviço falhou.
err-market-core = Não foi possível ler um arquivo de dados da câmera.
err-market-json = Os dados recebidos estavam malformados.
err-market-missing-field = Faltou um campo esperado na resposta.
err-market-no-apk = Nenhum aplicativo foi selecionado para instalação.
err-market-http = A requisição de rede falhou.
err-market-io = O servidor de instalação local falhou.
err-market-tls = Não foi possível estabelecer a conexão criptografada com a câmera.
err-apk = Não foi possível ler o arquivo do aplicativo.
err-io = Não foi possível ler ou gravar um arquivo.
err-json = A câmera enviou dados malformados.
err-unknown-locale = Este idioma não está disponível.
err-catalog-encoding = O arquivo de tradução não está salvo como texto UTF-8. Salve-o novamente com a codificação UTF-8.
err-internal = Ocorreu um erro interno. Isso é um defeito do aplicativo.

## Function analysis
nav-analysis = Análise
analysis-title = Análise de funções
analysis-experimental = Experimental. Esta função é nova e foi testada em poucas câmeras.
analysis-own-hardware = Use somente na sua própria câmera. O próprio modo de serviço já é uma intervenção que a Sony não prevê; a responsabilidade é sua.
analysis-explanation-toggle = Explicação: finalidade, procedimento, riscos
analysis-purpose-heading = Para que serve?
analysis-purpose = A análise de funções documenta em qual valor da memória de configurações (backup) a câmera guarda uma de suas próprias configurações de menu. O resultado descreve a memória de configurações desta câmera; pode ser comparado com outras câmeras, mas não altera o que uma câmera oferece.
analysis-how-heading = Como funciona?
analysis-how = Para cada valor de menu informado, o programa lê uma vez a memória de configurações completa, cada vez em uma sessão curta própria do modo de serviço. Entre as leituras, você desconecta o cabo e muda a configuração no menu da câmera; a câmera continua ligada. No fim, o primeiro valor é lido mais uma vez. Valores que acompanham o menu e no fim voltam ao valor inicial são candidatos. Valores que mudam sozinhos (contadores, relógio) são descartados.
analysis-requirements-heading = Requisitos
analysis-requirements = A câmera está conectada por USB, seu modo USB está em “Auto” e, após cada sessão, você escolhe “Armazenamento em massa (MSC)” na câmera. No Windows o programa precisa de direitos de administrador, porque atribui brevemente um driver para o modo de serviço. Carregue a bateria; cada sessão leva cerca de um minuto.
analysis-read-heading = Etapas de leitura
analysis-read = As capturas apenas leem. Nada muda nas configurações da câmera. O risco é baixo, mas não zero: mudar para o modo de serviço já é uma intervenção que a Sony não prevê.
analysis-write-heading = Etapa de escrita (teste de escrita)
analysis-write = Somente se você a ativar expressamente: o programa grava o valor encontrado na câmera, você verifica no menu após reiniciar se ele tem efeito e depois o valor original é gravado de volta. Só é gravado um valor que a própria câmera salvou, e só se exatamente um candidato adequado foi encontrado. O valor antigo é salvo antes. Mesmo assim, esta etapa altera a câmera; uma falha pode deixá-la em um estado que só a assistência técnica consegue reparar.
analysis-legal-heading = Aspectos legais
analysis-legal = O software é fornecido sem qualquer garantia (licença MIT). Intervenções na câmera podem afetar a garantia dela. Use a função somente em câmeras que pertencem a você. O relatório de análise não contém número de série nem nada da memória de configurações além dos candidatos encontrados; ele nunca é enviado automaticamente.
analysis-start-heading = Nova análise
analysis-setting-label = Configuração de menu, como a câmera a chama
analysis-values-label = Valores de menu, separados por vírgulas
analysis-values-hint = O primeiro valor é o que está configurado agora. De dois a oito valores; cada um custa uma sessão do modo de serviço.
analysis-write-option = Depois, fazer o teste de escrita (altera a câmera)
analysis-write-warning = O teste de escrita altera a câmera: grava um valor na memória de configurações e depois grava o valor original de volta. Somente na sua própria câmera, por sua conta e risco.
analysis-write-consent = A câmera é minha e assumo o risco.
btn-analysis-write-enable = Ativar teste de escrita
btn-analysis-start = Iniciar análise
btn-analysis-snapshot = Ler backup
btn-analysis-new = Nova análise
analysis-discard-confirm = Descartar a análise em andamento? O relatório até aqui continua salvo.
analysis-access-read = LEITURA
analysis-access-write = ESCRITA
analysis-access-user = MANUAL
analysis-access-local = PC
analysis-step-snapshot = Ler o backup, a configuração está em “{ $value }”
analysis-step-change = Mudar para “{ $value }” no menu da câmera
analysis-step-evaluate = Comparar as capturas
analysis-step-write = Gravar na câmera o valor de “{ $value }”
analysis-step-check = Desligar e ligar a câmera, ler o menu (esperado: “{ $value }”)
analysis-step-restore = Gravar de volta o valor original de “{ $value }”
analysis-step-report = Salvar o relatório de análise
analysis-do-prepare = PREPARAÇÃO: 1. No menu da câmera, defina “{ $setting }” como “{ $value }” e saia do menu com o botão MENU para que a câmera salve a configuração. 2. Deixe a câmera ligada. 3. Conecte o cabo e escolha “Armazenamento em massa (MSC)” na câmera.
analysis-do-change = SUA VEZ: 1. Desconecte o cabo. 2. A câmera continua ligada. 3. No menu da câmera, defina “{ $setting }” como “{ $value }” e saia do menu com o botão MENU para que a câmera salve a configuração. 4. Conecte o cabo de novo e escolha “Armazenamento em massa (MSC)”.
analysis-do-check = SUA VEZ: 1. Desconecte o cabo. 2. Desligue e ligue a câmera novamente. 3. Veja no menu o que aparece em “{ $setting }”.
analysis-do-reconnect = Conecte o cabo de novo e escolha “Armazenamento em massa (MSC)” na câmera.
analysis-reading = Captura { $index } de { $total }: NÃO desconecte o cabo durante a leitura.
analysis-cli-continue = Enter para continuar, q para cancelar:
analysis-confirm-word = sim
analysis-cli-confirm = Digite “{ $word }” para confirmar:
analysis-write-declined = Não confirmado. Nada foi gravado.
analysis-aborted = Cancelado. Nada foi gravado.
analysis-snapshot-done = Captura lida. A câmera saiu do modo de serviço.
analysis-result = { $compared } valores comparados: { $count } candidatos, { $running } valores mudaram sozinhos.
analysis-candidate-consistent = corresponde claramente ao menu
analysis-candidate-inconsistent = não corresponde claramente
analysis-write-preview-heading = Isto será gravado
analysis-write-preview = Valor { $id } ({ $size } bytes): de { $old } (“{ $old-label }”) para { $new } (“{ $new-label }”). A própria câmera salvou o novo valor quando “{ $new-label }” estava configurado.
btn-analysis-write = Gravar
btn-analysis-skip-write = Concluir sem teste de escrita
analysis-written = Valor gravado, relido e salvo permanentemente.
analysis-menu-question = O que o menu mostra agora?
analysis-menu-other = outra coisa
btn-analysis-menu-confirm = Confirmar
btn-analysis-restore = Gravar de volta o valor original
analysis-restored = Valor original gravado de volta e salvo permanentemente.
analysis-survived = O valor gravado ainda estava salvo após reiniciar.
analysis-not-survived = O valor gravado não estava mais salvo após reiniciar.
analysis-report-saved = Relatório de análise salvo: { $path }
analysis-report-hint = O relatório não contém número de série nem backup. Você pode repassá-lo para que o resultado seja verificado e documentado.
btn-analysis-export = Exportar relatório…
file-filter-analysis = Relatório de análise
analysis-pending = Um teste de escrita não foi concluído. O valor original está salvo e pode ser gravado de volta.
analysis-pending-none = Não há teste de escrita pendente salvo para esta câmera.
analysis-invalid-setting = Informe a configuração de menu como a câmera a chama.
analysis-invalid-values = Informe de dois a { $max } valores de menu diferentes. O primeiro é o que está configurado agora.
analysis-already-complete = Todas as capturas já foram lidas.
analysis-other-camera = Esta não é a mesma câmera da primeira captura. A análise foi interrompida.
analysis-write-not-allowed = O teste de escrita não é possível neste estado.
analysis-camera-changed = A câmera agora contém um valor diferente do da primeira captura. Nada foi gravado.
analysis-write-refused = A câmera recusou o valor; tudo está como antes. ({ $detail })
analysis-refuse-none = Teste de escrita impossível: nenhum valor acompanhou o menu.
analysis-refuse-ambiguous = Teste de escrita impossível: { $count } valores acompanharam o menu, nenhum pode ser atribuído com certeza.
analysis-refuse-inconsistent = Teste de escrita impossível: o valor não corresponde claramente aos valores do menu.
analysis-refuse-size = Teste de escrita impossível: o valor tem { $size } bytes; só valores pequenos são gravados.
analysis-refuse-read-only = Teste de escrita impossível: a câmera marca o valor como somente leitura.
analysis-refuse-protected = Teste de escrita impossível: a câmera marca o valor como protegido.
settings-analysis-heading = Análise de funções
settings-analysis-purpose = A análise de funções documenta em qual valor da memória de configurações a câmera guarda uma de suas próprias configurações de menu. Destina-se a quem quer entender a memória de configurações do seu modelo e compartilhar os resultados.
settings-analysis-why-off = Ela está desativada porque o uso normal não precisa dela, porque coloca a câmera várias vezes no modo de serviço e porque, se solicitado, também grava na câmera. É experimental e foi testada em poucos modelos.
settings-analysis-toggle = Ativar a análise de funções (item “Análise” no menu)
settings-analysis-confirm = Ativar a análise de funções? Ela coloca a câmera várias vezes no modo de serviço e, se solicitado, pode gravar na câmera. Use somente na sua própria câmera.
btn-analysis-enable = Ativar
settings-analysis-on = A análise de funções está ativada.
settings-analysis-off = A análise de funções está desativada.
analysis-needs-admin = Cada captura precisa de direitos de administrador para o modo de serviço. O Windows os pede uma vez por sessão do programa; você pode iniciar o auxiliar de driver agora ou esperar até que a primeira captura o solicite.
analysis-changed-hint = Verifique: nenhum candidato foi encontrado. Os valores a seguir mudaram, mas não voltaram ao valor inicial. Se um deles muda só uma vez entre dois valores, a câmera provavelmente não estava no primeiro valor do menu na primeira captura. Nesse caso, repita a análise com o valor inicial configurado corretamente.
analysis-changed-info = Info: valores que mudaram sozinhos durante a análise (por exemplo, contadores, relógio) são ignorados. Você não precisa fazer nada.
elevation-admin = Administrador
elevation-standard = Direitos padrão
elevation-admin-hint = O programa está sendo executado com direitos de administrador e pode atribuir o driver de que o modo de serviço precisa.
elevation-standard-hint = O programa está sendo executado com direitos padrão. Para o modo de serviço, os ajustes e a análise de funções, ele inicia quando necessário um auxiliar de driver com direitos de administrador; o Windows pergunta uma vez por sessão. Clique para iniciá-lo.
driver-helper-declined = A solicitação de direitos de administrador foi recusada. Sem eles, o driver do modo de serviço não pode ser atribuído; nada foi alterado.
btn-remove-foreign-drivers = Remover drivers antigos
btn-not-now = Agora não
btn-ok = OK
foreign-drivers-found = O Windows ainda mantém drivers que o Zadig ou uma ferramenta semelhante instalou para dispositivos Sony: { $packages }. O Windows entrega a câmera a eles antes que este programa possa atribuir o próprio driver; com isso, o modo de serviço falha e a câmera pode não aparecer como unidade. Removê-los agora? Depois a câmera usará os drivers que acompanham o Windows. A remoção precisa de direitos de administrador uma vez.
foreign-drivers-removed = Os drivers antigos foram removidos. Desconecte a câmera e conecte-a novamente.
foreign-drivers-remaining = Não foi possível remover estes drivers: { $packages }. Reinicie o Windows e tente de novo, ou desinstale o dispositivo junto com o driver no Gerenciador de Dispositivos.
foreign-drivers-hint = Ainda há drivers instalados pelo Zadig ou por uma ferramenta semelhante para dispositivos Sony ({ $packages }). O Windows entrega a câmera a eles antes que este programa possa atribuir o próprio driver, o que pode fazer o modo de serviço falhar. Remova-os com: pmca-console driver zadig --remove
driver-zadig-none = Nenhum driver do Zadig ou de ferramentas semelhantes está instalado para dispositivos Sony.
driver-zadig-entry = { $package } ({ $provider }): { $devices }
driver-zadig-remove-hint = Remova-os com: pmca-console driver zadig --remove
driver-zadig-removed = { $count } pacote(s) de driver antigo(s) removido(s). Desconecte a câmera e conecte-a novamente.
elevation-helper = Auxiliar de driver ativo
elevation-helper-hint = Um auxiliar com direitos de administrador atribui o driver do modo de serviço. Ele não tem janela e termina com o programa; não haverá outras solicitações.
elevation-helper-confirm = Iniciar o auxiliar de driver agora? O Windows pede direitos de administrador uma vez e não volta a pedir até o programa ser fechado.
analysis-cross-heading = Verificação em outra câmera
analysis-cross-intro = Opcional: verifica se outra câmera também tem o valor encontrado e o que está configurado nela. Apenas lê. O mesmo número não significa necessariamente a mesma função; só uma olhada no menu da outra câmera confirma isso. O resultado entra no mesmo relatório de análise.
analysis-cross-do = 1. Desconecte a câmera analisada. 2. Ligue a outra câmera, defina o modo USB dela como “Auto”, conecte o cabo e escolha “Armazenamento em massa (MSC)”.
btn-analysis-cross = Verificar outra câmera
analysis-cross-present = presente, valor { $value }
analysis-cross-match = corresponde a “{ $label }”
analysis-cross-nomatch = não corresponde a nenhum valor de menu medido
analysis-cross-absent = não presente
analysis-cross-same-camera = Esta é a própria câmera analisada. Conecte a outra câmera.
analysis-cross-not-now = A verificação só é possível depois que todas as capturas forem lidas e nenhum valor gravado estiver aguardando restauração.
analysis-cross-none = Não há candidato para verificar.
analysis-wrong-camera = A câmera conectada não é a analisada. Conecte a { $model }; nada foi gravado.
keys-fetching = Baixando a fonte…
keys-scanning = Procurando o material de chaves…
keys-complete = Todo o material de chaves está presente.
keys-incomplete = Ainda falta parte do material de chaves.
keys-present = presente
keys-missing = ausente
keys-class-senser = Chaves do modo de serviço (console de serviço, ajustes, análise)
keys-class-cic = Chave de checksum XPD (instalação de apps)
keys-class-spk = Material de chaves SPK (instalação de apps, apk2spk)
keys-class-fdat = Material de chaves FDAT (firmware do updater-shell)
plugin-heading = Plug-ins
plugin-intro = Os plug-ins ampliam este programa com idiomas, interruptores para ajustes da câmera, informações de compatibilidade, ferramentas e diagnósticos. Eles vêm apenas de arquivos que você mesmo escolhe — não há loja de plug-ins. O código deles roda em uma sandbox, sem acesso aos seus arquivos, à rede ou ao material de chaves. Instale apenas plug-ins de autores em quem você confia; as alterações valem a partir da próxima inicialização.
plugin-none = Nenhum plug-in instalado.
plugin-inactive = (ativo após reiniciar)
plugin-author = Autor
plugin-restart = Reinicie o programa para que a alteração tenha efeito.
btn-plugin-install = Instalar plug-in…
plugin-install-confirm = Este plug-in pode:
btn-confirm-plugin-install = Instalar plug-in
plugin-installed = O plug-in foi instalado.
plugin-remove-confirm = Remover o plug-in “{ $name }”?
plugin-removed = O plug-in { $id } foi removido.
plugin-perm-languages = adicionar idiomas ao programa
plugin-perm-camera-settings = alterar ajustes da câmera (somente entre os dois valores que ele indica e só após sua confirmação)
plugin-perm-compatibility = adicionar entradas à visão geral de compatibilidade
plugin-perm-code = executar código próprio na sandbox (ferramentas ou diagnósticos)
plugin-perm-host-os = consultar qual é o sistema operacional
plugin-perm-host-cameras = ver quais câmeras Sony estão conectadas e em qual modo
plugin-invalid = Isto não é um plug-in utilizável: { $reason }
plugin-unknown-field = Este plug-in usa um recurso que esta versão do programa não conhece ({ $field }). Ele foi feito para uma versão mais nova ou uma edição diferente do programa.
plugin-no-folder = Não foi possível determinar a pasta de dados do usuário; por isso, não é possível instalar plug-ins.
plugin-not-installed = Nenhum plug-in com o identificador { $id } está instalado.
plugin-failed = O plug-in falhou: { $reason }
plugin-unknown-entry = Nenhum plug-in carregado oferece { $id }.
plugin-run-done = O plug-in terminou.
plugin-run-warning = O plug-in terminou com observações.
plugin-run-error = O plug-in informou um erro.
plugin-tools-heading = Ferramentas de plug-ins
plugin-tools-hint = Elas vêm de plug-ins instalados e rodam em uma sandbox. Um plug-in recebe apenas o que você entrega a ele e o que ele declarou, e não pode alterar a câmera.
plugin-diagnostics-heading = Diagnósticos de plug-ins
plugin-compat-heading = De um plug-in
plugin-col-note = Observação
file-filter-plugin = Arquivo de plug-in
keys-not-set-up = O material de chaves necessário para esta operação não está configurado.
err-missing-keys = O material de chaves necessário para esta operação não está configurado.
keys-heading = Material de chaves necessário
keys-intro = As chaves do modo de serviço, a chave de checksum XPD e o material de chaves SPK não fazem parte deste programa. Informe o endereço de uma fonte que você tenha — um repositório, um arquivo compactado ou um ficheiro — e o programa encontra e importa as partes que reconhece. Nada é executado, e o material fica apenas neste computador.
keys-law-notice = Observe as leis vigentes onde você está e trabalhe apenas com câmeras e dados que sejam seus.
keys-url-label = Endereço da fonte (um por linha)
keys-url-required = Informe ao menos um endereço de fonte.
btn-keys-import = Importar
btn-import-keys = Configurar o material de chaves
