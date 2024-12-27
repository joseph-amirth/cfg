use cfg::grammar;

fn main() {
    let cfg = grammar! {
        sentence => noun_phrase verb_phrase
        noun_phrase => pronoun | proper_noun | determiner nominal
        nominal => nominal noun | noun
        verb_phrase => verb | verb noun_phrase | verb noun_phrase prep_phrase | verb prep_phrase
        prep_phrase => preposition noun_phrase
        noun => "flights" | "flight" | "breeze" | "trip" | "morning"
        verb => "is" | "prefer" | "like" | "need" | "want" | "fly" | "do"
        adjective => "cheapest" | "non-stop" | "first" | "latest" | "other" | "direct"
        pronoun => "me" | "I" | "you" | "it"
        proper_noun => "Alaska" | "Baltimore" | "Los Angeles" | "Chicago" | "United" | "American"
        determiner => "the" | "a" | "an" | "this" | "these" | "that"
        preposition => "from" | "to" | "on" | "near" | "in"
        conjunction => "and" | "or" | "but"
    };

    let sentence_count = 10;
    println!(
        "Printing {} random sentences from the english language",
        sentence_count
    );
    for _ in 0..sentence_count {
        println!("{:?}", cfg.random_word());
    }
}
