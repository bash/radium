module Radium
  class Socket
    property io : TCPSocket

    def initialize (@io : TCPSocket)
    end
    
    def write_msg_type (type : MessageType)
      @io.write_bytes(type, IO::ByteFormat::NetworkEndian)
    end

    def read_msg_type : MessageType
      @io.read_bytes(MessageType, IO::ByteFormat::NetworkEndian)
    end

    def request (msg_type : MessageType) : MessageType?
      write_msg_type msg_type
      read_msg_type
    end

    def close
      @io.close
    end
  end
end
