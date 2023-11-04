use rsrpc::RPCServer;

pub fn start_rpc_server() {
  let detectable = reqwest::blocking::get(
    "https://gist.githubusercontent.com/SpikeHD/209bd9b17c97f45dc5be4803c748726f/raw/ddf8ed33621933b4e3c58cf1113e1679ab9fd9b5/dorion_detectable.json",
  )
  .unwrap()
  .text()
  .unwrap();

  // This accepts both a `&str` or a `String`
  let mut server = RPCServer::from_json_str(detectable);

  server.process_scan_ms = Some(25);

  server.start();
}
