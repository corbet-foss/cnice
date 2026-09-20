// This module also runs in a real browser with no Node shims.
export function verify(api) {
    const check = (actual, expected) => {
        if (JSON.stringify(actual) !== JSON.stringify(expected)) {
            throw new Error(`${JSON.stringify(actual)} !== ${JSON.stringify(expected)}`);
        }
    };
    check(api.greet.salutation('de-ch', 'Frau Dr. Müller'), 'Sehr geehrte Frau Dr. Müller');
    check(api.greet.salutation('de', 'Frau Dr. Müller'), 'Sehr geehrte Frau Dr. Müller,');
    check(api.greet.salutation('rm', 'signur Schmid'), 'Stimà signur Schmid,');
    check(api.greet.isSupported('de-ch'), true);
    check(api.greet.isSupported('xx'), false);
    check(api.farewell.closing('de-ch'), 'Freundliche Grüsse');
    check(api.farewell.closing('rm'), 'Cordials salids');
    check(api.farewell.closing('xx', ''), '');
}
