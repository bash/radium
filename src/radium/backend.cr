module Radium
  class Backend
    def initialize
      @subscribers = Set(TCPSocket).new
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
  end
end
