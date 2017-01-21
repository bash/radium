module Radium::Actions
  struct Close < Radium::Action
    def initialize (@io : TCPSocket)
    end

    def perform(backend : Backend)
      @io.close

      backend.unsubscribe @io
    end
  end
end
