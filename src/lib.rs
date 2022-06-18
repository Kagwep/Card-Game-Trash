use std::vec;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;
use std::collections::HashSet;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]

// Struck that holds the card deck
struct  Deck{
    cards: Vec<String>,

}

#[near_bindgen]
// Generating card deck of 52 cards
impl Deck{
    
    // argument rank will be the rank of the 13 cards: King, Queen, Jack, 10, 9, 8, 7, 6, 5, 4, 3, 2, Ace
    // argument suit passes the suits of the cards: Diamond, King, Queen, Jack
    pub fn card_deck(&mut self, rank:Vec<String>, suit:Vec<String>) -> Vec<String>{

            // loops four times per suit
            for s in 0..4{
                // assing the ranks the suit
                for r in 0..13{
                    //get the suit form the vec containing the suits
                    let mut news = String::from(&rank[r]);
                    //get the rank and concatinate the suit to give it the suit
                    news.push_str(&suit[s]);
                    // push the card to the card deck
                    self.cards.push(news);
                   

                }
            }
            //creat a vec of srings
        let cards_d = &self.cards;
        //return the vec
        return cards_d.to_vec();

    }
}

#[near_bindgen]
// dealt cards and their classifications
struct Players{
    computer:Vec<String>,
    player:Vec<String>,
    remaining_card_deck:Vec<String>,
    cardvariant1:Vec<String>,cardvariant2:Vec<String>,cardvariant3:Vec<String>,cardvariant4:Vec<String>,
    cardvariant5:Vec<String>,cardvariant6:Vec<String>,cardvariant7:Vec<String>,cardvariant8:Vec<String>,
    cardvariant9:Vec<String>,cardvariant10:Vec<String>,cardvariant11:Vec<String>,cardvariant12:Vec<String>,
    cardvariant13:Vec<String>,
}

#[near_bindgen]
// contract 
impl  Players {
    // funtion returns the value equivalent  of the given vec of cards (computer || player)
    pub fn get_card_vec(&self, player:&Vec<String>) -> Vec<u8>{
        //vec to be returned
        let mut decision_vec:Vec<u8> = Vec::new();
        // checks
        // this loop returns the value equivalet of the card among the class
        // ACE-Diamond Ace-Spade Ace-Hearts Ace-Clubs will return a value of 1 
        for c in player{
            // value check for a Kings
            if self.cardvariant13.contains(&c){
                decision_vec.push(13);

            }
            // value checks for a  Queens
            else if self.cardvariant12.contains(&c){
                decision_vec.push(12);

            }
            // valu checks for a Jacks
            else if self.cardvariant11.contains(&c){
                decision_vec.push(11);

            }
            //value check for a Ten
            else if self.cardvariant10.contains(&c){
                decision_vec.push(10);

            }
            // valu check for a Nine
            else if self.cardvariant9.contains(&c){
                decision_vec.push(9);

            }
            // value check for a Eight
            else if self.cardvariant8.contains(&c){
                decision_vec.push(8);

            }
            //value check for a sevens 
            else if self.cardvariant7.contains(&c){
                decision_vec.push(7);

            }
            // value check for a six
            else if self.cardvariant6.contains(&c){
                decision_vec.push(6);

            }
            //value check for a Five
            else if self.cardvariant5.contains(&c){
                decision_vec.push(5);

            }
            //value check for a four
            else if self.cardvariant4.contains(&c){
                decision_vec.push(4);

            }
            //value check for a Three
            else if self.cardvariant3.contains(&c){
                decision_vec.push(3);

            }
            //value check for a two
            else if self.cardvariant2.contains(&c){
                decision_vec.push(2);

            }
            // value check for Ace
            else if self.cardvariant1.contains(&c){
                decision_vec.push(1);

            }

        }
        //This vector is returned to help the play finction decide whether the ten cards have been aranged from ACE to Ten
        return decision_vec;
    }

     // This function helps to get the value of an individual card picked from the remaining card deck
     //This will ensure that the player knows when the player has already used a spot in their deck of cards
    pub fn get_card_values(&self, card_picked:&String) -> usize{
        // value of card is return
        let value_of_card:usize;
        // checks card value form Ace - King
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
    
    pub fn compute(&self,mut card_value:usize,mut done_card:Vec<usize>,mut card:String,mut com_hidden_cards:Vec<String>,player_turn:Vec<String>) -> Vec<String>{
        // In Trash, King, Queen and Jack are wild cards. Each time a player draws one of this card from the deck the players turn end
        // cards King, Queen and Jack were assigned Valeues 13,12 and 11 for this purpose
        //while loop will not execute when any of the cards mention is picked.
        while card_value <= 10{

            //This loop checks whether a player has picked a card in the their deck and its equivalent position is taken
            //E.g players turn will end when he picks Seven of Diamonds but a Seven of clubs already filled the spot
            if done_card.contains(&card_value){
                println!("You aready filled that spot");
                //returns the updated players vector
                return com_hidden_cards;
            }

            else{

            //gets card whose position was taken
            let card_replaced = &player_turn[card_value-1];
            // replaces the vec of cards with an updated one eg replaces X with Ace
            com_hidden_cards[card_value-1] = card.to_string();
            // marks card as aready delt
            done_card.push(card_value);
            card_value = self.get_card_values(&card_replaced);
            //updates card initially picked with card now on hand
            card = card_replaced.to_string();
  
            }


        } 

        return com_hidden_cards;
    }
    
    pub fn play(&mut self, entry:String, cond:String) -> Vec<String>{

        // Hidden cards mimicing cards placed face down 
        //represents cards assigned to players
        let mut hidden_cards:Vec<String> = vec!["X".to_string(),"X".to_string(),"X".to_string(),"X".to_string(),"X".to_string(),"X".to_string(),"X".to_string(),"X".to_string(),"X".to_string(),"X".to_string()];
        let mut com_hidden_cards:Vec<String> =vec!["Y".to_string(),"Y".to_string(),"Y".to_string(),"Y".to_string(),"Y".to_string(),"Y".to_string(),"Y".to_string(),"Y".to_string(),"Y".to_string(),"Y".to_string()];
        //played cards vectors
        let done_card:Vec<usize> = Vec::new();
        let c_done_card:Vec<usize> = Vec::new();
        //vector containing the remaining cards 
        let mut r_cards = self.remaining_card_deck.clone();
        //this vector collects cards that have aready been used, picked but not used
        let mut played_cards: Vec<String> = Vec::new();
        println!(" You are the computer!!!: \n {:?}",self.computer);
        println!(" You are the player!!!: \n {:?}",self.player);
        println!(" You are the remainder!!!: \n {:?}",self.remaining_card_deck);

    // loop that makes sure the game continues untill a winner is found
     loop{

        // gets the vector value equivalent of the cards of both playes
        let a = self.get_card_vec(&hidden_cards);
        let b = self.get_card_vec(&com_hidden_cards);


       // checks whether Player/user has won
        if a.len() == 10{

             println!(" You are the winner!!!: \n {:?}",hidden_cards);
             return hidden_cards;
        }
        //checks whether computer has won 
        if b.len() == 10{

            println!(" I am  the winner!!!: \n {:?}",com_hidden_cards);
            return com_hidden_cards;
       }
       // incase the remaining cards after dealing are used during game play, 
        if r_cards.len() == 1 || r_cards.len() == 0 {
            //unused cards collected are picked and shuffled
            let shuffled_c: HashSet<String> =  played_cards.into_iter().collect();
            played_cards = shuffled_c.into_iter().collect();
            //assigned to reamaing cards
            r_cards=(&played_cards).to_vec();

        }
            // input used to check players decision
  
            if entry== cond{

                 // player picks card from the remaining card deck

                let card =&r_cards.clone()[0];

                // the card is removed from the deck
   
                r_cards.remove(0);
                
                //the value of the card is noted
    
                let card_value = self.get_card_values(&card);
    
                // variable will be used to check whether a card of that rank has been used 
                let card_x = card_value.clone() as u8;

                // if card is among the King, Queen or Clubs -- add to played cards
                if card_value > 10{
                    played_cards.push((&card).to_string());
                }

                println!("Remaining card deck is: {:?}",r_cards);
                // if card of the same rank has been used , next players round
                if a.contains(&card_x){
                    println!("spot filled");
                    played_cards.push((&card).to_string());
                    // push card to played cards
                }
                 // if not continue game play
                else{

                    let ci = self.get_card_values(card) as u8;
                    //check whether  cards is used
                    if self.get_card_vec(&hidden_cards).contains(&ci){

                        println!("spot also filled");
                        played_cards.push((&card).to_string());

                    }
                    else{

                        // compute the cards and update the cards of the player
                        hidden_cards = self.compute(card_value,done_card.clone(),card.clone().to_string(), hidden_cards.clone(),self.player.clone());
    

                    }

                }
                // repeat the same procedure to the next player
                let card_c = &r_cards.clone()[0];

                r_cards.remove(0);

                let card_value = self.get_card_values(&card_c);
    
    
                let card_y = card_value.clone() as u8;

                if b.contains(&card_y){
                    println!("spot filled");
                    played_cards.push((&card_c).to_string());

                }

                else{

                    let cv = self.get_card_values(card_c) as u8;

                    if self.get_card_vec(&com_hidden_cards).contains(&cv){

                        println!("spot also filled");
                        played_cards.push((&card_c).to_string());

                    }

                    else{


                        let c_card_value = self.get_card_values(&card_c);
            
                        
                        
                        com_hidden_cards = self.compute(c_card_value,c_done_card.clone(),card_c.clone().to_string(), com_hidden_cards.clone(),self.computer.clone());
            

                    }
    

                }
    

            }
            else{
    
                println!("Invalid input .... press 1 to pick card .");
    
            }


        }

    }
}  

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }
    
   //defines the suits of the cards
    fn suit_vec() -> Vec<String>{
        let suit=vec![ "\u{2666}".to_string(), "\u{2665}".to_string(), "\u{2660}".to_string() , "\u{2663}".to_string()];
        return suit;
     }
    // defines the ranks of the cards
    fn rank_vec() -> Vec<String>{
        let rank = vec!["K".to_string(), "Q".to_string(), "J".to_string(), "10".to_string(), "9".to_string(), "8".to_string(), "7".to_string(), "6".to_string(), "5".to_string(), "4".to_string(), "3".to_string(), "2".to_string(), "A".to_string()];
 
        return rank;
    }
    // sets to group cards
    fn card_sets() -> Vec<Vec<String>>{

        let  set1:Vec<String> = Vec::new();
        let  set2:Vec<String> = Vec::new();
        let  set3:Vec<String> = Vec::new();
        let  set4:Vec<String> = Vec::new();
        let  set5:Vec<String> = Vec::new();
        let  set6:Vec<String> = Vec::new();
        let  set7:Vec<String> = Vec::new();
        let set8:Vec<String> = Vec::new();
        let  set9:Vec<String> = Vec::new();
        let  set10:Vec<String> = Vec::new();
        let  set11:Vec<String> = Vec::new();
        let  set12:Vec<String> = Vec::new();
        let  set13:Vec<String> = Vec::new();
        
        let vec_con = vec![set1,set2,set3,set4,set5,set6,set7,set8,set9,set10,set11,set12,set13];
        
        return vec_con;
        
    }
    // shuffle card deck and return a vector of both shuffled and unshuffled
    fn card_deck_vecs() -> Vec<Vec<String>>{
        let mut vec_of_cards:Vec<Vec<String>> = Vec::new();
        let mut card = Deck {
            cards:Vec::new(),
        
        };

        let cards_to_deal_unshuffled= card.card_deck(rank_vec(), suit_vec());
        let  group_cards_vec = cards_to_deal_unshuffled.clone();
        let mut cards_to_deal = cards_to_deal_unshuffled;
        let shuffled: HashSet<String> = cards_to_deal.into_iter().collect();
        cards_to_deal = shuffled.into_iter().collect();
        // println!("the vector of ordered is: {:?}",group_cards_vec.clone());
        // println!("the vector of unorderdordered is: {:?}",cards_to_deal.clone());
        vec_of_cards.push(group_cards_vec);
        vec_of_cards.push(cards_to_deal);


    
    
        return vec_of_cards;
    }


   // assign classes to respective cards
    fn card_sets_final() -> Vec<Vec<String>>{
        let mut count_1:usize = 0;
        
        let mut vec_of_all_sets :Vec<Vec<String>>= Vec::new();
        
        
        for mut val in card_sets() {
        
            let s = &card_deck_vecs()[0][count_1+0];
            let g = &card_deck_vecs()[0][count_1+13];
            let h = &card_deck_vecs()[0][count_1+26];
            let i = &card_deck_vecs()[0][count_1+39];
        
        
             val.push(s.to_string());
             val.push(g.to_string());
             val.push(h.to_string());
             val.push(i.to_string());
        
            vec_of_all_sets.push(val);
            count_1 += 1;
        
        }

        return vec_of_all_sets;
    }

    // deal cards
    fn dealt_cards() -> Vec<Vec<String>>{

        let mut dealt_cards_players :Vec<Vec<String>>= Vec::new();

        let mut sum:u8 = 0;
        let mut comp:Vec<String> = Vec::new();
        let mut play:Vec<String> = Vec::new();
        let mut rem:Vec<String> = Vec::new();
        let mut count:u8 = 0;
        for i in &card_deck_vecs()[1]{
            if sum<20{
        
                if count % 2 == 0 {
                    play.push(i.to_string());

          
                }
                else{
                    comp.push(i.to_string());
                    
                    
        
                }
            }
        
            else  {
                let necs = &card_deck_vecs()[1][sum as usize];
                rem.push(necs.to_string());
                
            }
           
            count +=1;
            sum +=1;
        
        } 
        println!("The vector of play is: {:?}",play.clone());
        println!("The vector of comp is: {:?}",comp.clone());
        println!("The vector of rem is: {:?}",rem.clone());
        dealt_cards_players.push((&play).to_vec());
        dealt_cards_players.push((&comp).to_vec());
        dealt_cards_players.push((&rem).to_vec());

        println!("{:?}",rem);
        
        return dealt_cards_players;

    }
    fn card_set_up() -> Vec<Vec<String>> {

        let mut card_set_ups_vec :Vec<Vec<String>>= Vec::new();

        let kings = &card_sets_final()[0];
        card_set_ups_vec.push(kings.to_vec());
        let queens = &card_sets_final()[1];
        card_set_ups_vec.push(queens.to_vec());
        let jacks = &card_sets_final()[2];
        card_set_ups_vec.push(jacks.to_vec());
        let ten = &card_sets_final()[3];
        card_set_ups_vec.push(ten.to_vec());
        let nine = &card_sets_final()[4];
        card_set_ups_vec.push(nine.to_vec());
        let eight = &card_sets_final()[5];
        card_set_ups_vec.push(eight.to_vec());
        let seven = &card_sets_final()[6];
        card_set_ups_vec.push(seven.to_vec());
        let six = &card_sets_final()[7];
        card_set_ups_vec.push(six.to_vec());
        let five = &card_sets_final()[8];
        card_set_ups_vec.push(five.to_vec());
        let fours = &card_sets_final()[9];
        card_set_ups_vec.push(fours.to_vec());
        let thress = &card_sets_final()[10];
        card_set_ups_vec.push(thress.to_vec());
        let twos = &card_sets_final()[11];
        card_set_ups_vec.push(twos.to_vec());
        let aces = &card_sets_final()[12]; 
        card_set_ups_vec.push(aces.to_vec());
        

        return card_set_ups_vec;
     
    }
  //struct players 
    fn card_variables() -> Players{

        let play = Players{
            computer: (&dealt_cards()[1]).to_vec(),
            player: (&dealt_cards()[0]).to_vec(),
            remaining_card_deck: (&dealt_cards()[2]).to_vec(),
            cardvariant13:(&card_set_up()[0]).to_vec(),
            cardvariant12:(&card_set_up()[1]).to_vec(),
            cardvariant11:(&card_set_up()[2]).to_vec(),
            cardvariant10:(&card_set_up()[3]).to_vec(),
            cardvariant9:(&card_set_up()[4]).to_vec(),
            cardvariant8:(&card_set_up()[5]).to_vec(),
            cardvariant7:(&card_set_up()[6]).to_vec(),
            cardvariant6:(&card_set_up()[7]).to_vec(),
            cardvariant5:(&card_set_up()[8]).to_vec(),
            cardvariant4:(&card_set_up()[9]).to_vec(),
            cardvariant3:(&card_set_up()[10]).to_vec(),
            cardvariant2:(&card_set_up()[11]).to_vec(),
            cardvariant1:(&card_set_up()[12]).to_vec(),
        
        
        
        };
        return  play;

    }
    #[test]
    fn test_card_deck(){

        let mut card = Deck {
            
            cards:Vec::new(),
        
        };

        assert_eq!(card.card_deck(rank_vec(), suit_vec()).len(), 52);
        
    }      

    #[test]
    fn  test_get_card_vec(){

        let player1 = vec!["10\u{2665}".to_string(), "A\u{2666}".to_string(), "5\u{2666}".to_string(), "6\u{2665}".to_string(), "4\u{2660}".to_string(), "2\u{2660}".to_string(), "Q\u{2663}".to_string(), "K\u{2666}".to_string(), "4\u{2665}".to_string(), "A\u{2665}".to_string()];

        assert_eq!(card_variables().get_card_vec(&player1).len(), 10);

        }

    #[test]
    fn  test_get_card_values(){
    
        let card_picked:String = String::from("2\u{2666}");
        

        assert_eq!(card_variables().get_card_values(&card_picked), 2);

        }

    #[test]
    fn  test_compute(){
       
        let card_value:usize = 8;
        let  done_card:Vec<usize> = vec![1,2,3,4];
        let card:String = String::from("8\u{2666}");
        let com_hidden_cards:Vec<String> = vec!["10\u{2665}".to_string(), "A\u{2666}".to_string(), "5\u{2666}".to_string(), "6\u{2665}".to_string(), "4\u{2660}".to_string(), "2\u{2660}".to_string(), "Q\u{2663}".to_string(), "K\u{2666}".to_string(), "4\u{2665}".to_string(), "A\u{2665}".to_string()];
        
        let check_for = card_variables().compute(card_value,done_card,card.clone(),com_hidden_cards,card_variables().player);
       
        assert_eq!( check_for[7], card);

        }


 
    #[test]
    fn test_play(){

        let mut card = Deck {
            cards:Vec::new(),
        
        };
        
        let cards_to_deal_unshuffled= card.card_deck(rank_vec(), suit_vec());
        let group_cards_vec = cards_to_deal_unshuffled.clone();
        let mut cards_to_deal = cards_to_deal_unshuffled;
        let shuffled: HashSet<String> = cards_to_deal.into_iter().collect();
        cards_to_deal = shuffled.into_iter().collect();
        
        let mut sum:u8 = 0;
        let mut comp:Vec<String> = Vec::new();
        let mut play:Vec<String> = Vec::new();
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
            
        let mut count_1:usize = 0;
        
        let mut vec_of_all_sets :Vec<Vec<String>>= Vec::new();
        
        
        for mut val in card_sets() {
        
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
        
        let kings = &vec_of_all_sets[0];
        let queens = &vec_of_all_sets[1];
        let jacks = &vec_of_all_sets[2];
        let ten = &vec_of_all_sets[3];
        let nine = &vec_of_all_sets[4];
        let eight = &vec_of_all_sets[5];
        let seven = &vec_of_all_sets[6];
        let six = &vec_of_all_sets[7];
        let five = &vec_of_all_sets[8];
        let fours = &vec_of_all_sets[9];
        let thress = &vec_of_all_sets[10];
        let twos = &vec_of_all_sets[11];
        let aces = &vec_of_all_sets[12]; 
        
        
        
        
        let mut ply = Players{
            computer:comp,
            player:play,
            remaining_card_deck:rem,
            cardvariant13:kings.to_vec(),
            cardvariant12:queens.to_vec(),
            cardvariant11:jacks.to_vec(),
            cardvariant10:ten.to_vec(),
            cardvariant9:nine.to_vec(),
            cardvariant8:eight.to_vec(),
            cardvariant7:seven.to_vec(),
            cardvariant6:six.to_vec(),
            cardvariant5:five.to_vec(),
            cardvariant4:fours.to_vec(),
            cardvariant3:thress.to_vec(),
            cardvariant2:twos.to_vec(),
            cardvariant1:aces.to_vec(),
        
        
        
        };
        
        let cond:String = String::from("1") ;
        let entry:String = String::from("1") ;
        assert_eq!( ply.play(entry,cond).len(), 10);

    }




    // TESTS HERE
}
