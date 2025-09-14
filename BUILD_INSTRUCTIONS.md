# HyperVolume - Instrukcje Budowy i Instalacji

## Wymagania Systemowe

### Windows 11
- Windows 11 (wersja 21H2 lub nowsza)
- Visual Studio 2019/2022 z komponentami C++ (dla kompilacji Rust)
- Windows SDK 10.0.19041.0 lub nowszy

### Narzędzia Deweloperskie
- **Rust** (wersja 1.70 lub nowsza)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Node.js** (wersja 18 lub nowsza)
- **npm** lub **yarn**
- **Git**

## Instalacja Zależności

### 1. Klonowanie Repozytorium
```bash
git clone https://github.com/ggrp-byte/HyperVolume.git
cd HyperVolume/hypervolume-app
```

### 2. Instalacja Zależności Node.js
```bash
npm install
```

### 3. Instalacja Zależności Rust
```bash
cd src-tauri
cargo build
```

## Budowa Aplikacji

### Tryb Deweloperski
```bash
npm run tauri dev
```

### Budowa Produkcyjna
```bash
npm run tauri build
```

## Struktura Projektu

```
hypervolume-app/
├── src/                    # Frontend (TypeScript/HTML/CSS)
│   ├── main.ts            # Główna logika frontendu
│   ├── styles.css         # Style CSS
│   └── assets/            # Zasoby statyczne
├── src-tauri/             # Backend (Rust)
│   ├── src/
│   │   ├── main.rs        # Punkt wejścia
│   │   ├── lib.rs         # Główna biblioteka
│   │   ├── audio_manager.rs # Zarządzanie sesjami audio
│   │   └── audio_boost.rs # Wzmocnienie dźwięku
│   ├── Cargo.toml         # Zależności Rust
│   └── tauri.conf.json    # Konfiguracja Tauri
├── index.html             # Główny plik HTML
└── package.json           # Zależności Node.js
```

## Testowanie

### Testy Jednostkowe (Rust)
```bash
cd src-tauri
cargo test
```

### Testy Integracyjne
```bash
npm run test
```

## Rozwiązywanie Problemów

### Problem: Błąd kompilacji Rust
**Rozwiązanie:** Upewnij się, że masz zainstalowane Visual Studio z komponentami C++:
```bash
rustup toolchain install stable-x86_64-pc-windows-msvc
rustup default stable-x86_64-pc-windows-msvc
```

### Problem: Brak dostępu do Core Audio API
**Rozwiązanie:** Uruchom aplikację jako administrator lub sprawdź uprawnienia audio.

### Problem: Błąd podczas budowy Tauri
**Rozwiązanie:** Sprawdź czy wszystkie zależności są zainstalowane:
```bash
npm run tauri info
```

## Dystrybucja

Po udanej budowie, pliki wykonywalne znajdą się w:
- `src-tauri/target/release/hypervolume-app.exe`
- Instalator: `src-tauri/target/release/bundle/msi/HyperVolume_0.1.0_x64_en-US.msi`

## Uwagi Bezpieczeństwa

- Aplikacja wymaga uprawnień administratora do pełnej funkcjonalności
- Funkcja wzmocnienia dźwięku może być głośna - używaj ostrożnie
- Zawsze testuj na niskich poziomach głośności przed zwiększeniem do 777%

## Wsparcie

W przypadku problemów:
1. Sprawdź [Issues na GitHub](https://github.com/ggrp-byte/HyperVolume/issues)
2. Utwórz nowy issue z opisem problemu
3. Dołącz logi z `npm run tauri dev` lub `cargo build`

