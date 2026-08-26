pub fn byte_slice_len(input: &[u8]) -> usize {
    input.len()
}

pub fn takes_exactly_eight(input: [u8; 8]) -> u8 {
    input[0]
}

pub fn slice_to_array(input: &[u8]) -> [u8; 8] {
    input.try_into().unwrap()
}

pub fn read_u64(input: &[u8]) -> u64 {
    let bytes: [u8; 8] = input.try_into().unwrap();
    let value = u64::from_le_bytes(bytes);
    value
}

pub fn read_u64_be(input: &[u8]) -> u64 {
    let bytes: [u8; 8] = input.try_into().unwrap();
    let value = u64::from_be_bytes(bytes);
    value
}

pub fn try_read_u64(input: &[u8]) -> Result<u64, &'static str> {
    let bytes = input.try_into();
    match bytes {
        Err(_) => Err("expected 8 bytes"),
        Ok(bytes) => Ok(u64::from_le_bytes(bytes)),
    }
}

pub struct ByteView<'a> {
    pub data: &'a [u8],
}

pub fn make_view<'a>(input: &'a [u8]) -> ByteView<'a> {
    ByteView { data: input }
}


pub struct MiniAccount<'a> {
    pub key: &'a [u8; 32],
    pub data: &'a [u8],
    pub is_signer: bool,
}

pub fn make_mini_account<'a>(
    key: &'a [u8; 32],
    data: &'a [u8],
    is_signer: bool,
) -> MiniAccount<'a> {
    MiniAccount { key, data, is_signer }
}


pub fn signer_key<'a>(account: &MiniAccount<'a>) -> Option<&'a [u8; 32]> {
    if account.is_signer {
        Some(account.key)
    } else {
        None
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_bute_slice_len() {
        let input = [1, 2, 3, 4];
        let res = byte_slice_len(&input);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_takes_exactly_eight() {
        let input: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
        let res = takes_exactly_eight(input);
        assert_eq!(res, 1);
    }

    #[test]
    fn test_slice_to_array() {
        let input = [1, 2, 3, 4, 5, 6, 7, 8];
        let res = slice_to_array(&input);
        assert_eq!(res, [1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_read_u64() {
        let input = [1, 0, 0, 0, 0, 0, 0, 0];
        let res = read_u64(&input);
        assert_eq!(res, 1);
    }

    #[test]
    fn test_read_u64_be() {
        let input = [1, 0, 0, 0, 0, 0, 0, 0];
        let res = read_u64_be(&input);
        assert_eq!(res, 72057594037927936);
    }

    #[test]
    #[should_panic]
    fn test_read_u64_too_short() {
        let input = [1, 2, 3, 4];

        read_u64(&input);
    }

    #[test]
    fn test_try_read_u64() {
        let input = [1,0,0,0,0,0,0,0];
        let res = try_read_u64(&input).unwrap();
        assert_eq!(res, 1);
    }


    #[test]
    fn test_try_read_u64_negative() {
        let input = [1,2,3,4];
        let err = try_read_u64(&input).unwrap_err();
        assert_eq!(err, "expected 8 bytes")
    }

    #[test]
fn test_byte_view() {
    let input = [10, 20, 30, 40];

    let view = make_view(&input);

    assert_eq!(view.data, &[10, 20, 30, 40]);
}
}
