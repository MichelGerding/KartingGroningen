use dotenvy::dotenv;
use std::env;
use redis::{Client, Commands, Connection, FromRedisValue, RedisResult, ToRedisArgs};

//TODO:: implement better method to keep a connection alive without the use of constructing a new object

pub struct Redis {}

impl Redis {

    pub fn connect() -> Connection {
        dotenv().ok();

        let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
        Client::open(redis_url).unwrap().get_connection().unwrap()
    }

    pub fn set_data<K: ToRedisArgs, D: ToRedisArgs + FromRedisValue >(conn: &mut Connection, key: K, data: D) -> D{
        conn.set::<K, D, D>(key, data).expect("TODO: panic message")
    }

    pub fn get_data<K: ToRedisArgs, D: FromRedisValue>(conn: &mut Connection, key: K) -> RedisResult<D> {
        conn.get::<K, D>(key)
    }

    pub fn invalidate<K: ToRedisArgs + FromRedisValue>(conn: &mut Connection, key: K) -> RedisResult<K>{
        conn.expire::<K, K>(key, 0)
    }

    pub fn has_data<K: ToRedisArgs>(conn: &mut Connection, key: K) -> RedisResult<bool> {
        conn.exists(key)
    }

    pub fn keys<K:ToRedisArgs>(conn: &mut Connection, partial: K) -> RedisResult<Vec<String>> {
        conn.keys(partial)
    }
}