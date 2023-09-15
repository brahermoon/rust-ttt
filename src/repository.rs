use serde::{Deserialize, Serialize};

pub(crate) trait  Repository{
     fn fetch_data<T:Deserialize + Serialize>(filename:&String){
        let data: T = serde_json::from_str(filename).expect("JSON was not well-formatted");
    }
}
pub(crate) struct PlayerRepository{

}

impl PlayerRepository {
    
}