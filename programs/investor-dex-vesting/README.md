# MOODYCATS – SMART CONTRACTS
### moodycats.com / moodycats.io
### Whitepaper : https://moodycats.io/faq/Whitepaper

# 🚀 CONTRACT 1: investor-dex-vesting

## 📋 ÜBERBLICK
Dieser Contract verwaltet:

### 1. DEX PRE-LAUNCH (500 Mio)
- **Initial LP:** 100 Mio Token + ~15 SOL (manuell durch Dev via Raydium UI)
- **DEX-Vesting:** 400 Mio Token im Vault
- **4 Tranchen à 100 Mio**, 30 Tage Abstand
- Admin fügt jede Tranche + gesammelte SOL manuell ins LP hinzu

### 2. WEBSEITE-INVESTOREN (500 Mio)
- **Preis:** 0.001 $/Token (fester Preis, teurer als DEX)
- **Zahlung:** Investoren zahlen SOL off-chain an Admin-Wallet
- **Ausführung:** Nach ID-Scan + Zahlungseingang ruft Admin `register_investor()` auf
- **Transfer:** Tokens sofort an Investor (kein Vesting)

### 3. GIFT VAULT (500 Mio) – NEU
- **20% Bonus** für Investoren
- **Starter-Tokens:** 7 Token pro Neuregistrierung
- **Earlybird-Boni** für erste User
- **Dev-Anteile** für Team

## 📦 VAULTS (alle PDAs mit Config als Authority)
