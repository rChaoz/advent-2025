use crate::DayInfo;

macro_rules! days {
    ($num:literal) => {
        use seq_macro::seq;

        seq!(N in 1..=$num {
            pub const DAYS: [DayInfo; $num] = [
                #(
                    day_~N::INFO,
                )*
            ];
        });

        seq!(N in 1..=$num {
            mod day_~N;
        });
    };
}

days!(6);
