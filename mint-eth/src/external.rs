use near_sdk::{ ext_contract, near};

#[near(serializers = [borsh, json])]
pub struct Request {
    pub payload: Vec<u8>,
    pub path: String,
    pub key_version: u8,
}

#[near(serializers = [borsh, json])]
#[derive(Debug)]
pub struct BigR {
    pub affine_point: String,
}

#[near(serializers = [borsh, json])]
#[derive(Debug)]
pub struct S {
    pub scalar: String,
}

#[near(serializers = [borsh, json])]
#[derive(Debug)]
pub struct Response {
    pub big_r: BigR,
    pub s: S,
    pub recovery_id: u8,
}


#[ext_contract(mpc_contract)]
trait MpcContract {
    fn sign(&self, request: Request) -> Response;
}