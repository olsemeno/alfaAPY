import { Principal } from "@dfinity/principal";
import { getAnonActor } from "../utils";
import { VAULT_CANISTER_ID } from "../../constants";
import { _SERVICE as VaultType, UserEvent, SystemEvent } from "../../idl/vault";
import { idlFactory } from "../../idl/vault_idl";

export type EventRecordType = "Rebalance" | "Withdrawal" | "Deposit";

export type EventRecord = {
  id: number;
  amount: string;
  date: string;
  from: string;
  to: string;
  type: EventRecordType;
  token: string;
  userPrincipal?: Principal;
  error?: [];
  fee?: string;
};

export class EventRecordsService {
  async getEventRecords(
    filter: {
      user?: string;
      type?: EventRecordType;
      from?: string;
      to?: string;
    } = {}
  ): Promise<Array<EventRecord>> {
    const anonymousActor = await getAnonActor<VaultType>(
      VAULT_CANISTER_ID,
      idlFactory
    );
    const records = await Promise.all([
      anonymousActor.get_system_events(BigInt(0), BigInt(100)),
      anonymousActor.get_user_events(
        Principal.anonymous(),
        BigInt(0),
        BigInt(100)
      ),
    ]).then((arr) => arr.flat());
    // Map raw events to EventRecord shape
    const mappedRecords: EventRecord[] = records.map((event: UserEvent | SystemEvent, i: number) => {
      return {
        id: i + 1,
        amount: 'amount' in event && event.amount !== undefined ? String(event.amount) : "0",
        date: 'date' in event && event.date !== undefined ? String(event.date) : "",
        from: 'from' in event && typeof event.from === 'string' ? event.from : "",
        to: 'to' in event && typeof event.to === 'string' ? event.to : "",
        type: 'type' in event && typeof event.type === 'string' ? event.type as EventRecordType : "Deposit",
        token: 'token' in event && typeof event.token === 'string' ? event.token : "",
        userPrincipal: 'userPrincipal' in event && event.userPrincipal !== undefined ? event.userPrincipal as Principal : undefined,
        error: 'error' in event && event.error !== undefined ? event.error as [] : undefined,
        fee: 'fee' in event && event.fee !== undefined ? event.fee as string : undefined,
      };
    });
    return mappedRecords.filter((EventRecord) =>
      Object.entries(filter).every(
        ([key, value]) => EventRecord[key as keyof EventRecord] === value
      )
    );
  }
}

export const eventRecordsService = new EventRecordsService();
