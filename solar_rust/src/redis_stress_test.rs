

use redis::AsyncCommands;


#[tokio::main]
async fn main()->redis::RedisResult<()>{
    let connection = "redis://127.0.0.1";
    let client = redis::Client::open(connection)?;
    let mut con = client.get_multiplexed_async_connection().await?;


    for i in 1..=50{
        let ts_name: String = format!("solarKey:{}", i);
        let _: () = redis::cmd("TS.CREATE")
            .arg(&ts_name)
            .arg("RETENTION")
            .arg(60000)
            .arg("DUPLICATE_POLICY")
            .arg("LAST")
            .query_async(&mut con)
            .await?;
            }


    loop{


        let mut args: Vec<String> = Vec::new();

        for i in 1..=50 {
            let key = format!("solarKey:{}", i);

            let value = if i == 1 {
                22.5
            } else if i == 2 {
                60.1
            } else {
                1012.8
            };

            args.push(key);
            args.push("*".to_string()); 
            args.push(value.to_string());
        }

        let _: Vec<u64> = redis::cmd("TS.MADD").arg(&args).query_async(&mut con).await?;
    }
}



