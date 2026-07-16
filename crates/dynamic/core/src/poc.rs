//! Renders a [`Violation`] as a human-readable proof-of-concept — the thing
//! a bug-bounty submission actually needs: a minimal, ordered list of calls
//! that reproduces the broken invariant, not just "detector X fired".

use crate::sequence::Violation;

pub fn format_poc(violation: &Violation) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Invariant violated: {}\n{}\n\nReproduction ({} call{}):\n",
        violation.invariant_name,
        violation.message,
        violation.sequence.len(),
        if violation.sequence.len() == 1 { "" } else { "s" }
    ));
    for (i, call) in violation.sequence.0.iter().enumerate() {
        let args_hex = if call.calldata.len() > 4 {
            hex_string(&call.calldata[4..])
        } else {
            String::new()
        };
        out.push_str(&format!(
            "  {}. {}({}){}  [caller=0x{}]\n",
            i + 1,
            call.function.name,
            args_hex,
            if call.value > 0 {
                format!(" value={}", call.value)
            } else {
                String::new()
            },
            hex_string(&call.caller)
        ));
    }
    out.push_str(&format!(
        "\nFailing step: #{} ({})\n",
        violation.failing_step + 1,
        violation.sequence.0[violation.failing_step].function.name
    ));
    out
}

fn hex_string(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::{EncodedCall, FunctionSpec};
    use crate::sequence::CallSequence;

    #[test]
    fn renders_call_names_and_failing_step() {
        let f = FunctionSpec::new("withdraw", [0x01, 0x02, 0x03, 0x04], vec![], true);
        let call = EncodedCall {
            function: f,
            calldata: vec![0x01, 0x02, 0x03, 0x04],
            caller: [0xAB; 20],
            value: 0,
        };
        let violation = Violation {
            invariant_name: "conservation".to_string(),
            message: "sum(balanceOf) != totalSupply()".to_string(),
            failing_step: 0,
            sequence: CallSequence(vec![call]),
        };
        let poc = format_poc(&violation);
        assert!(poc.contains("withdraw"));
        assert!(poc.contains("conservation"));
        assert!(poc.contains("Failing step: #1"));
    }
}
