module Push
  class Socket
    def initialize (@io : TCPSocket)
    end
    
    def write_msg_type (msg_type : MessageType)
      @io.write_bytes(msg_type.to_u64, IO::ByteFormat::NetworkEndian)
    end

    def read_msg_type : MessageType?
      msg_type = @io.read_bytes(Int64, IO::ByteFormat::NetworkEndian)
      
      MessageType.from_value?(msg_type)
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
