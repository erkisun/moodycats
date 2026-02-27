# MOODYCATS – SMART CONTRACTS
### moodycats.com / moodycats.io
### Whitepaper : https://moodycats.io/faq/Whitepaper

# 🚀 CONTRACT 1: investor-dex-vesting

// ======================================================
// programs/investor-dex-vesting/SPECS.md
// INVARIANTEN FÜR CONTRACT 1 (müssen IMMER gelten)
// ======================================================

// 1. ADMIN
// I1: config.admin ≠ Pubkey::default() (bis revoke_admin)

// 2. VAULTS
// I2: config.dex_vault = PDA mit seeds=[b"dex_vault"]
// I3: config.investor_vault = PDA mit seeds=[b"investor_vault"]
// I4: config.gift_vault = PDA mit seeds=[b"gift_vault"]
// I5: Alle Vaults haben config als Authority
// I6: Alle Vaults haben denselben Mint (config.mint)

// 3. BUMPS
// I7: config.bump = Bump von config-PDA
// I8: config.dex_vault_bump = Bump von dex_vault
// I9: config.investor_vault_bump = Bump von investor_vault
// I10: config.gift_vault_bump = Bump von gift_vault

// 4. DEX-VESTING
// I11: released_tranches ∈ {0,1,2,3,4}
// I12: ∑ transferierte DEX-Tokens = released_tranches × 100 Mio
// I13: last_release + 30d ≤ current_time (bei erfolgreichem release_dex)
// I14: released_tranches = 4 ⇒ dex_vault.amount = 0

// 5. INVESTOREN-VERKAUF
// I15: investor_vault.amount + ∑ ausgezahlte Investoren = 500 Mio
// I16: Jeder Investor erhält: amount + (amount × 20%)

// 6. STARTER-TOKENS
// I17: Jeder User kann NUR EINMAL 7 Tokens erhalten
// I18: Für jeden User mit Starter-Tokens existiert StarterClaim-PDA
// I19: ∑ Starter-Tokens ≤ 500 Mio (gift_vault)

// 7. DEV-ALLOCATION
// I20: Phase 1,2,3 können NUR EINMAL ausgezahlt werden
// I21: ∑ (Phase1 + Phase2 + Phase3) = 50 Mio Tokens
// I22: dev_phaseX_paid = true ⇔ Phase X wurde ausgezahlt

// 8. GIFT-VAULT
// I23: gift_vault.amount + ∑ Auszahlungen = 500 Mio
// I24: Auszahlungen aus gift_vault nur für:
//      - Investoren-Bonus (register_investor)
//      - Starter-Tokens (claim_starter)
//      - Earlybird-Boni (earlybird_bonus)
//      - Dev-Anteile (dev_allocation)

// 9. ZEIT
// I25: last_release ≤ current_time (immer)