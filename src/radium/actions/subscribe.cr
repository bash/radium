module Radium::Actions
  struct Subscribe < Radium::Action
    def initialize(@io : TCPSocket)
    end

    def perform(backend : Backend)
      backend.subscribe(@io)

      @io.write_bytes(Radium::Messages::Ok.new, IO::ByteFormat::NetworkEndian)
    end
  end
end
