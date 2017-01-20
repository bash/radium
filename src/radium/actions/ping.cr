module Radium::Actions
  struct Ping < Radium::Action
    def process(storage : Storage) : Radium::Message
      Radium::Messages::Pong.new
    end
  end
end