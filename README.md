# En todo-applikasjon skrevet i Rust

## Intallasjon av rust
Applikasjonen er skrvet i Rust. Før man kan kjøre applikasjonen så må man ha rust-lang og cargo intrallert.
Rust kan installeres for Linux og Mac ved å kjøre `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`. Alternativt så kan rust installeres ved å gå til https://www.rust-lang.org/learn/get-started
Sjekk at Rust er oppdatert med `rustup update`.

## Hvordan programmet fungerer
Kompiler og start programmet med å kjøre `cargo run` i terminalen.

### Hvordan bruke programmet
Menyen vises når programmet starter og når man skriver inn en kommando som ikke vises i menyen. Nye todo-oppgaver er lagret i en liste og skrives til en `tasks.txt`-fil når programmet avsluttes. Oppagvene i `tasks.txt`-filen lastes inn igjen når programmet startes på nytt. Kommandoene som kan kjøres vises i menyen, og trenger ikke å være store bokstaver.

### Testing av programmet
Programmet er kunn testet i Linux og kan oppføre seg andreledes i et annet miljø.

## Kilder
Mye av informasjon som er brukt til å lage programmet er hentet fra Rust sin offisielle dokumentasjon.
* https://doc.rust-lang.org/book/
* https://doc.rust-lang.org/
