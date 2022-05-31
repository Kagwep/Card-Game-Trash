use std::vec;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;
#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Debug,PartialEq)]
enum status_state{
    InProgress,
    UnderReview,
    Complete,
}
#[near_bindgen]
#[derive(Debug)]
pub struct Project{
    project_id:u8,
    project_name:String,
    start_date:String,
    status:status_state,
    state_percentage:String,
    members:Vec<String>,
    no_of_tasks:Vec<String>,
}
#[near_bindgen]
#[derive(Debug)]
pub struct Task{
    task_id:u8,
    task_name:String,
    start_date:String,
    status:status_state,
    state_percentage:String,
}
#[near_bindgen]
#[derive(Debug)]
pub struct User{
    user_id:u8,
    user_name:String,
    user_rank:String,
    task_assigned:String,

}

#[near_bindgen]
impl  Task {

    fn list_add(&self){

        let mut task_in_progress:Vec<u8> = Vec::new();
        let mut task_under_review:Vec<u8> = Vec::new();
        let mut task_completed: Vec<u8> = Vec::new();
       

    
        
         if self.status == status_state::InProgress {
            task_in_progress.push(self.task_id);
            if  task_under_review.iter().any(|&i| i==self.task_id){
                task_under_review.retain(|&x| x != self.task_id);
            }
            if task_completed.iter().any(|&i| i==self.task_id){
                task_completed.retain(|&x| x != self.task_id);
            }

  
         }
         else if self.status == status_state::UnderReview {

            task_under_review.push(self.task_id);
            if   task_in_progress.iter().any(|&i| i==self.task_id){
                task_in_progress.retain(|&x| x != self.task_id);
            }
            if task_completed.iter().any(|&i| i==self.task_id){
                task_completed.retain(|&x| x != self.task_id);
            }
   
         }
         else{
            task_completed.push(self.task_id);
            if   task_under_review.iter().any(|&i| i==self.task_id){
                task_under_review.retain(|&x| x != self.task_id);
            }
            if task_in_progress.iter().any(|&i| i==self.task_id){
                task_in_progress.retain(|&x| x != self.task_id);
            }
            

         }

    }


}


/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[test]
    fn list_add(){
        }

    // TESTS HERE
}
