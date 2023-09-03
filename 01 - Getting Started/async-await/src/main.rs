use futures::executor::block_on;

// async fn hello_world(){
//     println!("hello, world");
// }

// fn main() {
//     let future = hello_world();

//     block_on(future);
// }

async fn learn_and_sing(){
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main(){
    let f1 = learn_and_sing();
    let f2 = dance();

    futures::join!(f1, f2);
}

fn main(){
    block_on(async_main());
}
