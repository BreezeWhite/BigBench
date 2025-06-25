use astro_float::{BigFloat as AFloat, RoundingMode};
use bigdecimal::BigDecimal;
use clap::{Parser, Subcommand};
use dashu::float::DBig;
use decimal_rs::Decimal as RDecimal;
use fastnum::dec1024;
use num_bigfloat::{ONE, ZERO};
use rug::Float;
use rust_decimal::prelude::*;
use primitive_fixed_point_decimal::{ConstScaleFpdec, fpdec};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    action: Actions,

    cnt: u64,
}

#[derive(Subcommand)]
enum Actions {
    RawBbp,
    RsDecimalLeibniz,
    RsDecimalBbp,
    BigDecimalBbp,
    BigDecimalLeibniz,
    RugBbp,
    RugLeibniz,
    DashuBbp,
    BigFloatBbp,
    AstroFloatBbp,
    FastnumBbp,
    DecimalRsBbp,
    PrimFpdecBbp,
    PrimFpdecLeibniz,
}

const PI_1000: &str = "3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679821\
48086513282306647093844609550582231725359408128481117450284102701938521105559644622948954930381964428810975665933446128475648233\
78678316527120190914564856692346034861045432664821339360726024914127372458700660631558817488152092096282925409171536436789259036\
00113305305488204665213841469519415116094330572703657595919530921861173819326117931051185480744623799627495673518857527248912279\
38183011949129833673362440656643086021394946395224737190702179860943702770539217176293176752384674818467669405132000568127145263\
56082778577134275778960917363717872146844090122495343014654958537105079227968925892354201995611212902196086403441815981362977477\
13099605187072113499999983729780499510597317328160963185950244594553469083026425223082533446850352619311881710100031378387528865\
8753320838142061717766914730359825349042875546873115956286388235378759375195778185778053217122680661300192787661119590921642019";

fn cmp_pi(pi: String) {
    let ref_pi = PI_1000;
    let min_len = pi.len().min(ref_pi.len());
    for (i, (c1, c2)) in pi.chars().zip(ref_pi.chars()).enumerate().take(min_len) {
        if c1 != c2 {
            println!("First difference at position {}: '{}' vs '{}'", i, c1, c2);
            return;
        }
    }
    if pi.len() != ref_pi.len() {
        println!("Strings differ in length at position {}", min_len);
    } else {
        println!("Strings are identical up to length {}", min_len);
    }
}

fn calc_pi_leibniz(start_idx: u64, end_idx: u64) -> String {
    let mut nume = if start_idx % 2 == 0 {
        Decimal::from_i8(1).unwrap()
    } else {
        Decimal::from_i8(-1).unwrap()
    };

    let mut sum = dec!(0);
    for i in start_idx..end_idx {
        let deno = Decimal::from_u64(i).unwrap() * dec!(2) + dec!(1);
        sum += nume / deno;
        nume *= dec!(-1);
    }

    (sum * dec!(4)).to_string()
}

fn raw_bbp(start_idx: u64, end_idx: u64) -> String {
    let mut pi = 0f64;
    let mut comm = 0.;
    let mut deno = 1.;
    for _ in start_idx..end_idx {
        let a = 4. / (comm + 1.);
        let b = 2. / (comm + 4.);
        let c = 1. / (comm + 5.);
        let d = 1. / (comm + 6.);
        pi += (a - b - c - d) / &deno;
        comm += 8.;
        deno *= 16.;
    }
    format!("{pi:.1000}")
}

fn rustdecimal_bbp(start_idx: u64, end_idx: u64) -> String {
    let mut pi = dec!(0);
    let mut comm = dec!(0);
    // let mut deno = dec!(1);
    for i in start_idx..end_idx {
        let a = dec!(4) / (comm + dec!(1));
        let b = dec!(2) / (comm + dec!(4));
        let c = dec!(1) / (comm + dec!(5));
        let d = dec!(1) / (comm + dec!(6));

        let mut s = a - b - c - d;
        for _ in 0..i {
            s /= dec!(16);
        }
        pi += s;
        comm += dec!(8);
        // deno *= dec!(16); // Doing this way will cause overflow
    }
    format!("{pi:.1000}")
}

fn bigdecimal_leibniz(start_idx: u64, end_idx: u64) -> String {
    let mut nume = if start_idx % 2 == 0 {
        BigDecimal::from(1)
    } else {
        BigDecimal::from(-1)
    };
    let mut sum = BigDecimal::from(0);
    for i in start_idx..end_idx {
        let deno = BigDecimal::from(i) * 2 + 1;
        sum += nume.clone() / deno;
        nume *= -1;
    }

    let pi = sum * 4;
    format!("{pi:.1000}")
}

fn bigdecimal_bbp(start_idx: u64, end_idx: u64) -> String {
    // Precision is set during compile time.
    // export RUST_BIGDECIMAL_DEFAULT_PRECISION=1000
    let mut pi = BigDecimal::from(0);
    let mut deno = BigDecimal::from(1);
    let mut comm = BigDecimal::from(0);

    for _ in start_idx..end_idx {
        let a = 4 / (&comm + 1);
        let b = 2 / (&comm + 4);
        let c = 1 / (&comm + 5);
        let d = 1 / (&comm + 6);

        pi += (a - b - c - d) / &deno;
        comm += 8;
        deno *= 16;
    }

    format!("{pi:.1000}")
}

fn rug_leibniz(start_idx: u64, end_idx: u64) -> String {
    let prec = 1000;
    let mut nume = if start_idx % 2 == 0 {
        Float::with_val(prec, 1)
    } else {
        Float::with_val(prec, -1)
    };
    let mut sum = Float::with_val(prec, 0);
    for i in start_idx..end_idx {
        let deno = Float::with_val(prec, i) * 2 + 1;
        sum += nume.clone() / deno;
        nume *= -1;
    }

    let pi = sum * 4;
    format!("{pi:.1000}")
}

fn rug_bbp(start_idx: u64, end_idx: u64) -> String {
    // The precision in rug refers to binary digits.
    // Here what we want is decimal digits, hence further transformation is required.
    let prec = 1000; // Decimal
    let prec_bin = (prec as f64 * 3.3219281).round() as u32 + 10; // Binary precision

    let mut pi = Float::with_val(prec_bin, 0.);
    let mut deno: Float = Float::with_val(prec_bin, 1);
    let mut comm: Float = Float::with_val(prec_bin, 0);

    for _ in start_idx..end_idx {
        let a = 4 / (comm.clone() + 1);
        let b = 2 / (comm.clone() + 4);
        let c = 1 / (comm.clone() + 5);
        let d = 1 / (comm.clone() + 6);

        pi += (a - b - c - d) / &deno;
        comm += 8;
        deno *= 16;
    }

    format!("{pi:.1000}")
}

fn dashu_bbp(start_idx: u64, end_idx: u64) -> String {
    let prec = 1000;
    let mut pi = DBig::from_str("0.0000000000000000")
        .unwrap()
        .with_precision(prec)
        .unwrap();
    let mut deno = DBig::from(1).with_precision(prec).unwrap();
    let mut comm = DBig::from(0).with_precision(prec).unwrap();
    for _ in start_idx..end_idx {
        let a = 4 / (&comm + 1);
        let b = 2 / (&comm + 4);
        let c = 1 / (&comm + 5);
        let d = 1 / (&comm + 6);

        pi += (a - b - c - d) / &deno;
        comm += 8;
        deno *= 16;
    }

    format!("{}", pi.to_string())
}

fn bigfloat_bbp(start_idx: u64, end_idx: u64) -> String {
    let mut pi = ZERO;
    let mut comm = ZERO;
    let mut deno = ONE;
    for _ in start_idx..end_idx {
        let a = ONE.mul(&4.into()) / (comm.add(&1.into()));
        let b = ONE.mul(&2.into()) / (comm.add(&4.into()));
        let c = ONE / (comm.add(&5.into()));
        let d = ONE / (comm.add(&6.into()));

        // The following code will lead to wrong results
        // let comm = BigFloat::from_u64(i).mul(&8.into());
        // let a = BigFloat::from_u8(4) / (comm.add(&1.into()));
        // let b = BigFloat::from_u8(2) / (comm.add(&4.into()));
        // let c = BigFloat::from_u8(1) / (comm.add(&5.into()));
        // let d = BigFloat::from_u8(1) / (comm.add(&6.into()));

        pi += (a - b - c - d) / deno;
        comm += &8.into();
        deno *= &16.into();
    }

    format!("{pi:.1000}")
}

fn astro_float_bbp(start_idx: u64, end_idx: u64) -> String {
    let prec = 1000; // Decimal
    let prec_bin = (prec as f64 * 3.3219281).round() as usize + 10; // Binary precision

    let rm = RoundingMode::None;

    let mut pi = AFloat::from_u8(0, prec_bin);
    let mut comm = AFloat::from_u8(0, prec_bin);
    let mut deno = AFloat::from_u8(1, prec_bin);
    for _ in start_idx..end_idx {
        let a = AFloat::from_u8(4, prec_bin).div(&comm.add(&1.into(), prec_bin, rm), prec_bin, rm);
        let b = AFloat::from_u8(2, prec_bin).div(&comm.add(&4.into(), prec_bin, rm), prec_bin, rm);
        let c = AFloat::from_u8(1, prec_bin).div(&comm.add(&5.into(), prec_bin, rm), prec_bin, rm);
        let d = AFloat::from_u8(1, prec_bin).div(&comm.add(&6.into(), prec_bin, rm), prec_bin, rm);

        let s = a
            .sub(&b, prec_bin, rm)
            .sub(&c, prec_bin, rm)
            .sub(&d, prec_bin, rm)
            .div(&deno, prec_bin, rm);
        pi = pi.add(&s, prec_bin, rm);
        comm = comm.add(&8.into(), prec_bin, rm);
        deno = deno.mul(&16.into(), prec_bin, rm);
    }

    format!("{pi:.1000}")
}

fn fastnum_bbp(start_idx: u64, end_idx: u64) -> String {
    let mut pi = dec1024!(0);
    let mut deno = dec1024!(1);
    let mut comm = dec1024!(0);

    let four = dec1024!(4);
    let two = dec1024!(2);
    let one = dec1024!(1);
    for _ in start_idx..end_idx {
        let a = four / (comm + 1);
        let b = two / (comm + 4);
        let c = one / (comm + 5);
        let d = one / (comm + 6);

        pi += (a - b - c - d) / deno;
        comm += 8;
        deno *= 16;
    }
    format!("{pi:.1000}")
}

fn decimal_rs_bbp(start_idx: u64, end_idx: u64) -> String {
    let mut pi = RDecimal::from(0);
    // let mut deno = RDecimal::from(1);
    let mut comm = RDecimal::from(0);
    for i in start_idx..end_idx {
        let a = 4 / (comm + 1);
        let b = 2 / (comm + 4);
        let c = 1 / (comm + 5);
        let d = 1 / (comm + 6);

        let mut s = a - b - c - d;
        for _ in 0..i {
            s /= 16;
        }
        pi += s;
        comm += 8;
        // deno *= 16; // This will cause overflow
    }
    format!("{}", pi.to_string())
}

type PrimFpdec = ConstScaleFpdec<i128, 35>;
fn prim_fpdec_bbp(start_idx: u64, end_idx: u64) -> String {
    let mut pi = PrimFpdec::ZERO;
    let mut comm = PrimFpdec::ZERO;

    for i in start_idx..end_idx {
        let n4: PrimFpdec = fpdec!(4);
        let n2: PrimFpdec = fpdec!(2);
        let n1: PrimFpdec = fpdec!(1);

        let a = n4.checked_div(comm + fpdec!(1)).unwrap();
        let b = n2.checked_div(comm + fpdec!(4)).unwrap();
        let c = n1.checked_div(comm + fpdec!(5)).unwrap();
        let d = n1.checked_div(comm + fpdec!(6)).unwrap();

        let mut s = a - b - c - d;
        for _ in 0..i {
            s = s.checked_div_int(16).unwrap();
        }
        pi += s;
        comm += fpdec!(8);
    }
    format!("{pi}")
}

fn prim_fpdec_leibniz(start_idx: u64, end_idx: u64) -> String {
    let mut nume: PrimFpdec = if start_idx % 2 == 0 {
        fpdec!(1)
    } else {
        fpdec!(-1)
    };
    let mut sum: PrimFpdec = fpdec!(0);
    for i in start_idx..end_idx {
        let deno = i * 2 + 1;
        sum += nume.checked_div_int(deno as i128).unwrap();
        nume = -nume;
    }

    let pi = sum.checked_mul_int(4).unwrap();
    format!("{pi}")
}

fn main() {
    let cli = Cli::parse();

    let func = match &cli.action {
        Actions::RawBbp => raw_bbp,
        Actions::RsDecimalLeibniz => calc_pi_leibniz,
        Actions::RsDecimalBbp => rustdecimal_bbp,
        Actions::BigDecimalBbp => bigdecimal_bbp,
        Actions::BigDecimalLeibniz => bigdecimal_leibniz,
        Actions::RugBbp => rug_bbp,
        Actions::RugLeibniz => rug_leibniz,
        Actions::DashuBbp => dashu_bbp,
        Actions::BigFloatBbp => bigfloat_bbp,
        Actions::AstroFloatBbp => astro_float_bbp,
        Actions::FastnumBbp => fastnum_bbp,
        Actions::DecimalRsBbp => decimal_rs_bbp,
        Actions::PrimFpdecBbp => prim_fpdec_bbp,
        Actions::PrimFpdecLeibniz => prim_fpdec_leibniz,
    };

    let pi = func(0, cli.cnt);
    println!("{pi}");
    cmp_pi(pi);
}
