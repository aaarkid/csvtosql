CREATE TABLE `RECORDS` (
	`NID` CHAR(10) NOT NULL,
	`Name` VARCHAR NOT NULL,
	`Birthday` CHAR(10),
	`Phone` CHAR(14),
	`NIPT` CHAR(10),
	`DRT` CHAR(20),
	`Wage` INT unsigned,
	`Job` VARCHAR,
	`Subject` VARCHAR,
	PRIMARY KEY (`NID`)
);