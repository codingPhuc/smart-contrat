#![allow(dead_code)]

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, log, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise};
use near_sdk::ext_contract;

// Define the contract structure
pub type JobId = String;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct User {
  pub user_id: AccountId,
  pub name: String,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Client {
  pub client_id: AccountId,
  pub name: String,
  pub number_of_job : u128  , 
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Job {
  pub job_id :  JobId,
  pub desc : String,
 
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
  pub platform_name: AccountId ,
  pub user : LookupMap<AccountId, User>,
  pub client  :  LookupMap<AccountId, Client> , 
  pub taken_job :  UnorderedMap<u128, Job> , 
  pub job_by_id : LookupMap<JobId, Job> , 
  pub job_by_client : UnorderedMap<Client, Vec<Job> >  ,  
  pub total_contract: u128 , 
  pub total_user : u128  ,
  pub total_client : u128 , 

}

// Outsourcing or E-Commerce
pub trait ImplementOutsourcing {
  fn create_user();// lookupmap 
  fn take_job();// unordermap 
  fn create_client();//lookup map 
  fn create_job();//unordermap 
  fn view_job_by_id();//lookup map 
  fn view_all_jobs_per_client(); // unordermap 
  fn view_all_jobs();//unordermap 
  fn payment(); // Payment -> Jobs will remove from list
}

// -> Thứ 3 tuần sau
pub trait ImplementECommerce {
  fn create_shop();
  fn create_product();
  fn view_all_products();
  fn view_all_products_per_shop();
  fn view_product_by_id();
  fn payment(); // Payment -> Product decrement total_supply;
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
  #[init]// this is to show that this function is needed to init the contract 
  pub fn init() -> Self {
    Self {
      platform_name: env::signer_account_id() ,
      user : LookupMap::new(b"user_contract".try_to_vec().unwrap()),
      client  :  LookupMap::new(b"client_contract".try_to_vec().unwrap()) , 
      taken_job :  UnorderedMap::new(b"takent_job".try_to_vec().unwrap()) , 
      job_by_id : LookupMap::new(b"job_id".try_to_vec().unwrap()), 
      job_by_client : UnorderedMap::new(b"job_by_client".try_to_vec().unwrap()) ,  
      total_contract: 0 , 
      total_user : 0   ,
      total_client : 0 , 

    
    }// a new contract is initialize 

  
  }
  //create the owner account 
  pub fn create_user(& mut self  , name :String ) -> User
  {
    let user =  evn::signer_account_id()  ; 
    assert!(!self.shops.contains_key(&user), "user already exists");
    let total_shop = self.total_user + 1; 

    let user = User { owner: env::signer_account_id(), name, desc, total_product: 0 };
    self.insert(evn::signer_account_id() , user  )  ; 
    

  }
}
