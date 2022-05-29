use crate::logic::{Board, File, Position, Rank};
use yew::prelude::*;
use yew_router::prelude::*;


#[function_component(BoardComponent)]

#[function_component(App)]
pub fn app() -> Html {
    let board = Board::new();
    html! {
        <>
            <div> {"Plumba"} </div>
            </>
        // <UserContextProvider>
        //     <BrowserRouter>
        //         <Header />
        //         <Switch<AppRoute> render={Switch::render(switch)} />
        //         <Footer />
        //     </BrowserRouter>
        // </UserContextProvider>
    }
}
