export type PaymentType = "Rebalance" | "Withdrawal" | "Deposit";

export type Payment = {
  id: number;
  amount: string;
  date: string;
  from: string;
  to: string;
  type: PaymentType;
  token: string;
};

export const paymentsService = {
  getPayments: async (
    filter: {
      user?: string;
      type?: PaymentType;
      from?: string;
      to?: string;
    } = {}
  ): Promise<Array<Payment>> => {
    const mockPayments: Array<Payment> = [
      {
        id: 1,
        type: "Rebalance",
        amount: "1,000",
        token: "ICP/CHAT",
        date: "2024-06-01",
        from: "IcpSwap pool #1",
        to: "KongSwap pool #2",
      },
      {
        id: 2,
        type: "Withdrawal",
        amount: "500",
        token: "ICP",
        date: "2024-05-28",
        from: "KongSwap pool #2",
        to: "address",
      },
      {
        id: 3,
        type: "Deposit",
        amount: "2,000",
        token: "ICP",
        date: "2024-05-20",
        from: "address",
        to: "KongSwap pool #1",
      },
    ];
    return new Promise((resolve) => {
      setTimeout(() => {
        resolve(
          mockPayments.filter((payment) =>
            Object.entries(filter).every(
              ([key, value]) => payment[key as keyof typeof payment] === value
            )
          )
        );
      }, 1500);
    });
  },
};
