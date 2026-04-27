import { Injectable, UnauthorizedException } from '@nestjs/common';
import { PassportStrategy } from '@nestjs/passport';
import { ExtractJwt, Strategy } from 'passport-jwt';
import { passportJwtSecret } from 'jwks-rsa';
import { PrismaService } from '@shared/database';

@Injectable()
export class JwtStrategy extends PassportStrategy(Strategy) {
  constructor(private prisma: PrismaService) {
    // Inyectamos Prisma
    const jwksUri = process.env['DYNAMIC_JWKS_URL'];
    const environmentId = process.env['DYNAMIC_PROJECT_ENVIRONMENT_ID'];

    if (!jwksUri || !environmentId) {
      throw new Error('Faltan variables de entorno de Dynamic en el .env');
    }

    super({
      secretOrKeyProvider: passportJwtSecret({
        cache: true,
        rateLimit: true,
        jwksRequestsPerMinute: 5,
        jwksUri: jwksUri,
      }),
      jwtFromRequest: ExtractJwt.fromAuthHeaderAsBearerToken(),
      issuer: `https://dynamic.xyz{environmentId}`,
      algorithms: ['RS256'],
    });
  }

  async validate(payload: any) {
    // 1. Buscamos el usuario e "hidratamos" sus relaciones
    const user = await this.prisma.user.findUnique({
      where: { dynamicId: payload.sub },
      include: {
        userType: true,
        promoter: true,
        roles: {
          include: {
            role: {
              include: {
                rolePermissions: {
                  include: { permission: true },
                },
              },
            },
          },
        },
      },
    });

    // 2. Si el usuario no existe en tu DB, lanzamos excepción
    if (!user || user.deletedAt) {
      throw new UnauthorizedException('Usuario no registrado o inactivo');
    }

    // 3. Aplanamos los permisos de todos sus roles (sin duplicados)
    interface Permission {
      name: string;
    }

    interface RolePermission {
      permission: Permission;
    }

    interface Role {
      rolePermissions: RolePermission[];
    }

    interface UserRole {
      role: Role;
    }

    const permissions: string[] = [
      ...new Set(
        user.roles.flatMap((ur: UserRole) =>
          ur.role.rolePermissions.map(
            (rp: RolePermission) => rp.permission.name
          )
        ) as string[]
      ),
    ];

    // 4. Este objeto es el que estará disponible en req.user
    return {
      id: user.id,
      dynamicId: user.dynamicId,
      email: user.email,
      type: user.userType.name,
      permissions: permissions,
      promoter: user.promoter
        ? { id: user.promoter.id, name: user.promoter.name }
        : null,
    };
  }
}
