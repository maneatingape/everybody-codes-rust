macro_rules! test {
    ($event:tt $($quest:tt),*) => {
        pub mod $event {$(pub mod $quest;)*}
    }
}

test!(event2024
    quest01, quest02, quest03, quest04, quest05, quest06, quest07, quest08, quest09, quest10,
    quest11, quest12, quest13, quest14, quest15, quest16, quest17, quest18, quest19, quest20
);

test!(story01
    quest01, quest02, quest03
);
