module Radium::Actions
  struct Push < Radium::Action
    def perform(backend : Backend)
      backend.each_subscriber do |io|
        io.write_bytes(Radium::Messages::Push.new, IO::ByteFormat::NetworkEndian)
      end
    end
  end
end
