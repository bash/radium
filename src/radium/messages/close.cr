module Radium::Messages
  struct Close < Radium::Message
    def to_io(io : IO, format : IO::ByteFormat)
      io.write_bytes(MessageType::Close, format)
    end

    def self.from_io(io : IO, format : IO::ByteFormat) : self
      self.new
    end
  end
end
