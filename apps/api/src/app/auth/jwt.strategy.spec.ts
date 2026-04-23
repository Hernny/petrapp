import { Test, TestingModule } from '@nestjs/testing';
import { ConfigService } from '@nestjs/config';
import { JwtStrategy } from './jwt.strategy';
import { UnauthorizedException } from '@nestjs/common';

describe('JwtStrategy', () => {
  let strategy: JwtStrategy;

  const mockConfigService = {
    get: jest.fn().mockImplementation((key: string) => {
      switch (key) {
        case 'DYNAMIC_JWKS_URL':
          return 'https://app.dynamic.xyz/api/v0/sdk/test-env-id/.well-known/jwks';
        case 'DYNAMIC_ENVIRONMENT_ID':
          return 'test-env-id';
        default:
          return null;
      }
    }),
  };

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      providers: [
        JwtStrategy,
        {
          provide: ConfigService,
          useValue: mockConfigService,
        },
      ],
    }).compile();

    strategy = module.get<JwtStrategy>(JwtStrategy);
  });

  it('should be defined', () => {
    expect(strategy).toBeDefined();
  });

  it('should throw Error if DYNAMIC_JWKS_URL is undefined on initialization', () => {
    const brokenConfigService = {
      get: jest.fn().mockReturnValue(undefined), // Simulating missing env
    };

    expect(() => new JwtStrategy(brokenConfigService as any)).toThrow(
      'DYNAMIC_JWKS_URL environment variable is not defined'
    );
  });

  describe('validate', () => {
    it('should return mapped user properties if payload is valid', async () => {
      const payload = {
        sub: 'dyn_123456789',
        email: 'test@petrapp.com',
        wallet_public_key: 'solana-wallet-address',
      };

      const result = await strategy.validate(payload);
      expect(result).toEqual({
        dynamicId: 'dyn_123456789',
        email: 'test@petrapp.com',
        wallet: 'solana-wallet-address',
      });
    });

    it('should throw an UnauthorizedException if payload has no sub (dynamicId)', async () => {
      const invalidPayload = {
        email: 'test@petrapp.com', // Missing sub
      };

      await expect(strategy.validate(invalidPayload)).rejects.toThrow(
        UnauthorizedException
      );
    });
  });
});
