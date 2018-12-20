use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use url::Url;

use fetch::{fetch_all_urls, url_status, UrlState};


pub struct Crawler {
    to_visit: Arc<Mutex<Vec<String>>>,
    active_count: Arc<Mutex<i32>>,
    url_states: Receiver<UrlState>
}

