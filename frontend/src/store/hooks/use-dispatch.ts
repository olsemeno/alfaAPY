import { useDispatch as useReduxDispatch } from "react-redux";
import { Dispatch } from "../store"; // Import the AppDispatch type

// Create a typed useDispatch hook
export const useDispatch = () => useReduxDispatch<Dispatch>();
