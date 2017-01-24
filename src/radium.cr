require "socket"
require "signal"
require "./radium/*"
require "./radium/actions/*"
require "./radium/event_loop/*"
require "./radium/messages/*"

module Radium
  channel = EventLoop::ProcessorChannel.new
  processor = EventLoop::Processor.new channel, Backend.new
  server = Server.new channel

  processor.run
  server.run
end
