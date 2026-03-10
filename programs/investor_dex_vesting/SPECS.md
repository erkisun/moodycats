# MOODYCATS – SMART CONTRACTS
### moodycats.com / moodycats.io
### Whitepaper : https://moodycats.io/faq/Whitepaper

# 🚀 CONTRACT 1: investor-dex-vesting - SPEZIFIKATIONEN

Haupt-README: /README.md
Contract-README: ./README.md
Invarianten: ./INVARIANTS.md

## 1. ZWECK

Contract 1 verwaltet die erste Phase der Moodycats-Tokenomics:

- DEX-Liquidität (400 Mio Tokens) – für Raydium LP
- Investoren-Verkauf (500 Mio Tokens) – OTC über Webseite
- Gift-Vault (500 Mio Tokens) – für Boni, Starter-Tokens und Dev-Anteile

## 2. PROGRAMM-ID

A35GmMxidLvM6LaL8n17PCFU9zoQeEp5Zm5TtmRRwddy

## 3. KONTEN (PDAs)

| Konto | Seeds | Zweck |
|-------|-------|-------|
| Config | [b"config"] | Globale Konfiguration |
| DEX-Vault | [b"dex_vault"] | 400 Mio Tokens |
| Investor-Vault | [b"investor_vault"] | 500 Mio Tokens |
| Gift-Vault | [b"gift_vault"] | 500 Mio Tokens |
| StarterClaim | [b"starter", user] | Replay-Schutz |

## 4. INSTRUKTIONEN

### 4.1 initialize()

Einmalige Initialisierung nach Deploy.

Erstellt:
- Config-PDA
- DEX-Vault
- Investor-Vault
- Gift-Vault

Aktionen:
- Alle Adressen in Config speichern
- Alle Bumps in Config speichern
- released_tranches = 0
- last_release = jetzt

### 4.2 release_dex()

DEX-Tranchen freigeben.

Aufrufer: Admin
Maximal: 4x
Abstand: 30 Tage
Betrag: 100 Mio Tokens pro Tranche

Prüfungen:
- admin == config.admin
- released_tranches < 4
- last_release + 30d <= jetzt
- dex_vault.amount >= 100 Mio

Aktionen:
- Transfer 100 Mio von dex_vault an admin_token_account
- released_tranches += 1
- last_release = jetzt

### 4.3 register_investor(amount)

Investoren-Kauf mit 20% Bonus.

Aufrufer: Admin (nach off-chain SOL-Zahlung)

Prüfungen:
- admin == config.admin
- investor_vault.amount >= amount
- gift_vault.amount >= bonus (amount * 20%)
- investor_token_account.owner == investor
- investor_token_account.mint == config.mint

Aktionen:
- bonus = amount * 20 / 100
- Transfer amount von investor_vault an investor_token_account
- Transfer bonus von gift_vault an investor_token_account

### 4.4 claim_starter()

Starter-Tokens (7 pro User, einmalig).

Aufrufer: Admin (Batch-Script)

Prüfungen:
- admin == config.admin
- gift_vault.amount >= 7 Tokens
- StarterClaim-PDA existiert noch nicht
- user_token_account.owner == user
- user_token_account.mint == config.mint

Aktionen:
- Transfer 7 Tokens von gift_vault an user_token_account
- StarterClaim-PDA erstellen (user, claimed_at, bump)

### 4.5 earlybird_bonus(amount)

Flexible Earlybird-Boni.

Aufrufer: Admin

Prüfungen:
- admin == config.admin
- gift_vault.amount >= amount
- amount > 0
- user_token_account.owner == user
- user_token_account.mint == config.mint

Aktionen:
- Transfer amount von gift_vault an user_token_account

### 4.6 dev_allocation(phase)

Dev-Anteile in 3 Phasen.

Phasen:
- Phase 1: 20 Mio Tokens (nach Contract 1)
- Phase 2: 15 Mio Tokens (nach Contract 2 live)
- Phase 3: 15 Mio Tokens (bei Contract 3 Start)

Prüfungen:
- admin == config.admin
- phase in {1,2,3}
- dev_phaseX_paid == false (für jeweilige Phase)
- gift_vault.amount >= phasen_betrag
- admin_token_account.owner == admin
- admin_token_account.mint == config.mint

Aktionen:
- Transfer phasen_betrag von gift_vault an admin_token_account
- dev_phaseX_paid = true setzen

### 4.7 revoke_admin() (später)

Admin entziehen.

Aktionen:
- config.admin = Null-Adresse
- Contract wird herrenlos

## 5. CONFIG-FELDER

| Feld | Typ | Zweck |
|------|-----|-------|
| admin | Pubkey | Admin (Dev) |
| mint | Pubkey | Token-Mint |
| dex_vault | Pubkey | DEX-Vault Adresse |
| investor_vault | Pubkey | Investor-Vault Adresse |
| gift_vault | Pubkey | Gift-Vault Adresse |
| released_tranches | u8 | 0..4 |
| last_release | i64 | Timestamp |
| dev_phase1_paid | bool | Flag |
| dev_phase2_paid | bool | Flag |
| dev_phase3_paid | bool | Flag |
| bump | u8 | Config-Bump |
| dex_vault_bump | u8 | DEX-Vault-Bump |
| investor_vault_bump | u8 | Investor-Vault-Bump |
| gift_vault_bump | u8 | Gift-Vault-Bump |

## 6. KONSTANTEN

| Konstante | Wert | Zweck |
|-----------|------|-------|
| TRANCHE_AMOUNT | 100.000.000 * 10^9 | 100 Mio Tokens |
| MIN_DAYS_BETWEEN_RELEASES | 30 * 86400 | 30 Tage in Sekunden |
| BONUS_PERCENT | 20 | 20% Bonus |
| STARTER_AMOUNT | 7 * 10^9 | 7 Tokens |
| DEV_PHASE_1 | 20.000.000 * 10^9 | 20 Mio Tokens |
| DEV_PHASE_2 | 15.000.000 * 10^9 | 15 Mio Tokens |
| DEV_PHASE_3 | 15.000.000 * 10^9 | 15 Mio Tokens |

## 7. ABHÄNGIGKEITEN

- Gleicher Token-Mint wie Contract 2 und Contract 3
- Phase 2/3 der Dev-Allocation erfordern Contract 2/3