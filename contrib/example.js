const net = require('net')

const UInt16 = (value) => {
  const buf = Buffer.alloc(2)

  buf.writeUInt16BE(value, 0)

  return buf
}

const UInt8 = (value) => {
  const buf = Buffer.alloc(1)

  buf.writeUInt8(value, 0)

  return buf
}

const ConnectionMode = Object.freeze({ Action: 0, Listen: 1 })

class Ping {
  write (socket) {
    socket.write(UInt16(0))
  }
}

class Close {
  write (socket) {
    socket.write(UInt16(2))
  }
}

class Radium {
  constructor (mode, host = '127.0.0.1', port = 3126) {
    this._client = new net.Socket()
    this._onConnected = new Promise((resolve, reject) => {
      this._client.connect(port, host, () => {
        // todo: error handling

        this._client.write(UInt8(mode))

        resolve()
      })
    })
  }

  onConnected () {
    return this._onConnected
  }

  send (action) {
    action.write(this._client)
  }

  action (action) {
    return new Promise((resolve) => {
      // todo: parse response types and parse response
      this._client.once('data', (data) => {
        resolve(data.readUInt16BE(0))
      })

      this.send(action)
    })
  }
}

const radium = new Radium(ConnectionMode.Action)

radium.onConnected()
  .then(() => {
    console.log('connected')
    return radium.action(new Ping())
  })
  .then((resp) => {
    console.log('Received', resp)

    radium.send(new Close())
  })