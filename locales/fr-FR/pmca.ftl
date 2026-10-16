# Localisation française pour PMCA-RE RELOADED.
# Ne traduire que la partie à droite du signe =. Ne jamais modifier les identifiants.

app-title = PMCA-RE RELOADED – Boîte à outils caméra

# Navigation / onglets
nav-camera-info = Infos caméra
nav-install-app = Apps
nav-tweaks = Réglages
nav-network = Réseau
nav-firmware = Firmware
nav-updater = Shell de l'updater
nav-service = Shell de service
nav-tools = Outils de fichiers
nav-settings = Paramètres
nav-license = Licence

# Page d'accueil : la page sur laquelle la fenêtre s'ouvre.
nav-welcome = Bienvenue
welcome-intro-heading = Ce que fait ce programme
welcome-intro-body = Ce programme aide à préserver les fonctions de votre appareil photo Sony et à l'adapter à vos besoins. Après la fermeture par Sony de la boutique des PlayMemories Camera Apps, il n'existait plus aucun moyen de sauvegarder ou de réinstaller les applications achetées ou déjà installées. Une réinitialisation de l'appareil efface définitivement les applications installées. Ce programme permet de sauvegarder les applications existantes et de les réinstaller, et peut en outre fournir des applications communautaires librement disponibles. Vous pouvez aussi installer vos propres applications (fichiers *.apk). En cas de problème, le programme lit les informations de l'appareil et ses journaux internes pour faciliter l'analyse. Les réglages que l'appareil possède déjà sans les proposer dans son menu peuvent être modifiés puis annulés. Le programme dialogue directement avec l'appareil par le câble USB. La liste des modèles pris en charge figure sur la page Infos caméra.
welcome-start-heading = La première étape
welcome-start-body = Allumez d'abord l'appareil et, tant que le câble n'est pas branché, réglez « Connexion USB » dans son menu sur Stockage de masse ou MTP. « Auto » fonctionne généralement aussi, car l'appareil choisit alors l'un des deux de lui-même. L'ordre importe : une fois le câble branché, le menu de l'appareil n'est plus accessible. Reliez seulement ensuite l'appareil en USB, ouvrez Infos caméra et appuyez sur « Obtenir les infos caméra ». Dès que le modèle et la version du micrologiciel apparaissent, la liaison est bonne et toutes les autres fonctions de ce programme sont disponibles.
welcome-start-driver = Si un avis concernant le pilote apparaît à la place, suivez-le d'abord. Sous Windows, un pilote système s'approprie l'appareil ; il faut le remplacer avant que l'accès USB direct dont cette boîte à outils a besoin soit possible. Le programme s'en charge lui-même, mais attribuer un pilote est un droit que Windows ne concède qu'à un administrateur. Dans ce cas, le programme vous propose de le redémarrer en tant qu'administrateur, et Windows vous demande de confirmer l'opération. L'attribution est levée à la fin de la session. Sous Linux et macOS, ce n'est pas nécessaire.
welcome-start-driver-linux = Si un avis concernant le pilote apparaît à la place, suivez-le d'abord. Sous Linux, rien n'a besoin d'être remplacé : l'appareil est atteint directement. Ce qu'il faut une fois pour toutes, c'est le droit d'ouvrir le périphérique — une règle udev pour l'identifiant fabricant de Sony — et, pour le mode stockage de masse, la libération du pilote noyau usb-storage. Les deux se préparent hors de ce programme avec les droits root ; le script scripts/linux-prepare.sh des sources fait les deux. Cette fenêtre ne demande jamais de tels droits elle-même.
welcome-start-driver-macos = Si un avis concernant le pilote apparaît à la place, suivez-le d'abord. Sous macOS, rien n'a besoin d'être remplacé pour l'accès USB direct. Seul le mode stockage de masse passe par le Camera Driver de Sony, qui s'installe séparément et demande un mot de passe administrateur comme tout installateur. Cette fenêtre ne demande jamais de tels droits elle-même.
welcome-tweaks-heading = Réglages avancés et langues
welcome-tweaks-body = Les réglages avancés sont des réglages que l'appareil possède déjà mais qu'il ne propose pas dans son menu : la limite de durée d'enregistrement, la limite d'enregistrement 4K, le choix entre PAL et NTSC, l'installateur d'applications par USB — et les langues du menu.
welcome-tweaks-languages-heading = Les langues de l'appareil
welcome-tweaks-languages-body = Les appareils photo Sony n'affichent que les langues de menu courantes dans leur région de vente. Quand on vit à l'étranger, on n'y trouve donc souvent pas sa propre langue et l'on doit se débrouiller avec une langue étrangère. Pourtant, d'autres langues figurent déjà dans le micrologiciel. Ce programme les fait apparaître dans le menu, pour que vous puissiez utiliser votre appareil dans une langue familière.
welcome-tweaks-ui-heading = La langue de ce programme
welcome-tweaks-ui-body = Le sélecteur en haut est tout autre chose. Il ne change que la langue de ce programme et ne touche jamais à l'appareil.
welcome-tweaks-safety = Chaque réglage avancé est écrit dans la mémoire de sauvegarde de l'appareil, et chacun peut être annulé avec le même interrupteur. Une écriture interrompue peut endommager l'appareil de façon définitive : ne débranchez ni le câble ni la batterie pendant l'opération.
welcome-privacy-heading = Vos données
welcome-privacy-where = La page « Licence » redit tout ceci en entier, avec les licences de toutes les bibliothèques dont ce programme est fait.
welcome-where-heading = Où trouver quoi
welcome-where-info = Modèle, numéro de série et version du micrologiciel. C'est aussi le test de connexion.
welcome-where-install = Des applications de la liste communautaire, ou votre propre fichier .apk.
welcome-where-tweaks = Les réglages que le menu de l'appareil n'atteint pas, dont les langues.
welcome-where-network = Données d'assistance GPS, points d'accès et services utilisés pour la diffusion en direct.
welcome-where-firmware = Écrire une image de micrologiciel Sony sur l'appareil.
welcome-where-tools = Décompresser et examiner des fichiers de micrologiciel et de paquet. Sans appareil.
welcome-where-shells = Accès direct à la mémoire et au système de fichiers de l'appareil. À afficher d'abord dans les Paramètres.
welcome-where-settings = Thème, mode avancé et modification de la traduction de ce programme.
welcome-where-license = D'où vient ce portage, sur le travail de qui il s'appuie et sous quelles conditions il est publié.
welcome-where-hint = Aucune page n'agit d'elle-même sur l'appareil. Rien n'est lu ni écrit tant que vous n'appuyez pas sur un bouton qui l'annonce.

# Boutons
btn-get-info = Obtenir les infos caméra
btn-refresh = Actualiser
btn-load-apps = Charger les applications disponibles
btn-list-tweaks = Afficher les réglages disponibles
btn-install = Installer l'app sélectionnée
btn-install-file = Installer un fichier .apk…
btn-apply = Appliquer
btn-save = Enregistrer sur l'appareil
btn-add = Ajouter
btn-remove = Supprimer
btn-reveal = Afficher
btn-update-gps = Transférer les données d'assistance GPS vers l'appareil
btn-cancel = Annuler
btn-browse = Parcourir…
btn-switch-msc = Passer en mode stockage de masse
btn-start-driver-helper = Démarrer l’assistant pilote

# Libellés des filtres de type de fichier dans les boîtes de dialogue
file-filter-apk = Application Android (*.apk)
file-filter-firmware = Image de micrologiciel Sony (*.dat)
file-filter-usage-log = Journal d'utilisation (*.bin)
file-filter-catalog = Fichier de traduction (*.ftl)
file-filter-spk = Paquet d'application Sony (*.spk)
file-filter-any = Tous les fichiers
file-filter-gps = Données d'assistance GPS (*.dat)

# Statut / messages
msg-usage-log-written = Journal d'utilisation écrit dans { $path }
msg-connecting = Connexion…
msg-apps-found = { $count } apps trouvées
msg-switch-mode = Veuillez passer la caméra en mode MTP ou stockage de masse.
msg-no-tweaks = Aucun réglage disponible pour cette caméra.
msg-done = Terminé.

# Changement de mode USB de l'appareil (MTP -> stockage de masse). Proposé sous
# un message « mauvais mode » ; aucune commande n'existe pour le retour.
mode-switching = Passage de l'appareil en mode stockage de masse…
mode-switch-wait = Attente de la reconnexion de l'appareil…
camera-confirm-hint = Les messages affichés par l'appareil (par exemple au sujet de www.sony.net/pm) peuvent être fermés sans gêner l'opération.
mode-switch-done = L'appareil est maintenant en mode stockage de masse.
mode-switch-timeout = L'appareil n'est pas réapparu en mode stockage de masse. Éteignez-le puis rallumez-le et réessayez.
mode-switch-impossible = Ce mode ne peut pas être changé via USB. Éteignez puis rallumez l'appareil.
mode-switch-hint = Seul un appareil en mode MTP peut être commuté via USB. Le changement est temporaire : le mode USB choisi dans le menu de l'appareil reste inchangé. Éteindre puis rallumer l'appareil, ou rebrancher le câble, rétablit le mode précédent — il n'existe aucune commande USB pour le retour.

# Champs d'info caméra
info-model = Modèle
info-product-code = Code produit
info-serial = Numéro de série
info-firmware = Version du firmware
info-lens = Objectif
info-region = Région
info-mode = Mode
mode-mass-storage = Stockage de masse
mode-mtp = MTP
mode-updater = Mode updater
mode-app-install = Mode installation d'application
mode-service = Mode service
mode-unknown = Inconnu
info-android-version = Version de la plateforme Android
info-photographer = Photographe
info-copyright = Copyright
info-photographer-1 = Photographe (entrée 1)
info-photographer-2 = Photographe (entrée 2)
info-copyright-1 = Copyright (entrée 1)
info-copyright-2 = Copyright (entrée 2)
diagnose-services-heading = L'appareil reste injoignable ?
diagnose-services-hint = Certains services Windows peuvent accaparer un appareil en mode MTP, si bien que cette application ne peut plus l'atteindre. La vérification demande seulement à Windows quels services sont actifs ; elle ne modifie rien.
btn-diagnose-services = Vérifier les services Windows
diagnose-services-none = Aucun service perturbateur connu n'est actif.
diagnose-services-found = Ces services actifs peuvent gêner l'accès à l'appareil :
diagnose-services-advice = Arrêtez le service dans Services (services.msc) ou avec la commande suivante en tant qu'administrateur, puis reconnectez l'appareil :

# Mise à jour du firmware / shell
updater-model = Modèle de l'appareil
updater-firmware-current = Version actuelle du firmware
updater-guard-ok = L'image du firmware correspond à cet appareil.
updater-no-fdat = Aucun firmware de mise à jour fourni n'est disponible pour le modèle { $model }.
updater-model-required = L'appareil est déjà en mode updater et ne peut pas y indiquer son modèle. Éteignez l'appareil puis rallumez-le pour qu'il démarre en mode normal, puis réessayez. Vous pouvez aussi indiquer un fichier de charge utile (--fdat-file) ou, dans la console, le modèle (--model <MODÈLE>).
updater-already-updater = L'appareil est déjà en mode mise à jour.
updater-flash-required = Modification de l'appareil refusée sans confirmation. Relancez avec --flash pour passer en mode mise à jour et écrire la charge utile.
updater-flashing = Écriture de la charge utile de mise à jour sur l'appareil…
updater-switch-wait = Veuillez suivre les instructions à l'écran de l'appareil. En attente du passage en mode mise à jour…
updater-switch-timeout = Délai d'attente dépassé. Veuillez relancer la commande une fois l'appareil reconnecté.
updater-shell-ready = La charge utile de mise à jour est active. Tapez 'help' pour les commandes, 'exit' pour quitter.
updater-shell-done = Session shell terminée.
updater-session-panic = La session de l'updater s'est terminée de façon inattendue. Éteignez puis rallumez l'appareil avant de réessayer.

# Les réponses du shell de l'updater. Les erreurs d'analyse d'une ligne ne sont
# formulées qu'une fois, dans les clés service-* : les deux shells partagent le
# même analyseur, donc un chemin entre guillemets et une valeur hexadécimale
# incorrecte échouent de la même manière dans l'un comme dans l'autre.
updater-backup-synced = Sauvegarde écrite dans la mémoire flash.
updater-backup-locked = Les réglages de sauvegarde protégés sont désormais verrouillés.
updater-backup-writable = Les réglages de sauvegarde protégés sont désormais modifiables.
updater-shell-console-only = Le shell root exige un terminal et n'est disponible que depuis la console : lancez pmca-console updatershell --flash et tapez-y « shell ».

# La page du shell de l'updater.
updater-warning = Attention : pour atteindre ce shell, l'appareil redémarre en mode updater et une charge utile y est écrite — un petit programme de ce projet que l'appareil accepte comme une mise à jour du micrologiciel et qui répond ensuite aux commandes de ce shell. Cette charge utile disparaît dès que l'appareil est éteint, mais les commandes qu'elle propose écrivent dans la mémoire de réglages permanente de l'appareil, et une écriture interrompue peut l'endommager définitivement. En outre, tous les réglages de l'appareil sont réinitialisés au passage, contrairement au shell de service.
updater-console-heading = Lancer le shell de l'updater dans la console
updater-driver-hint = Le shell de l'updater a besoin que l'appareil reste sur le pilote de stockage de masse de Windows, USBSTOR, car c'est par lui que la charge utile est transmise. C'est l'inverse de ce qu'exige le shell de service. L'application attribue et libère elle-même le pilote WinUSB, il n'y a donc rien à préparer à la main — mais un pilote attribué par un autre outil doit d'abord être annulé. Un appareil en mode updater se déclare sous son propre identifiant de produit et ne doit pas non plus être réaffecté.
updater-fdat-label = Fichier de charge utile (facultatif)
updater-confirm = L'appareil va redémarrer en mode updater et une charge utile y sera écrite. Continuer ?
updater-cancelled = Annulé. L'appareil n'a pas été modifié.

btn-shell-primer = À savoir sur ce shell
btn-shell-primer-hide = Masquer l'explication
shell-primer-compare-heading = Deux shells, deux protocoles
shell-primer-compare = Les deux pages offrent une ligne de commande vers l'appareil, mais ce n'est pas le même shell. Le shell de service parle le protocole de service d'usine de Sony, auquel l'appareil répond lui-même une fois l'authentification faite ; rien n'est écrit sur l'appareil pour y parvenir. Le shell de mise à jour n'existe que parce que ce projet écrit son propre programme dans l'appareil : celui-ci redémarre en mode mise à jour, accepte la charge utile comme s'il s'agissait d'un micrologiciel, et c'est elle qui répond ensuite aux commandes listées ci-dessous. Un seul des deux peut être ouvert à la fois.
shell-primer-payload = « Charge utile » (en anglais « payload ») désigne le contenu proprement dit qu'une voie de transport achemine. Pour le shell de mise à jour, il s'agit d'un petit programme de ce projet : l'appareil l'accepte par la même voie qu'une mise à jour du micrologiciel, mais ne le conserve que jusqu'à ce qu'il soit éteint. Le shell de service n'a pas besoin de charge utile, car l'appareil répond à ses commandes avec son propre micrologiciel. Pour les fichiers de micrologiciel, le terme désigne la même chose : l'image du micrologiciel proprement dite, dans le conteneur du fichier .dat.
shell-primer-when-service = Le shell de service sert à tout ce qui lit ou modifie les réglages enregistrés de l'appareil pendant qu'il continue d'exécuter son propre micrologiciel : lire le bloc HASP, sauvegarder une copie de secours de la zone de sauvegarde, changer des entrées isolées, récupérer des fichiers. C'est aussi ce que la page Réglages utilise en dessous ; tout ce qui s'y fait peut se faire ici à la main.
shell-primer-when-updater = Le shell de mise à jour s'impose quand l'appareil n'est joignable d'aucune autre façon, ou quand il vous faut un shell sur le système en marche plutôt que des commandes isolées. C'est la seule voie qui offre un vrai shell root, et elle fonctionne encore sur un appareil qui refuse le mode service. Elle coûte un redémarrage, une écriture et les réglages de l'appareil : c'est donc le second choix, pas le premier.
shell-primer-driver-heading = Pilotes : les deux exigent le contraire
shell-primer-driver = Le shell de service parle l'USB brut et exige sous Windows un appareil rattaché à WinUSB. L'application attribue elle-même ce pilote en entrant en mode service et le libère à la fin de la session ; aucun outil séparé n'intervient. Le shell de mise à jour passe par USBSTOR, le pilote de stockage de masse de Windows, et cesse de fonctionner tant que l'appareil est rattaché à WinUSB — libérer l'attribution fait donc partie de la clôture d'une session de service. L'attribution d'un pilote nécessite des droits d'administrateur. Sous Linux, aucun changement de pilote n'est nécessaire ; seul l'accès au nœud de périphérique l'est.
shell-primer-risk-heading = Ce qui peut mal tourner
shell-primer-risk = Les commandes marquées ci-dessous écrivent dans la mémoire de réglages permanente de l'appareil. Une écriture interrompue peut empêcher l'appareil de démarrer, et rien ici ne s'annule — c'est exactement pour cela que bk d existe : enregistrez une copie de la zone de sauvegarde avant de changer quoi que ce soit. La charge utile de mise à jour elle-même ne reste pas sur l'appareil : l'éteindre l'efface. Le passage par le mode mise à jour réinitialise en revanche tous les réglages de l'appareil.
shell-primer-commands-heading = Commandes de ce shell
shell-primer-writes = Les commandes marquées (WRITES) modifient l'appareil. Toutes les autres se contentent de lire.
shell-cmd-hasp = Lit le bloc HASP, la zone d'identité protégée de l'appareil.
shell-cmd-pull = Récupère un fichier de l'appareil. Sans nom local, le nom distant est conservé.
shell-cmd-push = Copie un fichier local sur l'appareil en remplaçant ce qui s'y trouve. (WRITES)
shell-cmd-rm = Supprime un fichier sur l'appareil. (WRITES)
shell-cmd-rmem = Écrit une région de la mémoire de l'appareil dans un fichier local. Décalage et longueur sont hexadécimaux.
shell-cmd-bk-r = Lit une entrée de la zone de sauvegarde et l'affiche sous forme hexadécimale.
shell-cmd-bk-d = Enregistre toute la zone de sauvegarde dans un fichier. À faire avant tout changement : c'est le seul retour en arrière possible.
shell-cmd-bk-dump-txt = Enregistre toute la zone de sauvegarde dans <file>.bin et une liste lisible dans <file>.txt.
shell-cmd-bk-stat = Affiche le bloc d'état de la sauvegarde de l'appareil, région comprise.
shell-cmd-bk-w = Écrit une entrée de la zone de sauvegarde, donnée en octets hexadécimaux séparés par des espaces. (WRITES)
shell-cmd-bk-s-sub = Fixe dans la mémoire flash la zone de sauvegarde d'un sous-système, pour que le changement survive à un redémarrage. (WRITES)
shell-cmd-bk-s = Fixe la zone de sauvegarde dans la mémoire flash, pour que le changement survive à un redémarrage. (WRITES)
shell-cmd-bk-lock = Active ou désactive la protection en écriture des entrées de sauvegarde protégées ; certains réglages refusent de changer tant qu'elle est active. (WRITES)
shell-cmd-term = Active ou désactive le terminal de débogage de l'appareil. (WRITES)
shell-cmd-info = Lit le bloc d'état de la sauvegarde, qui indique si la zone est seulement inscriptible.
shell-cmd-shell = Ouvre un shell root sur l'appareil. Il lui faut un terminal, il n'est donc disponible qu'en console ; shell tcp <port> le propose sur 127.0.0.1 pour un autre client. (WRITES)
shell-cmd-help = Liste les commandes que ce shell accepte.
shell-cmd-exit = Termine la session et quitte le shell.
shell-primer-diagnostics-heading = Diagnostic et réparation
shell-cmd-bk-trace = Enregistre dans un fichier, paquet par paquet, la façon dont l'appareil envoie toute la zone de sauvegarde par USB. Sert à examiner les sauvegardes volumineuses ; peut laisser l'appareil bloqué au milieu de la réponse. Débranchez alors le câble, puis éteignez et rallumez l'appareil, en retirant la batterie si nécessaire.
shell-cmd-bk-rewrite = Lit la zone de sauvegarde, la réécrit sans modification et la relit. Montre si l'appareil accepte une image complète, sans changer aucun réglage. (WRITES)
shell-cmd-bk-restore-attrs = Rétablit les attributs de toutes les entrées de sauvegarde, comme la protection en écriture, selon une copie enregistrée avec bk d ou bk dump-txt. Les valeurs restent inchangées. N'utilisez qu'une copie du même appareil. (WRITES)
shell-primer-diagnostics-env = Trois variables d'environnement agissent sur la console comme sur l'interface graphique si elles sont définies avant le démarrage, par exemple dans PowerShell avec $env:PMCA_TWEAKS_DIRECT = "1". PMCA_TWEAKS_DIRECT fait écrire les tweaks valeur par valeur, même si la sauvegarde fait moins de 1 Mo. PMCA_TWEAKS_IMAGE leur fait écrire l'image complète, même si elle dépasse 1 Mo ; si les deux sont définies, PMCA_TWEAKS_DIRECT l'emporte. PMCA_TLS_DUMP=<fichier> enregistre le premier message TLS de l'appareil (ClientHello) lors de l'installation d'une app, pour examiner un échec de connexion.

# Installation d'application
install-switching = Passage en mode installation d'application…
install-disabled = L'appareil a refusé de passer en mode installation d'application. En général, c'est le réglage « Activer l'installateur d'apps USB » : il doit être activé dans « Réglages ».
install-stays-app-mode = L'appareil photo reste en mode installation d'application, où d'autres applications peuvent être installées. Pour toutes les autres fonctions, débranchez le câble USB, puis éteignez et rallumez l'appareil.
err-usb-app-install-mode = L'appareil photo est encore en mode installation d'application, où il n'accepte que des installations d'applications. Débranchez le câble USB, puis éteignez et rallumez l'appareil — redémarré en restant connecté, il revient directement dans ce mode.
install-switch-wait = En attente de la reconnexion de l'appareil en mode installation d'application…
install-switch-timeout = Délai d'attente dépassé pour le passage en mode installation d'application. Veuillez relancer la commande une fois l'appareil reconnecté.
install-bad-mode = L'appareil ne peut pas être utilisé dans ce mode. Veuillez le basculer en mode MTP ou stockage de masse.
install-starting = Démarrage de l'installation…
install-error = Erreur de communication lors de l'installation
install-server-failed = Le serveur d'installation local n'a pas accepté la connexion de l'appareil photo :
install-done = Tâche terminée avec succès.

# Analyse de l'APK
apk-package = Paquet :
apk-unreadable = Avertissement : le fichier .apk n'a pas pu être analysé.
apk-incompatible = Avertissement : cette application exige un niveau de SDK que l'appareil photo ne fournit peut-être pas.
apk-unsigned = Avertissement : le fichier .apk ne comporte aucune signature.

# Magasin d'applications communautaire
store-loading = Chargement de la liste des applications…
store-resolving = Recherche de la dernière version…
store-downloading = Téléchargement
store-unknown-app = Application inconnue
store-no-release = Aucune version téléchargeable pour cette application
store-not-installable = aucune version
msg-no-apps = Aucune application trouvée.
store-heading = Magasin d'applications communautaire
store-discontinued = Sony a fermé le magasin officiel PlayMemories Camera Apps ; ses serveurs ne répondent plus, il n'est donc plus possible d'y acheter ou d'y télécharger des applications.
store-use-own-apk = Vous pouvez toujours installer votre propre .apk : choisissez le fichier ci-dessous ou faites-le glisser sur cette fenêtre. Le magasin communautaire plus bas fonctionne également toujours.
btn-store-sync = Vérifier l'appareil
btn-install-selected = Installer la sélection
btn-store-stop = Arrêter après cette application
btn-store-stopping = Arrêt après cette application…
btn-store-retry = Réessayer les applications en échec
btn-continue = Continuer
btn-reinstall = Réinstaller
btn-skip = Ignorer
btn-confirm-updater = Redémarrer et écrire
btn-confirm-tweaks = Écrire les réglages
btn-confirm-languages = Activer les langues
btn-confirm-wifi = Remplacer les points d'accès
btn-confirm-stream = Écraser les paramètres
btn-confirm-show-shells = Afficher les shells
btn-confirm-discard = Abandonner la traduction
store-select-all = Sélectionner toutes les applications
store-col-app = Application
store-col-version = Version
store-col-author = Auteur
store-col-camera = Version sur l'appareil
store-col-status = État
store-camera-absent = non installée
store-version-unknown = inconnue
store-batch-query = Lecture des applications installées sur l'appareil…
store-sync-done = Appareil vérifié. Applications de la liste installées : { $count }
store-batch-downloading = Téléchargement de l'application { $current } sur { $total }…
store-batch-installing = Application { $current } sur { $total } : installation de { $name }…
store-reinstall-confirm = { $name } est déjà installée sur l'appareil. Version installée : { $installed }. Version dans le magasin d'applications : { $store }. Faut-il la réinstaller ?
store-state-queued = en attente
store-state-installed = installée
store-state-failed = échec
store-state-skipped = ignorée
store-state-not-run = non exécutée
store-batch-summary = Installées : { $done } · échecs : { $failed } · ignorées : { $skipped } · non exécutées : { $notrun }

# Mode service (senser)
usb-mode-advice = Pour les réglages, les sauvegardes et le shell de service, réglez le mode de connexion USB de l’appareil sur « Auto », et non de façon fixe sur le stockage de masse. Lorsque l’appareil demande le mode après une session, choisissez « Stockage de masse (MSC) ». Avec le stockage de masse fixe, l’appareil refuse souvent le passage suivant en mode service.
service-switch-refused = L’appareil n’a pas terminé le passage en mode service et ne s’est pas reconnecté en USB ; il affiche alors généralement « Vérifiez l’appareil connecté ». Débranchez le câble USB, rebranchez-le et réessayez.
service-switching = Passage en mode service…
service-switch-wait = Attente de la reconnexion de l'appareil en mode service…
service-switch-timeout = L'appareil n'est pas apparu en mode service. Il se réénumère comme un périphérique USB distinct (identifiant produit 0336) qui doit lui aussi recevoir le pilote WinUSB ; vérifiez dans le gestionnaire de périphériques s'il existe un appareil Sony sans pilote. Éteindre puis rallumer l'appareil le ramène en mode normal.
service-authenticating = Authentification…
# Affiché lorsque l'appareil photo est encore en mode service à la suite d'une
# exécution précédente, et lorsque cela n'a pas pu être annulé.
service-stranded-reset = L'appareil photo est encore en mode service depuis une exécution précédente. Fin de cette session au préalable…
service-stranded-failed = L'appareil photo est bloqué en mode service depuis une exécution précédente et n'a pas répondu à la demande d'en sortir. Éteignez-le puis rallumez-le, et réessayez.
service-bad-mode = L'appareil ne peut pas être utilisé dans ce mode. Passez-le en mode stockage de masse.
service-driver-required = L'accès USB direct a été refusé et le pilote WinUSB n'a pas pu être attribué. L'attribution d'un pilote nécessite des droits d'administrateur.
service-warning = Attention : le mode service donne un accès direct à la mémoire, au système de fichiers et aux données d'étalonnage. Les commandes d'écriture peuvent endommager définitivement l'appareil.
service-shell-ready = Shell de service prêt. Tapez 'help' pour les commandes, 'exit' pour quitter.
service-shell-done = Session du shell de service terminée.
# Signalé au démarrage lorsqu'une exécution précédente s'est interrompue avant
# d'avoir pu libérer le pilote qu'elle avait affecté.
driver-swept = { $count } affectation(s) de pilote laissée(s) par une exécution précédente ont été supprimées.
# Signalé au démarrage lorsque cette même exécution interrompue a aussi laissé
# l'appareil photo en mode service, où il refuserait la session suivante.
driver-swept-camera = Un appareil photo laissé en mode service par une exécution précédente a été ramené à son mode normal.
driver-release-replug = Windows attend l’appareil pendant la libération du pilote. Débranchez le câble USB puis rebranchez-le pour que le nettoyage puisse se terminer.
# Résultats de la commande `driver`.
driver-bound = { $id } utilise désormais WinUSB (paquet de pilotes { $package }).
driver-released = { $count } affectation(s) de pilote libérée(s) ; l'appareil photo utilise de nouveau les pilotes fournis par Windows.
driver-needs-elevation = L'affectation d'un pilote requiert des droits d'administrateur. Veuillez exécuter cette commande dans une console lancée en tant qu'administrateur.
driver-leftover-needs-elevation = Une exécution précédente a laissé un pilote attribué à l’appareil. Tant qu’il est en place, l’appareil demande de vérifier le périphérique connecté et son stockage de masse ne fonctionne pas. Le libérer nécessite les droits d’administrateur ; confirmez la demande de Windows.
driver-binding = Attribution du pilote WinUSB à l'appareil photo…
driver-bind-wait = En attente de la reconnexion de l'appareil photo avec le nouveau pilote…
driver-bind-timeout = L'appareil photo n'est pas réapparu après l'attribution du pilote. Débranchez-le, rebranchez-le puis réessayez.
driver-bind-failed = Le pilote WinUSB n'a pas pu être attribué : { $detail }
# Réponses du shell. Elles viennent du backend, afin que la console et la
# fenêtre emploient les mêmes mots.
service-usage = Utilisation : { $usage }
service-bad-hex = Valeur hexadécimale non valide : { $value }
service-unknown-command = Commande inconnue : { $command }. Tapez 'help' pour la liste.
service-unterminated-quote = La ligne se termine à l'intérieur d'un argument entre guillemets.
service-help = Liste des commandes :
service-read-bytes = { $count } octets lus.
service-pulled = { $remote } copié depuis l'appareil vers { $local }.
service-pushed = { $local } copié vers { $remote } sur l'appareil.
service-removed = { $remote } supprimé de l'appareil.
service-dumped = { $size } octets extraits depuis { $offset } vers { $local }.
service-backup-saved = Image de sauvegarde ({ $count } octets) enregistrée dans { $local }.
service-backup-text-saved = Image de sauvegarde ({ $count } octets) enregistrée dans { $bin } et liste dans { $txt }.
service-backup-status = État de la sauvegarde, région : { $region }
service-backup-rewritten = Image ({ $count } octets) réécrite sans modification puis relue ; non enregistrée de façon permanente.
service-backup-attrs-restored = Attributs de { $count } entrées rétablis d’après { $local }.
service-backup-written = Entrée de sauvegarde { $id } écrite.
service-backup-persisted = Sauvegarde du sous-système { $id } écrite en mémoire flash.
service-terminal-on = Le terminal de débogage est maintenant activé.
service-terminal-off = Le terminal de débogage est maintenant désactivé.

# La page Service sans le mode avancé : comment atteindre le shell depuis la
# console et ce qu'il faut avoir préparé.
service-console-heading = Lancer le shell de service dans la console
service-console-hint = Ouvrez un terminal dans le dossier contenant pmca-console et exécutez cette commande :
service-driver-heading = Avant de commencer
service-driver-hint = Le mode service exige un accès USB direct, que Windows n'accorde pas via le pilote de stockage de masse. L'application attribue elle-même le pilote WinUSB — deux fois, une fois pendant que l'appareil est en mode stockage de masse et une seconde fois après sa réapparition en mode service sous un second identifiant produit — et libère les deux attributions à la fin de la session. L'attribution d'un pilote nécessite des droits d'administrateur : lancez donc la console en tant qu'administrateur ; aucun autre outil n'est requis.
driver-note-linux = Rien n'est à installer ici : libusb atteint l'appareil photo directement, sans déloger de pilote noyau. Il faut seulement le droit d'ouvrir le périphérique et un bureau qui ne le retient pas déjà. Installez une règle udev pour l'identifiant de fabricant Sony 054c et détachez le pilote usb-storage avant d'utiliser le mode stockage de masse. Le script scripts/linux-prepare.sh fait les deux et arrête également les services de volumes gvfs.
driver-note-macos = Rien n'est à installer pour l'accès USB direct : libusb atteint l'appareil photo directement. Le mode stockage de masse fait exception, car il passe par le Camera Driver de Sony, qui doit être installé séparément. Cette plateforme est la moins éprouvée des trois ; attendez-vous à signaler un problème plutôt qu'à le trouver déjà décrit.
service-advanced-hint = Le shell peut aussi être affiché ici, sous Réglages, Mode avancé.
btn-copy-command = Copier la commande
msg-command-copied = Commande copiée dans le presse-papiers.

# La page Service avec le mode avancé activé.
btn-service-open = Se connecter
btn-service-close = Se déconnecter
btn-service-run = Exécuter
btn-service-browse = Insérer un chemin de fichier…
shell-path-quote = Ce chemin ne peut pas être inséré : il contient un guillemet droit, le caractère par lequel la ligne de commande délimite le début et la fin d'un argument. Renommez le fichier ou déplacez-le vers un chemin dépourvu de ce caractère.
service-command-label = Commande
service-not-connected = Non connecté.

# Chaînes propres à l'interface graphique
install-apk-path = Chemin vers un fichier .apk local (facultatif)
install-drop-hint = Déposez ici un fichier .apk

# Descriptions des réglages
tweak-rec-limit = Enregistrer des vidéos sans limite de durée
tweak-rec-limit-4k = Enregistrer des vidéos 4K sans limite de durée
tweak-language = Afficher toutes les langues du menu
tweak-pal-ntsc = Activer le sélecteur PAL / NTSC et l'avertissement
tweak-pal-ntsc-mode = Système vidéo (PAL / NTSC)
tweak-pal-ntsc-pal = PAL
tweak-pal-ntsc-ntsc = NTSC
tweak-pal-ntsc-note = Après un changement de système vidéo, l'appareil exige le formatage de la carte mémoire. Sauvegardez d'abord tout son contenu : photos, vidéos, LUT. Le plus simple est de retirer la carte avant de basculer.
tweaks-set-malformed = --set ID=VALEUR attendu, reçu : { $arg }
tweaks-set-unknown-value = Valeur inconnue « { $value } ». Valeurs attendues : { $options }
tweaks-set-not-a-choice = { $id } est un interrupteur, pas un choix. Utilisez --enable ou --disable.
tweaks-needs-set = { $id } est un choix, pas un interrupteur. Utilisez --set { $id }=VALEUR avec l'une de : { $options }
tweak-usb-installer = Activer l'installateur d'apps USB

# Déroulement des réglages
tweaks-warning = Avertissement : l'application des réglages avancés réécrit la mémoire de sauvegarde de l'appareil. Une écriture interrompue peut endommager définitivement votre appareil.
tweaks-reading = Lecture des réglages de l'appareil…
tweaks-writing = Écriture des réglages de l'appareil…
tweaks-done = Réglages de l'appareil mis à jour.
tweaks-write-failed = L'appareil n'a pas enregistré les nouveaux réglages tels qu'ils ont été envoyés. Une partie a pu être écrite malgré tout : relisez les réglages et annulez-les si nécessaire. Une sauvegarde faite avant la modification est le moyen sûr de revenir en arrière.
tweaks-confirm-required = Écriture refusée sans confirmation. Relancez avec --confirm après avoir lu l'avertissement ci-dessus.
tweaks-state-enabled = activé
tweaks-state-disabled = désactivé
tweaks-full-read-failed = L'appareil n'a pas fourni sa sauvegarde complète ({ $detail }). Il ne répond souvent plus ensuite : éteignez-le, retirez la batterie quelques secondes, remettez-la et reconnectez l'appareil.
tweaks-direct-mode = La sauvegarde de cet appareil dépasse 1 Mo ; ses réglages sont donc lus et écrits un par un. Les anciennes valeurs sont enregistrées avant chaque modification et peuvent être réécrites avec « Annuler les modifications ».
tweaks-direct-forced = PMCA_TWEAKS_DIRECT impose la voie valeur par valeur ; les réglages sont donc lus et écrits un par un. Les anciennes valeurs sont enregistrées avant chaque modification et peuvent être réécrites avec « Annuler les modifications ».
tweaks-direct-refused = L'appareil a refusé la modification ({ $detail }). Les valeurs déjà écrites ont été rétablies ; rien n'a été modifié. Certains modèles, notamment japonais, n'autorisent ce réglage que via la sauvegarde complète.
tweaks-direct-revert-failed = L'appareil a refusé la modification ({ $detail }) et toutes les valeurs déjà écrites n'ont pas pu être rétablies. Les anciennes valeurs sont enregistrées : essayez « Annuler les modifications ».
tweaks-direct-restart = Réglages de l'appareil mis à jour. Éteignez puis rallumez l'appareil pour que la modification prenne effet.
tweaks-rollback-unsaved = Les valeurs actuelles n'ont pas pu être enregistrées pour une annulation ; rien n'a été modifié.
tweaks-image-saved = L'image de sauvegarde d'avant la modification a été enregistrée : { $path }
tweaks-whole-image-hint = Pour écrire plutôt la modification via l'image de sauvegarde complète, relancez la même commande avec --whole-image. L'image d'avant la modification est enregistrée au préalable.
tweaks-whole-image-confirm = Écrire la modification via l'image de sauvegarde complète ? L'appareil l'a refusée en tant que valeur isolée. L'image complète est réécrite avec la modification ; l'image d'avant la modification est d'abord enregistrée sur cet ordinateur.
btn-whole-image = Écrire la sauvegarde complète
tweaks-undo-none = Aucune valeur enregistrée n'existe pour cet appareil.
tweaks-undo-confirm = Réécrire les valeurs enregistrées dans l'appareil ? Cela annule toutes les modifications écrites réglage par réglage depuis leur enregistrement.
tweaks-undo-done = Les valeurs enregistrées ont été réécrites. Éteignez puis rallumez l'appareil.
tweaks-undo-available = Les valeurs d'avant la première modification sont enregistrées et peuvent être réécrites.
btn-undo-tweaks = Annuler les modifications
tweaks-identity-heading = Identité de l'appareil photo (lecture seule)
tweaks-caution-languages = Cet appareil ne peut pas afficher deux langues — probablement le chinois (simplifié) et le chinois (traditionnel). Le menu des langues redémarre lorsqu'on fait défiler jusqu'à elles ; elles restent donc désactivées par précaution.
tweaks-caution-accept = Activer quand même ces deux langues (à vos risques et périls)
tweaks-caution-confirm = Ces deux langues peuvent faire redémarrer le menu des langues de l'appareil. Rien n'est endommagé et la modification est réversible, mais vous devrez peut-être retrouver le réglage depuis le début. Les activer quand même ?

# Paramètres
settings-language = Langue
settings-theme = Thème
settings-theme-system = Système
settings-theme-light = Clair
settings-theme-dark = Sombre

# Mode avancé. Une protection contre un geste malheureux, pas contre un
# attaquant : tout ce qu'il affiche reste accessible depuis la console.
settings-advanced-heading = Mode avancé
settings-advanced-hint = Le mode avancé affiche le shell de service et le shell de l'updater sur les pages du même nom. Le mode service donne un accès direct à la mémoire de l'appareil, à son système de fichiers et à ses données d'étalonnage ; une commande mal tapée peut rendre l'appareil inutilisable. Laissez-le désactivé tant que vous ne savez pas ce que vous allez écrire.
settings-advanced-toggle = Afficher les shells de service et de l'updater
settings-advanced-on = Le mode avancé est activé. Les pages Shell de service et Shell de l'updater proposent désormais les shells eux-mêmes.
settings-advanced-off = Le mode avancé est désactivé. Les pages Shell de service et Shell de l'updater expliquent la marche à suivre dans la console.
settings-advanced-confirm = Les shells de service et de l'updater peuvent endommager définitivement votre appareil. Afficher les deux shells ?

# Modification d'une traduction. Les fichiers de langue peuvent être
# téléchargés, modifiés dans n'importe quel éditeur de texte puis renvoyés ;
# un fichier envoyé est vérifié avant d'être utilisé.
settings-catalog-heading = Modifier la traduction
settings-catalog-hint = Téléchargez un fichier de langue, modifiez-le dans l'éditeur de texte de votre choix, puis renvoyez-le. Seul le texte à droite du signe égal est traduit, et les entrées omises conservent leur formulation d'origine. La langue choisie ici ne change pas la langue de l'application.
settings-catalog-locale = Fichier de langue
settings-catalog-builtin = La traduction fournie est utilisée.
settings-catalog-custom = Votre propre traduction est utilisée :
btn-catalog-export = Télécharger…
btn-catalog-import = Envoyer…
btn-catalog-reset = Rétablir la traduction fournie
catalog-exported = Écrit dans
catalog-imported = Traduction appliquée.
catalog-reset-done = La traduction fournie a été rétablie.
catalog-reset-none = Il n'y avait aucune traduction personnelle à rétablir.
catalog-reset-confirm = Abandonner votre propre traduction pour cette langue ?
catalog-rejected = Le fichier n'a pas été appliqué. Veuillez corriger ces lignes :
catalog-count-accepted = Entrées appliquées
catalog-count-missing = Entrées conservées de la traduction fournie
catalog-count-untranslated = Entrées identiques à l'original anglais
catalog-line = Ligne
catalog-problem-syntax = n'est pas une entrée de la forme « id = texte »
catalog-problem-continuation = poursuivie à la ligne suivante, ce qui n'est pas pris en charge
catalog-problem-duplicate = définie plusieurs fois
catalog-problem-unknown-key = n'est pas une entrée utilisée par cette application
catalog-problem-empty = sans texte
catalog-problem-placeholder = espace réservé modifié, attendu :

# Origine, remerciements et licence. Affichés sur la page Licence et par
# `pmca-console license`. Le texte de la licence lui-même n'est pas traduit :
# c'est un document juridique, reproduit tel quel.
about-thanks-heading = Remerciements
about-thanks-body-1 = Cet outil existe grâce à ma1co, qui a compris comment les caméras Sony communiquent en USB et qui a publié Sony-PMCA-RE en 2015, ainsi qu'à toutes les personnes qui ont contribué à ce projet et aux projets OpenMemories dans les années qui ont suivi.
about-thanks-body-2 = La détection des caméras, le protocole de commandes externes, la procédure de mise à jour du firmware et le mode service sont entièrement leur travail, et ce portage prolonge ce travail. Merci. Sur cette base, PMCA-RE RELOADED ajoute de nombreuses fonctions nouvelles apportées par masterflai. La principale est la bascule automatique du pilote de périphérique sous Windows, désormais intégrée : il n'est plus nécessaire d'attribuer à la main un autre pilote à l'appareil avec un outil séparé, ce qui rend l'opération plus pratique, plus rapide et moins risquée.
about-contributors-heading = Contributeurs
about-contributors-body = Outre ma1co, les personnes ci-dessous ont contribué à Sony-PMCA-RE ou publié dans leurs propres copies des modifications reprises dans ce portage.
about-purpose-heading = Pourquoi ce portage existe
about-purpose-body-1 = L'application d'origine n'est plus développée. Elle a été écrite pour Python 2, dont la fin de vie est intervenue en 2020, et certaines de ses parties reposent sur des services que Sony a depuis fermés : elle ne fonctionne donc plus sans restrictions sur les systèmes actuels.
about-purpose-body-2 = Le but de ce portage est la préservation plutôt que le remplacement, et tout autant une utilisation sans difficulté aujourd'hui : garder ces caméras utilisables et faire fonctionner le programme sans peine sur les ordinateurs actuels comme avec des modèles de caméras plus récents. Tout ce que l'original savait faire doit rester possible ici.
about-origin-heading = Comment il a été réalisé
about-origin-body-1 = Le portage de Python vers Rust a été réalisé avec l'aide de l'IA.
about-origin-body-2 = Ce que cela ne signifie pas mérite d'être dit tout aussi clairement : le comportement est couvert par une suite de tests automatisés et par un plan de tests matériels. La version Windows en particulier a été vérifiée en détail, car les pilotes de périphérique en font la plateforme la plus complexe ; macOS et Linux n'ont en revanche pas pu être testés de manière exhaustive. Veuillez considérer ce logiciel comme expérimental.
about-license-heading = Licence
about-license-body = Ce portage est publié sous la licence MIT, la même licence que le projet d'origine. La notice de copyright d'origine est conservée telle quelle, comme cette licence l'exige.
about-license-full = Texte de la licence
about-third-party-heading = Licences des bibliothèques utilisées
about-third-party-body = Ce programme est lié statiquement : le code de chaque bibliothèque qu'il utilise se trouve dans l'exécutable, et leurs licences exigent que les mentions de droit d'auteur l'accompagnent. Elles sont reproduites ci-dessous dans leur intégralité, une entrée par paquet.
about-third-party-toggle = Afficher toutes les mentions de licence tierces
about-privacy-heading = Données
about-privacy-body-1 = Ce programme s'exécute sur votre propre ordinateur. Il n'envoie aucune donnée d'utilisation, il n'y a ni télémétrie ni compte obligatoire. Rien de ce que vous faites ici n'est signalé où que ce soit.
about-privacy-body-2 = Deux fonctions établissent toutefois d'elles-mêmes des connexions, et le correspondant apprend alors votre adresse IP : les données d'assistance GPS sont téléchargées chez Sony, et la liste des applications chez GitHub. Sans ces fonctions, le programme reste entièrement hors ligne.
about-privacy-body-3 = Ce qui est lu depuis l'appareil photo — dont son numéro de série et les mots de passe Wi-Fi enregistrés — ne quitte pas votre ordinateur. Si vous l'écrivez dans un fichier, faites attention à qui vous transmettez ce fichier.
about-privacy-body-4 = Aucun service n'est exploité derrière ce programme et aucune donnée ne parvient à ses auteurs ; il n'y a donc ici rien à demander ni à faire effacer. Ce que les deux services cités font des adresses qu'ils voient relève de leurs propres déclarations de confidentialité, non de celle-ci.
about-legal-heading = Mentions légales
about-legal-trademark = Ce projet est indépendant. Il n'est ni réalisé par Sony, ni approuvé par Sony, ni lié à Sony de quelque manière que ce soit. « Sony », « PlayMemories » et les noms des modèles d'appareils photo sont des marques de Sony Group Corporation, mentionnées ici uniquement pour indiquer avec quels appareils cet outil dialogue.
about-legal-own-device = Ce programme est destiné à être utilisé avec des appareils qui vous appartiennent.
about-legal-warranty = Ce que fait cet outil sort de ce que Sony prévoit pour ces appareils et peut mettre fin à votre garantie. Le logiciel lui-même est fourni sans aucune garantie et son utilisation se fait à vos propres risques. La mesure dans laquelle la rétro-ingénierie d'un appareil vous appartenant est permise dépend du pays.
about-links-heading = Projets
about-link-original = Sony-PMCA-RE — le projet d'origine
about-link-fwtool = fwtool.py — les outils firmware sur lesquels repose la génération des images de mise à jour
about-link-tweak = OpenMemories: Tweak — l'application recommandée pour les caméras qui la prennent en charge
about-link-applist = OpenMemories: App list — d'autres applications
about-link-platform = OpenMemories: Platform — le corps de l’updater fourni
about-link-devices = Modèles de caméras pris en charge
about-link-nexhack = Nex-Hack — les recherches communautaires sur lesquelles repose ce projet
about-link-zve10 = zv-e10-re — recherches sur les interfaces et la mémoire de réglages de la ZV-E10
about-version = Version
btn-copy-url = Copier l'adresse
msg-url-copied = Adresse copiée dans le presse-papiers.

# Avertissement de sécurité
notice-experimental = Logiciel expérimental. Utilisation à vos propres risques.

# Paramètres réseau (GPS / Wi-Fi / diffusion en direct)
gps-downloading = Téléchargement des données d'assistance GPS…
gps-writing = Écriture des données d'assistance GPS sur l'appareil…
gps-bad-size = Les données d'assistance GPS ont une taille inattendue et n'ont pas été écrites.
wifi-read = Lecture des points d'accès de diffusion en direct depuis l'appareil…
wifi-written = Points d'accès de diffusion en direct écrits.
stream-read = Lecture des paramètres de diffusion en direct depuis l'appareil…
stream-written = Paramètres de diffusion en direct écrits.
net-secret-warning = Attention : ce fichier contient des mots de passe Wi-Fi et des jetons d'accès en clair.
wifi-secret-warning = L'appareil photo enregistre ces mots de passe sans chiffrement et ils sont affichés ici tels qu'ils sont enregistrés. Ils restent sur cet ordinateur ; si vous les enregistrez dans un fichier, faites attention à qui vous le transmettez.
gps-heading = Données d'assistance GPS
gps-hint = Faites glisser un fichier ici, saisissez un chemin ou laissez le champ vide pour obtenir les données actuelles auprès de Sony.
btn-gps-devices = Appareils compatibles
btn-gps-devices-hide = Masquer les appareils compatibles
gps-devices-note = Ces modèles disposent d'un module GPS intégré. Pour beaucoup d'entre eux, seule la variante dont le nom se termine par V en est équipée ; les variantes sans V n'ont pas de GPS.
gps-devices-compact = Appareils compacts (DSC)
gps-devices-slt = Appareils SLT (monture A)
gps-devices-action = Caméras d'action
gps-devices-camcorder = Caméscopes (Handycam)
gps-col-variant = Variante GPS
gps-col-arch = Architecture
gps-col-module = Module GPS
gps-col-apps = Prise en charge des applis
gps-yes = oui
gps-no = non
gps-unknown = inconnu
gps-devices-untested = GPS présent, prise en charge non testée
gps-devices-untested-note = Ces modèles disposent d'un module GPS intégré mais ne figurent pas dans la liste des appareils d'OpenMemories. Le fonctionnement de cette application avec eux n'a pas été testé ; leur architecture et la prise en charge des applis sont donc inconnues.
wifi-heading = Points d'accès pour la diffusion en direct
wifi-hint = Ce sont les points d'accès utilisés par la fonction de diffusion en direct, et non les paramètres Wi-Fi de l'appareil. Un boîtier sans fonction de diffusion refuse cette commande.
wifi-multi = Liste à plusieurs points d'accès
wifi-none = Aucun point d'accès enregistré.
wifi-writing = Écriture des points d'accès de diffusion en direct sur l'appareil…
wifi-confirm = Cela remplace tous les points d'accès enregistrés sur l'appareil. Continuer ?
wifi-empty-refused = Écriture d'une liste vide refusée : elle effacerait tous les points d'accès enregistrés.
stream-heading = Diffusion en direct
stream-writing = Écriture des paramètres de diffusion en direct sur l'appareil…
stream-confirm = Cela écrase les paramètres de diffusion en direct de l'appareil. Continuer ?
stream-invalid-json = Les paramètres ne sont pas du JSON valide :

# Mise à jour du firmware et magasin officiel
firmware-warning = Attention : cela écrase le firmware de l'appareil. N'interrompez pas la mise à jour une fois lancée : ne débranchez pas le câble USB et n'éteignez pas l'appareil. Utilisez uniquement un fichier de firmware destiné exactement à ce modèle d'appareil, et assurez-vous que la batterie est suffisamment chargée. Sinon, l'appareil peut devenir définitivement inutilisable.
firmware-untested = Non testé : le flashage du micrologiciel a été porté mais jamais essayé sur un appareil réel. Une écriture ratée ou interrompue est irréversible ; considérez donc tout ce qui figure sur cette page comme non éprouvé.
firmware-confirm-required = Écriture refusée sans --confirm.
firmware-truncated = Le fichier de firmware est tronqué : la charge utile FDAT dépasse la fin du fichier.
firmware-updating = Mise à jour du firmware :
firmware-invalid = Le fichier n'est pas une image de firmware Sony valide
firmware-dat-path = Fichier de firmware (.dat)
firmware-drop-hint = Déposez ici un fichier de firmware .dat
firmware-file-required = Choisissez d'abord un fichier de firmware.
firmware-check-ok = L'appareil accepte cette image. Rien n'a été écrit.
firmware-cancelled = Flashage annulé. L'appareil n'a pas été modifié.
firmware-current-version = Version installée
firmware-new-version = Version de cette image
firmware-models-hint = Un appareil est compatible si le shell de l'updater peut l'atteindre.
btn-firmware-check = Vérifier l'image
btn-firmware-flash = Flasher le firmware
firmware-own-device = Confirmez que vous avez conscience de ces risques et que vous mettez à jour uniquement votre propre appareil.
btn-firmware-accept = Risques compris – flasher

# Outils de fichiers : les opérations qui n'ont besoin d'aucun appareil.
tools-hint = Ces opérations travaillent sur des fichiers et ne touchent jamais à l'appareil ; rien ici ne peut l'endommager. Ce sont les mêmes que celles que pmca-console propose sous les noms apk2spk, spk2apk, print-backup, print-usagelog, guess-firmware et xpd-checksum.
tools-operation = Opération
tools-apk2spk = Emballer un .apk dans un conteneur .spk
tools-spk2apk = Déballer un .spk en .apk
tools-print-backup = Afficher le contenu d'un Backup.bin
tools-print-usagelog = Afficher le contenu d'un Usagelog.bin
tools-guess-firmware = Trouver la charge utile dans un .dat de firmware
tools-xpd-checksum = Calculer la somme de contrôle CIC d'un fichier
tools-input = Fichier d'entrée
tools-file-required = Choisissez d'abord les fichiers dont cette opération a besoin.
tools-apk-invalid = Le fichier n'a pas pu être emballé en .spk
tools-spk-invalid = Le fichier n'est pas un paquet .spk valide
tools-backup-invalid = Le fichier n'est pas une image Backup.bin valide
tools-usagelog-invalid = Le fichier n'est pas un journal d'utilisation valide
tools-reset-value = valeur de réinitialisation :
tools-fdat-found = Charge utile trouvée à l'offset { $offset }, taille { $size }.
tools-checksum = Somme de contrôle
tools-written = Écrit dans { $path }

# Renvois vers la documentation du projet
docs-compatibility = Compatibilité des appareils
docs-compatibility-hint = Le projet d'origine a fourni un aperçu des appareils testés. Il porte surtout sur des modèles anciens et se trouve à cette adresse :
devices-note = Données reprises de l'aperçu du projet d'origine. Le nombre à côté de la prise en charge des applis est la version de la plateforme d'applications de l'appareil.
devices-more-note = Les appareils suivants n'ont pas été testés explicitement avec ce programme. Une coche dans la dernière colonne signifie que la communauté a signalé leur utilisation. Si le shell de service est en plus marqué ✖, cette utilisation a échoué.
btn-more-models = Autres appareils
btn-more-models-hide = Masquer les autres appareils
check-updater-yes = Votre appareil { $model } est compatible : selon la liste des appareils, le shell de l'updater peut l'atteindre.
check-updater-no = Votre appareil { $model } n'est pas compatible : selon la liste des appareils, le shell de l'updater ne peut pas l'atteindre.
check-updater-unknown = La compatibilité de votre appareil { $model } est inconnue : il ne figure dans aucune liste d'appareils de ce programme, on ne sait donc pas si le shell de l'updater peut l'atteindre.
check-apps-no = Votre appareil { $model } n'est pas compatible : selon la liste des appareils, il ne prend pas en charge les applis.
check-apps-untested = La compatibilité de votre appareil { $model } est inconnue : il figure dans la liste des appareils, mais la prise en charge des applis n'est pas confirmée.
check-apps-unknown = La compatibilité de votre appareil { $model } est inconnue : il ne figure dans aucune liste d'appareils de ce programme, on ne sait donc pas s'il prend en charge les applis.
check-gps-no = Votre appareil { $model } n'est pas compatible : il ne figure pas dans la liste des appareils dotés d'un récepteur GPS.
check-gps-untested = La compatibilité de votre appareil { $model } est inconnue : selon la liste des appareils, il possède un récepteur GPS, mais le transfert vers ce modèle n'a pas été testé.
check-gps-unknown = La compatibilité de votre appareil { $model } est inconnue : il ne figure dans aucune liste d'appareils de ce programme, on ne sait donc pas s'il possède un récepteur GPS.
check-unidentified = L'appareil connecté n'a pas pu être identifié ; sa compatibilité est donc inconnue.
check-proceed = Voulez-vous continuer malgré tout ?
check-cancelled = Opération annulée. L'appareil n'a pas été modifié.
devices-col-confirmed = Confirmé
devices-group-ilc = Appareils à objectifs interchangeables (montures E et A)
devices-group-remote = Télécommandes Live-View
shutter-heading = Compteur de déclenchements
shutter-hint = L'appareil compte chaque déclenchement de l'obturateur et inscrit la valeur actuelle dans chaque photo qu'il enregistre (ARW, JPEG ou HEIF). Sélectionnez la photo la plus récente pour lire la valeur actuelle. Le fichier est uniquement lu, jamais modifié.
shutter-source = L'emplacement du compteur dans le fichier varie selon le modèle. Les positions proviennent de recherches documentées publiquement (ExifTool) et ne sont pas confirmées pour tous les modèles.
shutter-drop-hint = Déposez ici une photo (ARW, JPEG, HEIF)
btn-shutter-read = Analyser la photo
shutter-file-required = Veuillez d'abord sélectionner une photo.
shutter-file = Fichier
shutter-taken = Date de prise de vue
shutter-count = Déclenchements
shutter-type = Obturateur
shutter-type-mechanical = Mécanique
shutter-type-electronic = Électronique
shutter-electronic-hint = Cette photo a été prise avec l'obturateur électronique. Sur de nombreux modèles, ces prises de vue ne sont pas comptées ; la valeur indique alors le compteur au dernier déclenchement mécanique.
shutter-not-sony = Cette photo n'a pas été prise avec un appareil Sony. Aucun compteur de déclenchements ne peut en être lu.
shutter-unknown-model = On ne sait pas où ce modèle enregistre le compteur de déclenchements dans le fichier ; aucune valeur n'est donc affichée.
shutter-not-recorded = Le fichier ne contient aucun compteur lisible. C'est le cas des photos enregistrées ou exportées par un logiciel de retouche et, sur certains modèles, des prises de vue avec l'obturateur électronique. Sélectionnez le fichier original sur la carte mémoire.
shutter-model-mismatch = La photo provient d'un autre modèle que l'appareil connecté.
shutter-invalid = Le fichier n'est pas une photo lisible
file-filter-photo = Photos (ARW, JPEG, HEIF)
usage-log-heading = Journal d'utilisation
usage-log-hint = L'appareil tient un journal interne dans un format non documenté. Il est enregistré tel quel ; l'appareil est uniquement lu.
usage-log-written = Écrit dans
btn-usage-log-info = À propos du journal d'utilisation
btn-usage-log-info-hide = Masquer l'explication
usage-log-info-what-heading = De quoi s'agit-il ?
usage-log-info-what = Sony appelle ce journal « KikiLog » en interne. L'appareil l'écrit lui-même ; ce programme se contente de le lire. Aucune description publique du format n'est connue.
usage-log-info-read = La lecture ne modifie rien : plusieurs téléchargements successifs étaient identiques octet par octet.
usage-log-info-flush-heading = Pourquoi des entrées récentes manquent
usage-log-info-flush = Les nouvelles entrées n'apparaissent souvent pas immédiatement : l'appareil peut ne les enregistrer qu'à l'extinction, parfois sur plusieurs cycles marche/arrêt. Éteignez puis rallumez l'appareil, téléchargez à nouveau et recommencez jusqu'à ce qu'aucune nouvelle entrée n'apparaisse.
usage-log-info-content-heading = Ce qu'on peut y lire
usage-log-info-shutter = Déclenchements : certains événements correspondent chacun à une prise de vue. Leur nombre ne donne pourtant pas le compteur de déclenchements : le journal a une taille fixe et ne couvre pas forcément toute la durée d'utilisation de l'appareil. L'appareil conserve le total ailleurs et l'inscrit dans les photos qu'il enregistre ; il peut être lu dans la section « Compteur de déclenchements ».
usage-log-info-clock = Les horodatages ne valent que ce que vaut l'horloge de l'appareil. Après un changement de batterie, l'horloge peut revenir en arrière et produire des entrées portant la date initiale de l'appareil.
usage-log-info-unknown = Aucune description publique des numéros d'événement n'est connue. Le programme affiche les numéros sans les interpréter.
usage-log-info-privacy-heading = Confidentialité
usage-log-info-privacy = Le fichier contient les numéros de série des cartes mémoire insérées. Ne le partagez pas et ne le publiez pas à la légère.
btn-usage-log = Enregistrer le journal
usagelog-size = Zone du journal
usagelog-cursor = Position d'écriture
usagelog-spare = Deuxième zone
usagelog-count = Entrées
usagelog-events = Événements par numéro
usagelog-entries = Entrées dans l'ordre
btn-copy-link = Copier le lien
msg-link-copied = Lien copié dans le presse-papiers.

# Messages d'erreur du backend, adressés par leur code stable.
# La partie variable (chemin, octet d'état, message de l'appareil) est ajoutée
# par l'interface ; ces textes forment donc des phrases complètes sans elle.
# Il n'existe volontairement pas de `err-operation` : ce code désigne un
# message déjà traduit, qui doit être affiché tel quel.
err-usb-no-device = Aucun appareil photo détecté. Connectez un appareil et allumez-le.
err-usb-camera-not-ready = L'appareil est connecté mais pas prêt : il est éteint ou attend sur son écran le choix du mode USB. Allumez-le ou choisissez « Stockage de masse (MSC) » sur l'appareil.
camera-choose-usb-mode = L'appareil attend le choix du mode USB. Choisissez « Stockage de masse (MSC) » sur l'appareil.
err-usb-too-many-devices = Plusieurs appareils sont connectés. Déconnectez tous les autres.
err-usb-transfer = Le transfert USB a échoué. Essayez un autre câble ou port, et évitez les concentrateurs USB.
err-usb-stall = L'appareil a interrompu le transfert USB.
err-usb-protocol = L'appareil a envoyé une réponse inattendue.
err-usb-msc-status = L'appareil a rejeté une commande de stockage.
cli-error-prefix = erreur
err-usb-driver = Windows a attribué cet appareil à un pilote qui n'autorise pas l'accès direct. Ce mode nécessite le pilote WinUSB, que l'application peut attribuer elle-même — mais uniquement avec des droits d'administrateur.
err-needs-elevation = Attribuer ou libérer un pilote nécessite les droits d’administrateur, et la demande de Windows a été refusée. Réessayez avec « Démarrer l’assistant pilote » ; rien n’est à changer sur l’appareil.
err-usb-no-mass-storage-volume = L'appareil est connecté en mode stockage de masse mais ne présente aucun lecteur. Réveillez-le, confirmez le mode USB sur son écran s'il le demande et vérifiez qu'une carte mémoire est insérée. Ne changez pas de pilote : ce mode a besoin du pilote standard de Windows.
err-usb-msc-sense = L'appareil a signalé une erreur de stockage.
err-usb-ptp-response = L'appareil a rejeté la commande.
err-usb-unsupported = Ce modèle d'appareil ne prend pas en charge cette fonction.
err-usb-updater-status = Le programme de mise à jour du micrologiciel a rejeté l'opération.
err-usb-senser = Une commande en mode service a échoué.
err-market-core = Un fichier de données de l'appareil n'a pas pu être lu.
err-market-json = Les données reçues sont mal formées.
err-market-missing-field = Un champ attendu est absent de la réponse.
err-market-no-apk = Aucune application n'est sélectionnée pour l'installation.
err-market-http = La requête réseau a échoué.
err-market-io = Le serveur d'installation local a échoué.
err-market-tls = La connexion chiffrée à l'appareil n'a pas pu être établie.
err-apk = Le fichier de l'application n'a pas pu être lu.
err-io = Un fichier n'a pas pu être lu ou écrit.
err-json = L'appareil a envoyé des données mal formées.
err-unknown-locale = Cette langue n'est pas disponible.
err-catalog-encoding = Le fichier de traduction n'est pas enregistré en texte UTF-8. Enregistrez-le à nouveau avec l'encodage UTF-8.
err-internal = Une erreur interne s'est produite. Il s'agit d'un bogue de l'application.

## Function analysis
nav-analysis = Analyse
analysis-title = Analyse des fonctions
analysis-experimental = Expérimental. Cette fonction est nouvelle et n’a été essayée que sur peu d’appareils.
analysis-own-hardware = À utiliser uniquement sur votre propre appareil. Le mode service est déjà une intervention que Sony ne prévoit pas ; la responsabilité vous incombe.
analysis-explanation-toggle = Explication : but, déroulement, risques
analysis-purpose-heading = À quoi sert-elle ?
analysis-purpose = L’analyse des fonctions documente dans quelle valeur de sa mémoire de réglages (sauvegarde) l’appareil enregistre l’un de ses propres réglages de menu. Le résultat décrit la mémoire de réglages de cet appareil ; il peut être comparé avec d’autres appareils, mais ne change rien à ce qu’un appareil propose.
analysis-how-heading = Comment se déroule-t-elle ?
analysis-how = Pour chaque valeur de menu indiquée, le programme lit une fois la mémoire de réglages complète, chaque fois dans une courte session du mode service. Entre deux lectures, vous débranchez le câble et changez le réglage dans le menu de l’appareil ; l’appareil reste allumé. Pour finir, la première valeur est relue. Les valeurs qui suivent le menu et reviennent à leur valeur de départ sont des candidates. Les valeurs qui changent d’elles-mêmes (compteurs, horloge) sont écartées.
analysis-requirements-heading = Prérequis
analysis-requirements = L’appareil est relié en USB, son mode USB est sur « Auto », et après chaque session vous choisissez « Stockage de masse (MSC) » sur l’appareil. Sous Windows, le programme a besoin des droits d’administrateur, car il attribue brièvement un pilote pour le mode service. Chargez la batterie ; chaque session dure environ une minute.
analysis-read-heading = Étapes de lecture
analysis-read = Les captures ne font que lire. Rien ne change dans les réglages de l’appareil. Le risque est faible mais pas nul : passer en mode service est déjà une intervention que Sony ne prévoit pas.
analysis-write-heading = Étape d’écriture (test d’écriture)
analysis-write = Uniquement si vous l’activez expressément : le programme écrit la valeur trouvée dans l’appareil, vous vérifiez dans le menu après un redémarrage si elle agit, puis la valeur d’origine est réécrite. Seule une valeur que l’appareil a lui-même enregistrée est écrite, et seulement si exactement une candidate a été trouvée. L’ancienne valeur est sauvegardée au préalable. Cette étape modifie néanmoins l’appareil ; une erreur peut le laisser dans un état que seul le service après-vente peut réparer.
analysis-legal-heading = Mentions légales
analysis-legal = Le logiciel est fourni sans aucune garantie (licence MIT). Les interventions sur l’appareil peuvent affecter sa garantie. N’utilisez la fonction que sur des appareils qui vous appartiennent. Le rapport d’analyse ne contient ni numéro de série ni rien de la mémoire de réglages hormis les candidates trouvées ; il n’est jamais envoyé automatiquement.
analysis-start-heading = Nouvelle analyse
analysis-setting-label = Réglage de menu, tel que l’appareil le nomme
analysis-values-label = Valeurs de menu, séparées par des virgules
analysis-values-hint = La première valeur est celle réglée actuellement. Deux à huit valeurs ; chacune coûte une session du mode service.
analysis-write-option = Ensuite, effectuer le test d’écriture (modifie l’appareil)
analysis-write-warning = Le test d’écriture modifie l’appareil : il écrit une valeur dans sa mémoire de réglages puis réécrit la valeur d’origine. Uniquement sur votre propre appareil, à vos risques.
analysis-write-consent = L’appareil m’appartient et j’en assume le risque.
btn-analysis-write-enable = Activer le test d’écriture
btn-analysis-start = Démarrer l’analyse
btn-analysis-snapshot = Lire la sauvegarde
btn-analysis-new = Nouvelle analyse
analysis-discard-confirm = Abandonner l’analyse en cours ? Le rapport obtenu jusqu’ici reste enregistré.
analysis-access-read = LECTURE
analysis-access-write = ÉCRITURE
analysis-access-user = MANUEL
analysis-access-local = PC
analysis-step-snapshot = Lire la sauvegarde, le réglage est sur « { $value } »
analysis-step-change = Passer sur « { $value } » dans le menu de l’appareil
analysis-step-evaluate = Comparer les captures
analysis-step-write = Écrire dans l’appareil la valeur de « { $value } »
analysis-step-check = Éteindre et rallumer l’appareil, lire le menu (attendu : « { $value } »)
analysis-step-restore = Réécrire la valeur d’origine de « { $value } »
analysis-step-report = Enregistrer le rapport d’analyse
analysis-do-prepare = PRÉPARATION : 1. Dans le menu de l’appareil, réglez « { $setting } » sur « { $value } » et quittez le menu avec la touche MENU pour que l’appareil enregistre le réglage. 2. Laissez l’appareil allumé. 3. Branchez le câble et choisissez « Stockage de masse (MSC) » sur l’appareil.
analysis-do-change = À VOUS : 1. Débranchez le câble. 2. L’appareil reste allumé. 3. Dans le menu de l’appareil, réglez « { $setting } » sur « { $value } » et quittez le menu avec la touche MENU pour que l’appareil enregistre le réglage. 4. Rebranchez le câble et choisissez « Stockage de masse (MSC) ».
analysis-do-check = À VOUS : 1. Débranchez le câble. 2. Éteignez puis rallumez l’appareil. 3. Regardez ce que le menu affiche pour « { $setting } ».
analysis-do-reconnect = Rebranchez le câble et choisissez « Stockage de masse (MSC) » sur l’appareil.
analysis-reading = Capture { $index } sur { $total } : NE débranchez PAS le câble pendant la lecture.
analysis-cli-continue = Entrée pour continuer, q pour annuler :
analysis-confirm-word = oui
analysis-cli-confirm = Tapez « { $word } » pour confirmer :
analysis-write-declined = Non confirmé. Rien n’a été écrit.
analysis-aborted = Annulé. Rien n’a été écrit.
analysis-snapshot-done = Capture lue. L’appareil a quitté le mode service.
analysis-result = { $compared } valeurs comparées : { $count } candidates, { $running } valeurs ont changé d’elles-mêmes.
analysis-candidate-consistent = correspond clairement au menu
analysis-candidate-inconsistent = ne correspond pas clairement
analysis-write-preview-heading = Ceci sera écrit
analysis-write-preview = Valeur { $id } ({ $size } octets) : de { $old } (« { $old-label } ») à { $new } (« { $new-label } »). L’appareil a lui-même enregistré la nouvelle valeur lorsque « { $new-label } » était réglé.
btn-analysis-write = Écrire
btn-analysis-skip-write = Terminer sans test d’écriture
analysis-written = Valeur écrite, relue et enregistrée durablement.
analysis-menu-question = Qu’affiche le menu maintenant ?
analysis-menu-other = autre chose
btn-analysis-menu-confirm = Valider
btn-analysis-restore = Réécrire la valeur d’origine
analysis-restored = Valeur d’origine réécrite et enregistrée durablement.
analysis-survived = La valeur écrite était toujours enregistrée après le redémarrage.
analysis-not-survived = La valeur écrite n’était plus enregistrée après le redémarrage.
analysis-report-saved = Rapport d’analyse enregistré : { $path }
analysis-report-hint = Le rapport ne contient ni numéro de série ni sauvegarde. Vous pouvez le transmettre pour que le résultat soit vérifié et documenté.
btn-analysis-export = Exporter le rapport…
file-filter-analysis = Rapport d’analyse
analysis-pending = Un test d’écriture n’a pas été terminé. La valeur d’origine est sauvegardée et peut être réécrite.
analysis-pending-none = Aucun test d’écriture inachevé n’est enregistré pour cet appareil.
analysis-invalid-setting = Indiquez le réglage de menu tel que l’appareil le nomme.
analysis-invalid-values = Indiquez de deux à { $max } valeurs de menu différentes. La première est celle réglée actuellement.
analysis-already-complete = Toutes les captures ont déjà été lues.
analysis-other-camera = Ce n’est pas le même appareil que lors de la première capture. L’analyse a été arrêtée.
analysis-write-not-allowed = Le test d’écriture n’est pas possible dans cet état.
analysis-camera-changed = L’appareil contient désormais une autre valeur que lors de la première capture. Rien n’a été écrit.
analysis-write-refused = L’appareil a refusé la valeur ; tout est comme avant. ({ $detail })
analysis-refuse-none = Test d’écriture impossible : aucune valeur n’a suivi le menu.
analysis-refuse-ambiguous = Test d’écriture impossible : { $count } valeurs ont suivi le menu, aucune ne peut être attribuée avec certitude.
analysis-refuse-inconsistent = Test d’écriture impossible : la valeur ne correspond pas clairement aux valeurs du menu.
analysis-refuse-size = Test d’écriture impossible : la valeur fait { $size } octets ; seules les petites valeurs sont écrites.
analysis-refuse-read-only = Test d’écriture impossible : l’appareil marque la valeur en lecture seule.
analysis-refuse-protected = Test d’écriture impossible : l’appareil marque la valeur comme protégée.
settings-analysis-heading = Analyse des fonctions
settings-analysis-purpose = L’analyse des fonctions documente dans quelle valeur de sa mémoire de réglages l’appareil enregistre l’un de ses propres réglages de menu. Elle s’adresse à celles et ceux qui veulent comprendre la mémoire de réglages de leur modèle et partager les résultats.
settings-analysis-why-off = Elle est désactivée parce que l’usage normal n’en a pas besoin, qu’elle met l’appareil plusieurs fois en mode service et que, sur demande, elle écrit aussi dans l’appareil. Elle est expérimentale et n’a été essayée que sur peu de modèles.
settings-analysis-toggle = Activer l’analyse des fonctions (entrée « Analyse » dans le menu)
settings-analysis-confirm = Activer l’analyse des fonctions ? Elle met l’appareil plusieurs fois en mode service et peut, sur demande, écrire dans l’appareil. À utiliser uniquement sur votre propre appareil.
btn-analysis-enable = Activer
settings-analysis-on = L’analyse des fonctions est activée.
settings-analysis-off = L’analyse des fonctions est désactivée.
analysis-needs-admin = Chaque capture a besoin des droits d’administrateur pour le mode service. Windows les demande une fois par session du programme ; vous pouvez démarrer l’assistant pilote maintenant ou attendre que la première capture le demande.
analysis-changed-hint = À vérifier : aucune candidate n’a été trouvée. Les valeurs suivantes ont changé mais ne sont pas revenues à leur valeur de départ. Si l’une d’elles ne change qu’une seule fois entre deux valeurs, l’appareil n’était probablement pas sur la première valeur du menu lors de la première capture. Répétez alors l’analyse avec la valeur de départ correctement réglée.
analysis-changed-info = Info : les valeurs qui ont changé d’elles-mêmes pendant l’analyse (p. ex. compteurs, horloge) sont ignorées. Rien à faire.
elevation-admin = Administrateur
elevation-standard = Droits standard
elevation-admin-hint = Le programme s’exécute avec les droits d’administrateur et peut attribuer le pilote dont le mode service a besoin.
elevation-standard-hint = Le programme s’exécute avec des droits standard. Pour le mode service, les réglages et l’analyse des fonctions, il démarre au besoin un assistant pilote avec droits d’administrateur ; Windows demande une fois par session. Cliquez pour le démarrer.
driver-helper-declined = La demande de droits d’administrateur a été refusée. Sans eux, le pilote du mode service ne peut pas être attribué ; rien n’a été modifié.
btn-remove-foreign-drivers = Supprimer les anciens pilotes
btn-not-now = Pas maintenant
btn-ok = OK
foreign-drivers-found = Windows conserve des pilotes que Zadig ou un outil similaire a installés pour des appareils Sony : { $packages }. Windows leur confie l’appareil photo avant que ce programme puisse attribuer son propre pilote ; le mode service échoue alors et l’appareil peut ne pas apparaître comme lecteur. Les supprimer maintenant ? L’appareil utilisera ensuite les pilotes fournis par Windows. La suppression demande une fois les droits d’administrateur.
foreign-drivers-removed = Les anciens pilotes ont été supprimés. Débranchez l’appareil photo, puis rebranchez-le.
foreign-drivers-remaining = Ces pilotes n’ont pas pu être supprimés : { $packages }. Redémarrez Windows et réessayez, ou désinstallez le périphérique avec son pilote dans le Gestionnaire de périphériques.
foreign-drivers-hint = Des pilotes installés par Zadig ou un outil similaire pour des appareils Sony sont encore présents ({ $packages }). Windows leur confie l’appareil photo avant que ce programme puisse attribuer son propre pilote, ce qui peut faire échouer le mode service. Pour les supprimer : pmca-console driver zadig --remove
driver-zadig-none = Aucun pilote de Zadig ou d’un outil similaire n’est installé pour des appareils Sony.
driver-zadig-entry = { $package } ({ $provider }) : { $devices }
driver-zadig-remove-hint = Pour les supprimer : pmca-console driver zadig --remove
driver-zadig-removed = { $count } ancien(s) paquet(s) de pilote supprimé(s). Débranchez l’appareil photo, puis rebranchez-le.
elevation-helper = Assistant pilote actif
elevation-helper-hint = Un assistant doté des droits d’administrateur attribue le pilote du mode service. Il n’a pas de fenêtre et se termine avec le programme ; aucune autre demande ne suivra.
elevation-helper-confirm = Démarrer maintenant l’assistant pilote ? Windows demande une fois les droits d’administrateur, puis plus jusqu’à la fermeture du programme.
analysis-cross-heading = Contre-vérification sur un autre appareil
analysis-cross-intro = Facultatif : vérifie si un autre appareil possède aussi la valeur trouvée et ce qui y est réglé. Lecture seule. Le même numéro ne signifie pas forcément la même fonction ; seul un coup d’œil au menu de l’autre appareil le confirme. Le résultat est ajouté au même rapport d’analyse.
analysis-cross-do = 1. Débranchez l’appareil analysé. 2. Allumez l’autre appareil, réglez son mode USB sur « Auto », branchez le câble et choisissez « Stockage de masse (MSC) ».
btn-analysis-cross = Vérifier un autre appareil
analysis-cross-present = présente, valeur { $value }
analysis-cross-match = correspond à « { $label } »
analysis-cross-nomatch = ne correspond à aucune valeur de menu mesurée
analysis-cross-absent = absente
analysis-cross-same-camera = C’est l’appareil analysé lui-même. Branchez l’autre appareil.
analysis-cross-not-now = La contre-vérification n’est possible qu’une fois toutes les captures lues et aucune valeur écrite en attente de restauration.
analysis-cross-none = Il n’y a aucune candidate à contre-vérifier.
analysis-wrong-camera = L’appareil branché n’est pas celui analysé. Branchez le { $model } ; rien n’a été écrit.
keys-fetching = Téléchargement de la source…
keys-scanning = Recherche du matériel de clés…
keys-complete = Tout le matériel de clés est présent.
keys-incomplete = Il manque encore du matériel de clés.
keys-present = présent
keys-missing = manquant
keys-class-senser = Clés du mode service (shell de service, tweaks, analyse)
keys-class-cic = Clé de somme de contrôle XPD (installation d’applications)
keys-class-spk = Matériel de clés SPK (installation d’applications, apk2spk)
keys-class-fdat = Matériel de clés FDAT (micrologiciel updater-shell)
plugin-heading = Extensions
plugin-intro = Les extensions ajoutent à ce programme des langues, des commutateurs de réglages de l’appareil, des indications de compatibilité, des outils et des diagnostics. Elles proviennent uniquement de fichiers que vous choisissez vous-même : il n’existe pas de boutique d’extensions. Leur code s’exécute dans un bac à sable, sans accès à vos fichiers, au réseau ni au matériel de clés. N’installez que des extensions dont vous faites confiance à l’auteur ; les modifications prennent effet au prochain démarrage.
plugin-none = Aucune extension installée.
plugin-inactive = (active après un redémarrage)
plugin-author = Auteur
plugin-restart = Redémarrez le programme pour que la modification prenne effet.
btn-plugin-install = Installer une extension…
plugin-install-confirm = Cette extension peut :
btn-confirm-plugin-install = Installer l’extension
plugin-installed = L’extension a été installée.
plugin-remove-confirm = Supprimer l’extension « { $name } » ?
plugin-removed = L’extension { $id } a été supprimée.
plugin-perm-languages = ajouter des langues au programme
plugin-perm-camera-settings = modifier des réglages de l’appareil (uniquement entre les deux valeurs qu’elle indique, et seulement après votre confirmation)
plugin-perm-compatibility = ajouter des entrées à l’aperçu de compatibilité
plugin-perm-code = exécuter son propre code dans le bac à sable (outils ou diagnostics)
plugin-perm-host-os = demander quel est le système d’exploitation
plugin-perm-host-cameras = voir quels appareils Sony sont connectés et dans quel mode
plugin-invalid = Ce fichier n’est pas une extension utilisable : { $reason }
plugin-no-folder = Le dossier des données utilisateur est introuvable ; les extensions ne peuvent donc pas être installées.
plugin-not-installed = Aucune extension portant l’identifiant { $id } n’est installée.
plugin-failed = L’extension a échoué : { $reason }
plugin-unknown-entry = Aucune extension chargée ne fournit { $id }.
plugin-run-done = L’extension a terminé.
plugin-run-warning = L’extension a terminé, avec des remarques.
plugin-run-error = L’extension a signalé une erreur.
plugin-tools-heading = Outils des extensions
plugin-tools-hint = Ils proviennent des extensions installées et s’exécutent dans un bac à sable. Une extension ne reçoit que ce que vous lui transmettez et ce qu’elle a déclaré, et ne peut pas modifier l’appareil.
plugin-diagnostics-heading = Diagnostics des extensions
plugin-compat-heading = Issu d’une extension
plugin-col-note = Remarque
file-filter-plugin = Fichier d’extension
keys-not-set-up = Le matériel de clés nécessaire n’est pas configuré.
err-missing-keys = Le matériel de clés nécessaire n’est pas configuré.
keys-heading = Matériel de clés requis
keys-intro = Les clés du mode service, la clé de somme de contrôle XPD et le matériel de clés SPK ne font pas partie de ce programme. Indiquez l’adresse d’une source que vous possédez — un dépôt, une archive ou un fichier — et le programme trouve et importe les éléments qu’il reconnaît. Rien n’est exécuté, et le matériel reste uniquement sur cet ordinateur.
keys-law-notice = Respectez les lois en vigueur là où vous êtes, et ne travaillez qu’avec des appareils et des données qui vous appartiennent.
keys-url-label = Adresse de la source (une par ligne)
keys-url-required = Veuillez indiquer au moins une adresse de source.
btn-keys-import = Importer
btn-import-keys = Configurer le matériel de clés
