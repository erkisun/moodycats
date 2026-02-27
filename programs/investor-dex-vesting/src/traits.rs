// Definiere POLICIES, die JEDE Transaction erfüllen muss
pub trait TransactionPolicy {
    fn check(tx: &Transaction) -> Result<()> {
        // 1. Nur bestimmte Contract-Funktionen erlaubt
        check_allowed_function()?;
        
        // 2. Algorithmus-Größe im Rahmen?
        check_size_limits()?;
        
        // 3. Hash ist plausibel?
        check_hash_format()?;
        
        // 4. Rate limiting: max 10 pro Tag
        check_rate_limit()?;
    }
}