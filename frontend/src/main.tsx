import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { IdentityKitProvider } from "@nfid/identitykit/react";
import { Provider } from "react-redux";
import { store } from "./store";
import "./index.css";
import "@nfid/identitykit/react/styles.css";
import App from "./App.tsx";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <Provider store={store}>
      <IdentityKitProvider
        signerClientOptions={{ targets: ["ownab-uaaaa-aaaap-qp2na-cai"] }}
      >
        <App />
      </IdentityKitProvider>
    </Provider>
  </StrictMode>
);
