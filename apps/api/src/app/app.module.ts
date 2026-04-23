import { Module } from '@nestjs/common';
import { ConfigModule } from '@nestjs/config';

import { AppController } from './app.controller';
import { AppService } from './app.service';
import { AuthModule } from './auth/auth.module';
import { UsersModule } from './users/users.module';
import { EventsModule } from './events/events.module';
import { TicketsModule } from './tickets/tickets.module';
import { PromotersModule } from './promoters/promoters.module';
import { FinanceModule } from './finance/finance.module';
import { AffiliatesModule } from './affiliates/affiliates.module';
import { BlockchainModule } from './blockchain/blockchain.module';
import { StorageModule } from './storage/storage.module';
import { AuthModule } from './auth/auth.module';

@Module({
  imports: [
    ConfigModule.forRoot({
      isGlobal: true,
    }),
    AuthModule,
    UsersModule,
    EventsModule,
    TicketsModule,
    PromotersModule,
    FinanceModule,
    AffiliatesModule,
    BlockchainModule,
    StorageModule,
  ],
  controllers: [AppController],
  providers: [AppService],
})
export class AppModule {}
