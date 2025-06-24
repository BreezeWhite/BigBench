use astro_float::{BigFloat as AFloat, RoundingMode};
use bigdecimal::BigDecimal;
use clap::{Parser, Subcommand};
use dashu::float::DBig;
use num_bigfloat::{ONE, ZERO};
use rug::Float;
use rust_decimal::prelude::*;

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
    for i in start_idx..end_idx {
        let comm = (i * 8) as f64;
        let a = 4. / (comm + 1.);
        let b = 2. / (comm + 4.);
        let c = 1. / (comm + 5.);
        let d = 1. / (comm + 6.);
        let mut s = a - b - c - d;
        for _ in 0..i {
            s /= 16.;
        }
        pi += s;
    }
    format!("{pi:.1000}")
}

fn rustdecimal_bbp(start_idx: u64, end_idx: u64) -> String {
    let mut pi = dec!(0);
    for i in start_idx..end_idx {
        let comm = dec!(8) * Decimal::from_u64(i).unwrap();
        let a = dec!(4) / (comm + dec!(1));
        let b = dec!(2) / (comm + dec!(4));
        let c = dec!(1) / (comm + dec!(5));
        let d = dec!(1) / (comm + dec!(6));

        let mut s = a - b - c - d;
        for _ in 0..i {
            s /= dec!(16);
        }
        pi += s;
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
    let prec = 1000;
    let mut pi = BigDecimal::from(0).with_prec(prec);
    for i in start_idx..end_idx {
        let comm: BigDecimal = 8 * BigDecimal::from(i).with_prec(prec);
        let a = 4 / (comm.clone() + 1);
        let b = 2 / (comm.clone() + 4);
        let c = 1 / (comm.clone() + 5);
        let d = 1 / (comm + 6);

        let mut s: BigDecimal = a - b - c - d;
        s = s.with_prec(prec);
        for _ in 0..i {
            s /= 16;
        }
        pi += s;
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
    // Be very careful about the precision settings. Sometime it's not
    // that the algorithm is wrong or have bug, but the precision itself
    // is not enough to represent the corret number.
    let prec = 1000;
    let mut pi = Float::with_val(prec, 0.);
    for i in start_idx..end_idx {
        let comm: Float = Float::with_val(prec, i) * 8;
        let a = 4 / (comm.clone() + 1);
        let b = 2 / (comm.clone() + 4);
        let c = 1 / (comm.clone() + 5);
        let d = 1 / (comm + 6);

        let mut s = a - b - c - d;
        for _ in 0..i {
            s /= 16;
        }
        pi += s;
    }

    format!("{pi:.1000}")
}

fn dashu_bbp(start_idx: u64, end_idx: u64) -> String {
    let prec = 1000;
    let mut pi = DBig::from_str("0.0000000000000000")
        .unwrap()
        .with_precision(prec)
        .unwrap();
    for i in start_idx..end_idx {
        let comm: DBig = DBig::try_from(i).unwrap().with_precision(prec).unwrap() * 8;
        let a = 4 / (comm.clone() + 1);
        let b = 2 / (comm.clone() + 4);
        let c = 1 / (comm.clone() + 5);
        let d = 1 / (comm + 6);

        let mut s: DBig = a - b - c - d;
        s = s.with_precision(prec).unwrap();
        for _ in 0..i {
            s /= 16;
        }
        pi += s;
    }

    format!("{}", pi.to_string())
}

fn bigfloat_bbp(start_idx: u64, end_idx: u64) -> String {
    let mut pi = ZERO;
    for i in start_idx..end_idx {
        let comm = ONE.mul(&i.into()).mul(&8.into());
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

        let mut s = a - b - c - d;
        for _ in 0..i {
            s /= ONE.mul(&16.into());
        }
        pi += s;
    }

    format!("{pi:.1000}")
}

fn astro_float_bbp(start_idx: u64, end_idx: u64) -> String {
    let prec = 1000;
    let rm = RoundingMode::None;
    let mut pi = AFloat::from_u8(0, prec);
    for i in start_idx..end_idx {
        let comm = AFloat::from_u64(i, prec).mul_full_prec(&8.into());
        let a = AFloat::from_u8(4, prec).div(&comm.add(&1.into(), prec, rm), prec, rm);
        let b = AFloat::from_u8(2, prec).div(&comm.add(&4.into(), prec, rm), prec, rm);
        let c = AFloat::from_u8(1, prec).div(&comm.add(&5.into(), prec, rm), prec, rm);
        let d = AFloat::from_u8(1, prec).div(&comm.add(&6.into(), prec, rm), prec, rm);

        let mut s = a.sub(&b, prec, rm).sub(&c, prec, rm).sub(&d, prec, rm);

        for _ in 0..i {
            s = s.div(&16.into(), prec, rm);
        }
        pi = pi.add(&s, prec, rm);
    }

    format!("{pi:.1000}")
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
    };

    let pi = func(0, cli.cnt);
    println!("{pi}");
    cmp_pi(pi);
}
