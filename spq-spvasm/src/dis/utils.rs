use num_traits::Float;

pub fn to_hexadecimal_float<F: Float>(f: F) -> String {
    let (mut mantissa, exponent, sign) = f.integer_decode();
    let mut exponent = exponent as i64;

    // Extract the sign bit.
    let sign = if sign >= 0 { "" } else { "-" };

    // Special case for zero.
    if mantissa == 0 {
        return format!("{}0x0p0", sign);
    }

    // Remove the leading zeros and the implicit one.
    let nleading_zero = mantissa.leading_zeros();
    mantissa <<= nleading_zero + 1;
    exponent -= nleading_zero as i64 + 1;

    // Adjust for the exponent bias.
    exponent += 64;

    let mut mantissa_str = String::new();
    while mantissa != 0 {
        let digit = mantissa >> 60;
        mantissa_str.push_str(&format!("{:x}", digit));
        mantissa <<= 4;
    }

    if mantissa_str.is_empty() {
        format!("{}0x1p{}", sign, exponent)
    } else {
        format!("{}0x1.{}p{}", sign, mantissa_str, exponent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_hex_float() {
        assert_eq!(to_hexadecimal_float(1.0), "0x1p0");
        assert_eq!(to_hexadecimal_float(2.0), "0x1p1");
        assert_eq!(to_hexadecimal_float(0.5), "0x1p-1");
        assert_eq!(to_hexadecimal_float(-0.5), "-0x1p-1");
        assert_eq!(to_hexadecimal_float(-0.75), "-0x1.8p-1");
        assert_eq!(to_hexadecimal_float(-0.875), "-0x1.cp-1");
        assert_eq!(to_hexadecimal_float(0.25), "0x1p-2");
    }
}
