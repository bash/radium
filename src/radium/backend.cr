module Radium
  class Backend
    def initialize
      @subscribers = Set(TCPSocket).new
      @timers = Hash(EntryId, Timer).new
    end

    def subscribe(client : TCPSocket)
      @subscribers.add client
    end

    def unsubscribe(client : TCPSocket)
      @subscribers.delete client
    end

    def each_subscriber(&block : TCPSocket -> _)
      @subscribers.each &block
    end

    def add_timer(id : EntryId, timer : Timer)
      @timers.set(id, timer)
    end

    def stop_timer(id : EntryId)
      if timer = @timers.delete(id)
        timer.cancel
      end
    end
  end
end
