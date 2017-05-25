const net = require('net')
const enableWatchMode = (process.argv[2] === 'w')

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

const WatchMode = Object.freeze({ None: 0, Watching: 1 })

class Ping {
  write (socket) {
    socket.write(UInt8(0))
  }
}

class SetWatchMode {
  constructor (mode) {
    this._mode = mode
  }

  write (socket) {
    socket.write(UInt8(7))
    socket.write(UInt8(this._mode))
  }
}

class Radium {
  constructor (host = '127.0.0.1', port = 3126) {
    this._client = new net.Socket()
    this._onConnected = new Promise((resolve, reject) => {
      this._client.connect(port, host, resolve)
    })
  }

  close () {
    this._client.end()
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
        resolve(data.readUInt8(0))
      })

      this.send(action)
    })
  }
}

const radium = new Radium()

radium._client.on('data', (data) => {
  console.info('received data chunk', data)
})

radium.onConnected()
  .then(() => {
    return Promise.all([
      radium.action(new Ping()),
      radium.action(new Ping())
    ])
  })
  .then((resp) => {
    console.log('Received', resp)

    if (enableWatchMode) {
      return radium.action(new SetWatchMode(WatchMode.Watching))
    }
  })
  .then(() => {
    if (!enableWatchMode) {
      radium.close()
    }
  })