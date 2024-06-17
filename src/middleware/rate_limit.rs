use actix_governor::{
  governor::{clock::QuantaInstant, middleware::NoOpMiddleware},
  Governor, GovernorConfigBuilder, PeerIpKeyExtractor,
};

pub fn get_rate_limit_config() -> Governor<PeerIpKeyExtractor, NoOpMiddleware<QuantaInstant>> {
  let brust_size: u32 = 10;
  let per_second: u64 = 2;

  let conf = GovernorConfigBuilder::default()
    .per_second(per_second)
    .burst_size(brust_size)
    .finish()
    .unwrap();

  Governor::new(&conf)
}
