export const idlFactory = ({ IDL }) => {
    const Conf = IDL.Record({ 'controllers' : IDL.Vec(IDL.Principal) });
    return IDL.Service({ 'get_config' : IDL.Func([], [Conf], ['query']) });
};
export const init = ({ IDL }) => {
    const Conf = IDL.Record({ 'controllers' : IDL.Vec(IDL.Principal) });
    return [IDL.Opt(Conf)];
};
