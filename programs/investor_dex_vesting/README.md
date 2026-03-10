# MOODYCATS – SMART CONTRACTS
### moodycats.com / moodycats.io
### Whitepaper : https://moodycats.io/faq/Whitepaper

# 🏦 CONTRACT 1: investor-dex-vesting

**Teil des Moodycats-Ökosystems**  
🔗 [Haupt-README](../../README.md) | 📊 [Tokenomics](../../README.md#-tokenomics) | 📖 [Spezifikationen](./SPECS.md)

## 📋 ÜBERSICHT

Contract 1 verwaltet die **erste Phase** der Moodycats-Tokenomics:
- **DEX-Liquidität** (400 Mio Tokens)
- **Investoren-Verkauf** (500 Mio Tokens)  
- **Gift-Vault** (500 Mio Tokens für Boni, Starter, Dev)

| Aspekt | Details |
|--------|---------|
| **Programm-ID** | `A35GmMxidLvM6LaL8n17PCFU9zoQeEp5Zm5TtmRRwddy` |
| **Token-Mint** | Wird bei initialize gesetzt |
| **Authority** | Config-PDA (Programm kontrolliert alle Vaults) |

## 🏗️ ARCHITEKTUR