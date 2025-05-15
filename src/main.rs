use crossterm::event::{self, Event, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::widgets::{Block, Paragraph};

mod cards;
use cards::deck;
use cards::hand;

fn main() {
    let mut deck = deck::Deck::gen_deck();
    let mut dealer = hand::Hand::new();
    let mut player = hand::Hand::new();

    let mut isplayerturn = true;

    // both start with one card
    dealer.hit(&mut deck, 2);
    player.hit(&mut deck, 2);

    let mut terminal = ratatui::init();
    loop {
        terminal
            .draw(|f| draw(f, &dealer, &player, &deck, isplayerturn))
            .expect("failed to draw");
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
    }

    ratatui::restore();
}

fn draw(
    f: &mut Frame,
    dealer: &hand::Hand,
    player: &hand::Hand,
    _deck: &deck::Deck,
    isplayerturn: bool,
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
    if isplayerturn {
        f.render_widget(
            Paragraph::new("press 'h' to hit and 's' to stand").block(Block::default()),
            chunks[1],
        );
    } else {
        f.render_widget(
            Paragraph::new("Dealer's turn...").block(Block::default()),
            chunks[1],
        );
    }

    f.render_widget(
        Paragraph::new(format!(
            "Dealer: {} total: {}",
            dealer.to_string(),
            dealer.count()
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
