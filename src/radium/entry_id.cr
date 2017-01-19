module Radium
  struct EntryId
    include Comparable(EntryId)

    property timestamp : UInt64
    property counter : UInt16

    def initialize(@timestamp : UInt64, @counter : UInt16)
    end
  end
end
