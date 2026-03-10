# 🐱 MOODYCATS – Smart Contracts

> **moodycats.com** | **moodycats.io**  

## 📖 Whitepaper
[Vollständiges Whitepaper](https://moodycats.io/faq/Whitepaper)

---

## 🎯 Überblick
Moodycats ist ein **dreiteiliges Smart-Contract-System** auf Solana Blockchain
- 🔐 **Sichere Token-Verteilung** (Vesting, OTC-Verkauf)
- 💡 **Nutzungsbasierte App-Integration** (1 Token = 1 API-Call)
- 🏦 **Langfristiges Treasury** (5-Jahres-Vesting für alle User)

**Alle Contracts nutzen den GLEICHEN Token Mint!**

---

## 📊 TOKENOMICS
**Total Supply:** `5.000.000.000` (5 Milliarden) Token mit 9 Decimals

| # | Kategorie | Tokens | % | Contract |
|---|-----------|--------|---|----------|
| 1 | **DEX PRE-LAUNCH** | 500 Mio | 10% | [`investor-dex-vesting`](/programs/investor-dex-vesting) |
| 2 | **WEBSEITE-INVESTOREN** | 500 Mio | 10% | [`investor-dex-vesting`](/programs/investor-dex-vesting) |
| 3 | **GIFT VAULT** | 500 Mio | 10% | [`investor-dex-vesting`](/programs/investor-dex-vesting) |
| 4 | **APP LAUNCH** | 1.000 Mio | 20% | [`app-pay-per-use`](/programs/app-pay-per-use) |
| 5 | **TREASURY VESTING** | 2.500 Mio | 50% | [`treasury-vesting`](/programs/treasury-vesting) |

---

## 📦 CONTRACT 1: `investor-dex-vesting`
**Zuständigkeit:** DEX-Liquidität + Investoren-Verkauf + Gift Vault  
**Tokens:** 1.500 Mio (30% vom Supply)  
**[👉 Detaillierte Beschreibung →](/programs/investor-dex-vesting/README.md)**

### Kernfunktionen
| Funktion | Beschreibung | Sicherheit |
|----------|--------------|------------|
| `initialize()` | Einmalige Initialisierung (Config + 3 Vaults) | Nur Admin |
| `release_dex()` | 4×100 Mio DEX-Tranchen (30d Abstand) | Time-Lock |
| `register_investor()` | OTC-Verkauf + 20% Bonus | Getrennte Vaults |
| `claim_starter()` | 7 Starter-Tokens pro User (einmalig) | PDA-Replay-Schutz |
| `earlybird_bonus()` | Flexible Boni für erste User | Manuell |
| `dev_allocation()` | 50 Mio Dev-Anteile (3 Phasen) | Flags |
| `revoke_admin()` | Admin entziehen (nach Abschluss) | 🔐 Final |

---

## 📦 CONTRACT 2: `app-pay-per-use`
**Zuständigkeit:** App-Nutzung (1 Token = 1 API-Call)  
**Tokens:** 1.000 Mio (20% vom Supply)  
**Launch:** November 2026  
**[👉 Detaillierte Beschreibung →](/programs/app-pay-per-use/README.md)**

### Kernfunktionen
| Funktion | Beschreibung | Sicherheit |
|----------|--------------|------------|
| `initialize()` |  | voll automatisiert |
| `()` |  |  |

---

## 📦 CONTRACT 3: `treasury-vesting`
**Zuständigkeit:** 5-Jahres-Vesting für alle User  
**Tokens:** 2.500 Mio (50% vom Supply)  
**[👉 Detaillierte Beschreibung →](/programs/treasury-vesting/README.md)**

### Kernfunktionen
| Funktion | Beschreibung | Sicherheit |
|----------|--------------|------------|
| `initialize()` |  | voll automatisiert |
| `()` |  |  |

---

## 🔗 WICHTIG
✅ **Alle drei Contracts nutzen den GLEICHEN Token Mint**  
✅ **Jeder Contract hat seine eigene README.md mit Details**  
✅ **On-Chain-Daten sind vollständig verifizierbar**

---

## 🛡️ Sicherheitsphilosophie
- **Manuelle Schritte** wo Automatisierung Risiken birgt (Web-Hacks)
- **Mehrstufige Prüfungen** in jeder Instruktion
- **PDA-basierte Vaults** mit Config als Authority
- **Keine Upgradeability** – Vertrauen durch Unveränderlichkeit

---

## 🚀 Deployment (vorläufig)
| Contract | Programm-ID | Status |
|----------|-------------|--------|
| investor-dex-vesting | `A35GmMxidLvM6LaL8n17PCFU9zoQeEp5Zm5TtmRRwddy` | ✅ Aktiv |
| app-pay-per-use | `...` | ⏳ (Nov 2026) |
| treasury-vesting | `...` | ⏳ |

---

## 📄 Lizenz
MIT