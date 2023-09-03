use st::thread;

// Typical threaded way
fn get_two_sites(){
    // Spawn two threads to do work.
    let thread_one = thread::spawn(|| download("https://www.foo.com"));
    let thread_two = thread::spawn(|| download("https://www.bar.com"));

    // Wait for both threads to complete.
    thread_one.join().expect("thread one panicked");
    thread_two.join().expect("thread two panicked");
}

// Asynchronous way
async fn get_two_site_async(){
    // Create two different "futures" which, when run to completion will asynchronously download the webpages.
    let future_one = download_async("https://www.foo.com");
    let future_two = download_async("https://www.bar.com");

    join!(future_one, future_two);
}