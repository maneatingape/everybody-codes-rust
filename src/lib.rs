macro_rules! library {
    ($year:tt $($day:tt),*) => {
        pub mod $year {
            $(pub mod $day;)*
        }
    }
}

library!(event2024
    quest01, quest02, quest03, quest04, quest05, quest06, quest07, quest08, quest09, quest10,
    quest11, quest12, quest13, quest14, quest15, quest16, quest17, quest18, quest19, quest20
);

library!(event2025
    quest01, quest02, quest03, quest04, quest05, quest06, quest07, quest08, quest09
);

library!(story01
    quest01, quest02, quest03
);

library!(story02
    quest01, quest02, quest03
);

library!(util
    ansi, grid, heap, integer, iter, math, parse, point
);
