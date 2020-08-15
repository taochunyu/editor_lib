const VIEW_DESC_NAME_ON_DMO = "🤘";
const MUTATION_OBSERVER_OPTIONS = {
    childList: true,
    characterData: true,
    characterDataOldValue: true,
    attributes: true,
    attributesOldValue: true,
    subtree: true,
};

const MUTATION_OBSERVER = 0;

const collectMutations = mutation_state => processedMutations => {
    const pendingMutations = mutation_state[MUTATION_OBSERVER]?.takeRecords();
    const mutations = pendingMutations.length > 0
        ? [...processedMutations, ...pendingMutations]
        : processedMutations;

    console.log(mutations);
    handleMutations(mutation_state)(mutations);
}

const handleMutations = mutation_state => mutations => {
    mutations.forEach(handleMutation(mutation_state))
}

const handleMutation = mutation_state => mutation => {
    const desc_id = nearestDesc(mutation.target);

    switch (mutation.type) {
        case "childList": break;
        case "attributes": break;
        case "characterData": break;
    }
}

const mutation_input = node => {
    const mutation_state = [];

    mutation_state[MUTATION_OBSERVER] = new MutationObserver(collectMutations(mutation_state));

    return [
        () => mutation_state[MUTATION_OBSERVER]?.observe(node, MUTATION_OBSERVER_OPTIONS),
        () => mutation_state[MUTATION_OBSERVER]?.disconnect(),
    ];
}

const initInput = node => {
    const [start, stop] = mutation_input(node);

    start();
}

const $editor = document.querySelector("#editor");

initInput($editor);

const nearestDesc = node => {
    for (let cursor = node; cursor; cursor = cursor.parentNode) {
        let desc_id = node[VIEW_DESC_NAME_ON_DMO] || null;

        if (desc_id) {
            return desc_id;
        }
    }

    return null;
}
