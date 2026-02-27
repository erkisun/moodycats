# CONTRACT 1: INVESTOR-DEX-VESTING - INVARIANTEN

Haupt-README: /README.md
Contract-README: ./README.md
Spezifikationen: ./SPECS.md

## ADMIN & AUTHORITY

I1: config.admin != Null-Adresse (bis revoke_admin)
I2: Nur config.admin darf kritische Funktionen aufrufen
I3: Config ist Authority aller Vaults

## VAULTS

I4: config.dex_vault = PDA mit seeds=[b"dex_vault"]
I5: config.investor_vault = PDA mit seeds=[b"investor_vault"]
I6: config.gift_vault = PDA mit seeds=[b"gift_vault"]
I7: Alle Vaults haben denselben Mint (config.mint)

## BUMPS

I8: config.bump = Bump der Config-PDA
I9: config.dex_vault_bump = Bump des DEX-Vaults
I10: config.investor_vault_bump = Bump des Investor-Vaults
I11: config.gift_vault_bump = Bump des Gift-Vaults

## DEX-VESTING

I12: released_tranches in {0,1,2,3,4}
I13: Summe transferierte DEX-Tokens = released_tranches * 100 Mio
I14: last_release + 30d <= current_time (bei erfolgreichem release_dex)
I15: released_tranches = 4 => dex_vault.amount = 0

## INVESTOREN-VERKAUF

I16: investor_vault.amount + Summe ausgezahlte Investoren = 500 Mio
I17: Jeder Investor erhält: amount + (amount * 20%)

## STARTER-TOKENS

I18: Jeder User kann nur einmal 7 Tokens erhalten
I19: Für jeden User mit Starter-Tokens existiert StarterClaim-PDA
I20: Summe Starter-Tokens <= 500 Mio (gift_vault)

## DEV-ALLOCATION

I21: Phase 1,2,3 können nur einmal ausgezahlt werden
I22: Summe (Phase1 + Phase2 + Phase3) = 50 Mio Tokens
I23: dev_phaseX_paid = true <=> Phase X wurde ausgezahlt

## GIFT-VAULT

I24: gift_vault.amount + Summe Auszahlungen = 500 Mio
I25: Auszahlungen aus gift_vault nur für:
     - Investoren-Bonus (register_investor)
     - Starter-Tokens (claim_starter)
     - Earlybird-Boni (earlybird_bonus)
     - Dev-Anteile (dev_allocation)

## ZEIT

I26: last_release <= current_time (immer)