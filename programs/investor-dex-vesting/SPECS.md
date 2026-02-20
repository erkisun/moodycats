# SPEC: Investor-DEX-Vesting

## Accounts
```
Config (PDA): admin, mint, dex_vault, investor_vault, released_tranches, last_release, bumps
DEX-Vault (PDA): 400 Mio Token (Quelle für release_dex)
Investor-Vault (PDA): 500 Mio Token (Quelle für register_investor)
Admin-Token-Account (ATA): Ziel für release_dex (muss vorher existieren!)
Investor-Token-Account (ATA): Ziel für register_investor
```

## Initialize
```rust
// Nur 1x nach Deploy
// Erstellt: Config, DEX-Vault, Investor-Vault
// Speichert: admin, mint, bumps, last_release = jetzt
```

## ReleaseDex
```rust
// Wer: Admin
// Wann: max 4x, 30d Abstand
// Prüft: 1. admin == config.admin
//        2. released_tranches < 4
//        3. last_release + 30d < now
//        4. dex_vault.amount >= 100 Mio
//        5. admin_token_account.owner == admin
//        6. admin_token_account.mint == config.mint
// Tut:   1. Transfer 100 Mio DEX-Vault → Admin-Token
//        2. released_tranches++
//        3. last_release = now
```