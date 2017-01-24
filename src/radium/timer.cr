module Radium
  class Timer
    def initialize(seconds : Number, &block)
      @channel = Channel(Nil).new(1)

      spawn do
        sleep seconds
        next if cancelled?
        block.call
      end
    end

    def cancelled?
      !@channel.empty?
    end

    def cancel
      return if cancelled?

      @channel.send(nil)
    end
  end
end
