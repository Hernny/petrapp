import { Injectable, Logger } from '@nestjs/common';
import { PinataSDK } from 'pinata';

@Injectable()
export class StorageService {
  private readonly pinata: PinataSDK;
  private readonly logger = new Logger(StorageService.name);

  constructor() {
    this.pinata = new PinataSDK({
      pinataJwt: process.env.PINATA_JWT || '',
      pinataGateway: process.env.PINATA_GATEWAY || '',
    });
  }

  async uploadJson(jsonData: Record<string, any>) {
    try {
      const response = await this.pinata.upload.json(jsonData);
      return response;
    } catch (error) {
      this.logger.error('Failed to upload JSON to Pinata', error);
      throw error;
    }
  }

  async uploadFile(file: File) {
    try {
      const upload = await this.pinata.upload.file(file);
      return upload;
    } catch (error) {
      this.logger.error('Failed to upload file to Pinata', error);
      throw error;
    }
  }
}
