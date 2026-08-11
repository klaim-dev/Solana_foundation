use foundations_derive::HasName;

pub trait HasName {
    fn name() -> &'static str;
}

#[derive(Debug, HasName)]
pub struct User {
    name: String,
}

#[derive(Debug, HasName)]
pub struct Balance {
    amount: u64,
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_has_name() {
        let user_name = User::name();
        let balance_name = Balance::name();
        assert_eq!(user_name, "User");
        assert_eq!(balance_name, "Balance");
    }
}