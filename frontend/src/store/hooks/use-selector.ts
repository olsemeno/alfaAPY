import {
  TypedUseSelectorHook,
  useSelector as useReduxSelector,
} from "react-redux";
import { RootState } from "../store"; // Import RootState type

// Create a typed useSelector hook
export const useSelector: TypedUseSelectorHook<RootState> = useReduxSelector;
