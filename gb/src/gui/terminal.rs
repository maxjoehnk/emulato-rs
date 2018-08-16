use tui::backend::RawBackend;
use tui::Terminal;
use tui::widgets::*;
use tui::layout::*;
use tui::style::{Style, Color};
use std::io::Result;
use gameboy::GameBoy;
use cpu::register::Flags;

pub struct Interface {
    terminal: Terminal<RawBackend>
}

impl Interface {
    pub fn draw(&mut self, gb: &GameBoy, instructions: &Vec<String>) -> Result<()> {
        let size = self.terminal.size()?;

        Group::default()
            .direction(Direction::Horizontal)
            .sizes(&[Size::Min(24), Size::Fixed(74)])
            .render(&mut self.terminal, &size, |t, chunks| {
                build_instructions(t, &chunks[0], instructions);
                build_sidebar(t, &chunks[1], gb);
            });

        self.terminal.draw()
    }
}

fn build_instructions(terminal: &mut Terminal<RawBackend>, target: &Rect, instructions: &Vec<String>) {
    let instructions = instructions.iter()
        .rev()
        .take(target.height as usize)
        .map(|instruction| Item::Data(instruction));

    let block = Block::default()
        .title("Instructions")
        .borders(Borders::ALL);
    List::new(instructions)
        .block(block)
        .start_corner(Corner::BottomLeft)
        .render(terminal, target);
}

fn build_sidebar(terminal: &mut Terminal<RawBackend>, target: &Rect, gb: &GameBoy) {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Fixed(5), Size::Min(75)])
        .render(terminal, target, |t, chunks| {
            build_cpu(t, &chunks[0], gb);
            build_ram(t, &chunks[1], gb);
        })
}

fn build_cpu(terminal: &mut Terminal<RawBackend>, target: &Rect, gb: &GameBoy) {
    Group::default()
        .direction(Direction::Horizontal)
        .sizes(&[Size::Fixed(64), Size::Fixed(10)])
        .render(terminal, target, |t, chunks| {
            build_register(t, &chunks[0], gb);
            build_flags(t, &chunks[1], gb);
        })
}

fn build_register(terminal: &mut Terminal<RawBackend>, target: &Rect, gb: &GameBoy) {
    let header = ["A", "B", "C", "D", "E", "F", "H", "L", "SP", "PC"];
    let registers = [
        format!("0x{:x?}", gb.register.a),
        format!("0x{:x?}", gb.register.b),
        format!("0x{:x?}", gb.register.c),
        format!("0x{:x?}", gb.register.d),
        format!("0x{:x?}", gb.register.e),
        format!("0x{:x?}", gb.register.f.bits()),
        format!("0x{:x?}", gb.register.h),
        format!("0x{:x?}", gb.register.l),
        format!("0x{:x?}", gb.register.sp),
        format!("0x{:x?}", gb.register.pc),
    ];
    Table::new(
        header.into_iter(),
        vec![
            Row::Data(registers.into_iter())
        ].into_iter()
    )
        .block(Block::default().title("Register").borders(Borders::ALL))
        .column_spacing(2)
        .widths(&[4, 4, 4, 4, 4, 4, 4, 4, 6, 6])
        .style(Style::default().fg(Color::White))
        .render(terminal, target);
}

fn print_flag(gb: &GameBoy, flag: Flags) -> u8 {
    if gb.register.f.contains(flag) {
        1
    }else {
        0
    }
}

fn build_flags(terminal: &mut Terminal<RawBackend>, target: &Rect, gb: &GameBoy) {
    let header = ["Z", "H", "N", "C"];
    let registers = [
        format!("{}", print_flag(gb, Flags::Z)),
        format!("{}", print_flag(gb, Flags::H)),
        format!("{}", print_flag(gb, Flags::N)),
        format!("{}", print_flag(gb, Flags::C))
    ];
    Table::new(
        header.into_iter(),
        vec![
            Row::Data(registers.into_iter())
        ].into_iter()
    )
        .block(Block::default().title("Flags").borders(Borders::ALL))
        .column_spacing(1)
        .widths(&[1, 1, 1, 1])
        .style(Style::default().fg(Color::White))
        .render(terminal, target);
}

/* TODO: highlight program counter */
fn build_ram(terminal: &mut Terminal<RawBackend>, target: &Rect, gb: &GameBoy) {
    const COLS: usize = 16;
    let header = (0..COLS).into_iter().map(|i| format!("0{:X?}", i));
    let widths = [2; COLS];
    let ram = gb.ram
        .iter()
        .map(|d| format!("{:X?}", d))
        .collect::<Vec<String>>();
    let data = ram
        .chunks(COLS)
        .map(|data| Row::Data(data.into_iter()));
    Table::new(
        header,
        data
    )
        .block(Block::default().title("RAM").borders(Borders::ALL))
        .column_spacing(1)
        .widths(&widths)
        .style(Style::default().fg(Color::White))
        .render(terminal, target);
}

pub fn build() -> Result<Interface> {
    let backend = RawBackend::new()?;
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    Ok(Interface {
        terminal
    })
}
