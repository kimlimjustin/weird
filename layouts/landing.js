import { Form as CreateAccountForm } from '../actions/create-account.js'
import { LandingBanner } from '../elements/landing-banner.js'
import { Form as CreateUsernameForm } from '../actions/create-username.js'

export const Landing = () => (
  <>
    <main>
      <CreateUsernameForm />

      <h1>Weird.inc index</h1>
      <p>Welcome to the index page</p>
    </main>

    {css``}
  </>
)
