function timeout(ms: number) {
    return new Promise((resolve) => setTimeout(resolve, ms));
}

async function succeed() {
    await timeout(5000);
    return 'Success';
}

async function fail() {
    await timeout(5000);
    return false;
}

export { succeed, fail };
