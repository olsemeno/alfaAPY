import * as Agent from "@dfinity/agent"
import { Identity, SubmitResponse } from "@dfinity/agent"
import "@dfinity/identity"
import { Principal } from "@dfinity/principal"

export const ic = {
  host: "https://ic0.app",
}

////////////
// Agent //
//////////

/** Agent which retries all failed calls in order to mitigate "certified state unavailable" and "service overload" 5XX errors. */
export class AgentWithRetry extends Agent.HttpAgent {
  RETRY_LIMIT = 5
  override call(
    canisterId: Principal | string,
    options: {
      methodName: string
      arg: ArrayBuffer
      effectiveCanisterId?: Principal | string
    },
    identity?: Identity | Promise<Identity>,
    attempt = 1,
  ) {
    try {
      return super.call(canisterId, options, identity)
    } catch (e: unknown) {
      if (attempt < this.RETRY_LIMIT) {
        console.warn(
          `Failed to fetch "${options.methodName}" from "${canisterId}" (attempt #${attempt})`,
          e,
        )
        return new Promise<SubmitResponse>((res) => {
          setTimeout(
            () => res(this.call(canisterId, options, identity, attempt + 1)),
            1000 * attempt,
          )
        })
      }
      console.error(`Failed to fetch after ${attempt} attempts`)
      throw e
    }
  }
}

//TODO fix deprecated
/** We share the same agent across all actors, and replace the identity when identity connection events occur. */
export const agent = new AgentWithRetry({ host: ic.host })

/**
 * Retrieve the current principal.
 */
export async function fetchPrincipal() {
  const principal = await agent.getPrincipal()
  return principal
}