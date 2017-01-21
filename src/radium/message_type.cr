module Radium
  enum MessageType: UInt16
    Ping
    Pong
    Close
    Add
    Remove
    Has
    Subscribe
    Push
    Ok
    Error

    def to_io(io : IO, format : IO::ByteFormat)
      io.write_bytes(self.value, format)
    end

    def self.from_io(io : IO, format : IO::ByteFormat) : self
      self.from_value(io.read_bytes(UInt16, format))
    end
  end
end
