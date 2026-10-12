use anyhow::{Context, Result};
use iced_x86::{Decoder, DecoderOptions, Formatter, Instruction, IntelFormatter};

use crate::hex::HexBytes;
use crate::image::DllImage;
use crate::rva::Rva;
use crate::symbols::Symbols;

/// Prints the instructions from `before` bytes ahead of `target` to `after` bytes past it,
/// marking the one at `target`. Calls to named functions are labeled with the name.
///
/// x86 instructions vary in length, so decoding has to start on an instruction: the start
/// of the named function containing `target`, or of its exception-table entry when there are
/// no names. The exception table can split a function into several entries, so the
/// named start is preferred.
pub fn print_around(
    image: &DllImage,
    symbols: Option<&Symbols>,
    target: Rva,
    before: u32,
    after: u32,
) -> Result<()> {
    let start = symbols
        .and_then(|symbols| symbols.containing(target))
        .map(|symbol| symbol.start)
        .or_else(|| image.function_at(target).map(|function| function.start))
        .with_context(|| format!("no known function contains {target}"))?;
    let first_shown = Rva(target.0.saturating_sub(before)).max(start);
    let end = target + after;

    let code = image
        .bytes_at(start, (end - start) as usize)
        .context("the code isn't in the file")?;
    let mut decoder = Decoder::with_ip(64, code, start.0.into(), DecoderOptions::NONE);
    let mut formatter = IntelFormatter::new();
    formatter.options_mut().set_hex_prefix("0x");
    formatter.options_mut().set_hex_suffix("");

    let mut text = String::new();
    for instruction in decoder.iter() {
        let ip = Rva(instruction.ip32());
        if ip < first_shown {
            continue;
        }
        text.clear();
        formatter.format(&instruction, &mut text);
        if let Some(name) = branch_target_name(&instruction, symbols) {
            text.push_str("  ; ");
            text.push_str(name);
        }

        let is_target = (ip..ip + instruction.len() as u32).contains(&target);
        let marker = if is_target { "=>" } else { "  " };
        let bytes = image.bytes_at(ip, instruction.len()).unwrap_or_default();
        println!("{marker} {ip:>10}  {:<30} {text}", HexBytes(bytes));
    }
    Ok(())
}

/// The name of the function a `call` or `jmp` goes to, if it goes to the start of one.
fn branch_target_name<'a>(
    instruction: &Instruction,
    symbols: Option<&'a Symbols>,
) -> Option<&'a str> {
    let target = u32::try_from(instruction.near_branch_target()).ok()?;
    symbols?
        .starting_at(Rva(target))
        .map(|symbol| symbol.name.as_str())
}
