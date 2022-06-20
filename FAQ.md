# Généralités sur le projet

## Structuration

* Il faut bien (au minimum) 3 crates : `client`, `shared`, `server` (nommage indicatif).
* Il faut bien un fichier `Cargo.toml` à la racine qui les unifie tous (le mot clef magique sera `[workspace]`
  et `members`).

Un exemple possible se trouve dans ma base d'exemples:
https://github.com/haveneer/rust-quicklook-training/tree/master/rs/full-tree

## Rejet par le serveur fourni des messages de votre client

Si vous rencontrez des « `message : Too large message size` » de la part du serveur fourni, il est bie possible que vous
n'ayez pas bien respecté le format de transfert.

En effet, tel qu'il est décrit
dans [le sujet](https://github.com/haveneer-training/la-patate-chaude#le-protocole-déchange), il y a
bien une part qui est la sérialisation en JSON du message souhaité (comme nous avons vu en cours) mais aussi le JSON
message size préalable qui doit contenir le taille du message que vous lui envoyer.
En effet, dans le même tuyau (même stream TCP) pourra circuler une suite de différents messages : M1 M2 M3; pour qu'il
soit plus facile de séparer un flot d'octets en différents messages, chacun des messages est préfixé par sa taille : S1
M1 S2 M2 S3 M3, ce qui permet à chaque fois de lire le nombre Si (entier de taille fixe) puis les Si octets devant
ensuite être décodé en JSON.
Cela demandera quelques primitives d'écriture/lecture sur TcpStream différentes de celles que nous avons vues (
lire un nombre, lire un nombre d'octets définis ; idem pour l'écriture).

Un *timeout* renvoyé par le serveur peut aussi être un comportement différent du même problème.
Pour préciser, si vous n'écrivez pas en ce moment les préfixes Si, ce sont les premiers caractères de votre message qui
sont lus par le serveur comme un entier 32 bits (4 octets). Par exemple un message "Hi" qui ne contient que 2 caractères
(ici 2 octets) sera insuffisant pour reconstruire l'entier 32 bits attendu par le serveur. Il attendra encore 2 octets
jusqu'au timeout.

# Spécifications du serveur

## Mécanisme de `StartGame`

Le mécanisme du start_server / StartGame est peu documenté, car il est aussi relativement libre (et ne contraint pas les
agents qui est la partie imposée).
Vous pouvez employer le mécanisme de votre choix pour lancer la partie. Dans mon cas, c'était par un message `StartGame`
envoyé par TCP par un autre programme (qui est écouté avec les clients qui entrent dans la partie). Vous pouvez aussi le
faire par une saisie clavier, la présence d'un fichier à un endroit précis... Tout est possible.

## Qu'est-ce que le champ `stream_id`

Le `stream_id` représente l'adresse réseau du joueur (ip + port). Il est là principalement à titre informatif pour
identifier les clients.

## *broadcast* de messages

Pour le *broadcast* de messages (tels que `RoundSummary`), il n'est effectivement pas directement disponible au niveau
de
TCP.

Il y a plusieurs approches possibles :

- la plus simple est une boucle sur tous clients avec un message (synchrone)
- une peu plus élaborée (ce qui n'est pas indispensable) est l'emploi de thread pour chaque client avec une primitive
  de type *barrier* pour attendre que tout le monde ait fini

