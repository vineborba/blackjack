# Blackjack Game loop

## 1. **Setup**
   - The game uses one or more decks of 52 playing cards. At the start of the game or after each shuffle, the dealer will ensure the deck is properly mixed.
   - Players place their bets on the table.

---

## 2. **Dealing Cards**
   - The dealer gives two cards to each player and two cards to themselves.
   - One of the dealer's cards is face-up (visible to all players), and the other card is face-down (the "hole" card).
   - Players' cards are both dealt face-up or face-down depending on the casino rules.

---

## 3. **Checking for Blackjack**
   - If a player or the dealer has a total of 21 with the initial two cards (an Ace and a 10-value card), this is called "Blackjack."
   - **If the Dealer Has Blackjack**:
     - All players lose, unless they also have Blackjack, in which case the round is a push (tie).
   - **If a Player Has Blackjack**:
     - The player is immediately paid (typically 3:2 odds), unless the dealer also has Blackjack, in which case it's a push.
   - If no one has Blackjack, the game continues.

---

## 4. **Player Turns**
   - Each player, starting from the left of the dealer (first base), plays their hand. Players have several options:
     - **Hit**: The player requests another card to add to their hand. They can keep hitting until they choose to stop or they bust (go over 21).
     - **Stand**: The player keeps their current total and ends their turn.
     - **Double Down**: The player doubles their original bet and receives exactly one more card, then they must stand.
     - **Split**: If the player’s first two cards have the same value, they can split them into two separate hands, placing an additional bet equal to the original bet. Each hand is then played separately.
     - **Surrender**: In some games, players can choose to forfeit half of their bet and end their turn immediately. This option is usually only available as the first decision.
   - Players must carefully consider their options based on their hand total and the dealer's visible card.

---

## 5. **Dealer’s Turn**
   - Once all players have completed their turns, the dealer reveals their face-down card.
   - The dealer must follow strict rules:
     - The dealer must hit if their hand total is 16 or less, or if they have a "soft 17" (a hand totaling 17 with an Ace counted as 11).
     - The dealer must stand on a hard 17 or higher.
   - The dealer continues hitting until they either reach 17 or more or bust by exceeding 21.

---

## 6. **Comparing Hands and Payouts**
   - If the dealer busts, all remaining player hands win.
   - If the dealer does not bust, each player’s hand is compared to the dealer's hand:
     - **Winning Hand**: A player wins if their hand total is higher than the dealer's without busting. They receive even money on their bet (1:1), unless they have Blackjack, which pays 3:2.
     - **Losing Hand**: A player loses if their hand total is lower than the dealer's or if they have busted.
     - **Push**: If a player's hand total is the same as the dealer's, it's a tie, and the player's bet is returned.

---

## 7. **Round End and Next Round**
   - Players collect their winnings or lose their bets based on the outcomes.
   - Players can choose to place new bets, and the next round begins.
   - The dealer may reshuffle the cards if needed, depending on the casino's rules for how often to shuffle.

This loop continues until players decide to leave the game or until the casino closes the table. Let me know if you want more details on any specific part of the game!