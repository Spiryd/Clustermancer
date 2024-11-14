# Slide 1

Systemy rozproszone to zbiór komputerów (lub innych urządzeń obliczeniowych) współpracujących ze sobą w celu realizacji wspólnych zadań. W takim systemie urządzenia te komunikują się i wymieniają informacje za pomocą sieci (najczęściej Internetu lub lokalnej sieci LAN). Z perspektywy użytkownika końcowego system rozproszony może wyglądać jak jedno spójne środowisko, mimo że jest złożony z wielu oddzielnych jednostek.

# Slide 2

Oto kluczowe cechy systemów rozproszonych, które warto uwzględnić we wstępie:

1. **Autonomia**: Każdy węzeł (komputer) w systemie działa samodzielnie, ale komunikuje się z innymi węzłami. Jest to odmienna cecha od systemów scentralizowanych, gdzie wszystkie obliczenia wykonuje jeden komputer.
2. **Brak współdzielonej pamięci**: W systemach rozproszonych każda jednostka posiada swoją własną pamięć i zasoby obliczeniowe. Wymiana informacji między nimi odbywa się przez wiadomości lub komunikaty, a nie przez współdzieloną pamięć.
3. **Równoległość i skalowalność**: Węzły mogą działać równolegle, co zwiększa efektywność systemu i umożliwia skalowanie – dodanie nowych jednostek zwiększa moc obliczeniową całego systemu.
4. **Niezawodność**: Systemy rozproszone są zaprojektowane tak, aby mogły działać nawet w przypadku awarii części składowych. Jeśli jeden z węzłów przestanie działać, pozostałe nadal mogą realizować swoje zadania (zależy to jednak od architektury systemu).
5. **Komunikacja i synchronizacja**: Węzły muszą być zsynchronizowane i komunikować się, by zapewnić spójność danych i realizację wspólnych celów. Wyzwania związane z komunikacją (takie jak opóźnienia sieciowe i awarie) są szczególnie istotne w systemach rozproszonych.

Systemy rozproszone są stosowane w wielu dziedzinach, takich jak przetwarzanie w chmurze, systemy IoT, aplikacje webowe, bazy danych, a także w systemach blockchain i algorytmach konsensusu (które, jak rozumiem, stanowią temat główny Twojej prezentacji).

# Slide 3

Konsensus w systemach rozproszonych to proces, który pozwala wszystkim węzłom (komputerom) w systemie osiągnąć zgodę co do określonego stanu lub decyzji, mimo że działają niezależnie i mogą doświadczać awarii. W przypadku systemów rozproszonych jest to kluczowe, ponieważ węzły nie mają współdzielonej pamięci i muszą wymieniać się informacjami, aby osiągnąć wspólne ustalenia.

# Slide 4

Oto główne aspekty konsensusu:

1. **Spójność**: Każdy węzeł powinien dojść do tej samej decyzji, by utrzymać jednolity stan systemu. Konsensus zapewnia, że wszyscy uczestnicy systemu uznają tę samą wersję prawdy, co jest szczególnie ważne w kontekście transakcji finansowych, węzłów blockchain czy rozproszonych baz danych.
2. **Tolerancja na awarie**: Systemy rozproszone mogą doświadczać różnych awarii (np. utraty komunikacji między węzłami, awarii sprzętu). Algorytmy konsensusu są zaprojektowane tak, aby mogły radzić sobie z tego typu problemami, umożliwiając systemowi dalsze działanie nawet w przypadku częściowych awarii.
3. **Zgoda przy rozbieżnościach**: W systemach rozproszonych niektóre węzły mogą mieć rozbieżne opinie na temat stanu systemu (np. na temat tego, które transakcje zostały już zatwierdzone). Algorytmy konsensusu pomagają rozwiązać te rozbieżności, umożliwiając systemowi ustalenie jednego spójnego stanu.
4. **Wydajność i skalowalność**: W zależności od rodzaju algorytmu konsensusu (np. Proof of Work, Paxos, Raft), proces konsensusu może być bardziej lub mniej efektywny i skalowalny. Wydajność konsensusu jest kluczowa dla płynnego działania systemów rozproszonych, zwłaszcza w dużych sieciach.

Podstawowe algorytmy konsensusu – takie jak Paxos, Raft czy inne oparte na konsensusie bizantyjskim (np. PBFT, Tendermint) – zostały zaprojektowane do pracy w różnych środowiskach rozproszonych, często przy zmiennych warunkach i w obecności „złośliwych” węzłów. Każdy z tych algorytmów ma swoje zasady działania i specyficzne zastosowania, a wybór odpowiedniego zależy od wymagań danego systemu.

# Slide 5

Systemy typu lider-replika składają się z węzła lidera, który proponuje, zarządza, akceptuje i serializuje zmiany, oraz węzłów repliki, które proponują zmiany obecnemu węzłowi liderowi. Ponieważ to jedna jednostka jest odpowiedzialna za akceptację i serializację, systemy lider-replika nie wymagają algorytmów konsensusu, aby uzgodnić kolejny stan. Jednakże, jeśli węzeł lidera stanie się niedostępny, węzły repliki muszą uzgodnić, który z nich powinien zostać nowym liderem, i zazwyczaj uruchamiają algorytmy konsensusu, aby osiągnąć to porozumienie.

# Slide 6

Systemy typu peer-to-peer składają się z węzłów, które mogą proponować zmiany i uczestniczyć w ich akceptacji. Węzły muszą uzgodnić, jaki będzie kolejny stan, aby zachować spójność, i zazwyczaj uruchamiają algorytmy konsensusu, aby dojść do tego porozumienia.

# Slide 7

- **Paxos**: Paxos to algorytm konsensusu stworzony do pracy w rozproszonych systemach, który pozwala węzłom na uzgodnienie jednego poprawnego stanu systemu. Proces przebiega etapowo i zapewnia, że każdy akceptant wybiera jedną, zgodną decyzję. Paxos jest trudny do implementacji, ale niezawodny i odporny na awarie.
- **Raft**: Raft jest bardziej zrozumiałą alternatywą dla Paxosa, zaprojektowaną z myślą o prostocie implementacji. Działa w systemie lider-replika, gdzie lider podejmuje decyzje, które są później akceptowane przez repliki. Raft umożliwia przyznanie lidera i zapewnia spójność oraz tolerancję na awarie w rozproszonym systemie.
- **Practical Byzantine Fault Tolerance (pBFT)**: pBFT jest algorytmem odpornym na awarie bizantyjskie, co oznacza, że działa poprawnie nawet w obecności złośliwych węzłów. W pBFT wszystkie węzły wymieniają się informacjami, aby osiągnąć konsensus. Jest bardzo efektywny w małych sieciach, ale jego wydajność spada w dużych, gdyż wymaga intensywnej komunikacji między węzłami.
- **Federated Byzantine Agreement (FBA)**: FBA jest odmianą konsensusu bizantyjskiego, używaną w systemach takich jak Stellar. Zamiast globalnego konsensusu każdy węzeł wybiera zestaw innych węzłów, którym ufa, tworząc sieć, w której konsensus osiągany jest przez przeplatające się „koła zaufania”. FBA umożliwia zdecentralizowany konsensus, dobrze sprawdzając się w dużych sieciach.
- **Proof of Work (PoW)**: PoW jest algorytmem konsensusu używanym głównie w blockchainach (np. w Bitcoinie). Polega na tym, że węzły (górnicy) rozwiązują skomplikowane zagadki kryptograficzne, aby zaproponować kolejny blok w łańcuchu. Jest bardzo bezpieczny, ale wymaga dużej ilości energii obliczeniowej i czasu.
- **Proof of Stake (PoS)**: PoS to algorytm konsensusu, który wybiera węzły do tworzenia bloków na podstawie ilości posiadanych zasobów (np. tokenów w blockchainie). Węzły z większym udziałem mają większą szansę na dodanie nowego bloku. Jest bardziej energooszczędny niż PoW i stosowany w blockchainach takich jak Ethereum po przejściu na wersję 2.0.

# Slide 8

Mamy zdefiniowane 3 role:
- $\color{yellow}{proposer}$: proponują wartości na jakich dochodzimy do konsensusu
- $\color{red}{acceptor}$: biorą udział w samym dochodzeniu do konsensusu
- $\color{green}{learners}$: uczą się już uzgodnionej wartości

# Slide 9

yap


