metadata package = [
  {
    namespace: "com.redbadger.oled_ssd_1306",
    crate: "oled_ssd1306_interface"
  }
]

namespace com.redbadger.oled_ssd_1306

use org.wasmcloud.model#wasmbus

@wasmbus(
  contractId: "red-badger:oled-ssd1306",
  actorReceive: true,
  providerReceive: true
)
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
