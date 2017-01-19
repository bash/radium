module Radium::Messages
  struct Add < Radium::Message
    property timestamp : Radium::Timestamp
    property flags : EntryFlags
    property data : Bytes

    def initialize(@timestamp : Timestamp, @flags : EntryFlags, @data : Bytes)
    end

    def to_io(io : IO, format : IO::ByteFormat)
      io.write_bytes(MessageType::Add, format)
      io.write_bytes(@timestamp, format)
      io.write_bytes(@flags, format)
      io.write_bytes(@data.size.to_u16, format)
      io.write(@data)
    end

    def self.from_io(io : IO, format : IO::ByteFormat) : self
      timestamp = io.read_bytes(UInt64, format)
      flags = io.read_bytes(UInt16, format)
      length = io.read_bytes(UInt16, format)
      
      data = Bytes.new(length.to_i32)
      io.read(data)

      self.new(timestamp, EntryFlags.from_raw(flags), data)      
    end
  end
end
