import * as dna from 'dnaviz';

import("../pkg/index.js").then(module => {
    function randomSeq(length) {
        var result = '';
        var characters = 'ATGC';
        var charactersLength = characters.length;
        for (var i = 0; i < length; i++) {
            result += characters.charAt(Math.floor(Math.random() * charactersLength));
        }
        return result;
    }

    let seq = randomSeq(1000000);
    let j = 0
    while (j < 20) {
        let then = new Date();
        module.squiggle(seq);
        let now = new Date();
        console.log((now.getTime() - then.getTime()) / 1000 + ' wasm-rust');
        j++;
    }

    j = 0
    while (j < 20) {
        let then = new Date();
        dna.squiggle(seq);
        let now = new Date();
        console.log((now.getTime() - then.getTime()) / 1000 + ' typescript');
        j++;
    }
})
