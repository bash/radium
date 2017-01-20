module Radium
  abstract struct Action
    abstract def process(storage : Storage) : Message
  end
end