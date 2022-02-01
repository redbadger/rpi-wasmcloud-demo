metadata package = [ { namespace: "com.redbadger.interfaces.oled", crate: "interface" } ]

namespace com.redbadger.interfaces.oled

use org.wasmcloud.model#wasmbus

@wasmbus(
    contractId: "redbadger:oled",
    providerReceive: true )

service Oled {
  version: "0.1",
  operations: [ Update, Clear ]
}

operation Update { 
  input: Request
}

operation Clear { 
}

structure Request {
  @required
  text: String
}
