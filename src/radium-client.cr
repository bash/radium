require "socket"
require "./radium/*"
require "./radium/messages/*"

module Radium
    tcp = TCPSocket.new("localhost", 3126)
    socket = Socket.new(tcp)
    
    begin
      puts socket.request MessageType::Ping
      puts socket.request MessageType::Ping

      # [flags: uint16] [ts: uint64] [length: uint64] [data: ...]

      add = Messages::Add.new(Time.now.epoch.to_u64, EntryFlags::None, "foo".to_slice)

      tcp.write_bytes(add, IO::ByteFormat::NetworkEndian)

      puts socket.request(MessageType::Ping)

    ensure
      socket.write_msg_type MessageType::Close
    end
end


