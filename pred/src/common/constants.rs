

pub mod config{
   pub const DEFAULT_MODEL_KEY :&str  = "default_model"; 
   pub const MODEL_PARAMETERS_KEY :&str  = "model_parameters"; 
   pub const PREDICT_FLAG_KEY :&str  = "predict_flag";
   pub const TIMER_INTERVAL_KEY :&str  = "timer_interval";
   pub const CURRENT_VERSION_KEY :&str  = "current_version";
   pub const FIVE_MIN_TIMER_INTERVAL :u64  = 5*60;
   pub const ONE_HOUR_IMER_INTERVAL :u64  = 60*60;
   pub const F_FLAG :&str  = "0";
   pub const T_FLAG :&str  = "1";


   pub const _KEY :&str  = "_key";
   pub const _VALUE:&str  = "_value"; 
}

pub mod duration{
   pub const NANOS_PER_MICRO: u64 = 1_000;
   pub const NANOS_PER_MILLI: u64 = 1_000_000;
   pub const NANOS_PER_SEC: u64 = 1_000_000_000;
   pub const NANOS_PER_MINUTE: u64 = 60 * NANOS_PER_SEC;
   pub const NANOS_PER_HOUR: u64 = 60 * NANOS_PER_MINUTE;
   pub const NANOS_PER_DAY: u64 = 24 * NANOS_PER_HOUR;
}
//定义各种canister_id 集中放置
pub mod canister_id{
   pub const IC_BACKEND_CANISTER_ID: &str = "eov5t-niaaa-aaaah-arepa-cai";
   pub const LOCAL_BACKEND_CANISTER_ID: &str = "uxrrr-q7777-77774-qaaaq-cai";
}

