// -----------------------------------------------
// programs/investor-dex-vesting/instructions/release_investor.rs
//
// INVESTOREN-VERKAUF (OTC)
// -----------------------------------------------
// 
// WAS PASSIERT HIER?
// Diese Funktion wird AUFGERUFEN, wenn ein Investor auf der Webseite SOL bezahlt hat.
// Der Off-Chain-Teil (Zahlung) passiert separat, diese Funktion transferiert NUR die Tokens.
//
// ABLAUF:
// 1. Admin/Webseite ruft Funktion auf (mit Investor-Infos)
// 2. Berechnet Token-Menge + 20% Bonus
// 3. Prüft: Genug Tokens im Investor-Vault?
// 4. Transferiert Tokens SOFORT an Investor
// 5. Optional: Erstellt InvestorReceipt (für Transparenz)
//
// WICHTIG:
// - KEINE SOL-Transaktion! SOL wurde bereits OFF-CHAIN gezahlt
// - Sofortige Übertragung, kein Vesting
// - Investor erhält Tokens direkt in seine Wallet
// -----------------------------------------------