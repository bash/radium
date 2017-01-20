module Radium
  class Server

    class Stop
      INSTANCE = new
    end

    def initialize(@channel : EventLoop::ProcessorChannel)
      @parser = MessageParser.new
    end

    def run
      server = TCPServer.new "localhost", 3126

      loop do 
        if client = server.accept?
          handle_connection(client)
        else
          break
        end
      end
    end
    
    def handle_connection(io : TCPSocket)
      # todo: make configurable
      io.tcp_keepalive_count = 2
      io.tcp_keepalive_idle = 5
      
      loop do
        if handle_message(io).is_a?(Stop)
          break
        end
      end
    end

    def handle_message (io : TCPSocket)
      type = io.read_bytes(MessageType, IO::ByteFormat::NetworkEndian)

      if type.close?
        io.close
        return Stop::INSTANCE
      end

      # todo: move parsing of message type to MessageParser
      message = @parser.parse(type, io)
      respond = Channel(Message).new

      puts message

      # todo: move to message handler
      action = 
        case message
          when Messages::Add
            Actions::Add.new(message)
          when Messages::Ping
            Actions::Ping.new
        end
    
      unless action
        return
      end

      @channel.send({action, respond})
      
      io.write_bytes(respond.receive, IO::ByteFormat::NetworkEndian)
    end
  end
end
