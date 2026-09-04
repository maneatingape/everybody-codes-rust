#[macro_export]
macro_rules! solutions {
    ($callback:ident) => {
        $callback! {
            event2024
            quest01 quest02 quest03 quest04 quest05 quest06 quest07 quest08 quest09 quest10
            quest11 quest12 quest13 quest14 quest15 quest16 quest17 quest18 quest19 quest20,

            event2025
            quest01 quest02 quest03 quest04 quest05 quest06 quest07 quest08 quest09 quest10
            quest11 quest12 quest13 quest14 quest15 quest16 quest17 quest18 quest19 quest20,

            story01 quest01 quest02 quest03,
            story02 quest01 quest02 quest03,
            story03 quest01 quest02 quest03,
            story04 quest01 quest02 quest03
        }
    };
}

#[macro_export]
macro_rules! library {
    ($($event:ident $($quest:ident)*),*) => {
        $(pub mod $event {
            $(pub mod $quest;)*
        })*
    }
}

library!(util ansi grid heap integer iter math parse point);

solutions!(library);
