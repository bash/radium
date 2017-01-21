module Radium::EventLoop
  alias ProcessorChannel = Channel::Unbuffered(Radium::Action)

  class Processor

    def initialize(@channel : ProcessorChannel, @backend : Backend)
    end

    def run
      spawn do
        while true
          if action = @channel.receive?
            action.perform(@backend)
          end
        end
      end
    end
  end
end
