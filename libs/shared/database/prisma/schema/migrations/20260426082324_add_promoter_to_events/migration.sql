/*
  Warnings:

  - Added the required column `promoter_id` to the `events` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "events" ADD COLUMN     "promoter_id" UUID NOT NULL;

-- AddForeignKey
ALTER TABLE "events" ADD CONSTRAINT "events_promoter_id_fkey" FOREIGN KEY ("promoter_id") REFERENCES "promoters"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
