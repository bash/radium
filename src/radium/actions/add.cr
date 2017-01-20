module Radium::Actions
  struct Add < Radium::Action
    property message : Radium::Messages::Add

    def initialize (@message : Radium::Messages::Add)
    end

    def process(storage : Storage) : Radium::Message
      # todo implement
      @message
    end
  end
end