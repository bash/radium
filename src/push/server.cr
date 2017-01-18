module Push
  class Server

    class Stop
      INSTANCE = new
    end

    def initialize
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
      io.tcp_keepalive_count = 2
      io.tcp_keepalive_idle = 5

      socket = Socket.new(io)
      
      loop do        
        if handle_message(socket, socket.read_msg_type)
          break
        end
      end
    end

    def handle_message (socket : Socket, msg_type : MessageType?) : Stop?

      puts "#{msg_type}"

      unless msg_type
        socket.write_msg_type(MessageType::ERROR)
        return
      end

      if msg_type.close?
        socket.close
        return Stop::INSTANCE
      end

      if msg_type.ping?
        socket.write_msg_type(MessageType::PONG)
        return
      end

    end
  end
end
