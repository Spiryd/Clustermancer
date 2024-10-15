# Algorytmy Klasteryzacji Strumieni Danych

## Cel

Celem pracy jest omówienie znanych algorytmów klasteryzacji strumieni danych, zbadanie skuteczności kilku wybranych algorytmów oraz zaproponowanie nowych rozwiązań bazujących na algorytmach próbkowania strumieni danych, w których prawdopodobieństwo dodawania nowo obserwowanych elementów strumienia będzie zależne od atrakcyjności (specyficznie, dynamicznie definiowanej) obserwacji.

## Koncepty
### Klasteryzacja Danych

Process w dziedzinie uczenia maszynowego, który polega na grupowaniu zbioru w taki sposób, aby elementy w tej samej grupie (nazywanej klastrem) były bardziej podobne do siebie niż do elementów w innych grupach.

### Strumienie Danych

$S = \{ x_1, x_2, x_3,\dots, x_i, \dots, x_N\}$ gdzie $x_i$ jest $i^{th}$
instancja. Kazda instancja($x_i$) jest $d$-wymiaowym vectorem. Długość $N$ strumienia zmierza do nieskończoności.

## Online Approach vs Classic Approach

|                   | Online                         | Classic                |
| ----------------- | ------------------------------ | ---------------------- |
| Results           | Approximate Results ecceptable | Exact results expected |
| Data Velocity     | Fast                           | Stationary             |
| Data Avaliability | Only one pass can be perfored  | Unbound access to data |
| Storage           | Only a synopsis is stored      | All of the raw data    |

## What's new?



