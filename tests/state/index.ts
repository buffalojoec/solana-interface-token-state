
export class JoeTokenV1 {

    mint_authority: COption<PublicKey>
    supply: BN
    decimals: number
    is_initialized: boolean
    freeze_authority: COption<PublicKey>
    slot_created: BN
    largest_mint: BN
    organization: string

    constructor(props: {
        mint_authority: COption<PublicKey>
        supply: BN
        decimals: number
        is_initialized: boolean
        freeze_authority: COption<PublicKey>
        slot_created: BN
        largest_mint: BN
        organization: string
    }) {
        this.mint_authority = props.mint_authority
        this.supply = props.supply
        this.decimals = props.decimals
        this.is_initialized = props.is_initialized
        this.freeze_authority = props.freeze_authority
        this.slot_created = props.slot_created
        this.largest_mint = props.largest_mint
        this.organization = props.organization
    }

    toBuffer() { 
        return Buffer.from(borsh.serialize(JoeTokenV1Schema, this)) 
    };
    
    static fromBuffer(buffer: Buffer) {
        return borsh.deserialize(JoeTokenV1Schema, JoeTokenV1, buffer);
    };

}

const JoeTokenV1Schema = new Map([
    [ JoeTokenV1, { 
        kind: 'struct', 
        fields: [ 
            ['mint_authority', 'COption<PublicKey>'],
            ['supply', 'BN'],
            ['decimals', 'number'],
            ['is_initialized', 'boolean'],
            ['freeze_authority', 'COption<PublicKey>'],
            ['slot_created', 'BN'],
            ['largest_mint', 'BN'],
            ['organization', 'string'],
        ],
    }]
]);

export class JoeTokenV2 {

    mint_authority: COption<PublicKey>
    supply: BN
    decimals: number
    is_initialized: boolean
    freeze_authority: COption<PublicKey>
    slot_created: BN
    largest_mint: BN
    organization: string

    constructor(props: {
        mint_authority: COption<PublicKey>
        supply: BN
        decimals: number
        is_initialized: boolean
        freeze_authority: COption<PublicKey>
        slot_created: BN
        largest_mint: BN
        organization: string
    }) {
        this.mint_authority = props.mint_authority
        this.supply = props.supply
        this.decimals = props.decimals
        this.is_initialized = props.is_initialized
        this.freeze_authority = props.freeze_authority
        this.slot_created = props.slot_created
        this.largest_mint = props.largest_mint
        this.organization = props.organization
    }

    toBuffer() { 
        return Buffer.from(borsh.serialize(JoeTokenV2Schema, this)) 
    };
    
    static fromBuffer(buffer: Buffer) {
        return borsh.deserialize(JoeTokenV2Schema, JoeTokenV2, buffer);
    };

}

const JoeTokenV2Schema = new Map([
    [ JoeTokenV2, { 
        kind: 'struct', 
        fields: [ 
            ['mint_authority', 'COption<PublicKey>'],
            ['supply', 'BN'],
            ['decimals', 'number'],
            ['is_initialized', 'boolean'],
            ['freeze_authority', 'COption<PublicKey>'],
            ['slot_created', 'BN'],
            ['largest_mint', 'BN'],
            ['organization', 'string'],
        ],
    }]
]);

export class JoeTokenV3 {

    mint_authority: COption<PublicKey>
    supply: BN
    decimals: number
    is_initialized: boolean
    freeze_authority: COption<PublicKey>
    title: string
    symbol: string
    uri: string
    update_authority: COption<PublicKey>
    slot_created: BN
    largest_mint: BN
    organization: string

    constructor(props: {
        mint_authority: COption<PublicKey>
        supply: BN
        decimals: number
        is_initialized: boolean
        freeze_authority: COption<PublicKey>
        title: string
        symbol: string
        uri: string
        update_authority: COption<PublicKey>
        slot_created: BN
        largest_mint: BN
        organization: string
    }) {
        this.mint_authority = props.mint_authority
        this.supply = props.supply
        this.decimals = props.decimals
        this.is_initialized = props.is_initialized
        this.freeze_authority = props.freeze_authority
        this.title = props.title
        this.symbol = props.symbol
        this.uri = props.uri
        this.update_authority = props.update_authority
        this.slot_created = props.slot_created
        this.largest_mint = props.largest_mint
        this.organization = props.organization
    }

    toBuffer() { 
        return Buffer.from(borsh.serialize(JoeTokenV3Schema, this)) 
    };
    
    static fromBuffer(buffer: Buffer) {
        return borsh.deserialize(JoeTokenV3Schema, JoeTokenV3, buffer);
    };

}

const JoeTokenV3Schema = new Map([
    [ JoeTokenV1, { 
        kind: 'struct', 
        fields: [ 
            ['mint_authority', 'COption<PublicKey>'],
            ['supply', 'BN'],
            ['decimals', 'number'],
            ['is_initialized', 'boolean'],
            ['freeze_authority', 'COption<PublicKey>'],
            ['title', 'string'],
            ['symbol', 'string'],
            ['uri', 'string'],
            ['update_authority', 'COption<PublicKey>'],
            ['slot_created', 'BN'],
            ['largest_mint', 'BN'],
            ['organization', 'string'],
        ],
    }]
]);
