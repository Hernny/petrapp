import { Test, TestingModule } from '@nestjs/testing';
import { ConfigService } from '@nestjs/config';
import { DynamicAuthGuard } from './dynamic-auth.guard';
import { ExecutionContext, UnauthorizedException } from '@nestjs/common';

describe('DynamicAuthGuard', () => {
  let guard: DynamicAuthGuard;

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      providers: [DynamicAuthGuard],
    }).compile();

    guard = module.get<DynamicAuthGuard>(DynamicAuthGuard);
  });

  it('should be defined', () => {
    expect(guard).toBeDefined();
  });

  describe('handleRequest', () => {
    it('should throw an UnauthorizedException if an error is present', () => {
      const mockError = new Error('Test Error');
      expect(() => guard.handleRequest(mockError, null, null)).toThrow('Test Error');
    });

    it('should throw an UnauthorizedException if user is falsy', () => {
      expect(() => guard.handleRequest(null, null, null)).toThrow(UnauthorizedException);
      expect(() => guard.handleRequest(null, null, null)).toThrow('Invalid or expired Dynamic JWT token');
    });

    it('should return the user object if validation succeeds', () => {
      const mockUser = { dynamicId: '123' };
      expect(guard.handleRequest(null, mockUser, null)).toEqual(mockUser);
    });
  });
});