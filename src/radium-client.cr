require "socket"
require "./radium/*"
require "./radium/messages/*"

module Radium
    tcp = TCPSocket.new("localhost", 3126)
    socket = Socket.new(tcp)
    
    puts socket.request Messages::Ping.new
    puts socket.request Messages::Ping.new

    add = Messages::Add.new(Time.now.epoch.to_u64, EntryFlags::RandomClient, "foo".to_slice)

    puts socket.request add
    puts socket.request Messages::Ping.new
    
    socket.subscribe do |message|
      puts message
    end
end


