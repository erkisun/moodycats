-----------------------------------------------
moodycats.com / moodycats.io

programs/investor-dex-vesting/README.md

-----------------------------------------------
CONTRACT 1: INVESTOREN-VERKAUF + DEX-VESTING
-----------------------------------------------

A. VORBEREITUNG (Dev / Admin)
├─► Token mit Solana CLI erstellen, Metadata hinzufügen
├─► Raydium LP: 100 Mio Token + ~15 SOL (manuell, via UI)
├─► Mint Authority REVOKE (sofort nach Mint)
├─► Freeze Authority REVOKE (sofort)
├─► Update Authority / Metadaten REVOKE (sofort)
└─► ✅ Token handelbar auf DEX

B. CONTRACT 1 DEPLOY & INIT
├─► initialize(): Config mit Admin = Dev, Token-Mint, Vault-Adressen, Bumps
├─► 500 Mio Token (Investoren) + 400 Mio Token (DEX-Rest) in Contract-Vaults transferieren
└─► ✅ Contract bereit

C. DEX-VESTING (manuelle Freigabe durch Admin)
├─► Nur Admin darf release_dex_tokens() aufrufen
├─► Pro Aufruf: 100 Mio Token aus DEX-Vault an Admin-Wallet
├─► Maximal 4 Aufrufe, mindestens 30 Tage Abstand
├─► Keine Automatisierung – Admin entscheidet, wann freigegeben wird
└─► Nach jeder Freigabe: Admin fügt 100 Mio Token + passende SOL manuell ins Raydium-LP hinzu

D. INVESTOREN-KÄUFE (OTC)
├─► Investor zahlt SOL auf Webseite (off-chain)
├─► Webseite ruft register_investor() auf:
    • Berechnet Token-Menge + 20% Bonus
    • Erstellt optional eine InvestorReceipt (PDA, nur für Transparenz)
    • Transferiert Token SOFORT aus Investor-Vault an Investor-Wallet
├─► Kein Vesting, keine Sperrfrist
└─► SOL verbleibt in Admin-Wallet (für spätere LP-Erhöhungen)

E. LP-ERHÖHUNG (manuell, durch Admin)
├─► Admin hat 100 Mio Token aus release_dex_tokens() erhalten
├─► Admin hat SOL von Investoren gesammelt (in eigener Wallet)
├─► Admin geht zu Raydium UI → Add Liquidity
├─► Fügt 100 Mio Token + SOL-Betrag (1:1 USD-Wert, geschätzt oder per Oracle) hinzu
├─► Wiederholen, bis alle 400 Mio DEX-Token im LP sind
└─► Jeder Schritt ist on-chain sichtbar → Vertrauensbeweis

F. DEZENTRALISIERUNG (nach Abschluss aller DEX-Freigaben)
├─► revoke_admin(): Admin setzt sich selbst auf 0x000… (Null-Adresse)
├─► Danach kann niemand mehr release_dex_tokens() aufrufen
├─► Update Authority revoken (Metaplex) → Token immutable
├─► LP Authority (falls gesetzt) burnen oder an Null-Adresse senden
└─► ✅ Contract 1 ist herrenlos, DEX-Vesting abgeschlossen

G. NICHT VERKAUFTE INVESTOREN-TOKENS (optional)
├─► Falls nach Ende der Pre-Launch-Phase noch Tokens im Investor-Vault sind
├─► Können verbrannt werden (Supply-Reduktion) oder in App-Contract wandern
└─► Stärkt Vertrauen und Knappheit

-----------------------------------------------
WICHTIG: Was Contract 1 NICHT kann
-----------------------------------------------
❌ Kein SOL empfangen (SOL geht off-chain an Admin-Wallet)
❌ Kein CPI an Raydium (keine automatische LP-Erhöhung)
❌ Kein Oracle
❌ Kein Vesting für Investoren
❌ Keine Mint-/Freeze-Funktionen
❌ Keine Upgradeability

Die 5 Gründe, warum Automatisierung hier schadet
1. Komplexität explodiert
   Mein Contract müsste plötzlich SOL empfangen und verwalten – das bedeutet, ich brauche einen Treasury-PDA, der SOL hält.
   Ich brauche einen Oracle (oder einen festen Kurs), um den aktuellen Preis zu kennen.
   Ich brauche Raydium-CPI mit Dutzenden von Accounts.
   Ich brauche Fehlerbehandlung: Was, wenn der Raydium-Call fehlschlägt? Kriegt der Investor dann trotzdem seine Tokens? Oder wird die ganze Transaktion rückgängig gemacht?

👉 Aus einem 3-Funktionen-Contract wird ein 500-Zeilen-Monster.
2. Sicherheitsrisiken schießen durch die Decke
   Ein Contract, der SOL hält, ist ein lohnendes Ziel für Hacker.
   Ein falscher Oracle-Wert kann dazu führen, dass du zu wenig SOL ins LP steckst (der Pool wird unfair) oder zu viel (du verschenkst Liquidität).
   Ein Fehler im CPI-Call kann bedeuten, dass Tokens oder SOL verschwinden.

👉 Ohne teures Audit will das kein Investor anfassen.
3. Transparenz geht verloren
   Bei manuellen Aktionen sieht jeder on-chain: *„Am 15.03. hat der Dev 100M Token + 500 SOL ins LP gesteckt.“*
   Das ist ein Vertrauensbeweis.
   Bei Automatisierung sieht man nur noch: „Irgendein Contract hat irgendwas getan.“
   Investoren fragen sich: „Wurde da richtig gerechnet? Hat der Dev sich selbst bevorzugt?“

👉 Manuell ist transparenter.
4. Flexibilität wird eingeschränkt
   Wenn du manuell LP hinzufügst, kannst du den perfekten Zeitpunkt wählen.
      Ist der Markt gerade bullish? Dann steckst du mehr SOL rein.
      Ist der Markt flau? Dann wartest du etwas.
   Automatisch heißt: Immer zum gleichen Kurs, egal ob gut oder schlecht.

👉 Manuell = strategisch, automatisch = starr.
5. Zeit- und Kostenfresser
    Automatisierung bedeutet:
       Wochen statt Tage Entwicklung
       Audits ($5k–$20k statt $0)
       Testing, Debugging, Frustration
    All das lenkt dich von deiner eigentlichen Mission ab: Die App zu bauen.

👉 Mein Ziel ist nicht, der beste DeFi-Developer zu sein, sondern eine erfolgreiche App zu haben.

-----------------------------------------------