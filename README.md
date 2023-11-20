# AeroDune LA3

Repositori ini merupakan wadah pengembangan aplikasi kalibrasi instrumentasi pengujian.

## Untuk Memulai

Hal-hal yang perlu disiapkan untuk mulai melakukan pengembangan ataupun kompilasi aplikasi, yaitu:
1. Pasang [Rust](https://www.rust-lang.org/) melalui [rustup](https://rustup.rs/) dan [NodeJS](https://nodejs.org/).
2. Pasang [SurrealDB](https://surrealdb.com/) sebagai *Database*.
3. Pindah ke direktori utama yang terdapat file `tailwind.config.js`.
4. Pasang aplikasi terminal [Tauri](https://tauri.app/) dengan perintah `cargo install tauri-cli` dan beri perhatian pada `dependencies` sesuai halaman [INI](https://tauri.app/v1/guides/getting-started/prerequisites).
5. Jalankan instruksi `npm install`.
6. Jalankan instruksi `npm run dev` untuk pengembangan atau `npm run build` untuk finalisasi
7. Jalankan instruksi `cargo tauri dev` untuk pengembangan atau `cargo tauri build` untuk kompilasi aplikasi.

