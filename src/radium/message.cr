module Radium
  abstract struct Message
    TYPE_MAP = {
      MessageType::Add => Messages::Add,
      MessageType::Close => Messages::Close,
      MessageType::Ok => Messages::Ok,
      MessageType::Ping => Messages::Ping,
      MessageType::Pong => Messages::Pong,
      MessageType::Push => Messages::Push,
      MessageType::Subscribe => Messages::Subscribe,
    }

    abstract def to_io(io : IO, format : IO::ByteFormat)

    def self.parse(io : IO, format : IO::ByteFormat) : self
      type = io.read_bytes(MessageType, format)
      message_class = TYPE_MAP[type]

      unless message_class
        raise Exception.new("unable to parse message")
      end

      io.read_bytes(message_class, IO::ByteFormat::NetworkEndian)
    end
  end
end
