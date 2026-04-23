import { Injectable, Logger } from '@nestjs/common';
import { Connection, PublicKey, Keypair } from '@solana/web3.js';
import * as anchor from '@coral-xyz/anchor';

@Injectable()
export class BlockchainService {
  private readonly logger = new Logger(BlockchainService.name);
  private connection: Connection;
  private provider: anchor.AnchorProvider;

  constructor() {
    const rpcUrl = process.env.SOLANA_RPC_URL || 'https://api.devnet.solana.com';
    this.connection = new Connection(rpcUrl, 'confirmed');
    
    // Configurar una wallet local para que el backend pueda subsidiar transacciones (Platform Expense)
    // En produccion puede ser un Keypair cargado desde AWS Secrets o ENV
    const dummyWallet = new anchor.Wallet(Keypair.generate());
    this.provider = new anchor.AnchorProvider(this.connection, dummyWallet, { commitment: 'confirmed' });
    anchor.setProvider(this.provider);
  }

  getProvider() {
    return this.provider;
  }

  getConnection() {
    return this.connection;
  }

  // TODO: Integrar metodos reales contra el IDL del workspace de Anchor
  // async mintTicket(...) {}
}
