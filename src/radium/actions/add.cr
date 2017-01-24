module Radium::Actions
  struct Add < Radium::Action
    property message : Radium::Messages::Add

    def initialize (@message : Radium::Messages::Add, @io : IO)
    end

    def perform(backend : Backend)
      id = EntryId.new(@message.timestamp, 0_u16)
      entry = Entry.new(id, @message.flags, @message.data)
      
      # todo: check if in the future and push with delay
      timer = Timer.new (@message.timestamp - Time.now.epoch) do
        Actions::Push.new.perform(backend)
      end

      # todo send 'Queued' message
      @io.write_bytes(@message, IO::ByteFormat::NetworkEndian)
    end
  end
end
