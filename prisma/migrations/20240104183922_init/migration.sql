-- CreateTable
CREATE TABLE "Manufacturer" (
    "uuid" TEXT NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL
);

-- CreateTable
CREATE TABLE "Aircraft" (
    "uuid" TEXT NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "identifier" TEXT NOT NULL,
    "manufacturer_uuid" TEXT NOT NULL,
    "performance_uuid" TEXT,
    CONSTRAINT "Aircraft_manufacturer_uuid_fkey" FOREIGN KEY ("manufacturer_uuid") REFERENCES "Manufacturer" ("uuid") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "Aircraft_performance_uuid_fkey" FOREIGN KEY ("performance_uuid") REFERENCES "Performance" ("uuid") ON DELETE SET NULL ON UPDATE CASCADE
);

-- CreateTable
CREATE TABLE "Performance" (
    "uuid" TEXT NOT NULL PRIMARY KEY
);

-- CreateIndex
CREATE UNIQUE INDEX "Aircraft_performance_uuid_key" ON "Aircraft"("performance_uuid");
