module Radium::Actions
  struct Add < Radium::Action
    property message : Radium::Messages::Add

    def initialize (@message : Radium::Messages::Add, @io : IO)
    end

    def perform(backend : Backend)
      # todo implement
      @io.write_bytes(@message, IO::ByteFormat::NetworkEndian)
    end
  end
end
