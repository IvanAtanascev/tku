use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    widgets::{Block, Borders, Paragraph},
};

use crate::{
    data_io::csv::CsvIoHandler,
    logic::{self, cards_logic::CardsLogic},
};

pub struct Cards {
    card_logic_current_state: CardsLogic<CsvIoHandler>,
    display_original: bool,
    display_translation: bool,
    display_description: bool,
}

impl Cards {
    pub fn new(path: String) -> Self {
        let io_handler: CsvIoHandler = CsvIoHandler::new();
        let mut card_logic: CardsLogic<CsvIoHandler> = CardsLogic::new(io_handler);
        card_logic.init_entries(path);
        Self {
            card_logic_current_state: card_logic,
            display_original: true,
            display_translation: false,
            display_description: true,
        }
    }

    pub fn execute(&mut self) -> color_eyre::Result<()> {
        color_eyre::install()?;
        let terminal = ratatui::init();
        let result = self.run(terminal);
        ratatui::restore();
        result
    }

    fn run(&mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;

            if let Event::Key(key) = event::read()? {
                match (key.code, key.modifiers) {
                    (KeyCode::Char('q'), KeyModifiers::CONTROL) => break Ok(()),
                    (KeyCode::Char('n'), KeyModifiers::CONTROL) => {
                        self.card_logic_current_state.increment_current_entry();
                    }
                    (KeyCode::Char('t'), KeyModifiers::CONTROL) => {
                        self.display_translation = !self.display_translation;
                    }
                    (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                        self.display_description = !self.display_description;
                    }
                    (KeyCode::Char('o'), KeyModifiers::CONTROL) => {
                        self.display_original = !self.display_original;
                    }
                    _ => {}
                }
            }
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        use Constraint::Percentage;

        let vertical = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(vec![Percentage(50), Percentage(50)])
            .split(frame.area());

        let horizontal = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(vec![Percentage(50), Percentage(50)])
            .split(vertical[0]);

        frame.render_widget(
            Paragraph::new(if self.display_original {
                self.card_logic_current_state.get_current_original()
            } else {
                "_".repeat(self.card_logic_current_state.get_current_original().len())
            })
                .block(Block::new().borders(Borders::ALL)),
            horizontal[0],
        );
        frame.render_widget(
            Paragraph::new(if self.display_translation {
                self.card_logic_current_state.get_current_translation()
            } else {
                "_".repeat(self.card_logic_current_state.get_current_translation().len())
            })
                .block(Block::new().borders(Borders::ALL)),
            horizontal[1],
        );
        frame.render_widget(
            Paragraph::new(if self.display_description {
                self.card_logic_current_state.get_current_description()
            } else {
                "_".repeat(self.card_logic_current_state.get_current_description().len())
            })
                .block(Block::new().borders(Borders::ALL)),
            vertical[1],
        );
    }
}
