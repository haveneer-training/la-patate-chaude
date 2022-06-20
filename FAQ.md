# Généralités sur le projet

## Structuration

* Il faut bien (au minimum) 3 crates : `client`, `shared`, `server` (nommage indicatif).
* Il faut bien un fichier `Cargo.toml` à la racine qui les unifie tous (le mot clef magique sera `[workspace]`
  et `members`).

Un exemple possible se trouve dans ma base d'exemples:
https://github.com/haveneer/rust-quicklook-training/tree/master/rs/full-tree

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

Pour le *broadcast* de messages (tels que `RoundSummary`), il n'est effectivement pas directement disponible au niveau de
TCP.

Il y a plusieurs approches possibles :

- la plus simple est une boucle sur tous clients avec un message (synchrone)
- une peu plus élaborée (ce qui n'est pas indispensable) est l'emploi de thread pour chaque client avec une primitive
  de type *barrier* pour attendre que tout le monde ait fini

