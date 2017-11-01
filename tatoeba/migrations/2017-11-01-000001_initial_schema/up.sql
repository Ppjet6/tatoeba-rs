--
-- Table structure for table `sentences`
--
-- Sentence graph vertices.

CREATE TABLE `sentences` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `lang` varchar(4) DEFAULT NULL,
  `text` varbinary(1500) NOT NULL,
  `correctness` tinyint(2) NOT NULL DEFAULT '0',
  `user_id` int(11) DEFAULT NULL,
  `created` datetime DEFAULT NULL,
  `modified` datetime DEFAULT NULL,
  `dico_id` int(11) DEFAULT NULL,
  `hasaudio` enum('no','from_users','shtooka') NOT NULL DEFAULT 'no',
  `script` varchar(4) DEFAULT NULL,
  `hash` binary(16) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `user_id` (`user_id`),
  KEY `dico_id` (`dico_id`),
  KEY `lang` (`lang`),
  KEY `hasaudio_idx` (`hasaudio`),
  KEY `modified_idx` (`modified`),
  KEY `hash` (`hash`),
  KEY `created` (`created`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

--
-- Table structure for table `sentences_translations`
--
-- Sentence graph edges

CREATE TABLE `sentences_translations` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `sentence_id` int(11) NOT NULL,
  `translation_id` int(11) NOT NULL,
  `sentence_lang` varchar(4) DEFAULT NULL,
  `translation_lang` varchar(4) DEFAULT NULL,
  `distance` smallint(2) NOT NULL DEFAULT '1',
  PRIMARY KEY (`id`),
  UNIQUE KEY `sentence_id` (`sentence_id`,`translation_id`),
  KEY `translation_id` (`translation_id`),
  KEY `sentence_lang` (`sentence_lang`),
  KEY `translation_lang` (`translation_lang`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;
