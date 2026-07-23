//! Fetches deployed bytecode from a live EVM JSON-RPC endpoint, for fuzzing
//! contracts that have no available source — the common case for a real
//! bug-bounty target, which may not be verified on any block explorer.
//!
//! UNVERIFIED IN THIS CHANGE: the actual network call (`fetch_bytecode`)
//! needs a real RPC endpoint and was written in a network-isolated
//! environment — same caveat as `backend.rs`. The response-parsing logic
//! (`parse_eth_get_code_response`) is pure and has its own tests using a
//! hand-written JSON-RPC response fixture, so at least the parsing is
//! proven correct independent of any live endpoint.

use serde::Deserialize;

#[derive(Deserialize)]
struct JsonRpcResponse {
    result: Option<String>,
    error: Option<JsonRpcError>,
}

#[derive(Deserialize)]
struct JsonRpcError {
    message: String,
}

/// Parses a JSON-RPC `eth_getCode` response body into raw bytecode bytes.
/// `"0x"` (an address with no code — an EOA, or nothing deployed there)
/// decodes to an empty `Vec`, which callers should treat as "nothing to
/// fuzz here" rather than an error.
pub fn parse_eth_get_code_response(body: &str) -> anyhow::Result<Vec<u8>> {
    let parsed: JsonRpcResponse = serde_json::from_str(body)
        .map_err(|e| anyhow::anyhow!("malformed JSON-RPC response: {e}"))?;

    if let Some(error) = parsed.error {
        anyhow::bail!("RPC endpoint returned an error: {}", error.message);
    }

    let result = parsed
        .result
        .ok_or_else(|| anyhow::anyhow!("RPC response had neither a result nor an error"))?;

    let trimmed = result.strip_prefix("0x").unwrap_or(&result);
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }
    hex::decode(trimmed).map_err(|e| anyhow::anyhow!("invalid hex in eth_getCode result: {e}"))
}

fn to_hex_address(address: [u8; 20]) -> String {
    format!("0x{}", hex::encode(address))
}

/// Fetches the deployed (runtime) bytecode at `address` from `rpc_url` via
/// a standard `eth_getCode` JSON-RPC call. Returns an empty `Vec` if the
/// address has no code deployed (an EOA, or an address nothing has ever
/// deployed to).
pub fn fetch_bytecode(rpc_url: &str, address: [u8; 20]) -> anyhow::Result<Vec<u8>> {
    let request_body = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "eth_getCode",
        "params": [to_hex_address(address), "latest"],
        "id": 1,
    });

    let response_body = ureq::post(rpc_url)
        .set("Content-Type", "application/json")
        .send_string(&request_body.to_string())
        .map_err(|e| anyhow::anyhow!("RPC request to {rpc_url} failed: {e}"))?
        .into_string()
        .map_err(|e| anyhow::anyhow!("failed to read RPC response body: {e}"))?;

    parse_eth_get_code_response(&response_body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_successful_response_with_bytecode() {
        let body = r#"{"jsonrpc":"2.0","id":1,"result":"0x6080604052"}"#;
        assert_eq!(
            parse_eth_get_code_response(body).unwrap(),
            vec![0x60, 0x80, 0x60, 0x40, 0x52]
        );
    }

    #[test]
    fn treats_bare_0x_as_no_code_deployed_here() {
        let body = r#"{"jsonrpc":"2.0","id":1,"result":"0x"}"#;
        assert_eq!(parse_eth_get_code_response(body).unwrap(), Vec::<u8>::new());
    }

    #[test]
    fn surfaces_rpc_error_messages_rather_than_failing_silently() {
        let body =
            r#"{"jsonrpc":"2.0","id":1,"error":{"code":-32602,"message":"invalid address"}}"#;
        let err = parse_eth_get_code_response(body).unwrap_err();
        assert!(err.to_string().contains("invalid address"));
    }

    #[test]
    fn rejects_malformed_json() {
        assert!(parse_eth_get_code_response("not json").is_err());
    }

    #[test]
    fn errors_when_response_has_neither_result_nor_error() {
        // A well-formed JSON object that is neither a success nor an error
        // must not be mistaken for "no code" — it is a protocol violation.
        let body = r#"{"jsonrpc":"2.0","id":1}"#;
        let err = parse_eth_get_code_response(body).unwrap_err();
        assert!(err.to_string().contains("neither a result nor an error"));
    }

    #[test]
    fn rejects_non_hex_in_the_result() {
        let body = r#"{"jsonrpc":"2.0","id":1,"result":"0xZZ"}"#;
        let err = parse_eth_get_code_response(body).unwrap_err();
        assert!(err.to_string().contains("invalid hex"));
    }

    #[test]
    fn hex_address_encoding_is_lowercase_and_prefixed() {
        let addr = [0xABu8; 20];
        assert_eq!(to_hex_address(addr), format!("0x{}", "ab".repeat(20)));
    }
}
