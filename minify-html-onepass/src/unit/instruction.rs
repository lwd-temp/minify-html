use crate::err::ProcessingResult;
use crate::proc::MatchAction::*;
use crate::proc::MatchMode::*;
use crate::proc::Processor;
use aho_corasick::AhoCorasick;
use once_cell::sync::Lazy;

static INSTRUCTION_END: Lazy<AhoCorasick> = Lazy::new(|| AhoCorasick::new(["?>"]));

#[inline(always)]
pub fn process_instruction(proc: &mut Processor) -> ProcessingResult<()> {
  proc.m(IsSeq(b"<?"), Keep).expect();
  proc
    .m(ThroughSeq(&INSTRUCTION_END), Keep)
    .require("instruction end")?;
  Ok(())
}
