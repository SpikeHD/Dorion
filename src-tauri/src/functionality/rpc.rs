use rsrpc::RPCServer;

pub fn start_rpc_server() {
  let detectable = reqwest::blocking::get("https://raw.githubusercontent.com/OpenAsar/arrpc/main/src/process/detectable.json").unwrap().text().unwrap();

  // This accepts both a `&str` or a `String`
  let mut server = RPCServer::from_json_str(detectable);

  server.process_scan_ms = Some(100);

  server.start();
}