module Radium
  @[Flags]
  enum EntryFlags : UInt16
    RandomClient

    def self.from_raw (value : UInt16) : self
      if value == 0_u16
        EntryFlags::None
      else
        EntryFlags.from_value(value)
      end
    end

    def to_io(io : IO, format : IO::ByteFormat)
      io.write_bytes(self.value, format)
    end

    def self.from_io(io : IO, format : IO::ByteFormat) : self
      self.from_raw(io.read_bytes(UInt16, format))
    end
  end
end
