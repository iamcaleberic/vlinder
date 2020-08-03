use warp::Filter;
use pretty_env_logger;
use std::net::{IpAddr, Ipv4Addr};
#[macro_use] extern crate log;

// mod filters;

static HOST:IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)); 
static PORT:u16 = 3030;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let root =  warp::path::end().map(|| "Vlinder");

    let v1 = warp::path("v1");

    let v2 = warp::path("v2");

    let fly_v1 = v1.and(warp::path("fly")
        .map(|| 
            format!("Cant Fly")
        ));

    let fly_v2 = v2.and(warp::path("fly")
        .map(||
             format!("Flying!")
        ));

    let healthz =  warp::path("healthz")
        .map(|| {
            let reply: &str = "OK";
            warp::reply::json(&reply)
        });
    
    let routes = warp::get().and(root.or(fly_v1).or(fly_v2).or(healthz));

    info!("Vlinder is running");

    warp::serve(routes)
        .run((HOST, PORT))
        .await;
}