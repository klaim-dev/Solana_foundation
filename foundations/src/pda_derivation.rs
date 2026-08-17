use sha2::{Digest, Sha256};
use solana_pubkey::Pubkey;

pub fn find_pda_standard_method(program_id: Pubkey, seeds: &[&[u8]]) -> (Pubkey, u8) {
    let (pda, bump) = Pubkey::find_program_address(seeds, &program_id);
    (pda, bump)
}

pub fn find_pda_manual_method(program_id: Pubkey, seeds: &[&[u8]]) -> (Pubkey, u8) {
    let mut bump = 255u8;
    loop {
        let bump_seed = [bump];
        let bump_slice = &bump_seed;

        let mut hasher = Sha256::new();
        for seed in seeds {
            hasher.update(seed);
        }
        hasher.update(bump_slice);
        hasher.update(program_id.as_ref());
        hasher.update(b"ProgramDerivedAddress");

        let hash = hasher.finalize();
        let hash_bytes = hash.into();
        let candidate = Pubkey::new_from_array(hash_bytes);

        if !candidate.is_on_curve() {
            return (candidate, bump);
        }

        if bump == 0 {
            panic!("No valid PDA found")
        }

        bump -= 1;
    }
}

pub fn find_alternative_pda(
    program_id: Pubkey,
    seeds: &[&[u8]],
    canonical_bump: u8,
) -> (Pubkey, u8) {
    for bump in (0..canonical_bump).rev() {
        let bump_seed = [bump];
        let bump_slice = &bump_seed;

        let mut hasher = Sha256::new();
        for seed in seeds {
            hasher.update(seed);
        }
        hasher.update(bump_slice);
        hasher.update(program_id.as_ref());
        hasher.update(b"ProgramDerivedAddress");

        let hash = hasher.finalize();
        let hash_bytes = hash.into();
        let candidate = Pubkey::new_from_array(hash_bytes);

        if !candidate.is_on_curve() {
            return (candidate, bump);
        }
    }

    panic!("No alternative PDA found")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn find_three_pda() {
        let program_id = Pubkey::from_str("72puLt71H93Z9CzHuBRTwFpL4TG3WZUhnoCC7p8gxigu").unwrap();
        let cases: [&[&[u8]]; 3] = [&[b"rust"], &[b"sol"], &[b"curve"]];
        for seeds in cases {
            let (pda, bump) = find_pda_standard_method(program_id, seeds);
            println!("pda: {}, bump: {}", pda, bump);
        }
    }

    #[test]
    fn find_three_pda_manual() {
        let program_id = Pubkey::from_str("72puLt71H93Z9CzHuBRTwFpL4TG3WZUhnoCC7p8gxigu").unwrap();
        let cases: [&[&[u8]]; 3] = [&[b"rust"], &[b"sol"], &[b"curve"]];

        for seeds in cases {
            let (pda, bump) = find_pda_manual_method(program_id, seeds);
            println!("pda: {}, bump{}", pda, bump)
        }
    }

    #[test]
    fn gate_test() {
        let program_id = Pubkey::from_str("72puLt71H93Z9CzHuBRTwFpL4TG3WZUhnoCC7p8gxigu").unwrap();
        let cases: [&[&[u8]]; 3] = [&[b"rust"], &[b"sol"], &[b"curve"]];
        for seeds in cases {
            let (sdk_pda, sdk_bump) = find_pda_standard_method(program_id, seeds);
            let (manual_pda, manual_bump) = find_pda_manual_method(program_id, seeds);

            assert_eq!(sdk_pda, manual_pda);
            assert_eq!(sdk_bump, manual_bump);
        }
    }

    #[test]
    fn test_find_alternative_pda() {
        let program_id = Pubkey::from_str("72puLt71H93Z9CzHuBRTwFpL4TG3WZUhnoCC7p8gxigu").unwrap();
        let seed: &[&[u8]] = &[b"sol"];

        let (canonical_pda, canonical_bump) = find_pda_standard_method(program_id, seed);
        let (alternative_pda, alternative_bump) =
            find_alternative_pda(program_id, seed, canonical_bump);

        println!(
            "canonical PDA = {}, bump = {}",
            canonical_pda, canonical_bump
        );
        println!(
            "alternative PDA = {}, bump = {}",
            alternative_pda, alternative_bump
        );

        assert_ne!(canonical_pda, alternative_pda);
        assert!(canonical_bump > alternative_bump);
        assert!(!alternative_pda.is_on_curve());
        assert!(!canonical_pda.is_on_curve());
    }

    #[test]
    fn test_seed_boundary_collision() {
        let program_id = Pubkey::from_str("72puLt71H93Z9CzHuBRTwFpL4TG3WZUhnoCC7p8gxigu").unwrap();
        let cases: [&[&[u8]]; 3] = [&[b"abcdef"], &[b"abc", b"def"], &[b"ab", b"cd", b"ef"]];
        let mut result = Vec::new();
        for seeds in cases {
            let (pda, bump) = find_pda_standard_method(program_id, seeds);
            result.push((pda, bump));
        }
        assert_eq!(result[0], result[1]);
        assert_eq!(result[1], result[2]);

        let user_seeds: &[&[u8]] = &[b"user", b"abcdef"];
        let vault_seeds: &[&[u8]] = &[b"vault", b"abcdef"];

        let (user_pda, _) = find_pda_standard_method(program_id, user_seeds);
        let (vault_pda, _) = find_pda_standard_method(program_id, vault_seeds);
        assert_ne!(user_pda, vault_pda);
    }
}
