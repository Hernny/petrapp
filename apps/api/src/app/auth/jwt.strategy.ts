import { Injectable, UnauthorizedException } from '@nestjs/common';
import { PassportStrategy } from '@nestjs/passport';
import { ExtractJwt, Strategy } from 'passport-jwt';
import { passportJwtSecret } from 'jwks-rsa';
import { ConfigService } from '@nestjs/config';

@Injectable()
export class JwtStrategy extends PassportStrategy(Strategy) {
  constructor(configService: ConfigService) {
    const jwksUrl = configService.get<string>('DYNAMIC_JWKS_URL');

    if (!jwksUrl) {
      throw new Error('DYNAMIC_JWKS_URL environment variable is not defined');
    }

    super({
      secretOrKeyProvider: passportJwtSecret({
        cache: true,
        rateLimit: true,
        jwksRequestsPerMinute: 5,
        jwksUri: jwksUrl,
      }),
      jwtFromRequest: ExtractJwt.fromAuthHeaderAsBearerToken(),
      audience: configService.get<string>('DYNAMIC_ENVIRONMENT_ID'),
      issuer: `https://app.dynamic.xyz/`,
      algorithms: ['RS256'],
    });
  }

  async validate(payload: any) {
    if (!payload.sub) {
      throw new UnauthorizedException();
    }

    // The payload.sub corresponds to the dynamic_id of the user
    return {
      dynamicId: payload.sub,
      email: payload.email,
      wallet: payload.wallet_public_key,
      // You can return anything you want from the JWT payload here
      // Later it will be appended to request.user
    };
  }
}
