# [Card  Game - Trash](https://playingcarddecks.com/blogs/how-to-play/trash-game-rules)


Trash, or Garbage, is a classic card game for two player. It requires a standard 52 playing card deck. The objective of Trash is to be the first person with a complete hand of 10 cards.

This smart contract code impliments the Trash card game by following the card game rules.


## Set Up
To set up a game of Trash, players need to first sit around a suitable gameplay area. Before gameplay can begin, every player draws a card from a shuffled deck. The player with the highest card becomes the first dealer. Ties are broken by a redraw. The dealer then shuffles the deck and passes out ten cards, faced down, arranged in a 2 x 5 grid. Players do not look at these cards.

The remaining deck forms the the stock pile.

In Trash, Aces are 1, 2s-10s are their face value, Jacks are wildcards and Queens and Kings are unplayable.

For a better understanding please visist [Trash Game Rules](https://playingcarddecks.com/blogs/how-to-play/trash-game-rules)


## Required software

1. Rust 1.58 + cargo
2. Node.js
3. NEAR CLI 3.1

### Getting started

The first objective  is to get a deck of cards with 52 cards and deal the cards:

1. Function that gets the deck of card.

    ![card deck](https://github.com/Kagwep/Card-Game-Trash/blob/master/deck.PNG)

2. To process the cards we will use their corresponding ranks values. The standard card rankings, [from highest to lowest, are: King, Queen, Jack, 10, 9, 8, 7, 6, 5, 4, 3, 2, Ace](https://www.pokerzone.com/dictionary/ranks). struct Cardvariant 1- 13 holds the ranks.

    ![card values](https://github.com/Kagwep/Card-Game-Trash/blob/master/valvec.PNG)
    ![card values](https://github.com/Kagwep/Card-Game-Trash/blob/master/ranks.PNG)

3. The code shuffles and deal the cards giving each player a set 10 of cards and noting the remaining card deck. (players - Computer and user):

    ![card deal](https://github.com/Kagwep/Card-Game-Trash/blob/master/vsdr.PNG)

Playing the game 

 1. By following the rules of the game the code impliments the game play and returns the winner of the round.
 

    ![card deal](https://github.com/Kagwep/Card-Game-Trash/blob/master/fedts.PNG)

**Get more info at:**

* Program comments
* [Trash Game Rules
](https://playingcarddecks.com/blogs/how-to-play/trash-game-rules)

