use dotenvy::dotenv;
use redis::{Client, Commands, Connection, FromRedisValue, RedisResult, ToRedisArgs};
use std::env;

//TODO:: implement better method to keep a connection alive without the use of constructing a new object

pub struct Redis {}

impl Redis {
    pub fn connect() -> RedisResult<Connection> {
        dotenv().ok();

        let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
        Client::open(redis_url).unwrap().get_connection()
    }

    pub fn set_data<K: ToRedisArgs, D: ToRedisArgs + FromRedisValue>(
        conn: &mut Connection,
        key: K,
        data: D,
    ) -> RedisResult<D> {

        conn.set::<K, D, D>(key, data)
    }

    pub fn get_data<K: ToRedisArgs, D: FromRedisValue>(
        conn: &mut Connection,
        key: K,
    ) -> RedisResult<D> {
        conn.get::<K, D>(key)
    }

    pub fn get_keys(conn: &mut Connection, pattern: &str) -> RedisResult<Vec<String>> {
        Redis::keys::<String>(conn, format!("*{}*", pattern))
    }

    pub fn has_data<K: ToRedisArgs>(conn: &mut Connection, key: K) -> RedisResult<bool> {
        conn.exists(key)
    }

    pub fn keys<K: ToRedisArgs>(conn: &mut Connection, partial: K) -> RedisResult<Vec<String>> {
        conn.keys(partial)
    }

    pub fn delete<K: ToRedisArgs>(conn: &mut Connection, key: K) -> RedisResult<()> {
        conn.del::<K, ()>(key)
    }
}
