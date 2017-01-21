module Radium
  abstract struct Action
    abstract def perform(storage : Storage) : Message
  end
end
