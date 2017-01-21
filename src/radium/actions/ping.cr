module Radium::Actions
  struct Ping < Radium::Action
    def initialize(@io : IO)
    end

    def perform(backend : Backend)
      @io.write_bytes(Radium::Messages::Pong.new, IO::ByteFormat::NetworkEndian)
    end
  end
end
