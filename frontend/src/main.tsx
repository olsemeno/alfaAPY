import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { IdentityKitProvider } from "@nfid/identitykit/react";
import { Provider } from "react-redux";
import { store } from "./store";
import App from "./App.tsx";
import { ToastContainer } from "react-toastify";
import { VAULT_CANISTER_ID } from "./constants";

import "@nfid/identitykit/react/styles.css";
import "react-toastify/dist/ReactToastify.css";
import "./index.css";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <ToastContainer />
    <Provider store={store}>
      <IdentityKitProvider
        signerClientOptions={{ targets: [VAULT_CANISTER_ID] }}
      >
        <App />
      </IdentityKitProvider>
    </Provider>
  </StrictMode>
);
