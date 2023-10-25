interface FTReturn {
    account: string | null;
    error: string | null;
}

declare function createAccountAndClaim(
    privkey: string,
    accountId: string,
    pinCode: number,
    _network: string
): FTReturn;

export { createAccountAndClaim };
