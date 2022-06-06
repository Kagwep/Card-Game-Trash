use std::vec;
use rand::thread_rng;
use rand::seq::SliceRandom;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
struct  Deck{
    cards: Vec<String>,

}

#[near_bindgen]
impl Deck{
    fn cardDeck(&mut self, rank:Vec<String>, suit:Vec<String>) -> Vec<String>{


            for s in 0..4{
                for r in 0..13{
                
                    let mut news = String::from(&rank[r]);
                    news.push_str(&suit[s]);

                    self.cards.push(news);
                   

                }
            }

        let mut cards_D = &self.cards;
        println!("{:?}",self.cards);
        println!("{:?}",self.cards.len());
        
        return cards_D.to_vec();

    }
}

#[near_bindgen]
struct Players{
    Computer:Vec<String>,
    player:Vec<String>,
    remaining_card_deck:Vec<String>,
    cardvariant1:Vec<String>,cardvariant2:Vec<String>,cardvariant3:Vec<String>,cardvariant4:Vec<String>,
    cardvariant5:Vec<String>,cardvariant6:Vec<String>,cardvariant7:Vec<String>,cardvariant8:Vec<String>,
    cardvariant9:Vec<String>,cardvariant10:Vec<String>,cardvariant11:Vec<String>,cardvariant12:Vec<String>,
    cardvariant13:Vec<String>,
}

#[near_bindgen]
impl  Players {
    fn get_card_vec(&self, player:&Vec<String>) -> Vec<u8>{
        let mut Decision_vec:Vec<u8> = Vec::new();
        for c in player{
            if self.cardvariant13.contains(&c){
                Decision_vec.push(13);

            }
            else if self.cardvariant12.contains(&c){
                Decision_vec.push(12);

            }
            else if self.cardvariant11.contains(&c){
                Decision_vec.push(11);

            }
            else if self.cardvariant10.contains(&c){
                Decision_vec.push(10);

            }
            else if self.cardvariant9.contains(&c){
                Decision_vec.push(9);

            }
            else if self.cardvariant8.contains(&c){
                Decision_vec.push(8);

            }
            else if self.cardvariant7.contains(&c){
                Decision_vec.push(7);

            }
            else if self.cardvariant6.contains(&c){
                Decision_vec.push(6);

            }
            else if self.cardvariant5.contains(&c){
                Decision_vec.push(5);

            }
            else if self.cardvariant4.contains(&c){
                Decision_vec.push(4);

            }
            else if self.cardvariant3.contains(&c){
                Decision_vec.push(3);

            }
            else if self.cardvariant2.contains(&c){
                Decision_vec.push(2);

            }
            else if self.cardvariant1.contains(&c){
                Decision_vec.push(1);

            }
        }
        return Decision_vec;
    }


    fn get_card_values(&self, card_picked:&String) -> usize{
        let mut value_of_card:usize = 0;
            if self.cardvariant13.contains(card_picked){
                value_of_card = 13;

            }
            else if self.cardvariant12.contains(card_picked){
                value_of_card = 12;

            }
            else if self.cardvariant11.contains(card_picked){
                value_of_card = 11;

            }
            else if self.cardvariant10.contains(card_picked){
                value_of_card = 10;

            }
            else if self.cardvariant9.contains(card_picked){
                value_of_card = 9;

            }
            else if self.cardvariant8.contains(card_picked){
                value_of_card = 8;

            }
            else if self.cardvariant7.contains(card_picked){
                value_of_card = 7;

            }
            else if self.cardvariant6.contains(card_picked){
                value_of_card = 6;

            }
            else if self.cardvariant5.contains(card_picked){
                value_of_card = 5;

            }
            else if self.cardvariant4.contains(card_picked){
                value_of_card = 4;
            }
            else if self.cardvariant3.contains(card_picked){
                value_of_card = 3;

            }
            else if self.cardvariant2.contains(card_picked){
                value_of_card = 2;

            }
            else if self.cardvariant1.contains(card_picked){
                value_of_card = 1;

            }
            else {
                value_of_card = 0;
            }
        
        return value_of_card;
    }
    
    fn compute(&self,mut card_value:usize,mut done_card:Vec<usize>,mut card:String,mut com_hidden_cards:Vec<String>) -> Vec<String>{

        while card_value <= 10{


            if done_card.contains(&card_value){
                card_value = 15;
                println!("You aready filled that spot")
            }

            else{

                
            let card_replaced = &self.player[card_value-1];

            println!("current card:{} ",card_replaced);

            std::mem::replace(&mut com_hidden_cards[card_value-1], card.to_string());


            println!("updated cards: {:?}",com_hidden_cards);

            done_card.push(card_value);
            card_value = self.get_card_values(card_replaced);
            card = card_replaced.to_string();


            }


        } 


        println!("  current card is {},  turn is over .....",card);

        return com_hidden_cards;

        

    }
    
    fn play(&mut self, entry:String, cond:String) -> Vec<String>{

        let mut hidden_cards:Vec<String> = vec!["X".to_string(),"X".to_string(),"X".to_string(),"X".to_string(),"X".to_string(),"X".to_string(),"X".to_string(),"X".to_string(),"X".to_string(),"X".to_string()];
        let mut com_hidden_cards:Vec<String> =vec!["Y".to_string(),"Y".to_string(),"Y".to_string(),"Y".to_string(),"Y".to_string(),"Y".to_string(),"Y".to_string(),"Y".to_string(),"Y".to_string(),"Y".to_string()];
        let mut done_card:Vec<usize> = Vec::new();
        let mut  C_done_card:Vec<usize> = Vec::new();
        let mut r_cards = self.remaining_card_deck.clone();

        println!("Initial set player: {:?}",hidden_cards);
        println!("Initial set Computer: {:?}",com_hidden_cards);
        
        let counter = 1;

     loop{


        let a = self.get_card_vec(&hidden_cards);
        let b = self.get_card_vec(&com_hidden_cards);

        
        println!("sorted:{:?}",a);
        println!("sorted:{:?}",b);
        
 

        if a.len() == 10{

             println!(" You are the winner!!!: \n {:?}",hidden_cards);
             return hidden_cards;
        }

        if b.len() == 10{

            println!(" I am  the winner!!!: \n {:?}",com_hidden_cards);
            return com_hidden_cards;
       }


        
     


        let mut card =&r_cards.clone()[0];
   
        r_cards.remove(0);

        let mut card_c = &r_cards.clone()[0];

        r_cards.remove(0);
  
 

            
        
            if entry == cond {
                

                println!("You picked: {}",card);
    
                let mut card_value = self.get_card_values(&card);
    
                println!("card value: {}",card_value);
    
                
    
                hidden_cards = self.compute(card_value,done_card.clone(),card.clone().to_string(), hidden_cards.clone());
    

                println!("********************My turn***********************");
           
                println!("My cards: {:?}",  com_hidden_cards);

                
    
                println!("Picked card:{}",card_c);
    
                let mut c_card_value = self.get_card_values(&card_c);
    
                
                
                com_hidden_cards = self.compute(c_card_value,C_done_card.clone(),card_c.clone().to_string(), com_hidden_cards.clone());
    
            }
            else{
    
                println!("Invalid input .... press 1 to pick card .");
    
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
    fn test_cardDeck(){

        let mut rank = vec!["K".to_string(), "Q".to_string(), "J".to_string(), "10".to_string(), "9".to_string(), "8".to_string(), "7".to_string(), "6".to_string(), "5".to_string(), "4".to_string(), "3".to_string(), "2".to_string(), "A".to_string()];
        let mut suit=vec!["D".to_string(),"H".to_string(),"S".to_string(),"C".to_string()];

        let mut card = Deck {
            
            cards:Vec::new(),
        
        };

        assert_eq!(card.cardDeck(rank, suit).len(), 52);
        
    }

    
    #[test]

    fn test_play(){
        let mut rank = vec!["K".to_string(), "Q".to_string(), "J".to_string(), "10".to_string(), "9".to_string(), "8".to_string(), "7".to_string(), "6".to_string(), "5".to_string(), "4".to_string(), "3".to_string(), "2".to_string(), "A".to_string()];
        let mut suit=vec!["D".to_string(),"H".to_string(),"S".to_string(),"C".to_string()];


        let mut card = Deck {
            cards:Vec::new(),
        
        };

        let mut cards_to_deal_unshuffled= card.cardDeck(rank,suit);
        let mut group_cards_vec = cards_to_deal_unshuffled.clone();
        let mut cards_to_deal = cards_to_deal_unshuffled;
        cards_to_deal.shuffle( &mut thread_rng());
        
        let mut sum:u8 = 0;
        let mut comp:Vec<String> = Vec::new();
        let mut play:Vec<String> = Vec::new();
        let mut upc = String::from("");
        let mut rem:Vec<String> = Vec::new();
        let mut count:u8 = 0;
        for i in &cards_to_deal{
            if sum<20{
        
                if count % 2 == 0 {
                    play.push(i.to_string());
          
                }
                else{
                    comp.push(i.to_string());
        
                }
            }
        
            else  {
                let necs = &cards_to_deal[sum as usize];
                rem.push(necs.to_string());
            }
           
            count +=1;
            sum +=1;
        
        }
        
        let mut set1:Vec<String> = Vec::new();
        let mut set2:Vec<String> = Vec::new();
        let mut set3:Vec<String> = Vec::new();
        let mut set4:Vec<String> = Vec::new();
        let mut set5:Vec<String> = Vec::new();
        let mut set6:Vec<String> = Vec::new();
        let mut set7:Vec<String> = Vec::new();
        let mut set8:Vec<String> = Vec::new();
        let mut set9:Vec<String> = Vec::new();
        let mut set10:Vec<String> = Vec::new();
        let mut set11:Vec<String> = Vec::new();
        let mut set12:Vec<String> = Vec::new();
        let mut set13:Vec<String> = Vec::new();
        
        let mut Vec_con = vec![set1,set2,set3,set4,set5,set6,set7,set8,set9,set10,set11,set12,set13];
        
        
        
        let mut count_1:usize = 0;
        
        let mut vec_of_all_sets :Vec<Vec<String>>= Vec::new();
        
        
        for mut val in Vec_con {
        
            let s = &group_cards_vec[count_1+0];
            let g = &group_cards_vec[count_1+13];
            let h = &group_cards_vec[count_1+26];
            let i = &group_cards_vec[count_1+39];
        
        
             val.push(s.to_string());
             val.push(g.to_string());
             val.push(h.to_string());
             val.push(i.to_string());
        
            vec_of_all_sets.push(val);
            count_1 += 1;
        
        }
        
        let Kings = &vec_of_all_sets[0];
        let Queens = &vec_of_all_sets[1];
        let Jacks = &vec_of_all_sets[2];
        let Ten = &vec_of_all_sets[3];
        let Nine = &vec_of_all_sets[4];
        let Eight = &vec_of_all_sets[5];
        let Seven = &vec_of_all_sets[6];
        let Six = &vec_of_all_sets[7];
        let Five = &vec_of_all_sets[8];
        let Fours = &vec_of_all_sets[9];
        let Thress = &vec_of_all_sets[10];
        let Twos = &vec_of_all_sets[11];
        let Aces = &vec_of_all_sets[12]; 
        
        
        
        
        let mut ply = Players{
            Computer:comp,
            player:play,
            remaining_card_deck:rem,
            cardvariant13:Kings.to_vec(),
            cardvariant12:Queens.to_vec(),
            cardvariant11:Jacks.to_vec(),
            cardvariant10:Ten.to_vec(),
            cardvariant9:Nine.to_vec(),
            cardvariant8:Eight.to_vec(),
            cardvariant7:Seven.to_vec(),
            cardvariant6:Six.to_vec(),
            cardvariant5:Five.to_vec(),
            cardvariant4:Fours.to_vec(),
            cardvariant3:Thress.to_vec(),
            cardvariant2:Twos.to_vec(),
            cardvariant1:Aces.to_vec(),
        
        
        
        };
        
        let cond:String = String::from("1") ;
        let entry:String = String::from("1") ;

        assert_eq!(ply.play(entry,cond).len(), 10);
        


    }


    #[test]
    fn  test_get_card_vec(){

        let player1 = vec!["10H".to_string(), "AD".to_string(), "5D".to_string(), "6H".to_string(), "4S".to_string(), "2S".to_string(), "QC".to_string(), "KD".to_string(), "4H".to_string(), "AH".to_string()];
        
        let mut rank = vec!["K".to_string(), "Q".to_string(), "J".to_string(), "10".to_string(), "9".to_string(), "8".to_string(), "7".to_string(), "6".to_string(), "5".to_string(), "4".to_string(), "3".to_string(), "2".to_string(), "A".to_string()];
        let mut suit=vec!["D".to_string(),"H".to_string(),"S".to_string(),"C".to_string()];


        let mut card = Deck {
            cards:Vec::new(),
        
        };

        let mut cards_to_deal_unshuffled= card.cardDeck(rank,suit);
        let mut group_cards_vec = cards_to_deal_unshuffled.clone();
        let mut cards_to_deal = cards_to_deal_unshuffled;
        cards_to_deal.shuffle( &mut thread_rng());
        
        let mut sum:u8 = 0;
        let mut comp:Vec<String> = Vec::new();
        let mut play:Vec<String> = Vec::new();
        let mut upc = String::from("");
        let mut rem:Vec<String> = Vec::new();
        let mut count:u8 = 0;
        for i in &cards_to_deal{
            if sum<20{
        
                if count % 2 == 0 {
                    play.push(i.to_string());
          
                }
                else{
                    comp.push(i.to_string());
        
                }
            }
        
            else  {
                let necs = &cards_to_deal[sum as usize];
                rem.push(necs.to_string());
            }
           
            count +=1;
            sum +=1;
        
        }
        
        let mut set1:Vec<String> = Vec::new();
        let mut set2:Vec<String> = Vec::new();
        let mut set3:Vec<String> = Vec::new();
        let mut set4:Vec<String> = Vec::new();
        let mut set5:Vec<String> = Vec::new();
        let mut set6:Vec<String> = Vec::new();
        let mut set7:Vec<String> = Vec::new();
        let mut set8:Vec<String> = Vec::new();
        let mut set9:Vec<String> = Vec::new();
        let mut set10:Vec<String> = Vec::new();
        let mut set11:Vec<String> = Vec::new();
        let mut set12:Vec<String> = Vec::new();
        let mut set13:Vec<String> = Vec::new();
        
        let mut Vec_con = vec![set1,set2,set3,set4,set5,set6,set7,set8,set9,set10,set11,set12,set13];
        
        
        
        let mut count_1:usize = 0;
        
        let mut vec_of_all_sets :Vec<Vec<String>>= Vec::new();
        
        
        for mut val in Vec_con {
        
            let s = &group_cards_vec[count_1+0];
            let g = &group_cards_vec[count_1+13];
            let h = &group_cards_vec[count_1+26];
            let i = &group_cards_vec[count_1+39];
        
        
             val.push(s.to_string());
             val.push(g.to_string());
             val.push(h.to_string());
             val.push(i.to_string());
        
            vec_of_all_sets.push(val);
            count_1 += 1;
        
        }
        
        let Kings = &vec_of_all_sets[0];
        let Queens = &vec_of_all_sets[1];
        let Jacks = &vec_of_all_sets[2];
        let Ten = &vec_of_all_sets[3];
        let Nine = &vec_of_all_sets[4];
        let Eight = &vec_of_all_sets[5];
        let Seven = &vec_of_all_sets[6];
        let Six = &vec_of_all_sets[7];
        let Five = &vec_of_all_sets[8];
        let Fours = &vec_of_all_sets[9];
        let Thress = &vec_of_all_sets[10];
        let Twos = &vec_of_all_sets[11];
        let Aces = &vec_of_all_sets[12]; 
        
        
        
        
        let mut play = Players{
            Computer:comp,
            player:play,
            remaining_card_deck:rem,
            cardvariant13:Kings.to_vec(),
            cardvariant12:Queens.to_vec(),
            cardvariant11:Jacks.to_vec(),
            cardvariant10:Ten.to_vec(),
            cardvariant9:Nine.to_vec(),
            cardvariant8:Eight.to_vec(),
            cardvariant7:Seven.to_vec(),
            cardvariant6:Six.to_vec(),
            cardvariant5:Five.to_vec(),
            cardvariant4:Fours.to_vec(),
            cardvariant3:Thress.to_vec(),
            cardvariant2:Twos.to_vec(),
            cardvariant1:Aces.to_vec(),
        
        
        
        };
        assert_eq!(play.get_card_vec(&player1).len(), 10);

        }

    #[test]
    fn  test_get_card_values(){
        let mut rank = vec!["K".to_string(), "Q".to_string(), "J".to_string(), "10".to_string(), "9".to_string(), "8".to_string(), "7".to_string(), "6".to_string(), "5".to_string(), "4".to_string(), "3".to_string(), "2".to_string(), "A".to_string()];
        let mut suit=vec!["D".to_string(),"H".to_string(),"S".to_string(),"C".to_string()];
        let mut card_picked:String = String::from("2D");


        let mut card = Deck {
            cards:Vec::new(),
        
        };


        let mut cards_to_deal_unshuffled= card.cardDeck(rank,suit);
        let mut group_cards_vec = cards_to_deal_unshuffled.clone();
        let mut cards_to_deal = cards_to_deal_unshuffled.clone();
        cards_to_deal.shuffle( &mut thread_rng());
        let mut sum:u8 = 0;
        let mut comp:Vec<String> = Vec::new();
        let mut play:Vec<String> = Vec::new();
        let mut upc = String::from("");
        let mut rem:Vec<String> = Vec::new();
        let mut count:u8 = 0;
        for i in &cards_to_deal{
            if sum<20{
        
                if count % 2 == 0 {
                    play.push(i.to_string());
          
                }
                else{
                    comp.push(i.to_string());
        
                }
            }
        
            else  {
                let necs = &cards_to_deal[sum as usize];
                rem.push(necs.to_string());
            }
           
            count +=1;
            sum +=1;
        
        }
        
        let mut set1:Vec<String> = Vec::new();
        let mut set2:Vec<String> = Vec::new();
        let mut set3:Vec<String> = Vec::new();
        let mut set4:Vec<String> = Vec::new();
        let mut set5:Vec<String> = Vec::new();
        let mut set6:Vec<String> = Vec::new();
        let mut set7:Vec<String> = Vec::new();
        let mut set8:Vec<String> = Vec::new();
        let mut set9:Vec<String> = Vec::new();
        let mut set10:Vec<String> = Vec::new();
        let mut set11:Vec<String> = Vec::new();
        let mut set12:Vec<String> = Vec::new();
        let mut set13:Vec<String> = Vec::new();
        
        let mut Vec_con = vec![set1,set2,set3,set4,set5,set6,set7,set8,set9,set10,set11,set12,set13];
        
        
        
        let mut count_1:usize = 0;
        
        let mut vec_of_all_sets :Vec<Vec<String>>= Vec::new();
        
        for mut val in Vec_con {
        
            let s = &group_cards_vec[count_1+0];
            let g = &group_cards_vec[count_1+13];
            let h = &group_cards_vec[count_1+26];
            let i = &group_cards_vec[count_1+39];
        
        
             val.push(s.to_string());
             val.push(g.to_string());
             val.push(h.to_string());
             val.push(i.to_string());
        
            vec_of_all_sets.push(val);
            count_1 += 1;
        
        }


        
        let Kings = &vec_of_all_sets[0];
        let Queens = &vec_of_all_sets[1];
        let Jacks = &vec_of_all_sets[2];
        let Ten = &vec_of_all_sets[3];
        let Nine = &vec_of_all_sets[4];
        let Eight = &vec_of_all_sets[5];
        let Seven = &vec_of_all_sets[6];
        let Six = &vec_of_all_sets[7];
        let Five = &vec_of_all_sets[8];
        let Fours = &vec_of_all_sets[9];
        let Thress = &vec_of_all_sets[10];
        let Twos = &vec_of_all_sets[11];
        let Aces = &vec_of_all_sets[12]; 
        
        
        
        let mut plays = Players{
            Computer:comp,
            player:play,
            remaining_card_deck:rem,
            cardvariant13:Kings.to_vec(),
            cardvariant12:Queens.to_vec(),
            cardvariant11:Jacks.to_vec(),
            cardvariant10:Ten.to_vec(),
            cardvariant9:Nine.to_vec(),
            cardvariant8:Eight.to_vec(),
            cardvariant7:Seven.to_vec(),
            cardvariant6:Six.to_vec(),
            cardvariant5:Five.to_vec(),
            cardvariant4:Fours.to_vec(),
            cardvariant3:Thress.to_vec(),
            cardvariant2:Twos.to_vec(),
            cardvariant1:Aces.to_vec(),


            
        
        
        
        };

        assert_eq!(plays.get_card_values(&card_picked), 2);

        }

    #[test]
    fn  play(){
        }


 


    // TESTS HERE
}
