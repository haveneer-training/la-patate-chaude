### [HashCash](https://fr.wikipedia.org/wiki/Hashcash) : Le challenge de preuve de travail

En entrée du challenge, soient:

* `message`: une chaîne de caractères en UTF8,
* `complexity`: un nombre de type `u32`

Soit une valeur nommée `seed` (de type `u64`).
Soit la chaîne de caractères jouxtant la valeur `seed` écrite en une chaine de 16 caractères
hexadécimaux (lettres en majuscules et complétés à gauche par des `0`) et concaténée à un message fourni en entrée du
challenge.

L'objet est de trouver une valeur de `seed` telle que le hashage [MD5](https://fr.wikipedia.org/wiki/MD5) (écrit sur 32
caractères hexadécimaux, lettres en majuscules et complétés à gauche par des `0`) comprenne au moins `complexity` bits
égaux à `0` (⚠️ bits et non caractères hexadécimaux).

Le résultat attendu est de la forme:

| seed  | hashcode |
|-------|----------|
| `u64` | `String` |

Par exemple, pour l'entrée

| complexity | message     |
|------------|-------------|
| 9          | `"hello"`   |

Nous avons:

| seed               | hashcode                             |
|--------------------|--------------------------------------|
| `000000000000034C` | `"00441745D9BDF8E5D3C7872AC9DBB2C3"` |

qui correspond bien à

```shell
$ echo -n "000000000000034Chello" | md5sum | tr a-z A-Z
```

```
00441745D9BDF8E5D3C7872AC9DBB2C3  -
```


Nous utiliserons ainsi les types suivants en entrée et sortie de du challenge `MD5HashCash`. 

```rust
struct MD5HashCashInput {
    // complexity in bits
    complexity: u32,
    // message to sign
    message: String,
}

struct MD5HashCashOutput {
    // Seed used to solve the challenge
    seed: u64,
    // hashcode found using seed + message
    hashcode: String,
}
```