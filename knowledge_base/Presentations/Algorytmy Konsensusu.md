# Systemy Rozproszone

## Definicja

System rozproszony to zbiór niezależnych komputerów, które dla użytkowników systemu wyglądają jak jeden spójny system. Komputery te, często nazywane „węzłami”, współpracują, aby osiągnąć wspólny cel.

## Kluczowe cechy systemu rozproszonego

- **Skalowalność**: Zdolność do rozwoju wraz ze wzrostem obciążenia jest niezbędną cechą systemów rozproszonych. Osiąga się ją przez dodawanie nowych jednostek obliczeniowych lub węzłów do sieci, gdy jest to potrzebne.
- **Współbieżność**: Komponenty systemu rozproszonego działają jednocześnie. Cechuje je również brak „globalnego zegara,” co oznacza, że zadania mogą być realizowane w różnej kolejności i z różną prędkością.
- **Dostępność i odporność na awarie**: W przypadku awarii jednego węzła pozostałe węzły mogą nadal działać, nie zakłócając całościowego procesu obliczeniowego.
- **Heterogeniczność**: W większości systemów rozproszonych węzły i komponenty są asynchroniczne, a ich sprzęt, oprogramowanie pośrednie, systemy operacyjne i oprogramowanie mogą się różnić. Dzięki temu systemy rozproszone można łatwo rozbudować o nowe komponenty.
- **Replikacja**: Systemy rozproszone umożliwiają współdzielenie informacji i komunikację, zapewniając spójność pomiędzy nadmiarowymi zasobami, takimi jak komponenty sprzętowe lub programowe, co zwiększa odporność na awarie, niezawodność i dostępność.
- **Przezroczystość**: Użytkownik końcowy postrzega system rozproszony jako pojedynczą jednostkę obliczeniową, a nie jako zbiór elementów składowych. Dzięki temu użytkownicy mogą korzystać z jednego logicznego urządzenia, nie martwiąc się o architekturę systemu.

# Konsensus

## Definicja konsensusu w kontekście systemów rozproszonych

Konsensus w systemach rozproszonych to proces osiągania zgody między niezależnymi węzłami na temat jednego, wspólnego stanu systemu lub wartości, nawet gdy każdy węzeł działa samodzielnie i może mieć odmienny punkt widzenia na aktualny stan systemu. Dzięki konsensusowi wszystkie węzły mogą podejmować decyzje, które są zgodne i spójne, zapewniając integralność danych oraz poprawne działanie systemu jako całości.

## Dlaczego potrzeby jest konsensus?

W systemach rozproszonych konsensus jest kluczowy, aby wszystkie węzły zgadzały się co do jednego, wspólnego źródła prawdy lub stanu. Jest to niezbędne, aby zapewnić:

- **Spójność**: Konsensus pozwala na utrzymanie integralności danych między węzłami, dzięki czemu wszystkie węzły w systemie odzwierciedlają te same informacje w każdym momencie. W przeciwnym razie różne węzły mogłyby przechowywać sprzeczne dane, co prowadziłoby do niespójności systemu.
- **Koordynację działań**: W przypadku zadań, takich jak zatwierdzanie transakcji, jeśli każdy węzeł podejmie decyzję niezależnie, może dojść do konfliktów lub niespójnych stanów danych. Konsensus zapewnia, że wszystkie węzły podejmują wspólną decyzję, co pozwala na jednolite wykonywanie operacji i eliminowanie rozbieżności.
- **Odporność na awarie**: Konsensus pomaga zapewnić, że system jest zdolny do podejmowania spójnych decyzji nawet wtedy, gdy niektóre węzły zawodzą lub dostarczają sprzeczne informacje. Dzięki temu system może nadal funkcjonować poprawnie pomimo niepełnej komunikacji lub chwilowych problemów z węzłami.

Bez mechanizmu konsensusu systemy rozproszone mogą stać się zawodne, ponieważ każdy węzeł może mieć różny obraz stanu systemu.

## Dlaczego osiągnięcie konsensusu jest wyzwaniem w środowisku rozproszonym?

Osiągnięcie konsensusu w systemach rozproszonych wiąże się z wieloma trudnościami, w tym:

- **Podziały sieciowe (network partitions)**: Ze względu na opóźnienia sieciowe lub awarie połączeń, węzły mogą czasowo utracić możliwość komunikacji z innymi węzłami. W takiej sytuacji część systemu może podejmować decyzje niezależnie od innych części, co prowadzi do niezgodności danych.
- **Asynchroniczność komunikacji**: Węzły nie zawsze otrzymują wiadomości w tej samej kolejności i czasie. W rozproszonym środowisku asynchronicznym trudno jest zagwarantować, że wszystkie węzły uzgodnią taki sam stan w tym samym momencie, co zwiększa ryzyko niezgodności.
- **Awaria węzłów**: W systemach rozproszonych zawsze istnieje ryzyko, że jeden lub więcej węzłów przestanie działać. Węzły mogą być offline lub mogą zachowywać się nieprzewidywalnie, a osiągnięcie konsensusu wymaga metod radzenia sobie z tymi awariami, aby uniknąć zakłóceń w pracy całego systemu.
- **Awaria bizantyjska**:  Dany element nie przestanie pracować, a zacznie wysyłać błędne komunikaty.

## Cechy algorytmów konsensusu
Główne cechy algorytmów konsensusu w systemach rozproszonych obejmują:

1. **Zgoda**: Algorytmy konsensusu dążą do osiągnięcia zgody w grupie węzłów co do jednej wartości lub sekwencji operacji. Wszystkie węzły w systemie powinny ostatecznie dojść do tej samej decyzji.
2. **Odporność na awarie**: Algorytmy konsensusu są zaprojektowane tak, aby radzić sobie z awariami węzłów, opóźnieniami w komunikacji oraz podziałami sieciowymi. Zapewniają, że system może kontynuować działanie nawet w przypadku wystąpienia awarii.
3. **Bezpieczeństwo**: Algorytmy konsensusu zapewniają bezpieczeństwo poprzez gwarantowanie, że tylko jedna wartość lub decyzja zostanie uzgodniona. Zapobiega to konfliktującym decyzjom i utrzymuje spójność danych.
4. **Żywotność**: Żywotność oznacza, że system jest w stanie robić postępy i nadal uzgadniać nowe wartości lub decyzje, nawet przy braku awarii.
5. **Kworum**: Wiele algorytmów konsensusu wymaga kworum, czyli większości węzłów, aby zgodziły się na wartość przed jej zaakceptowaniem lub zatwierdzeniem. Pomaga to zapewnić, że wystarczająca liczba węzłów widziała i zaakceptowała daną wartość.
6. **Podejście dwufazowe**: Większość algorytmów konsensusu stosuje podejście dwufazowe, gdzie propozycje są najpierw przygotowywane, a następnie akceptowane. Zmniejsza to ryzyko konfliktów i zapewnia bezpieczeństwo.
7. **Z liderem lub bez lidera**: Niektóre algorytmy konsensusu wykorzystują podejście oparte na liderze, gdzie wyznaczony lider koordynuje proces konsensusu, podczas gdy inne są bezliderowe, gdzie węzły współpracują, aby osiągnąć konsensus.
8. **Wymiana wiadomości**: Węzły komunikują się ze sobą, wymieniając wiadomości, aby proponować wartości, głosować i informować się nawzajem o swoim stanie.
9. **Replikacja logów**: Wiele algorytmów konsensusu obejmuje mechanizmy replikacji logów, aby zapewnić, że wszystkie węzły mają spójny widok danych.
10. **Spójność i replikacja**: Algorytmy konsensusu zapewniają spójność danych pomiędzy replikami, poprzez replikację danych i gwarantując, że wszystkie repliki zgadzają się co do tych samych danych.
11. **Zmiany członkostwa**: Niektóre algorytmy konsensusu wspierają dynamiczne zmiany członkostwa w klastrze, umożliwiając węzłom dołączanie lub opuszczanie systemu w sposób uporządkowany.
12. **Kompromisy wydajności**: Różne algorytmy konsensusu oferują różne kompromisy między odpornością na awarie, wydajnością a prostotą. Niektóre stawiają na prostotę dla zrozumiałości, podczas gdy inne priorytetyzują wydajność w systemach na dużą skalę.
# Algorytmy

## Paxos
