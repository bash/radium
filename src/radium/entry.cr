module Radium
  struct Entry
    property id : EntryId
    property flags : EntryFlags
    property data : Bytes

    def initialize(@id : EntryId, @flags : EntryFlags, @data : Bytes)
    end
  end
end
