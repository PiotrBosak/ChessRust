use yew::html::SendAsMessage;
use yew::prelude::*;
use std::thread;
use std::sync::{mpsc, Mutex};
use std::time::Duration;
use chessTUI::components::App;

fn main() {
    yew::start_app::<App>();
}

