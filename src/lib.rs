use crate::srs::card::Card;
use crate::srs::config::Config;
use crate::srs::scheduler::{Choice, Sched, Scheduler};

mod srs;
mod svc;

fn new(config: Config, day_cut_off: i64, day_today: i64) -> Option<Scheduler> {
    let scheduler = Scheduler::new(config, day_cut_off, day_today);
    Some(scheduler)
}

fn next_interval(
    card: Card,
    scheduler: Scheduler,
    choice: Choice,
) -> Option<i64> {
    Some(scheduler.next_interval(&card, choice))
}

fn next_interval_string(
    card: Card,
    scheduler: Scheduler,
    choice: Choice,
) -> Option<String> {
    Some(scheduler.next_interval_string(&card, choice))
}

fn answer_card(
    card: Card,
    scheduler: Scheduler,
    choice: Choice,
) -> Option<Card> {
    let mut card = card.clone();
    scheduler.answer_card(&mut card, choice);
    Some(card)
}

fn bury_card(card: Card, scheduler: Scheduler) -> Option<Card> {
    let mut card = card.clone();
    scheduler.bury_card(&mut card);
    Some(card)
}

fn unbury_card(card: Card, scheduler: Scheduler) -> Option<Card> {
    let mut card = card.clone();
    scheduler.unbury_card(&mut card);
    Some(card)
}

fn suspend_card(card: Card, scheduler: Scheduler) -> Option<Card> {
    let mut card = card.clone();
    scheduler.suspend_card(&mut card);
    Some(card)
}

fn unsuspend_card(card: Card, scheduler: Scheduler) -> Option<Card> {
    let mut card = card.clone();
    scheduler.unsuspend_card(&mut card);
    Some(card)
}

fn schedule_card_as_new(card: Card, scheduler: Scheduler) -> Option<Card> {
    let mut card = card.clone();
    scheduler.schedule_card_as_new(&mut card);
    Some(card)
}

fn set_new_position(card: Card, position: i64) -> Option<Card> {
    let mut card = card.clone();
    card.set_new_position(position);
    Some(card)
}

fn schedule_card_as_review(
    card: Card,
    scheduler: Scheduler,
    min_days: i32,
    max_days: i32,
) -> Option<Card> {
    let mut card = card.clone();
    scheduler.schedule_card_as_review(&mut card, min_days, max_days);
    Some(card)
}
