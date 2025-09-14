# HyperVolume - Przewodnik Wdrożenia i Dystrybucji

## Przegląd Systemu Dystrybucji

HyperVolume wykorzystuje **GitHub Releases** jako darmową platformę dystrybucji, zapewniając:
- Bezpłatny hosting plików instalacyjnych
- Automatyczne generowanie linków do pobierania
- Wersjonowanie i changelog
- API do sprawdzania aktualizacji

## Struktura Dystrybucji

### 1. Automatyczna Budowa (GitHub Actions)
```
.github/workflows/build.yml
```
- Automatyczna budowa przy każdym tagu `v*`
- Tworzenie instalatora NSIS
- Publikacja w GitHub Releases

### 2. Instalator NSIS
```
installer/installer.nsi
```
- Profesjonalny instalator Windows
- Automatyczna rejestracja w "Dodaj/Usuń programy"
- Tworzenie skrótów na pulpicie i w menu Start
- Opcjonalne auto-uruchamianie

### 3. System Aktualizacji
```
src-tauri/src/updater.rs
```
- Sprawdzanie aktualizacji przez GitHub API
- Automatyczne pobieranie i instalacja
- Konfigurowalny harmonogram sprawdzania

## Proces Wydania Nowej Wersji

### Krok 1: Przygotowanie Kodu
```bash
# Aktualizacja wersji w Cargo.toml
[package]
version = "1.1.0"

# Aktualizacja wersji w updater.rs
let current_version = updater::AppVersion::new(1, 1, 0);
```

### Krok 2: Tworzenie Tagu i Release
```bash
git tag v1.1.0
git push origin v1.1.0
```

### Krok 3: Automatyczna Budowa
GitHub Actions automatycznie:
1. Buduje aplikację dla Windows
2. Tworzy instalator NSIS
3. Publikuje w GitHub Releases
4. Generuje changelog

### Krok 4: Weryfikacja
- Sprawdzenie poprawności plików w Release
- Test instalatora na czystym systemie
- Weryfikacja działania auto-aktualizacji

## Konfiguracja Auto-Aktualizacji

### Ustawienia Domyślne
```json
{
  "auto_check": true,
  "check_interval_hours": 24,
  "auto_download": true,
  "auto_install": false
}
```

### Lokalizacja Konfiguracji
- Windows: `%APPDATA%\HyperVolume\update_config.json`
- Konfiguracja per-użytkownik

## Bezpieczeństwo Aktualizacji

### Weryfikacja Integralności
- Pobieranie tylko z oficjalnego repozytorium GitHub
- HTTPS dla wszystkich połączeń
- Weryfikacja podpisów cyfrowych (planowane)

### Uprawnienia
- Instalator wymaga uprawnień administratora
- Aktualizacje mogą być instalowane bez uprawnień admin (jeśli zainstalowane per-user)

## Monitoring i Analityka

### GitHub Insights
- Statystyki pobierań z GitHub Releases
- Analiza popularności wersji
- Śledzenie problemów przez Issues

### Telemetria (Opcjonalna)
```rust
// Planowane: anonimowe statystyki użytkowania
struct TelemetryData {
    version: String,
    os_version: String,
    usage_stats: UsageStats,
}
```

## Backup i Odzyskiwanie

### Backup Konfiguracji
```
%APPDATA%\HyperVolume\
├── update_config.json
├── audio_profiles.json
└── user_settings.json
```

### Procedura Odzyskiwania
1. Reinstalacja z najnowszego instalatora
2. Przywrócenie konfiguracji z backupu
3. Weryfikacja działania

## Rozwiązywanie Problemów Dystrybucji

### Problem: Aktualizacja się nie powiodła
**Rozwiązanie:**
1. Sprawdzenie połączenia internetowego
2. Ręczne pobranie instalatora z GitHub
3. Reinstalacja aplikacji

### Problem: Instalator nie uruchamia się
**Rozwiązanie:**
1. Sprawdzenie uprawnień administratora
2. Wyłączenie antywirusa tymczasowo
3. Pobranie najnowszej wersji

### Problem: Aplikacja nie wykrywa aktualizacji
**Rozwiązanie:**
1. Sprawdzenie konfiguracji auto-aktualizacji
2. Ręczne sprawdzenie aktualizacji w menu
3. Reset konfiguracji aktualizacji

## Metryki Sukcesu

### KPI Dystrybucji
- Liczba pobrań na release
- Wskaźnik adopcji nowych wersji
- Czas między wydaniem a adopcją
- Liczba zgłoszonych problemów

### Cele
- 90% użytkowników na najnowszej wersji w ciągu 30 dni
- <1% problemów z instalacją
- <24h czas reakcji na krytyczne aktualizacje

## Roadmapa Dystrybucji

### Faza 1 (Aktualna)
- ✅ GitHub Releases
- ✅ NSIS Installer
- ✅ Auto-updater

### Faza 2 (Planowana)
- 🔄 Podpisy cyfrowe
- 🔄 Delta updates (tylko zmiany)
- 🔄 Rollback mechanism

### Faza 3 (Przyszłość)
- 📋 Microsoft Store
- 📋 Chocolatey package
- 📋 Winget package

## Koszty i Zasoby

### Całkowite Koszty: **0 PLN/miesiąc**
- GitHub: Darmowe dla projektów open source
- GitHub Actions: 2000 minut/miesiąc za darmo
- GitHub Releases: Nieograniczone przechowywanie

### Wymagane Zasoby
- Konto GitHub (darmowe)
- Podstawowa znajomość Git
- Windows do testowania instalatora

## Wsparcie i Dokumentacja

### Dla Użytkowników
- README.md z instrukcjami instalacji
- FAQ w GitHub Wiki
- Issues dla zgłaszania problemów

### Dla Deweloperów
- BUILD_INSTRUCTIONS.md
- TECHNICAL_PLAN.md
- Komentarze w kodzie

## Zgodność z Prawem

### Licencja MIT
- Pozwala na komercyjne użycie
- Wymaga zachowania informacji o prawach autorskich
- Brak gwarancji

### GDPR/RODO
- Brak zbierania danych osobowych
- Lokalne przechowywanie konfiguracji
- Opcjonalna telemetria z zgodą użytkownika

