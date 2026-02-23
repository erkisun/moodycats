# MOODYCATS – TECHNISCHE SPEZIFIKATIONEN

## 🎯 ÜBERGREIFENDE REGELN
- **Token Mint:** Einheitlich für alle Contracts
- **Decimals:** 9 (Standard)
- **Admin:** Dev-Wallet (für alle Contracts initial)
- **Upgrade Authority:** Wird nach Deploy revoken
- **Freeze Authority:** Wird nach Deploy revoken

---

## CONTRACT 1: investor-dex-vesting

### 📦 VAULTS (3 Stück)

| Vault | Menge | Seeds | Zweck |
|-------|-------|-------|-------|
| `dex_vault` | 500 Mio | `b"dex_vault"` | LP + Vesting (4×100 Mio) |
| `investor_vault` | 500 Mio | `b"investor_vault"` | Verkauf an Investoren (0.001 $/Token) |
| `gift_vault` | 500 Mio | `b"gift_vault"` | Earlybirds, Starter (7), Dev, 20% Bonus |

### ⚙️ INSTRUCTIONS

#### `initialize()`
- Erstellt Config-PDA
- Erstellt alle 3 Vault-PDAs (mit Config als Authority)
- Speichert alle Bumps

#### `release_dex()`
- Nur Admin
- Holt Tokens aus `dex_vault`
- Maximal 4 Tranchen (je 100 Mio)
- 30 Tage Abstand zwischen Tranchen

#### `register_investor(amount: u64)`
- Nur Admin (nach manueller Prüfung: ID-Scan, Zahlungseingang)
- Holt Grundbetrag aus `investor_vault`
- Holt **20% Bonus** aus `gift_vault`
- Transfer SOFORT an Investor

#### `claim_starter(user: Pubkey)`
- Nur Admin (per Batch-Script)
- Holt **7 Tokens** aus `gift_vault`
- Pro User einmalig (PDA als Proof)
- Für jede Neuregistrierung (mit Verifikation)

#### `earlybird_bonus(user: Pubkey, amount: u64)`
- Nur Admin
- Holt aus `gift_vault`
- Für erste Investoren/User (zeitlich begrenzt)

#### `dev_allocation(amount: u64)`
- Nur Admin
- Holt aus `gift_vault`
- Fairer Anteil für Team

### 🔒 SICHERHEIT CONTRACT 1
- Nur Admin darf Funktionen aufrufen
- Config ist Authority für alle Vaults (PDA-Signer)
- Replay-Schutz durch PDAs (z.B. StarterClaim)
- 30-Tage-Time-Lock für DEX-Tranchen

### 💰 OFF-CHAIN KOMPONENTE
- **Zahlung:** Investoren zahlen SOL an Admin-Wallet
- **Verifikation:** ID-Scan + Google Vision + 2FA optional
- **LP-Erhöhungen:** Admin fügt manuell SOL + Tokens bei Raydium hinzu
- **Batch-Script:** Für Starter-Tokens (CSV-Export aus DB)

---

## CONTRACT 2: app-per-pay-use (folgt später)

### 📦 VAULTS
- `app_vault`: 500 Mio – Pay-per-use

### ⚙️ LOGIK
- **1 Token = 1 API-Call**
- Start: November 2026

---

## CONTRACT 3: treasury-vesting (folgt später)

### 📦 VAULTS
- `treasury_vault`: 2.500 Mio – 5-Jahres-Vesting

### ⚙️ LOGIK
- **5 Jahre linear**, 500 Mio pro Jahr
- Empfänger: Alle registrierten Nutzer
- Bei Unterschreitung der Neuregistrierungen: **Rest gleichmässig an alle**
- Verwendung: Neuregistrierung, Airdrops, App-Subventionen, LP-Erhöhungen