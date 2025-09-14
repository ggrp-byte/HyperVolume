# HyperVolume: Szczegółowy Plan Techniczny

**Autor:** Manus AI
**Data:** 14 września 2025

## 1. Wprowadzenie

Dokument ten przedstawia szczegółowy plan techniczny dla projektu HyperVolume, zaawansowanego menedżera dźwięku dla systemu Windows 11. Celem aplikacji jest umożliwienie użytkownikom precyzyjnej kontroli nad głośnością poszczególnych aplikacji oraz innowacyjnej funkcji wzmocnienia dźwięku do 777% standardowego poziomu. Plan ten obejmuje architekturę, wybór technologii, etapy implementacji oraz kluczowe wyzwania techniczne.

## 2. Koncepcja i Wymagania Funkcjonalne

### 2.1. Nazwa Projektu

HyperVolume

### 2.2. Główne Funkcjonalności

*   **Indywidualna Kontrola Głośności Aplikacji:** Użytkownik będzie mógł niezależnie regulować poziom głośności dla każdej aktywnej aplikacji odtwarzającej dźwięk w systemie. Aplikacja powinna dynamicznie wykrywać nowe sesje audio i usuwać zakończone.
*   **Wyciszanie/Odciszanie (Mute/Unmute):** Możliwość szybkiego wyciszenia lub odciszenia dźwięku dla poszczególnych aplikacji.
*   **Wzmocnienie Dźwięku (Audio Boost do 777%):** Kluczowa funkcja pozwalająca na zwiększenie głośności powyżej 100% (do 777%). Będzie to wymagało zaawansowanego przetwarzania sygnału audio.
*   **Intuicyjny Interfejs Użytkownika (UI):** Nowoczesny, responsywny interfejs, który wizualnie integruje się z estetyką Windows 11, zapewniając łatwość obsługi.
*   **Wydajność i Niskie Zużycie Zasobów:** Aplikacja powinna działać w tle, minimalnie obciążając procesor i pamięć RAM.
*   **Wsparcie dla Wielu Urządzeń Audio:** Możliwość zarządzania dźwiękiem na różnych urządzeniach wyjściowych (np. głośniki, słuchawki, HDMI).

### 2.3. Wymagania Niefunkcjonalne

*   **Stabilność:** Aplikacja musi być stabilna i niezawodna, nie powodując awarii systemu audio.
*   **Bezpieczeństwo:** Implementacja musi być bezpieczna, nie narażając systemu na luki.
*   **Kompatybilność:** Docelowo Windows 11, z potencjalnym wsparciem dla Windows 10.
*   **Łatwość Instalacji:** Prosty proces instalacji i deinstalacji.

## 3. Architektura Systemu

Architektura HyperVolume będzie oparta na modelu klient-serwer, gdzie 


logika biznesowa i interakcje z systemem (backend) będą realizowane w Rust, natomiast interfejs użytkownika (frontend) zostanie zbudowany przy użyciu technologii webowych i osadzony w aplikacji desktopowej za pomocą frameworka Tauri.

### 3.1. Komponenty Główne

*   **Backend (Rust):** Odpowiedzialny za całą logikę związaną z zarządzaniem dźwiękiem, komunikacją z Core Audio APIs, przetwarzaniem sygnału audio (wzmocnienie) oraz komunikacją z frontendem.
*   **Frontend (Tauri/Web Technologies):** Odpowiedzialny za prezentację interfejsu użytkownika, odbieranie danych z backendu i wysyłanie poleceń do backendu w odpowiedzi na interakcje użytkownika.

### 3.2. Przepływ Danych

1.  **Inicjalizacja:** Po uruchomieniu, aplikacja HyperVolume inicjuje backend Rust, który z kolei nawiązuje połączenie z Core Audio APIs systemu Windows.
2.  **Wykrywanie Sesji Audio:** Backend Rust cyklicznie skanuje system w poszukiwaniu aktywnych sesji audio. Po wykryciu nowej sesji (np. uruchomienie przeglądarki odtwarzającej YouTube), zbiera informacje o procesie (nazwa, PID, ikona) oraz aktualnym stanie audio (głośność, wyciszenie).
3.  **Komunikacja Backend-Frontend:** Zebrane dane o sesjach audio są przesyłane do frontendu Tauri. Tauri udostępnia mechanizmy do dwukierunkowej komunikacji między procesem Rust a procesem webview (JavaScript).
4.  **Renderowanie UI:** Frontend renderuje listę aktywnych aplikacji, wyświetlając dla każdej z nich suwak głośności, przycisk wyciszenia i ikonę.
5.  **Interakcja Użytkownika:** Użytkownik zmienia głośność lub wycisza aplikację za pomocą interfejsu. Te akcje są przechwytywane przez frontend i wysyłane jako polecenia do backendu Rust.
6.  **Modyfikacja Stanu Audio:** Backend Rust odbiera polecenia i używa Core Audio APIs do zmiany głośności lub stanu wyciszenia odpowiedniej sesji audio w systemie.
7.  **Wzmocnienie Dźwięku (dla >100%):** Jeśli użytkownik ustawi głośność powyżej 100%, backend Rust przekierowuje strumień audio tej aplikacji przez wirtualne urządzenie audio, gdzie następuje cyfrowe wzmocnienie sygnału przed przesłaniem go do fizycznego urządzenia wyjściowego.

## 4. Wybór Technologii (Stack Technologiczny)

### 4.1. Język Programowania Backendu: Rust

**Uzasadnienie:**
*   **Wydajność:** Rust oferuje wydajność zbliżoną do C/C++, co jest kluczowe dla aplikacji przetwarzających dźwięk w czasie rzeczywistym i minimalizujących opóźnienia.
*   **Bezpieczeństwo Pamięci:** System typów i model własności (ownership) Rust eliminuje wiele typowych błędów związanych z pamięcią (np. null pointer dereferences, data races), co przekłada się na większą stabilność aplikacji.
*   **Niskie Zużycie Zasobów:** Brak wbudowanego garbage collectora pozwala na precyzyjną kontrolę nad alokacją i dealokacją pamięci, co jest ważne dla aplikacji działających w tle.
*   **Integracja z C/C++:** Rust ma doskonałe wsparcie dla FFI (Foreign Function Interface), co ułatwia integrację z istniejącymi bibliotekami systemowymi Windows napisanymi w C/C++.
*   **Wsparcie dla `windows-rs`:** Oficjalne powiązania Rust dla Windows API (`windows-rs`) zapewniają bezpieczny i idiomatyczny dostęp do funkcji systemowych.

### 4.2. Framework Interfejsu Użytkownika: Tauri

**Uzasadnienie:**
*   **Natywna Wydajność:** W przeciwieństwie do Electrona, Tauri wykorzystuje natywne webview systemowe (WebView2 na Windows), co znacząco zmniejsza rozmiar aplikacji i zużycie pamięci RAM.
*   **Elastyczność UI:** Umożliwia tworzenie interfejsów użytkownika przy użyciu dowolnych technologii webowych (React, Vue, Angular, Svelte, vanilla JS/HTML/CSS), co daje dużą swobodę w projektowaniu nowoczesnego i estetycznego UI.
*   **Bezpieczeństwo:** Tauri kładzie duży nacisk na bezpieczeństwo, oferując szereg mechanizmów ochronnych.
*   **Integracja z Rust:** Bezproblemowa komunikacja między frontendem JavaScript a backendem Rust, co pozwala na łatwe wywoływanie funkcji systemowych z poziomu UI.

### 4.3. Interfejsy Systemowe: Windows Core Audio APIs

**Uzasadnienie:**
*   **Natywna Kontrola:** Core Audio APIs to oficjalny i najbardziej zaawansowany zestaw interfejsów do zarządzania dźwiękiem w systemie Windows.
*   **Precyzyjna Kontrola Sesji:** Umożliwiają enumerację i kontrolę głośności poszczególnych sesji audio (czyli aplikacji).
*   **Dostęp do Urządzeń:** Pozwalają na zarządzanie urządzeniami audio (wejściowymi i wyjściowymi).
*   **Interfejsy Kluczowe:**
    *   `IMMDeviceEnumerator`: Do wyliczania dostępnych urządzeń audio.
    *   `IAudioSessionManager2`: Do zarządzania sesjami audio, w tym ich wykrywaniem i tworzeniem.
    *   `ISimpleAudioVolume`: Do ustawiania głośności i stanu wyciszenia dla konkretnej sesji audio.
    *   `IAudioClient`: Do bezpośredniego dostępu do strumieni audio, co będzie kluczowe dla implementacji wzmocnienia dźwięku.

## 5. Etapy Implementacji

### Faza 1: Stworzenie Repozytorium GitHub i Dokumentacji Technicznej (W toku)

*   Utworzenie publicznego repozytorium GitHub.
*   Stworzenie pliku `README.md` z ogólnym opisem projektu.
*   Stworzenie pliku `TECHNICAL_PLAN.md` (ten dokument) z szczegółowym planem technicznym.
*   Utworzenie pliku `todo.md` do śledzenia postępów.

### Faza 2: Przygotowanie Struktury Projektu Rust + Tauri

*   Inicjalizacja nowego projektu Tauri za pomocą `cargo create-tauri-app`.
*   Konfiguracja `Cargo.toml` dla zależności Rust (np. `windows-rs`).
*   Ustawienie podstawowej struktury katalogów dla kodu Rust i plików frontendu.

### Faza 3: Implementacja Rdzenia Aplikacji w Rust - Core Audio API

*   **Moduł `audio_manager`:** Stworzenie modułu Rust, który będzie opakowywał funkcje Core Audio API.
*   **Funkcja `enumerate_audio_sessions()`:** Implementacja funkcji, która będzie zwracać listę aktywnych sesji audio, wraz z ich nazwami procesów, ikonami i aktualnymi stanami głośności/wyciszenia.
*   **Funkcja `set_session_volume(session_id, volume)`:** Implementacja funkcji do ustawiania głośności dla danej sesji audio.
*   **Funkcja `toggle_session_mute(session_id)`:** Implementacja funkcji do przełączania stanu wyciszenia.
*   **Monitorowanie Zmian:** Implementacja mechanizmu nasłuchiwania na zmiany w sesjach audio (np. nowa aplikacja zaczyna odtwarzać dźwięk, aplikacja kończy działanie).

### Faza 4: Stworzenie Interfejsu Użytkownika w Tauri

*   **Projekt UI/UX:** Opracowanie projektu interfejsu w Figma lub podobnym narzędziu, wzorując się na mikserze głośności Windows 11, ale z rozszerzonymi funkcjonalnościami.
*   **Implementacja Frontendu:** Budowa interfejsu przy użyciu HTML, CSS i JavaScript/TypeScript (np. z użyciem React, Vue lub Svelte).
*   **Komunikacja z Backendem:** Wykorzystanie mechanizmów Tauri do wywoływania funkcji Rust z JavaScript i odbierania danych z Rust.
*   **Dynamiczne Renderowanie:** Interfejs powinien dynamicznie aktualizować listę aplikacji i ich suwaków głośności w odpowiedzi na zmiany zgłaszane przez backend.

### Faza 5: Implementacja Wirtualnego Urządzenia Audio i Wzmocnienia Dźwięku

*   **Badanie Opcji:** Analiza dostępnych podejść do tworzenia wirtualnych urządzeń audio w Windows (np. WDM/KS driver, WASAPI loopback, Audio Processing Objects - APO).
    *   **APO (Audio Processing Object):** Jest to preferowane podejście dla zaawansowanego przetwarzania dźwięku, ponieważ integruje się bezpośrednio z potokiem audio systemu Windows. Wymaga jednak znajomości C++ i DDK (Driver Development Kit).
    *   **Wirtualny Sterownik Audio:** Bardziej złożone, ale daje pełną kontrolę. Może być zaimplementowany w C++.
*   **Implementacja Wirtualnego Urządzenia:** Stworzenie komponentu, który będzie działał jako wirtualne urządzenie audio. Aplikacje będą mogły kierować do niego swój dźwięk.
*   **Przetwarzanie Sygnału (DSP):** Wewnątrz wirtualnego urządzenia, przechwycony strumień audio zostanie poddany cyfrowemu przetwarzaniu:
    *   **Wzmocnienie:** Amplituda próbek audio zostanie pomnożona przez współczynnik wzmocnienia (np. 7.77 dla 777%).
    *   **Limiter/Kompresor:** Niezbędny komponent DSP, który zapobiegnie przesterowaniu sygnału (clipping) i ochroni słuch użytkownika oraz sprzęt. Limiter będzie obcinał sygnał powyżej określonego progu, zapewniając, że maksymalna głośność nie przekroczy bezpiecznego poziomu, nawet przy wysokim wzmocnieniu.
*   **Przekierowanie do Fizycznego Urządzenia:** Przetworzony sygnał audio zostanie następnie przekierowany do wybranego przez użytkownika fizycznego urządzenia wyjściowego.

### Faza 6: Testy, Budowa i Przygotowanie do Dystrybucji

*   **Testy Jednostkowe:** Pisanie testów jednostkowych dla kluczowych modułów Rust, zwłaszcza tych odpowiedzialnych za interakcję z Core Audio API i przetwarzanie DSP.
*   **Testy Integracyjne:** Testowanie komunikacji między frontendem a backendem oraz działania funkcji wzmocnienia dźwięku.
*   **Testy Wydajnościowe:** Monitorowanie zużycia CPU i RAM, aby upewnić się, że aplikacja działa efektywnie.
*   **Budowa Aplikacji:** Skompilowanie aplikacji Tauri do postaci wykonywalnej dla Windows.
*   **Tworzenie Instalatora:** Przygotowanie instalatora (np. za pomocą narzędzi takich jak NSIS, WiX lub wbudowanych funkcji Tauri), który zainstaluje aplikację i zarejestruje wirtualne urządzenie audio.

### Faza 7: Dostarczenie Wyników Użytkownikowi

*   Prezentacja działającej aplikacji.
*   Dostarczenie kompletnej dokumentacji technicznej i użytkowej.
*   Udostępnienie repozytorium GitHub.

## 6. Kluczowe Wyzwania Techniczne

*   **Implementacja Wirtualnego Urządzenia Audio:** To najbardziej złożony element projektu, wymagający głębokiej wiedzy o architekturze audio Windows i potencjalnie pisania sterowników (APO).
*   **Stabilne Przetwarzanie DSP:** Zapewnienie, że wzmocnienie dźwięku jest realizowane bez artefaktów, opóźnień i przesterowań, z efektywnym limiterem.
*   **Dynamiczne Wykrywanie Sesji:** Skuteczne i wydajne monitorowanie aktywnych sesji audio w systemie.
*   **Obsługa Błędów:** Solidne zarządzanie błędami w interakcjach z systemowymi API.

## 7. Harmonogram (Szacunkowy)

*   **Faza 1-2:** 1-2 tygodnie (Ukończono/W toku)
*   **Faza 3:** 3-4 tygodnie
*   **Faza 4:** 2-3 tygodnie
*   **Faza 5:** 6-8 tygodni (najbardziej czasochłonna)
*   **Faza 6:** 2-3 tygodnie
*   **Faza 7:** 1 tydzień

**Całkowity Czas:** Około 16-21 tygodni (4-5 miesięcy).

## 8. Referencje

[1] Microsoft Docs: Core Audio APIs. Dostępne pod adresem: `https://docs.microsoft.com/en-us/windows/win32/coreaudio/core-audio-apis`
[2] Tauri Framework. Dostępne pod adresem: `https://tauri.app/`
[3] Rust Programming Language. Dostępne pod adresem: `https://www.rust-lang.org/`
[4] `windows-rs` crate. Dostępne pod adresem: `https://github.com/microsoft/windows-rs`
[5] Microsoft Docs: Audio Processing Objects. Dostępne pod adresem: `https://docs.microsoft.com/en-us/windows-hardware/drivers/audio/audio-processing-objects`


