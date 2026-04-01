## MOODYCATS – SMART CONTRACT 1

🔗 investor_dex_vesting

## 📋 ÜBERSICHT

Contract 1 verwaltet die **erste Phase** der Moodycats-Tokenomics:
- **DEX-Liquidität** (400 Mio Tokens)
- **Investoren-LP-Erhöhung** (500 Mio Tokens)  
- **Gift-Vault** (500 Mio Tokens für Boni, Starter, Dev)

| Aspekt | Details |
|--------|---------|
| **Programm-ID** | `A35GmMxidLvM6LaL8n17PCFU9zoQeEp5Zm5TtmRRwddy` |
| **Token-Mint** | Wird bei initialize gesetzt |
| **Authority** | Config-PDA (Programm kontrolliert alle Vaults) |
                                                                        
## 🏗️ ARCHITEKTUR

```rust

// -----------------------------------------------
// MOODYCATS TOKENOMICS
//
// TOTAL SUPPLY: 5.000.000.000 Token (5 Mia)
//
// 1. DEX PRE-LAUNCH: 500 Mio (10%)
//    - Initial LP auf Raydium: 100 Mio Token + ~15 SOL (manuell durch Dev)
//    - DEX-Vesting: 400 Mio Token im DEX-Vault (4 Tranchen à 100 Mio)
//
// 2. WEBSEITE-INVESTOREN: 500 Mio (10%)
//    - Verkauf über Webseite: 0.001 $/Token (fester Preis, teurer als DEX-Start)
//    - Investoren zahlen SOL (off-chain), erhalten Token SOFORT aus Investor-Vault
//    - 20% Bonus kommt aus GIFT-VAULT
//    - KEIN Vesting, KEINE Sperre
//    - Gesammelte SOL landen in Admin-Wallet und werden für LP-Erhöhungen genutzt
//
// 3. GIFT-VAULT (NEU): 500 Mio (10%)
//    - 20% Bonus für Investoren
//    - Starter-Tokens (7 pro User)
//    - Earlybird-Boni (100 Tokens für erste 100k User)
//    - Dev-Anteile (50 Mio, gestaffelt in 3 Phasen)
//    - Flexible Community-Geschenke
//
// 4. CONTRACT 2
//    - App Launch : 500 Mio (10%), ab Nov 2026
//    - 1 Token = 1 API-Call (app_per_pay_use)
//
// 5. CONTRACT 3
//    - Treasury Vesting : 2.500 Mio (50%)
//    - 5 Jahre linear, 500 Mio pro Jahr an Nutzer
//    - Ausschüttung aller restlichen Token an alle, falls Ziele unterschritten

```
