module Radium
  class MessageParser

    def parse (type : MessageType, io : IO) : Message
      case type
        when .add?
          io.read_bytes(Messages::Add, IO::ByteFormat::NetworkEndian)
        else
          raise Exception.new("unable to parse message")
      end
    end
  end
end
