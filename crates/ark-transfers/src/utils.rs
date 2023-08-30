use std::env;

use ark_starknet::utils::get_contract_property_string;
use log::info;
use reqwest::Client as ReqwestClient;

pub async fn sanitize_uri(token_uri: &str) -> (String, String) {
    let mut request_uri = token_uri
        .trim()
        .replace(['\u{0003}', '\u{2}'], "")
        .replace("-https://", "https://");
    request_uri = convert_ipfs_uri_to_http_uri(request_uri);
    (request_uri.clone(), request_uri)
}

pub fn convert_ipfs_uri_to_http_uri(request_uri: String) -> String {
    if request_uri.contains("ipfs://") {
        let ipfs_gateway_uri = env::var("IPFS_GATEWAY_URI").expect("IPFS_GATEWAY_URI must be set");
        format!(
            "{}/ipfs/{}",
            ipfs_gateway_uri,
            request_uri.split("ipfs://").last().unwrap()
        )
    } else {
        request_uri
    }
}

pub async fn get_token_uri(
    client: &ReqwestClient,
    token_id_low: u128,
    token_id_high: u128,
    contract_address: &str,
    block_number: u64,
) -> String {
    info!("get_token_id: [{:?}, {:?}]", token_id_low, token_id_high);

    let token_id_low_hex = format!("{:x}", token_id_low);
    let token_id_high_hex = format!("{:x}", token_id_high);

    let token_uri_cairo0 = get_contract_property_string(
        client,
        contract_address,
        "tokenURI",
        vec![token_id_low_hex.clone(), token_id_high_hex.clone()],
        block_number,
    )
    .await;

    if token_uri_cairo0 != "undefined" && !token_uri_cairo0.is_empty() {
        return token_uri_cairo0;
    }

    let token_uri = get_contract_property_string(
        client,
        contract_address,
        "token_uri",
        vec![token_id_low_hex.clone(), token_id_high_hex.clone()],
        block_number,
    )
    .await;

    info!("token_uri: {:?}", token_uri);

    if token_uri != "undefined" && !token_uri.is_empty() {
        return token_uri;
    }

    "undefined".to_string()
}
