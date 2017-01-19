module Radium
  class Server

    class Stop
      INSTANCE = new
    end

    def initialize
      @parser = MessageParser.new
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

    def handle_message (io : TCPSocket) : Stop?
      type = io.read_bytes(MessageType, IO::ByteFormat::NetworkEndian)

      puts "#{type}"

      if type.close?
        io.close
        return Stop::INSTANCE
      end

      if type.ping?
        io.write_bytes(MessageType::Pong, IO::ByteFormat::NetworkEndian)
        return
      end

      # todo: move parsing of message type to MessageParser
      message = @parser.parse(type, io)

      puts message

    end
  end
end
