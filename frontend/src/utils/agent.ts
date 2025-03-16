import { HttpAgent } from "@dfinity/agent";

export const agent = HttpAgent.createSync({ host: "https://ic0.app" });
