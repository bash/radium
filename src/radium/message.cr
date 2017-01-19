module Radium
  abstract struct Message
    abstract def to_io(io : IO, format : IO::ByteFormat = IO::ByteFormat::SystemEndian)
  end
end
