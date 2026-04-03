<h1 align="center">PulseLayer</h1>

<p align="center">
  <a href="https://github.com/wielorzeczownik/pulse-layer/actions/workflows/release.yml"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/actions/workflow/status/wielorzeczownik/pulse-layer/release.yml?branch=main&style=flat-square&labelColor=2d333b&color=3fb950"/><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/actions/workflow/status/wielorzeczownik/pulse-layer/release.yml?branch=main&style=flat-square&color=2ea043"/><img src="https://img.shields.io/github/actions/workflow/status/wielorzeczownik/pulse-layer/release.yml?branch=main&style=flat-square&labelColor=2d333b&color=3fb950" alt="Build"/></picture></a> <a href="https://github.com/wielorzeczownik/pulse-layer/releases/latest"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/v/release/wielorzeczownik/pulse-layer?style=flat-square&labelColor=2d333b&color=3fb950"/><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/v/release/wielorzeczownik/pulse-layer?style=flat-square&color=2ea043"/><img src="https://img.shields.io/github/v/release/wielorzeczownik/pulse-layer?style=flat-square&labelColor=2d333b&color=3fb950" alt="Najnowsze wydanie"/></picture></a> <a href="https://github.com/wielorzeczownik/pulse-layer/blob/main/LICENSE"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/badge/License-MIT-3fb950?style=flat-square&labelColor=2d333b"/><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/badge/License-MIT-2ea043?style=flat-square"/><img src="https://img.shields.io/badge/License-MIT-3fb950?style=flat-square&labelColor=2d333b" alt="Licencja: MIT"/></picture></a>
  <br/>
  <img src="https://img.shields.io/badge/Rust-B7410E?style=flat-square&logo=rust&logoColor=white" alt="Rust"/>
  <img src="https://img.shields.io/badge/Iced-4D9DE0?style=flat-square&logo=iced&logoColor=white" alt="Iced"/>
  <img src="https://img.shields.io/badge/TypeScript-3178C6?style=flat-square&logo=typescript&logoColor=white" alt="TypeScript"/>
</p>

<p align="center">🇬🇧 <a href="README.md">English</a> | 🇵🇱 Polski</p>

Nakładka na OBS wyświetlająca **tętno w czasie rzeczywistym** - odczytuje BPM z pierścienia Bluetooth i serwuje widget przeglądarki bezpośrednio na stream. Bez subskrypcji, bez chmury - działa w całości lokalnie.

Stworzona dla VTuberów i streamerów, którzy chcą pokazywać tętno na streamie bez niczego więcej niż tani pierścień i OBS. Pierwotnie zrobiona dla **[KitsuneTsuyu](https://www.twitch.tv/kitsunetsuyu)**.

## Funkcje

- Tętno na żywo z kolorowymi strefami (spokój → alarm)
- Dwa style nakładki: **serce** lub **EKG**
- Własne kolory hex dla każdej strefy

## Kompatybilność

Testowane na **Smartring COLMI R12**. Starsze modele COLMI i inne pierścienie korzystające z tego samego protokołu BLE powinny też działać.

> [!NOTE]
> Aktualnie obsługiwane są tylko pierścienie z aplikacji Qring. Dodanie innych marek nie jest planowane, ale jeśli będzie zainteresowanie - jestem otwarty.

## Pobieranie i instalacja

Najnowsze wydanie: [GitHub Releases](https://github.com/wielorzeczownik/pulse-layer/releases/latest)

Pobierz archiwum dla swojej platformy:

- [pulse-layer-aarch64-apple-darwin.tar.gz](https://github.com/wielorzeczownik/pulse-layer/releases/latest/download/pulse-layer-aarch64-apple-darwin.tar.gz) - macOS na Apple Silicon (M1/M2/M3/M4)
- [pulse-layer-x86_64-apple-darwin.tar.gz](https://github.com/wielorzeczownik/pulse-layer/releases/latest/download/pulse-layer-x86_64-apple-darwin.tar.gz) - macOS na Intel
- [pulse-layer-x86_64-pc-windows-msvc.zip](https://github.com/wielorzeczownik/pulse-layer/releases/latest/download/pulse-layer-x86_64-pc-windows-msvc.zip) - Windows 64-bit
- [pulse-layer-x86_64-unknown-linux-gnu.tar.gz](https://github.com/wielorzeczownik/pulse-layer/releases/latest/download/pulse-layer-x86_64-unknown-linux-gnu.tar.gz) - Linux 64-bit

### macOS

Po rozpakowaniu archiwum otrzymasz `PulseLayer.app`. Przy pierwszym uruchomieniu macOS zablokuje aplikację, bo nie jest podpisana płatnym certyfikatem Apple. Żeby to obejść:

**Opcja A - prawy przycisk myszy:**

1. Kliknij prawym przyciskiem na `PulseLayer.app` → **Otwórz**
2. Kliknij **Otwórz** w oknie dialogowym

**Opcja B - terminal (jednorazowo):**

```bash
xattr -cr PulseLayer.app
open PulseLayer.app
```

### Windows

Rozpakuj archiwum i uruchom `pulse-layer.exe`. Instalator nie jest potrzebny.

### Linux

```bash
tar -xzf pulse-layer-*.tar.gz
./pulse-layer
```

## Łączenie z pierścieniem

### Pierwsze parowanie

1. Upewnij się, że pierścień jest naładowany i w pobliżu.
2. Uruchom PulseLayer i kliknij **Scan**.
3. Pierścień pojawi się na liście - kliknij **Connect**.

### Pierścień jest już sparowany z telefonem

Pierścień może rozmawiać tylko z jednym urządzeniem na raz. Jeśli jest połączony z aplikacją Qring na telefonie, PulseLayer go nie zobaczy.

Rozwiązanie: **wyłącz Bluetooth na telefonie** przed skanowaniem. PulseLayer wykryje pierścień normalnie.

### Powrót do telefonu po użyciu PulseLayer

Pierścień pamięta parowanie. Żeby sparować go z telefonem od nowa, trzeba usunąć parowanie z obu stron:

1. W PulseLayer - kliknij **Disconnect**.
2. Na komputerze - ustawienia Bluetooth, znajdź pierścień, kliknij **Zapomnij / Usuń urządzenie**.
3. Na telefonie - też zapomnij pierścień.
4. Sparuj od nowa przez aplikację Qring.

> [!IMPORTANT]
> Pominięcie kroku 2 lub 3 spowoduje błąd parowania po stronie telefonu. Oba urządzenia muszą najpierw zapomnieć pierścień.

## Konfiguracja OBS

Nakładka działa jako lokalna strona przeglądarki serwowana przez PulseLayer. Zewnętrzny hosting nie jest potrzebny.

1. **Uruchom PulseLayer** i połącz się z pierścieniem - serwer startuje na porcie `9000`.
2. W OBS kliknij **+** w panelu Źródła → **Przeglądarka**.
3. Ustaw URL na:
   ```
   http://localhost:9000
   ```
4. Ustaw **Szerokość** na `400` i **Wysokość** na `300` (możesz dowolnie zmieniać rozmiar i przycinać w OBS).
5. Zaznacz **Odśwież przeglądarkę gdy scena staje się aktywna**, jeśli chcesz automatycznego ponownego połączenia.
6. Kliknij **OK** - widget pojawi się na scenie. Użyj **Edytuj transformację**, żeby umieścić go gdzie chcesz.

> [!TIP]
> Widget jest zakotwiczony do lewego dolnego rogu ramki przeglądarki. Przytnij źródło przeglądarki ciasno, a możesz umieścić je w dowolnym miejscu na scenie.

Nakładka sama się reconnectuje po restarcie PulseLayer - nie musisz dotykać źródła przeglądarki.

## Style nakładki

Przełączaj między stylami w panelu **Settings** w PulseLayer.

**Serce** - bijąca ikona serca z dużą liczbą BPM. Proste i czytelne.

**EKG** - animowana linia EKG rysująca się raz na uderzenie serca, z BPM poniżej. Lepsze dla bardziej technicznego wyglądu.

Oba zmieniają kolor w zależności od strefy BPM. Wszystkie kolory są konfigurowalne.

| Strefa   | Domyślny zakres BPM | Domyślny kolor |
| -------- | ------------------- | -------------- |
| Spokój   | 0 - 64              | Zielony        |
| Normalne | 65 - 80             | Niebieski      |
| Wysokie  | 81 - 100            | Żółty          |
| Szybkie  | 101 - 130           | Pomarańczowy   |
| Alarm    | 131+                | Czerwony       |

## Budowanie ze źródeł

Wymagania: [Rust](https://rustup.rs) stable, [Node.js](https://nodejs.org) 24+.

```bash
git clone https://github.com/wielorzeczownik/pulse-layer
cd pulse-layer
cargo build --release
```

Frontend nakładki jest kompilowany przez Vite w ramach `cargo build` i wbudowany w binarny plik - nie ma osobnego kroku budowania.

**Szybkie uruchomienie na macOS (build debug):**

```bash
./run_macos.sh
```

Buduje bundle `.app` w `target/`, podpisuje go ad-hoc i otwiera.

### Zależności systemowe na Linuxie

```bash
# Ubuntu / Debian
sudo apt-get install libdbus-1-dev pkg-config libxkbcommon-dev \
  libxcb-shape0-dev libxcb-xfixes0-dev libwayland-dev
```

## Zastrzeżenie

Nieoficjalny projekt, niezwiązany z COLMI ani żadnym producentem pierścieni. Może przestać działać, jeśli firmware pierścienia zmieni protokół BLE.
