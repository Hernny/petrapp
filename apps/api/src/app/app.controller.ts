import { Controller, Get, UseGuards } from '@nestjs/common';
import { AppService } from './app.service';
import { DynamicAuthGuard } from './auth/dynamic-auth.guard';
import { CurrentUser } from './auth/current-user.decorator';

@Controller()
export class AppController {
  constructor(private readonly appService: AppService) {}

  @Get()
  getData() {
    return this.appService.getData();
  }

  @Get('test-auth')
  @UseGuards(DynamicAuthGuard)
  testAuth(@CurrentUser() user: any) {
    return { message: 'Authenticated successfully with Dynamic JWT', user };
  }
}
