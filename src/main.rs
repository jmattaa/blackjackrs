use crossterm::event::{self, Event, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::widgets::{Block, Paragraph};

mod cards;
use cards::deck;
use cards::hand;

fn main() {
    let mut deck = deck::Deck::gen_deck();
    let mut dealer = hand::Hand::new(&mut deck);
    let mut player = hand::Hand::new(&mut deck);

    let mut isplayerturn = true;
    let mut playerbust = false;

    let mut terminal = ratatui::init();
    loop {
        terminal
            .draw(|f| draw(f, &dealer, &player, isplayerturn, None))
            .expect("failed to draw");

        if player.count() > 21 {
            playerbust = true;
        }

        if isplayerturn && !playerbust {
            match event::read().expect("failed to read event") {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('h') => {
                        if isplayerturn {
                            player.hit(&mut deck, 1)
                        }
                    }
                    KeyCode::Char('s') => isplayerturn = false,
                    _ => (),
                },
                // other event types than key
                _ => (),
            }
        } else {
            if dealer.count() < 17 && !playerbust {
                dealer.hit(&mut deck, 1);
                // sleep to not make it instant
                std::thread::sleep(std::time::Duration::from_millis(750));
            } else {
                let message = if playerbust {
                    "You bust! You lose"
                } else if dealer.count() > 21 {
                    "Dealer bust! You win"
                } else if player.count() > dealer.count() {
                    "You win!"
                } else if player.count() < dealer.count() {
                    "You lose"
                } else {
                    "Draw"
                };
                terminal
                    .draw(|f| draw(f, &dealer, &player, isplayerturn, Some(message)))
                    .expect("failed to draw");

                // this is blocking and waits for input before continuing
                match event::read().expect("failed to read event") {
                    Event::Key(key) => match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('c') => {
                            deck = deck::Deck::gen_deck();
                            dealer = hand::Hand::new(&mut deck);
                            player = hand::Hand::new(&mut deck);
                            isplayerturn = true;
                            playerbust = false;
                        }
                        _ => (),
                    },
                    // other event types than key
                    _ => (),
                }
            }
        }
    }

    ratatui::restore();
}

fn draw(
    f: &mut Frame,
    dealer: &hand::Hand,
    player: &hand::Hand,
    isplayerturn: bool,
    msg: Option<&str>,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // top
            Constraint::Length(3), // info
            Constraint::Length(2), // second hand
            Constraint::Length(2), // second hand
            Constraint::Min(1),    // rest
        ])
        .split(f.area());

    f.render_widget(
        Paragraph::new("press 'q' to quit").block(Block::default()),
        chunks[0],
    );
    if msg.is_none() {
        if isplayerturn {
            f.render_widget(
                Paragraph::new("press 'h' to hit and 's' to stand").block(Block::default()),
                chunks[1],
            );
        } else {
            f.render_widget(
                Paragraph::new("Delers turn...").block(Block::default()),
                chunks[1],
            );
        }
    } else {
        f.render_widget(
            Paragraph::new(format!("{} Press 'c' to continue playing", msg.unwrap()))
                .block(Block::default())
                .alignment(Alignment::Center),
            chunks[1],
        );
    }

    f.render_widget(
        Paragraph::new(format!(
            "Dealer: {} total: {}",
            dealer.dealerstr(isplayerturn),
            dealer.dealercount(isplayerturn)
        ))
        .block(Block::default())
        .alignment(Alignment::Center),
        chunks[2],
    );
    f.render_widget(
        Paragraph::new(format!(
            "Player: {} total: {}",
            player.to_string(),
            player.count()
        ))
        .block(Block::default())
        .alignment(Alignment::Center),
        chunks[3],
    );
}
