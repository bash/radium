module Radium
  class MessageParser

    def parse (type : MessageType, io : IO) : Message
      message_class =
        case type
          when .add?
            Messages::Add
          when .ping?
            Messages::Ping
        end
        
      unless message_class
        raise Exception.new("unable to parse message")
      end
          
      io.read_bytes(message_class, IO::ByteFormat::NetworkEndian)
    end
  end
end
