import { accountExists, claim, generateKeys, getPubFromSecret, initKeypom } from '@keypom/core';

async function createAccountAndClaim(privKey, accountId, pinCode, _network) {
    const network = _network ? _network : 'testnet';
    const accountSuffix = network === 'testnet' ? '.testnet' : '.near';
    const newAccountId = accountId + accountSuffix;
    
    const refractionKey = process.env.REACT_APP_REFRACTION_KEY || "master-key";
    await initKeypom({
        network
    });

    let {publicKeys, secretKeys} = await generateKeys({
        numKeys: 1,
        rootEntropy: `${pinCode}${refractionKey}`,
    })
    console.log('secretKeys: ', secretKeys)

    const doesAccountExist = await accountExists(newAccountId);
    console.log('doesAccountExist: ', doesAccountExist)
    if (doesAccountExist) {
        console.log(doesAccountExist)
        return {
            error: 'Account exists',
            account: null,
        }
    } else {
        console.log('claiming account')
        await claim({
            secretKey: privKey,
            newAccountId,
            newPublicKey: publicKeys[0],
        })
    }

    
    // Generate the auto import link for the new account
    let baseUrl = `${network === "testnet" ? `https://testnet.mynearwallet.com` : `https://app.mynearwallet.com`}`
    const walletAutoImportLink = `${baseUrl}/auto-import-secret-key#${newAccountId}/${secretKeys[0]}`;
    console.log('walletAutoImportLink: ', walletAutoImportLink);
    // return walletAutoImportLink;
    return {
        error: null,
        account: walletAutoImportLink,
    };
}
export { createAccountAndClaim };
