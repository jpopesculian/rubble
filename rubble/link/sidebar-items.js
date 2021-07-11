initSidebarItems({"constant":[["CRC_POLY","The CRC polynomial to use for CRC24 generation."],["MIN_DATA_PAYLOAD_BUF","Min. size a data PDU payload buffer must have (assuming only the bare minimum PDU size is supported)."],["MIN_DATA_PDU_BUF","Min. size a data PDU buffer must have."],["MIN_PACKET_BUF","Min. size a buffer for Link-Layer packets must have to comply with the spec."],["MIN_PAYLOAD_BUF","Min. size a PDU payload buffer must have (to cover both advertising and data channels)."],["MIN_PDU_BUF","Min. size a Link-Layer PDU buffer must have (to cover both advertising and data channels)."]],"enum":[["AddressKind","Specifies whether a device address is randomly generated or a LAN MAC address."],["NextUpdate","Specifies when the Link Layer’s `update` method should be called the next time."],["RadioCmd","Specifies if and how the radio should listen for transmissions."],["State","Link-Layer state machine, according to the Bluetooth spec."]],"mod":[["ad_structure","Advertising Data / Extended Inquiry Response (EIR) data."],["advertising","Advertising channel operations."],["channel_map",""],["comp_id","Company identifiers."],["connection","Link-Layer connection management and LLCP implementation."],["data","Data Channel structures."],["device_address",""],["features",""],["filter","Link-Layer Device Filtering."],["llcp","Defines packet structures used by the Link Layer Control Protocol."],["queue","An SPSC queue for data channel PDUs."],["responder",""],["seq_num",""]],"struct":[["Cmd","Command returned by the Link-Layer to the user."],["CompanyId","Company identifier for use in link layer Control PDUs."],["Connection","Connection state and parameters."],["DeviceAddress","A Bluetooth device address."],["FeatureSet","A set of optional Link Layer features."],["LinkLayer","Implementation of the real-time BLE Link-Layer logic."],["Responder","Data channel packet processor."]],"trait":[["Transmitter","Trait for Link Layer packet transmission."]]});