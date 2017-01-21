module Radium
  class Server

    class Stop
      INSTANCE = new
    end

    def initialize(@channel : EventLoop::ProcessorChannel)
    end

    def run
      server = TCPServer.new "localhost", 3126

      while true 
        if client = server.accept?
          spawn handle_connection(client)
        else
          break
        end
      end
    end
    
    def handle_connection(io : TCPSocket)
      # todo: make configurable
      io.tcp_keepalive_count = 2
      io.tcp_keepalive_idle = 5
      
      while true
        handle_message(io)
        
        break if io.closed?
      end

    rescue
      close io
    end

    def close (io : TCPSocket)
      @channel.send(Actions::Close.new(io))
    end

    def handle_message (io : TCPSocket)
      message = Message.parse(io, IO::ByteFormat::NetworkEndian)

      # todo: doesn't belong here
      action = 
        case message
          when Messages::Add
            Actions::Add.new(message, io)
          when Messages::Ping
            Actions::Ping.new(io)
          when Messages::Close
            Actions::Close.new(io)
          when Messages::Subscribe
            Actions::Subscribe.new(io)
        end
    
      unless action
        # todo: error
        return
      end

      @channel.send(action)
    end
  end
end
