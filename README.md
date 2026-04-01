# 🏙️ MoodyCats


![MoodyCats Banner](./moodycats.io.png)

---

## 🎯 Überblick
Moodycats ist ein **dreiteiliges Smart-Contract-System** auf Solana Blockchain
- 🔐 **Sichere Token-Verteilung** (Vesting, OTC-Verkauf)
- 💡 **Nutzungsbasierte App-Integration** (1 Token = 1 API-Call)
- 🏦 **Langfristiges Treasury** (5-Jahres-Vesting für alle User)

✅ **Alle drei Contracts nutzen den GLEICHEN Token Mint**  
✅ **Jeder Contract hat seine eigene README.md mit Details**  
✅ **On-Chain-Daten sind vollständig verifizierbar**

---

## 📊 TOKENOMICS
**Total Supply:** `5.000.000.000` (5 Milliarden) Token mit 9 Decimals

| # | Kategorie | Tokens | % | Contract | Off-Chain |
|---|-----------|--------|---|----------|-----------|
| 1 | **DEX PRE-LAUNCH** | 500 Mio | 10% | [`investor_dex_vesting`](./programs/investor_dex_vesting) | https://moodycats.io |
| 1 | **INVESTOREN WEB** | 500 Mio | 10% | [`investor_dex_vesting`](./programs/investor_dex_vesting) | https://moodycats.io |
| 1 | **GIFT VAULT** | 500 Mio | 10% | [`investor_dex_vesting`](./programs/investor_dex_vesting) | https://moodycats.io |
| 2 | **APP LAUNCH** | 1.000 Mio | 20% | [`app_pay_per_use`](./programs/app_pay_per_use) | https://moodycats.com |
| 3 | **TREASURY VESTING** | 2.500 Mio | 50% | [`treasury_vesting`](./programs/treasury_vesting) | https://moodycats.com |

[Vollständiges Whitepaper](https://moodycats.io/faq/Whitepaper)

---

## 📦 CONTRACT 1 : `investor_dex_vesting`
**Zuständigkeit:** DEX-Liquidität + Investoren-Verkauf + Gift Vault  
**Tokens:** 1.500 Mio (30% vom Supply)  
**[👉 Detaillierte Beschreibung →](./programs/investor_dex_vesting)**

### Kernfunktionen
| Funktion | Beschreibung | Sicherheit |
|----------|--------------|------------|
| `initialize()` | Einmalige Initialisierung (Config + 3 Vaults) | Nur Admin |
| `release_dex()` | 4×100 Mio DEX-Tranchen (30d Abstand) | Time-Lock |
| `register_investor()` | OTC-Verkauf + Bonus Tokens | Getrennte Vaults |
| `claim_starter()` | 7 Starter-Tokens pro User (einmalig) | PDA-Replay-Schutz |
| `earlybird_bonus()` | Flexible Boni für erste User | Manuell |
| `dev_allocation()` | 50 Mio Dev-Anteile (3 Phasen) | Flags |
| `revoke_admin()` | Admin entziehen (nach Abschluss) | ✅ Final |

---

## 📦 CONTRACT 2 : `app_pay_per_use`
**Zuständigkeit:** App-Nutzung (1 Token = 1 API-Call)  
**Tokens:** 1.000 Mio (20% vom Supply)  
**Launch:** November 2026  
**[👉 Detaillierte Beschreibung →](/programs/app_pay_per_use)**

### Kernfunktionen
| Funktion | Beschreibung | Sicherheit |
|----------|--------------|------------|
| `initialize()` |  | voll automatisiert |
| `()` |  |  |

---

## 📦 CONTRACT 3 : `treasury_vesting`
**Zuständigkeit:** 5-Jahres-Vesting für alle User  
**Tokens:** 2.500 Mio (50% vom Supply)  
**[👉 Detaillierte Beschreibung →](/programs/treasury_vesting)**

### Kernfunktionen
| Funktion | Beschreibung | Sicherheit |
|----------|--------------|------------|
| `initialize()` |  | voll automatisiert |
| `()` |  |  |

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
| investor-dex-vesting | `A35GmMxidLvM6LaL8n17PCFU9zoQeEp5Zm5TtmRRwddy` | ✅ Testnet |
| app-pay-per-use | `...` | ⏳ (Nov 2026) |
| treasury-vesting | `...` | ⏳ (Nov 2026) |

---

## 📄 Lizenz
MIT