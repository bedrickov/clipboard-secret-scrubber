# Clipboard-Secret-Scrubber

<div align="center">

![Platform](https://img.shields.io/badge/Platform-Windows-0078D4?style=flat-square&logo=windows)
![Language](https://img.shields.io/badge/Language-Rust%202021-DEA584?style=flat-square&logo=rust)
![License](https://img.shields.io/badge/License-MIT-blue?style=flat-square)
![Build](https://img.shields.io/badge/Build-Passing-brightgreen?style=flat-square)

**Автоматический санитайзер буфера обмена для защиты от случайной утечки API-токенов и приватных ключей.**  
*Детекция и цензурирование GitHub PAT, AWS Access Keys, Google Cloud API Keys и RSA/SSH ключей в реальном времени.*

</div>

---

## Архитектура

- Оптимизированные регулярные выражения на базе детерминированных конечных автоматов (`regex-automata`).
- Мгновенная замена критических секретов на плейсхолдер `[REDACTED_SECRET]` до передачи в сторонние мессенджеры или логи.

## Сборка и тесты

```powershell
cargo test
cargo build --release
```

## Лицензия
MIT

<!-- verified: 2026-10-31 -->