module Radium::EventLoop
  alias ProcessorChannel = Channel::Unbuffered({Radium::Action, Channel::Unbuffered(Radium::Message)})

  class Processor

    def initialize(@channel : ProcessorChannel, @storage : Storage)
    end

    def run
      spawn do
        loop do 
          action, respond = @channel.receive
          
          respond.send action.process(@storage)
        end
      end
    end
  end
end