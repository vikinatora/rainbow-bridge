#[cfg(test)]
mod tests_unlock {
    use crate::EthProver;
    use hex::{FromHex};
    use near_sdk::PromiseOrValue;
    use serde::{Deserialize, Deserializer};

    #[derive(Debug)]
    struct Hex(pub Vec<u8>);

    impl<'de> Deserialize<'de> for Hex {
        fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
        where
            D: Deserializer<'de>,
        {
            let mut s = <String as Deserialize>::deserialize(deserializer)?;
            if s.starts_with("0x") {
                s = s[2..].to_string();
            }
            if s.len() % 2 == 1 {
                s.insert_str(0, "0");
            }
            Ok(Hex(Vec::from_hex(&s).map_err(|err| {
                serde::de::Error::custom(err.to_string())
            })?))
        }
    }

    // TESTS

    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>) -> VMContext {
        VMContext {
            current_account_id: "alice.near".parse().unwrap(),
            signer_account_id: "bob.near".parse().unwrap(),
            signer_account_pk: "ed25519:6E8sCci9badyRkXb3JoRpBj5p8C6Tw41ELDZoiihKEtp"
                .parse()
                .unwrap(),
            predecessor_account_id: "carol.near".parse().unwrap(),
            input,
            block_index: 0,
            block_timestamp: 0,
            epoch_height: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: near_sdk::Gas(10u64.pow(18)),
            random_seed: vec![1; 32].try_into().unwrap(),
            view_config: None,
            output_data_receivers: vec![],
        }
    }
   
        #[test]
        pub fn test_storage_proof() {
            testing_env!(get_context(vec![]));

            let expected_value = "01";
            let expected_root = "9dc8b927bc1f203931c70cc3850246046859c40e0044964753b28ff41285b75d"; //storage hash
            let key = "1c8ba9af7041ec3098c4d818db9972f67827520c1db7d022f6c3041b6f40ecc3"; //keccak256 of key of storage proof
            let proof_rlp = vec!["f90211a0786e2b7a3a24cfafe31c1cac9b0cc4b57cfb9e27264b9aeb8ba52d48181fd013a0bbf8c0df9d3837792180786e217cbcbfb6cb6c1ee790ec0dba83fc05248f2627a060d5477cb7243063a66f6533e342632aea890e13cf45c0144ad7f8254bb74242a02dd5604620fcf3990474b172d1bb5653bf7bfc9ee1d269e765da3f7a91ac2ec3a04134b666e4a8e3e28701e33bed36680ae025cc5f89f8d29457975148338017c4a07bf7ed874ff02f5938267592e1a3cfca0ca8d7d4b6fccd617b815fad456c2c53a076515bac0f65924da357aba2f6c312472a9ef94d1c459f2cd0380b84a91d46aca0f49dd3a32c4adb7b6f02188b32a7823c79bb5b2ed1f3cdf4da11c36304ae4e25a05ba06e342a112365ced395b4bf830d148ad1e6a787912a9a603293a4e079e3f5a029f967b2d7ed040cda92ab4905f2976636a7bb16ba850679eca942e02d5b3458a024bdf35e1d8fd46cdee76420d508c722d7f47825356620c266094f6c66c67f7ea070a8abf4345f7148bbcbd0f443fe00cfed0a17ab828b7051cb3ede4bee4499b1a00d9cd9aa39cdd4a91bcd4feb8c73db017dce78ec82ba6f14287775567615f247a02f3d28876d69fd8945b3fdeff87186bcb97c236ccefa3b6182507bc6e504606fa03b4078e843b8927c29753dfc91a44c246cb22ea3673a3bad8dab7c8f6b8ce226a061983eaffadbdc7d6e7a0f9d9f5c1c64f983a4a6724aaa2126a0b6c03c984f3280",
                                 "f901d1a0269a0b881e99b091fd1f85a9e44ee4daf1bfaa127412213b0ec1edacf2fcb4e0a0e7f129286b917093dd713ebf19c11b699dd3b7e2c8dd81b96da000d8929e6bd7a042e1e974e7e7b3829b6fd86c08ec48c92789a4c5b29aaba295ac47eb78da8979a0af69958bb871e61c3dab996738ae596fd6d8b0c6dd126361053ca23c67576aa6a0957f511c654ba45de24daa58d5069b8e854e1af4230f5e00ff80942f32179f9280a002cca7c62c8aaf39f3300a3694c5df5909d2365d68935dead512cec79b859546a0c150cdf6bf843bf02c321ad13d4b680a5ac6816a93b607b68b3d2944645cc573a0d44663526783865e11155d5490966374eaf24421c59522f6350769715d83f5f280a028d3420f6f3cfd2eff10d769032ec92a9ce6a26004a4b5e432577f7d218cb8eea095fcf2b12e8b70bb0c075d41576e00242769a18622ab1d936326fa053aeaa501a0ecf0ef75da3fd6cd5bc5a73049bbbdedd2fb0ccdb86bb07895d012b1b93354d5a02c8f51d330045fd36258abd5dc39dbc9ac14103ca91925bed4f55066f7d51830a024fff2a5c88286da53434ffeafc0962a9c2230a26c303850a04853096882be92a0a0eb992284319d09507645086aaf48f97428294e89aab782574993ed0543caef80",
                                 "f851808080808080a05a173679dbc21d5c5b16c40e4d0fdd5ec92f602c2db2adbe71c9d4b2aedb585480a04cd868f531723c2438dce2df71b16e2d4d6f49299867484a662073b40aaac5ca8080808080808080",
                                 "e19f3ba9af7041ec3098c4d818db9972f67827520c1db7d022f6c3041b6f40ecc301"
                                ];

            let expected_root = hex::decode(expected_root).unwrap().into();
            let key = hex::decode(key).unwrap();
            let proof = proof_rlp
                .into_iter()
                .map(|x| hex::decode(x).unwrap())
                .collect();
            let expected_value = hex::decode(expected_value).unwrap();

            assert_eq!(
                EthProver::verify_trie_proof(expected_root, key, proof),
                expected_value
            );
        }

        #[test]
        pub fn test_unlock_proof() {
            testing_env!(get_context(vec![]));
            let contract = EthProver::init("ethbridge".to_string());

            let header_data = "f9021ba0695d799c7fbeda2651dd907991909e4ae68612851ce5398f2efc9506e69247cda01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d4934794b64a30399f7f6b0c154c2e7af0a3ec7b0a5b131aa0eea9136af6ad8b65ad4c2184bbe6ab400f1ae214ed069d56d89639d7329d083ea063bbd1ab5c4e3922601b38d701e091e1ad3bc30a8e6e6a166728a6b5c9a61d31a0108126d36fb961af5e2d8359a6fced486e368965bbcba286205983dd06011aceb901000120011010a05000008a4800841200c0042918c020a44820008210109a02018c028d000000004001009a014c1085c000430019108410580008041204143226407a2001200470804449a0620802440121001124020cc07a040318018480a146110002528216204104220200806080086020c018a02000120020204818800000504604c1200000202100004003a188820624281c212102000901010d6051ec081002001a0000c8041010806460121600222104002188072024090001c60020088d2060c8ca500a1224161090040c82620c014060040608011048a1401010803004ba159d5800002802000300c241100296888004086a0030c015003b0212003004308380ffb58401c9c38083f281638463e31ac89f496c6c756d696e61746520446d6f63726174697a6520447374726962757465a086a600953aa2865ceb8c7a6348f388aaad93ca30c4a7dc718479ab45d98d8ea488000000000000000011";
            let account_data = "f8440180a09dc8b927bc1f203931c70cc3850246046859c40e0044964753b28ff41285b75da0932cddc50793da935ccf915651ad67f6b746e9936fcc5614f0ff492563782c75";
            let key = "487bcece3757038b7fea6585ca2d0a3dacd72d84bf0c7b916f169cea0348681b"; // contract add
            let proof_rlp = vec!["f90211a0a08073a06d519f053d34fbc01d7b52b5deff8bca1ecce1c6f5b64549215a1faca04354caea0b1a81283d78f0d2f876f8ef9232d3eff9c9266edf485d39a0ca9deaa01109339731fb837d1593f6fd40e9bcf9bf8b3869fc1f98c5e9fb91c19f6be941a0a699fab3b6374669ae93411b7f6f066b1a041a2f7456cac84ff29a93c6632da7a081c0eae435200744e1c85ff9918c7240790fd0d3f399cff2f90cde9d94e65fa4a04a04579f3a8b75d085a0f5a93858bf22ca3128232b69a3303e6342fea230f87fa0243bcb398a13ca1167b73593e895dd34ee0cebe89e9e553968155ab756b9b4caa022b916d96ea298160f55466ad1016b2ea60c9c60173cd193f82f0113e451280fa06f632d62fcffd685eee2357bd714772cea7cdc77f12454ad82a95f8de9059a16a0a16e576d1ab0bfbde3206d7efe42ac02b9d571421a69c91c698acf2afdc7e9e0a0392f3c5d59d3ed96044e32f152c76ce3eef74677d08df74fecbb163ca62da46ca0d84050065aa6746b033d129952fb8a3e1d92a24ceec830f44d0345e6bcb4e11fa098bfe8fd1931b9945e33ed558eab4764185294e5e8862df1709b6172ce52107aa036b54d7eb3e330033fc001bd65a07858eb98b3422bd30668da866256e3cc5381a04471a857d2967a0a1f37702d7fb19e983e46523affd393a87e6ee0b28e1dfe46a0c735f88df737eaeebb1156765174ab718dbea4e72f905767dd8cd38c19583eea80",
                                 "f90211a0c08abe73e8834debc4a228b4e0a5b8784a4163295ac6e991c057743a03911bf7a043770020cdf744075d88d216bc196b1ca01c2e67a98ed89535117f844e3f23efa04751dd21338bc05d356942b97ed568b2c3914af8cdb9e07ca608a94baa7695e7a0099b11c93b8021809fe5cc59fa5604623a0a8fe2abdbd48a559acb1b22a50e50a000fa6e28df7c9c8526b90f2494b645348bc52ccdb63765332623769e4015e967a08636c0d061574bf8d72bd229c0a6c4739f6e32d80557810096d28f9f877cfa49a0e7ca1390f44c7c0dc903b6ba88a4a4fde4d136531cb44766e3c6ded1273475dea00595c31ec3e7afe28019a5c6f4952efd16b67d94edaee043a4056a2782c31c5aa0f4bb8fd565bb24cc39b0157120afb40be99cde20c20d99363caebf955c5a13c6a0d8672a85e53d075497ff4da81422bfb10faec5c8c2d0044fccbc94954601c754a094cc70327bd4edbbff663f537d8b31ba8a903d6b22a4f47e020e01e07b1375b7a0fb3a0b5bda02815f4241d7a91ad1a428640c68a7753a5d692d5ade6d2fc9fb22a00dbe28cfae02f8b7ddc3ea544371b82df122997505672f4ca68e82b5431254cda0e434486a2f3f737717284c03014507ac7e9aeb5b33909e60aaead9cc729876daa0f490e0a91ae868d122649d0414fbf526df55191b428434d243e85bfeef6e2141a0ed079a0ef4612b5be78c7881f650bbfff67c291a418730f540636cd67803b2ba80",
                                 "f90211a099f584601ddfa456be8fb1bc11a9d04f89fca85d5a7b742751d1e632a112b0c5a03119421d72e57140841a5f019273c85a439a843da331185cdce9d2ca3541233aa06c63778935b724f11a966aa70e7fab290b5c742d4cdd67b6051d418304c9dd91a02069229873253405803d8e1d0d5d01911cc7671a8e22f8fa7e83d496ed6445bda08f4ea7df159e23d87f0e6a370c0b49dffb13b4de3557d36eae308ee70797f8dda044f9840dff4d78e22cf0c8cc6c77b51e8f8c0be913629e021066bc15c1da0e23a0666695b8555c0085aec60bd50f941b1ed1a06a64567886d12cc79810ada94e77a069593ccc40ed0a9a3eeaaf5ea77ec454ebd72ec1671046c50c6da8271043c79ca0455ed090dfc3ed5d4097591f5ef4117a4213bd785a02903d1b25ded965821e28a0155d59491e1bd7e76f03eb56f1052bc5af2f7f82358f6008228ba369985175dca08eaa3e82ba004f48941dea816449e49bbc121ad7ce7b90570468aad29274b4b8a041daac098787faaaea6de08d08f7fbac5adc72fe783daff123bf93d08dd3b4cda0bbc313e8353da043272aaa2b4dad484fd65ad832ade431cd9d4d6c567c8c0fb9a074c85dc357f3c01fad6fe1d5f801ac15c1cd0695d0cf19be226ebcb2ee69715da0a4b7e2287d825c0aaa494697012507e8ce86f2decea6fa0f18ed44a09e3d7778a09ea8b3cef67c5e5c6cf7b78b71171991c5a012e7441a8746569934cc35ec72e380",
                                 "f90211a088af0c98f29b354f5801982231cfaf32a21291de4a50cddb5919b7af4a3affc8a0646fd9cd70121f7aed81dcd3a3fa4032905b49ada1b269e1008dbc439ed8dd6ea09360adf6f6de594882463d6afb384a32bcbe075aab2ca772d32c25d877f015cea03f3fa2f3170926c970e1292dc31a5676af1aed81079c2f5c7934db9a412a04eea0df0c1e8fbc8bb7e9e86c7b87e915faae2c0c4e3fc15f786f3c815efba85f2c94a0c54057f44f64aacd0c20280168eb36c886b5ef2fdf91766990e7827425a6af90a06023a11acc770d7745556dc13a151ced8e06e69be2d8a2f2b77069af7472c43fa07556b4b2c3e648c5260055434e49551a4d58f4b0eb6e9199e3aa9850a8b9e85ea0a00895f7c1890ce6ab871e1936f544c16404344a0244fdf2a96d4b312bc1792ca04e617e01777906cff90a5497519ac519ea3303cc072f59da280750222581b30fa0aac76734716f149c1452cc6a24a7c6d6486126991dd0d9c8f409fa804544438ca0784105f279920dd248fd8ccd7f18855e2ce14d02d43a17339ef4fe85630c53a8a0f947985382b0d54e24536f2e08ee353994e298b928d946295853b98918e3c6f8a036b5d525ce50feef47ce0f7d1c95429fc607f21e2608242fcd22ab89e8dec82fa0a190c3e86d4b7806fc730f72f4a12a567e19a66b2d13281ac327adef4bb9a322a014fbde855de56cc7b2c8ceeb21bf728d9b16eb52f75519c7ef2e0d7438f9f52580",
                                 "f90211a096da0777dbb2f875bd357722a4387a78ae24efe13e04ed75f65b2c991247203fa0c0082d34071d1d65e712999126ac28e277b079d939821eaa755c4ef0fd2e5baaa03d98c1cea07206967cd60311f762514cefea1b945b65e35837fa91dadfd9adfba00930aa5c1b70190cd2231cc759fec0c863a2e11bc89b1685d17da64f6bdc5e72a010b3d58569454bcfe1e6c9fad76e7034972f2abb6a3081b3c6b057c7d3151e2ca0868e866d17747182e462be130ceb1b0b3b5011db9f10f3fc8a00743c50497eaaa0aff0dd0b6a09d0e5659b96333f74fd56545addb56451693c1daeb2dbd7fe4470a0dc8377293634ad6f039399e84e930e83ec270294221d00327b08c58ce9c1528ea049f69f18f5acee6f6b1a74a315eb74a51518e88065d645839f5831b371b23dfba016414a0cc05a4ff3a564f32519ab0f21a7d2c263efc1b1c54870db75ec53fb76a09458802b0e109265aacd42f071615e6dd83621b049caa8f05efd5f38b0559cd5a02d8273d7361b92d48df473707dea7efd87a4cfad1f60ffa7fb8cd0f352183dfda0aba4a49f637c164f4a7623b7286c6ff7f146704d4517b2c3c869d5a4db4b8122a0a274c42f4ccef6f20a413218137c6ec00ac7e3fa172f61a7c6487497fe427daba07b654da472355715776ca568289e057e2f0119930f77e557f238911ce4bd52d9a022ae048dd1bc27273ee6cb79991f8e0f34d17a4cba3b13d968cc49d36fe370ab80",
                                 "f90171a02a6e86561c33eacbb1826e9832a25f9f2841c281c9e6b6c18c95ce713dae6c868080a0807528ab397894421968b62bead3b9dd446bda7dc79a5b6ec5a29bd3f19a10c5a0072bce4e78f01cc370c9d92dd39bfcee6059d2e60a95135867991c59184d219980a0415df007b0977e95ba9b3d86b64bae14c02250b3da81f81bed47973f9e4a55b180a0e45736d96c006965b5cad5ec8008d31eb2c34e00c36f677b288701f76f43d627a055324754b7752c9149df27e03392c0598ec89f408040b15140a679f02e695017a04fc91448749dcab3e66cc8e45eb8d85a416fa3a33f8951128d17ca208bc3bb38a085a91d9af48d4f428da097b31c09b3f6b27d38212678a30ffd100364546a4d1ea0c60b5a89f1f89e97528a8b352ed55e5de2e7b963d73ba7de9e922c0ae964e816a048ff598a81a0114eb4afe71ef547cf6fecbb970310ef84049c9353d01c17103aa0e47f3c0d8c3b154c308fed503d87fc39546e1a7bbe143612fce1580110d5d6c08080",
                                 "f871808080808080a03f1587ebf71f19f47449b08f0960630ddbd60d045ea68ee7d89fb1c5682d215a8080808080a001210de038b44d1a57a695f46e2604e6646593958c64441b4fb679f4d16179d88080a05344081f911ad616b960a8f2841eb55f4936a97ec5dae614e726c8c0d51e0eff80",
                                 "f8669d3e3757038b7fea6585ca2d0a3dacd72d84bf0c7b916f169cea0348681bb846f8440180a09dc8b927bc1f203931c70cc3850246046859c40e0044964753b28ff41285b75da0932cddc50793da935ccf915651ad67f6b746e9936fcc5614f0ff492563782c75"
                                ];

            let header_data = hex::decode(header_data).unwrap().into();
            let key = hex::decode(key).unwrap().into();
            let proof = proof_rlp
                .into_iter()
                .map(|x| hex::decode(x).unwrap())
                .collect();
            let account_data = hex::decode(account_data).unwrap();

            if let PromiseOrValue::Value(true) =  contract.verify_account_proof(
                header_data, 
                proof,
                key,
                account_data,
                true
            ) {
            } else {
                panic!();
            }
        }
    
}