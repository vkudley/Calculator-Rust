/// iced-based calculator application.
///
/// Provides the GUI for the calculator using the iced framework.

use iced::widget::{Button, Column, Row, Space, Text};
use iced::{Alignment, Color, Font, Length, Task};

use crate::calculator::{Calculator, Operation};

/// The calculator application name.
const APP_NAME: &str = "Calculator";

/// The font size for the display.
const DISPLAY_FONT_SIZE: f32 = 48.0;

/// The font size for buttons.
const BUTTON_FONT_SIZE: f32 = 24.0;

/// The padding for buttons in pixels.
const BUTTON_PADDING: f32 = 16.0;

/// The spacing between buttons.
const BUTTON_SPACING: f32 = 6.0;

/// The left offset for the button grid to center it visually.
const BUTTON_GRID_LEFT_OFFSET: f32 = 4.0;

/// The width of each button column.
const BUTTON_WIDTH: f32 = 80.0;

/// The height of each button row.
const BUTTON_HEIGHT: f32 = 65.0;

/// The footer text.
const FOOTER_TEXT: &str = "Made in Rust with Qwen 3.6 driven by Cline";

/// The footer text font size.
const FOOTER_FONT_SIZE: f32 = 10.0;

/// Messages that the calculator can handle.
#[derive(Debug, Clone, Copy)]
pub enum CalculatorMessage {
    /// A digit (0-9) was pressed.
    DigitPressed(u8),
    /// The decimal point was pressed.
    DecimalPressed,
    /// An operation (+, -, ×, ÷) was pressed.
    OperationPressed(Operation),
    /// The equals (=) button was pressed.
    EqualsPressed,
    /// The clear (C) button was pressed.
    ClearPressed,
    /// The percentage (%) button was pressed.
    PercentagePressed,
    /// The negate (±) button was pressed.
    NegatePressed,
}

/// The state of the calculator application.
pub struct CalculatorState {
    calculator: Calculator,
    display_text: String,
}

/// Run the calculator application using the functional API.
pub fn run() -> iced::Result {
    iced::application(CalculatorState::boot, CalculatorState::update, CalculatorState::view)
        .title(APP_NAME)
        .theme(iced::Theme::Dark)
        .centered()
        .window_size(iced::Size::new(340.0, 520.0))
        .run()
}

impl CalculatorState {
    /// Boot the calculator state.
    fn boot() -> (Self, Task<CalculatorMessage>) {
        (
            Self {
                calculator: Calculator::new(),
                display_text: "0".to_string(),
            },
            Task::none(),
        )
    }

    /// Update the calculator state.
    fn update(state: &mut Self, message: CalculatorMessage) -> Task<CalculatorMessage> {
        state.handle_message(message);
        Task::none()
    }

    /// Handle a message.
    fn handle_message(&mut self, message: CalculatorMessage) {
        match message {
            CalculatorMessage::DigitPressed(d) => {
                if let Err(e) = self.calculator.enter(d) {
                    self.display_text = format!("{}", e);
                } else {
                    self.display_text = self.calculator.display();
                }
            }
            CalculatorMessage::DecimalPressed => {
                self.calculator.decimal();
                self.display_text = self.calculator.display();
            }
            CalculatorMessage::OperationPressed(op) => {
                if let Err(e) = self.calculator.operation(op) {
                    self.display_text = format!("{}", e);
                } else {
                    self.display_text = self.calculator.display();
                }
            }
            CalculatorMessage::EqualsPressed => {
                match self.calculator.equals() {
                    Ok(_) => {
                        self.display_text = self.calculator.display();
                    }
                    Err(e) => {
                        self.display_text = format!("{}", e);
                    }
                }
            }
            CalculatorMessage::ClearPressed => {
                self.calculator = Calculator::new();
                self.display_text = "0".to_string();
            }
            CalculatorMessage::PercentagePressed => {
                match self.calculator.percentage() {
                    Ok(_) => {
                        self.display_text = self.calculator.display();
                    }
                    Err(e) => {
                        self.display_text = format!("{}", e);
                    }
                }
            }
            CalculatorMessage::NegatePressed => {
                if let Err(e) = self.calculator.negate() {
                    self.display_text = format!("{}", e);
                } else {
                    self.display_text = self.calculator.display();
                }
            }
        }
    }

    /// View the calculator state.
    fn view(state: &Self) -> Column<'_, CalculatorMessage> {
        // Display area - right-aligned text
        let display = Row::new()
            .width(Length::Fixed(326.0))
            .height(Length::Fixed(80.0))
            .padding(20)
            .align_y(iced::alignment::Vertical::Center)
            .push(Space::new().width(Length::Fill))
            .push(
                Text::new(&state.display_text)
                    .font(Font::DEFAULT)
                    .size(DISPLAY_FONT_SIZE)
                    .color(Color::from_rgb(1.0, 1.0, 1.0)),
            );

        // Button rows
        let button_rows = Self::build_button_rows(state);

        let footer = Row::new()
            .push(Space::new().width(Length::Fixed(BUTTON_GRID_LEFT_OFFSET)))
            .push(
                Text::new(FOOTER_TEXT)
                    .font(Font::DEFAULT)
                    .size(FOOTER_FONT_SIZE)
                    .color(Color::from_rgb(0.5, 0.5, 0.5)),
            );

        Column::new()
            .push(display)
            .push(button_rows)
            .push(Space::new().height(Length::Fixed(16.0)))
            .push(footer)
            .width(Length::Fixed(326.0))
    }

    /// Build the button rows for the calculator.
    fn build_button_rows(_state: &Self) -> Column<'_, CalculatorMessage> {
        let clear_row = Row::new()
            .spacing(BUTTON_SPACING)
            .width(Length::Fixed(326.0))
            .push(Space::new().width(Length::Fixed(BUTTON_GRID_LEFT_OFFSET)))
            .push(Self::number_button("C", CalculatorMessage::ClearPressed))
            .push(Self::number_button("±", CalculatorMessage::NegatePressed))
            .push(Self::number_button("%", CalculatorMessage::PercentagePressed))
            .push(Self::op_button("÷", Operation::Divide));

        let row7 = Row::new()
            .spacing(BUTTON_SPACING)
            .width(Length::Fixed(326.0))
            .push(Space::new().width(Length::Fixed(BUTTON_GRID_LEFT_OFFSET)))
            .push(Self::number_button("7", CalculatorMessage::DigitPressed(7)))
            .push(Self::number_button("8", CalculatorMessage::DigitPressed(8)))
            .push(Self::number_button("9", CalculatorMessage::DigitPressed(9)))
            .push(Self::op_button("×", Operation::Multiply));

        let row4 = Row::new()
            .spacing(BUTTON_SPACING)
            .width(Length::Fixed(326.0))
            .push(Space::new().width(Length::Fixed(BUTTON_GRID_LEFT_OFFSET)))
            .push(Self::number_button("4", CalculatorMessage::DigitPressed(4)))
            .push(Self::number_button("5", CalculatorMessage::DigitPressed(5)))
            .push(Self::number_button("6", CalculatorMessage::DigitPressed(6)))
            .push(Self::op_button("-", Operation::Subtract));

        let row1 = Row::new()
            .spacing(BUTTON_SPACING)
            .width(Length::Fixed(326.0))
            .push(Space::new().width(Length::Fixed(BUTTON_GRID_LEFT_OFFSET)))
            .push(Self::number_button("1", CalculatorMessage::DigitPressed(1)))
            .push(Self::number_button("2", CalculatorMessage::DigitPressed(2)))
            .push(Self::number_button("3", CalculatorMessage::DigitPressed(3)))
            .push(Self::op_button("+", Operation::Add));

        let row0 = Row::new()
            .spacing(BUTTON_SPACING)
            .width(Length::Fixed(326.0))
            .push(Space::new().width(Length::Fixed(BUTTON_GRID_LEFT_OFFSET)))
            .push(Self::number_button("0", CalculatorMessage::DigitPressed(0)))
            .push(Self::number_button(".", CalculatorMessage::DecimalPressed))
            .push(Self::equals_button("="));

        Column::new()
            .spacing(BUTTON_SPACING)
            .push(clear_row)
            .push(row7)
            .push(row4)
            .push(row1)
            .push(row0)
    }

    /// Create a number button.
    fn number_button(label: &str, message: CalculatorMessage) -> Button<'_, CalculatorMessage> {
        Button::new(
            Text::new(label)
                .size(BUTTON_FONT_SIZE)
                .color(Color::from_rgb(1.0, 1.0, 1.0))
                .align_x(Alignment::Center),
        )
        .padding(BUTTON_PADDING)
        .width(Length::Fixed(BUTTON_WIDTH))
        .height(Length::Fixed(BUTTON_HEIGHT))
        .on_press(message)
    }

    /// Create an operation button.
    fn op_button(label: &str, op: Operation) -> Button<'_, CalculatorMessage> {
        Button::new(
            Text::new(label)
                .size(BUTTON_FONT_SIZE)
                .color(Color::from_rgb(1.0, 1.0, 1.0))
                .align_x(Alignment::Center),
        )
        .padding(BUTTON_PADDING)
        .width(Length::Fixed(BUTTON_WIDTH))
        .height(Length::Fixed(BUTTON_HEIGHT))
        .on_press(CalculatorMessage::OperationPressed(op))
    }

    /// Create an equals button.
    fn equals_button(label: &str) -> Button<'_, CalculatorMessage> {
        Button::new(
            Text::new(label)
                .size(BUTTON_FONT_SIZE)
                .color(Color::from_rgb(1.0, 1.0, 1.0))
                .align_x(Alignment::Center),
        )
        .padding(BUTTON_PADDING)
        .width(Length::Fixed(BUTTON_WIDTH))
        .height(Length::Fixed(BUTTON_HEIGHT))
        .on_press(CalculatorMessage::EqualsPressed)
    }
}

