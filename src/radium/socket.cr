module Radium
  # Todo: rename
  class Socket
    property io : TCPSocket

    def initialize (@io : TCPSocket)
    end

    def send_message(message : Message)
      @io.write_bytes(message, IO::ByteFormat::NetworkEndian)
    end

    def receive_message : Message
      Message.parse(@io, IO::ByteFormat::NetworkEndian)
    end

    def subscribe
      request Messages::Subscribe.new

      while true
        yield receive_message
      end
    end

    def request (message : Message) : Message
      send_message message
      receive_message
    end
  end
end
