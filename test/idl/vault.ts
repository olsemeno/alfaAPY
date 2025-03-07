import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Conf { 'controllers' : Array<Principal> }
export interface _SERVICE { 'get_config' : ActorMethod<[], Conf> }
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
