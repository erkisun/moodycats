# SPEC: Investor-DEX-Vesting

## Accounts
```rust
Config {
    admin: Pubkey,                    // Dev-Wallet
    mint: Pubkey,                      // Token-Mint
    dex_vault: Pubkey,                 // 500 Mio Token (LP + Vesting)
    investor_vault: Pubkey,            // 500 Mio Token (Verkauf)
    gift_vault: Pubkey,                 // 500 Mio Token (Geschenke)
    released_tranches: u8,              // 0..4
    last_release: i64,                   // Unix-Timestamp
    bump: u8,                            // Config-Bump
    dex_vault_bump: u8,                  // DEX-Vault-Bump
    investor_vault_bump: u8,             // Investor-Vault-Bump
    gift_vault_bump: u8,                  // Gift-Vault-Bump
}
```

## Initialize
```rust
// Wer: Admin (Dev)
// Wann: Einmalig nach Deploy
// Tut:
//   1. Config-PDA erstellen (seeds = [b"config"])
//   2. DEX-Vault-PDA erstellen (seeds = [b"dex_vault"])
//   3. Investor-Vault-PDA erstellen (seeds = [b"investor_vault"])
//   4. Gift-Vault-PDA erstellen (seeds = [b"gift_vault"])  ← NEU
//   5. In Config speichern:
//      - admin, mint
//      - alle 3 Vault-Adressen
//      - alle 4 Bumps
//      - released_tranches = 0
//      - last_release = jetzt
```

## ReleaseDex
```rust
// Wer: Admin
// Wann: Maximal 4x, mindestens 30 Tage Abstand
// Prüft:
//   1. admin == config.admin
//   2. released_tranches < 4
//   3. last_release + 30d < now
//   4. dex_vault.amount >= 100 Mio
//   5. admin_token_account.owner == admin
//   6. admin_token_account.mint == config.mint
// Tut:
//   1. Transfer 100 Mio Token von dex_vault → admin_token_account
//   2. released_tranches += 1
//   3. last_release = now
```

## RegisterInvestor
```rust
// Wer: Admin (nach manueller Prüfung: ID-Scan, Zahlungseingang)
// Parameter: amount (gekaufte Menge ohne Bonus)
// Prüft:
//   1. admin == config.admin
//   2. investor_token_account.owner == investor
//   3. investor_token_account.mint == config.mint
//   4. investor_vault.amount >= amount
//   5. gift_vault.amount >= (amount * 20 / 100)  // Bonus
// Tut:
//   1. Berechne bonus = amount * 20 / 100
//   2. Transfer amount von investor_vault → investor_token_account
//   3. Transfer bonus von gift_vault → investor_token_account
//   4. (Optional) InvestorReceipt-PDA erstellen
```

## ClaimStarter
```rust
// Wer: Admin (per Batch-Script)
// Parameter: user (Pubkey des Empfängers)
// Prüft:
//   1. admin == config.admin
//   2. user_token_account.owner == user
//   3. user_token_account.mint == config.mint
//   4. gift_vault.amount >= 7 * 10^9
//   5. StarterClaim-PDA für user existiert noch NICHT
// Tut:
//   1. Transfer 7 Tokens von gift_vault → user_token_account
//   2. StarterClaim-PDA erstellen (seeds = [b"starter", user.key()])
//      mit: user, claimed_at = now, bump
```

## Earlybird
```rust
// Wer: Admin
// Parameter: user, amount
// Prüft:
//   1. admin == config.admin
//   2. user_token_account.owner == user
//   3. user_token_account.mint == config.mint
//   4. gift_vault.amount >= amount
// Tut:
//   1. Transfer amount von gift_vault → user_token_account
```

## DevAllocation
```rust
// Wer: Admin
// Parameter: amount
// Prüft:
//   1. admin == config.admin
//   2. admin_token_account.owner == admin
//   3. admin_token_account.mint == config.mint
//   4. gift_vault.amount >= amount
// Tut:
//   1. Transfer amount von gift_vault → admin_token_account
```
