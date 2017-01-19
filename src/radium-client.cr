require "socket"
require "./radium/*"

module Radium
    tcp = TCPSocket.new("localhost", 3126)
    socket = Socket.new(tcp)
    
    begin
      puts socket.request MessageType::PING
    ensure
      socket.write_msg_type MessageType::CLOSE
    end
end


