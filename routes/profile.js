import Document from '../layouts/document.js'
import Profile from '../layouts/profile.js'

const handler = context => {
  const { req, res } = context

  const host = req.headers.host
  const [username] = host.split('.')

  return (
    <HttpResponse
      res={res}
      status={200}
      headers={{ 'Content-Type': 'text/html' }}
    >
      <Document>
        <Profile username={username} />
      </Document>
    </HttpResponse>
  )
}

export { handler }
