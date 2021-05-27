const { spawn } = require('child_process')

const s = './target/debug/words'

const child = spawn(s, ["get"])

child.stdout.on('data', data => {
  console.log(`stdout: ${data}`)
  //child.stdin.setEncoding('utf-8');
  //child.stdout.pipe(process.stdout);
  //child.stdin.write("1;2;3\n");
  child.stdin.write("y\n");
})

child.stderr.on('data', data => {
  console.log(`stderr: ${data}`)
})

child.on('error', (error) => {
  console.log(`error: ${error.message}`)
})

child.on('close', code => {
  console.log(`child process exited with code ${code}`)
})

child.on('message', message => {
  console.log('>>>>>>>>', message)
})

