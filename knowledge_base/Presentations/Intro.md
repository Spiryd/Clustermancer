# Algorytmy Klasteryzacji Strumieni Danych

## Co oznacza Tytuł?

### [[Key Concepts#Data Clustering |Klasteryzacja Danych]]
Process w dziedzinie uczenia maszynowego, który polega na grupowaniu zbioru w taki sposób, aby elementy w tej samej grupie (nazywanej klastrem) były bardziej podobne do siebie niż do elementów w innych grupach.
### [[Key Concepts#Data Streams |Strumienie Danych]]

Strumienie danych to uporządkowany ciąg obiektów. $S = \{ x_1, x_2, x_3,\dots, x_i, \dots, x_N\}$ gdzie $x_i$ jest $i^{th}$ instancja. Każda instancja($x_i$) jest $d$-wymiarowym wektorem. Długość $N$ strumienia zmierza do nieskończoności.

#### [[Concept Drift]]
Nieprzewidywalna zmiana własności statystycznych obserwacji w strumieniach danych. Cztery kategorie: Sudden, Gradual, Incremental, Recurring. 

## Cel
Celem pracy jest omówienie znanych algorytmów klasteryzacji strumieni danych, zbadanie skuteczności kilku wybranych algorytmów oraz zaproponowanie nowych rozwiązań bazujących na algorytmach próbkowania strumieni danych, w których prawdopodobieństwo dodawania nowo obserwowanych elementów strumienia będzie zależne od atrakcyjności (specyficznie, dynamicznie definiowanej) obserwacji.


