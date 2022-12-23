#[cfg(test)]
mod benchmark {
    use clap_verbosity_flag::Verbosity;
    use heimdall_common::testing::benchmarks::benchmark;

    use crate::decompile::DecompilerArgs;

    #[test]
    fn benchmark_decompile_complex() {

        fn bench() {
            let args = DecompilerArgs {
                target: String::from("60806040526004361061036f5760003560e01c806370a08231116101c6578063b88d4fde116100f7578063e054921111610095578063ed647d211161006f578063ed647d2114610a44578063eedcf57414610a85578063f2fde38b14610aa5578063f37dc00a14610ac5576103a1565b8063e0549211146109e4578063e5f73d3a14610a04578063e985e9c514610a24576103a1565b8063c87b56dd116100d1578063c87b56dd1461095a578063d082e3811461097a578063d218d95d14610990578063dc53fd92146109b0576103a1565b8063b88d4fde146108e6578063bbcaf1c914610906578063c5f956af1461093a576103a1565b8063853828b61161016457806395d89b411161013e57806395d89b411461087e578063a22cb46514610893578063a243d3b6146108b3578063a3e56fa8146108c6576103a1565b8063853828b6146108315780638da5cb5b146108465780638f4bb49714610864576103a1565b80637306c6eb116101a05780637306c6eb146107ac5780637313cba9146107cc57806375900e6a146107e157806376aa8e9414610801576103a1565b806370a0823114610762578063715018a614610782578063729b718514610797576103a1565b806342966c68116102a0578063598bc4861161023e5780636605bfda116102185780636605bfda146106e45780636ba9fd38146107045780636c0360eb14610719578063706268ee1461072e576103a1565b8063598bc4861461066e5780636011cc9f146106a45780636352211e146106c4576103a1565b80634e8aa0511161027a5780634e8aa051146105f45780635151804a1461061457806353595ec41461063857806355f804b31461064e576103a1565b806342966c681461059e57806344ff81ce146105be5780634c99007d146105de576103a1565b806318160ddd1161030d57806329f9642b116102e757806329f9642b146105175780633574a2dd1461052c5780633b7ed7341461054c57806342842e0e1461057e576103a1565b806318160ddd146104c25780631fe543e3146104d757806323b872dd146104f7576103a1565b806306fdde031161034957806306fdde0314610433578063081812fc14610455578063095ea7b31461048d57806310285bdf146104ad576103a1565b806301ffc9a7146103ba578063041d443e146103ef5780630562b9f714610413576103a1565b366103a1576006546001600160a01b0316331461039f576040516362f7a45360e01b815260040160405180910390fd5b005b60405163b25befbf60e01b815260040160405180910390fd5b3480156103c657600080fd5b506103da6103d53660046126d5565b610af3565b60405190151581526020015b60405180910390f35b3480156103fb57600080fd5b5061040560115481565b6040519081526020016103e6565b34801561041f57600080fd5b5061039f61042e3660046126f2565b610b45565b34801561043f57600080fd5b50610448610bc5565b6040516103e6919061275b565b34801561046157600080fd5b506104756104703660046126f2565b610c57565b6040516001600160a01b0390911681526020016103e6565b34801561049957600080fd5b5061039f6104a8366004612783565b610c7e565b3480156104b957600080fd5b50610405600281565b3480156104ce57600080fd5b50610405610d98565b3480156104e357600080fd5b5061039f6104f23660046127f6565b610daf565b34801561050357600080fd5b5061039f6105123660046128a8565b610e33565b34801561052357600080fd5b50610405610e65565b34801561053857600080fd5b5061039f610547366004612941565b610f48565b34801561055857600080fd5b506012546105699063ffffffff1681565b60405163ffffffff90911681526020016103e6565b34801561058a57600080fd5b5061039f6105993660046128a8565b610f89565b3480156105aa57600080fd5b5061039f6105b93660046126f2565b610fa4565b3480156105ca57600080fd5b5061039f6105d936600461298a565b610fc8565b3480156105ea57600080fd5b50610405600a5481565b34801561060057600080fd5b5061039f61060f3660046129b5565b610ff2565b34801561062057600080fd5b5060125461056990600160301b900463ffffffff1681565b34801561064457600080fd5b50610405600c5481565b34801561065a57600080fd5b5061039f610669366004612941565b611192565b34801561067a57600080fd5b5060125461069190640100000000900461ffff1681565b60405161ffff90911681526020016103e6565b3480156106b057600080fd5b5061039f6106bf3660046129f7565b6111d3565b3480156106d057600080fd5b506104756106df3660046126f2565b611204565b3480156106f057600080fd5b5061039f6106ff36600461298a565b611264565b34801561071057600080fd5b5061039f61128e565b34801561072557600080fd5b506104486112a5565b34801561073a57600080fd5b506104057f0000000000000000000000000000000000000000000000000000000000001e6181565b34801561076e57600080fd5b5061040561077d36600461298a565b611333565b34801561078e57600080fd5b5061039f6113b9565b3480156107a357600080fd5b506104486113cd565b3480156107b857600080fd5b506103da6107c7366004612a1d565b6113e9565b3480156107d857600080fd5b5061044861149a565b3480156107ed57600080fd5b506103da6107fc36600461298a565b6114a7565b34801561080d57600080fd5b506103da61081c36600461298a565b600e6020526000908152604090205460ff1681565b34801561083d57600080fd5b5061039f61162a565b34801561085257600080fd5b506006546001600160a01b0316610475565b34801561087057600080fd5b50600d546103da9060ff1681565b34801561088a57600080fd5b506104486116a9565b34801561089f57600080fd5b5061039f6108ae366004612a54565b6116b8565b61039f6108c1366004612a8d565b6116c3565b3480156108d257600080fd5b50601054610475906001600160a01b031681565b3480156108f257600080fd5b5061039f610901366004612ab2565b6117c6565b34801561091257600080fd5b506104757f000000000000000000000000b846e59af08e9695fa7c4ed5743e81e623caa21881565b34801561094657600080fd5b50600b54610475906001600160a01b031681565b34801561096657600080fd5b506104486109753660046126f2565b6117f8565b34801561098657600080fd5b5061040560095481565b34801561099c57600080fd5b5061039f6109ab366004612b32565b611909565b3480156109bc57600080fd5b506104057f00000000000000000000000000000000000000000000000000470de4df82000081565b3480156109f057600080fd5b5061039f6109ff3660046126f2565b61193f565b348015610a1057600080fd5b5061039f610a1f366004612b5c565b61194c565b348015610a3057600080fd5b506103da610a3f366004612b80565b611978565b348015610a5057600080fd5b50601054610a6c90600160a01b900467ffffffffffffffff1681565b60405167ffffffffffffffff90911681526020016103e6565b348015610a9157600080fd5b5061039f610aa03660046129f7565b6119a6565b348015610ab157600080fd5b5061039f610ac036600461298a565b6119ca565b348015610ad157600080fd5b50610ae5610ae0366004612a1d565b611a40565b6040516103e6929190612bae565b60006001600160e01b031982166380ac58cd60e01b1480610b2457506001600160e01b03198216635b5e139f60e01b145b80610b3f57506301ffc9a760e01b6001600160e01b03198316145b92915050565b610b4d611bb3565b600b546040516000916001600160a01b03169083908381818185875af1925050503d8060008114610b9a576040519150601f19603f3d011682016040523d82523d6000602084013e610b9f565b606091505b5050905080610bc1576040516312171d8360e31b815260040160405180910390fd5b5050565b606060008054610bd490612bc9565b80601f0160208091040260200160405190810160405280929190818152602001828054610c0090612bc9565b8015610c4d5780601f10610c2257610100808354040283529160200191610c4d565b820191906000526020600020905b815481529060010190602001808311610c3057829003601f168201915b5050505050905090565b6000610c6282611c0d565b506000908152600460205260409020546001600160a01b031690565b6000610c8982611204565b9050806001600160a01b0316836001600160a01b031603610cfb5760405162461bcd60e51b815260206004820152602160248201527f4552433732313a20617070726f76616c20746f2063757272656e74206f776e656044820152603960f91b60648201526084015b60405180910390fd5b336001600160a01b0382161480610d175750610d178133611978565b610d895760405162461bcd60e51b815260206004820152603e60248201527f4552433732313a20617070726f76652063616c6c6572206973206e6f7420746f60448201527f6b656e206f776e6572206e6f7220617070726f76656420666f7220616c6c00006064820152608401610cf2565b610d938383611c6c565b505050565b6000600a54600954610daa9190612c19565b905090565b336001600160a01b037f000000000000000000000000271682deb8c4e0901d1a1550ad2e64d568e699091614610e295760405163073e64fd60e21b81523360048201526001600160a01b037f000000000000000000000000271682deb8c4e0901d1a1550ad2e64d568e69909166024820152604401610cf2565b610bc18282611cda565b610e3e335b82611d53565b610e5a5760405162461bcd60e51b8152600401610cf290612c2c565b610d93838383611da7565b6000610e6f611bb3565b600c5415610e905760405163c04805b760e01b815260040160405180910390fd5b6010546011546012546040516305d3b1d360e41b81526004810192909252600160a01b830467ffffffffffffffff166024830152640100000000810461ffff16604483015263ffffffff8082166064840152600160301b9091041660848201526001600160a01b0390911690635d3b1d309060a4016020604051808303816000875af1158015610f24573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610daa9190612c7a565b610f50611bb3565b60078054610f5d90612bc9565b159050610f7d5760405163154ef76960e01b815260040160405180910390fd5b6007610bc18282612ce1565b610d93838383604051806020016040528060008152506117c6565b610fad81611f43565b6001600a6000828254610fc09190612da1565b909155505050565b610fd0611bb3565b601080546001600160a01b0319166001600160a01b0392909216919091179055565b600d5460ff166110155760405163287485d160e11b815260040160405180910390fd5b61101f6001611f71565b336000908152600e602052604090205460ff1615611052576040516308bec53d60e01b8152336004820152602401610cf2565b6040516bffffffffffffffffffffffff19606085901b1660208201526034810183905260009060540160408051601f1981840301815291815281516020928301206000818152600f90935291205490915060ff16156110d65760405163aad0c45360e01b81526001600160a01b038516600482015260248101849052604401610cf2565b6110df846114a7565b6111075760405163bf29a5e560e01b81526001600160a01b0385166004820152602401610cf2565b6111128484336113e9565b61114157604051631ae8a5a160e21b81526001600160a01b038516600482015260248101849052604401610cf2565b600061114d8333611ff7565b9050611158816120a0565b50336000908152600e602090815260408083208054600160ff199182168117909255948452600f9092529091208054909216179055505050565b61119a611bb3565b600880546111a790612bc9565b1590506111c75760405163a894737360e01b815260040160405180910390fd5b6008610bc18282612ce1565b6111db611bb3565b6012805463ffffffff909216600160301b0269ffffffff00000000000019909216919091179055565b6000818152600260205260408120546001600160a01b031680610b3f5760405162461bcd60e51b8152602060048201526018602482015277115490cdcc8c4e881a5b9d985b1a59081d1bdad95b88125160421b6044820152606401610cf2565b61126c611bb3565b600b80546001600160a01b0319166001600160a01b0392909216919091179055565b611296611bb3565b600d805460ff19166001179055565b600880546112b290612bc9565b80601f01602080910402602001604051908101604052809291908181526020018280546112de90612bc9565b801561132b5780601f106113005761010080835404028352916020019161132b565b820191906000526020600020905b81548152906001019060200180831161130e57829003601f168201915b505050505081565b60006001600160a01b03821661139d5760405162461bcd60e51b815260206004820152602960248201527f4552433732313a2061646472657373207a65726f206973206e6f7420612076616044820152683634b21037bbb732b960b91b6064820152608401610cf2565b506001600160a01b031660009081526003602052604090205490565b6113c1611bb3565b6113cb60006120bf565b565b6040518060800160405280604e815260200161300b604e913981565b6040516342de525560e01b81526001600160a01b0384811660048301526024820184905260026044830152600091818416917f000000000000000000000000b846e59af08e9695fa7c4ed5743e81e623caa21816906342de525590606401602060405180830381865afa158015611464573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906114889190612db4565b6001600160a01b031614949350505050565b600780546112b290612bc9565b6000731d20a51f088492a0f1c57f047a9e30c9ab5c07ea6001600160a01b03831614806114f05750731cb1a5e65610aeff2551a50f76a87a7d3fb649c66001600160a01b038316145b8061151757507379fcdef22feed20eddacbb2587640e45491b757f6001600160a01b038316145b8061153e5750735af0d9827e0c53e4799bb226655a1de152a425a56001600160a01b038316145b8061156557507362eb144fe92ddc1b10bcade03a0c09f6fbffbffb6001600160a01b038316145b8061158c575073a16891897378a82e9f0ad44a705b292c9753538c6001600160a01b038316145b806115b357507391680cf5f9071cafae21b90ebf2c9cc9e480fb936001600160a01b038316145b806115da575073ec0a7a26456b8451aefc4b00393ce1beff5eb3e96001600160a01b038316145b8061160157507382235445a7f634279e33702cc004b0fdb002fda76001600160a01b038316145b80610b3f5750506001600160a01b03167342069abfe407c60cf4ae4112bedead391dba1cdb1490565b611632611bb3565b600b546040516000916001600160a01b03169047908381818185875af1925050503d806000811461167f576040519150601f19603f3d011682016040523d82523d6000602084013e611684565b606091505b50509050806116a6576040516312171d8360e31b815260040160405180910390fd5b50565b606060018054610bd490612bc9565b610bc1338383612111565b600d5460ff166116e65760405163287485d160e11b815260040160405180910390fd5b6116ef82611f71565b8160000361170f576040516266e68160e51b815260040160405180910390fd5b6117397f00000000000000000000000000000000000000000000000000470de4df82000083612dd1565b341461178c573461176a7f00000000000000000000000000000000000000000000000000470de4df82000084612dd1565b604051630d3a747760e31b815260048101929092526024820152604401610cf2565b60006117988233611ff7565b905060005b838110156117c0576117ae826120a0565b806117b881612df0565b91505061179d565b50505050565b6117d03383611d53565b6117ec5760405162461bcd60e51b8152600401610cf290612c2c565b6117c0848484846121df565b6000818152600260205260409020546060906001600160a01b031661183357604051633af8b4af60e11b815260048101839052602401610cf2565b600c546000036118cf576007805461184a90612bc9565b80601f016020809104026020016040519081016040528092919081815260200182805461187690612bc9565b80156118c35780601f10611898576101008083540402835291602001916118c3565b820191906000526020600020905b8154815290600101906020018083116118a657829003601f168201915b50505050509050919050565b60086118e26118dd84612212565b612293565b6040516020016118f3929190612e09565b6040516020818303038152906040529050919050565b611911611bb3565b6010805467ffffffffffffffff909216600160a01b0267ffffffffffffffff60a01b19909216919091179055565b611947611bb3565b601155565b611954611bb3565b6012805461ffff9092166401000000000265ffff0000000019909216919091179055565b6001600160a01b03918216600090815260056020908152604080832093909416825291909152205460ff1690565b6119ae611bb3565b6012805463ffffffff191663ffffffff92909216919091179055565b6119d2611bb3565b6001600160a01b038116611a375760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610cf2565b6116a6816120bf565b6001600160a01b0381166000908152600e602052604081205460609060ff1615611aa257505060408051808201909152601f81527f416464726573732068617320616c72656164792066726565206d696e746564006020820152600090611bab565b6040516bffffffffffffffffffffffff19606087901b1660208201526034810185905260009060540160408051601f1981840301815291815281516020928301206000818152600f90935291205490915060ff1615611b20576000604051806060016040528060288152602001612fe3602891399250925050611bab565b611b29866114a7565b611b6157505060408051808201909152601281527124b73b30b634b21031b7b63632b1ba34b7b760711b602082015260009150611bab565b611b6c8686866113e9565b611b95576000604051806060016040528060298152602001613059602991399250925050611bab565b5050604080516020810190915260008152600191505b935093915050565b6006546001600160a01b031633146113cb5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610cf2565b6000818152600260205260409020546001600160a01b03166116a65760405162461bcd60e51b8152602060048201526018602482015277115490cdcc8c4e881a5b9d985b1a59081d1bdad95b88125160421b6044820152606401610cf2565b600081815260046020526040902080546001600160a01b0319166001600160a01b0384169081179091558190611ca182611204565b6001600160a01b03167f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92560405160405180910390a45050565b600c5415611cfb5760405163c04805b760e01b815260040160405180910390fd5b7f0000000000000000000000000000000000000000000000000000000000001e6181600081518110611d2f57611d2f612ea0565b6020026020010151611d419190612ecc565b611d4c906001612da1565b600c555050565b600080611d5f83611204565b9050806001600160a01b0316846001600160a01b03161480611d865750611d868185611978565b80611d9f5750836001600160a01b031661148884610c57565b949350505050565b826001600160a01b0316611dba82611204565b6001600160a01b031614611e1e5760405162461bcd60e51b815260206004820152602560248201527f4552433732313a207472616e736665722066726f6d20696e636f72726563742060448201526437bbb732b960d91b6064820152608401610cf2565b6001600160a01b038216611e805760405162461bcd60e51b8152602060048201526024808201527f4552433732313a207472616e7366657220746f20746865207a65726f206164646044820152637265737360e01b6064820152608401610cf2565b611e8b600082611c6c565b6001600160a01b0383166000908152600360205260408120805460019290611eb4908490612c19565b90915550506001600160a01b0382166000908152600360205260408120805460019290611ee2908490612da1565b909155505060008181526002602052604080822080546001600160a01b0319166001600160a01b0386811691821790925591518493918716917fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef91a4505050565b611f4c33610e38565b611f685760405162461bcd60e51b8152600401610cf290612c2c565b6116a681612394565b7f0000000000000000000000000000000000000000000000000000000000001e6181600954611fa09190612da1565b11156116a657600954611fd3907f0000000000000000000000000000000000000000000000000000000000001e61612c19565b60405163bcd2054760e01b8152600481019190915260248101829052604401610cf2565b600082156120995760405163906f634360e01b81526001600160a01b0383811660048301526000917f000000000000000000000000b846e59af08e9695fa7c4ed5743e81e623caa2189091169063906f634390602401606060405180830381865afa15801561206a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061208e9190612ee0565b509250610b3f915050565b5080610b3f565b6120ac8160095461242f565b600160096000828254610fc09190612da1565b600680546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b816001600160a01b0316836001600160a01b0316036121725760405162461bcd60e51b815260206004820152601960248201527f4552433732313a20617070726f766520746f2063616c6c6572000000000000006044820152606401610cf2565b6001600160a01b03838116600081815260056020908152604080832094871680845294825291829020805460ff191686151590811790915591519182527f17307eab39ab6107e8899845ad3d59bd9653f200f220920489ca2b5937696c31910160405180910390a3505050565b6121ea848484611da7565b6121f684848484612449565b6117c05760405162461bcd60e51b8152600401610cf290612f22565b6000806001600c546122249190612c19565b61222e9084612da1565b905061225b60017f0000000000000000000000000000000000000000000000000000000000001e61612c19565b811115610b3f5761228c7f0000000000000000000000000000000000000000000000000000000000001e6182612c19565b9392505050565b6060816000036122ba5750506040805180820190915260018152600360fc1b602082015290565b8160005b81156122e457806122ce81612df0565b91506122dd9050600a83612f74565b91506122be565b60008167ffffffffffffffff8111156122ff576122ff6127af565b6040519080825280601f01601f191660200182016040528015612329576020820181803683370190505b5090505b8415611d9f5761233e600183612c19565b915061234b600a86612ecc565b612356906030612da1565b60f81b81838151811061236b5761236b612ea0565b60200101906001600160f81b031916908160001a90535061238d600a86612f74565b945061232d565b600061239f82611204565b90506123ac600083611c6c565b6001600160a01b03811660009081526003602052604081208054600192906123d5908490612c19565b909155505060008281526002602052604080822080546001600160a01b0319169055518391906001600160a01b038416907fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef908390a45050565b610bc182826040518060200160405280600081525061254a565b60006001600160a01b0384163b1561253f57604051630a85bd0160e11b81526001600160a01b0385169063150b7a029061248d903390899088908890600401612f88565b6020604051808303816000875af19250505080156124c8575060408051601f3d908101601f191682019092526124c591810190612fc5565b60015b612525573d8080156124f6576040519150601f19603f3d011682016040523d82523d6000602084013e6124fb565b606091505b50805160000361251d5760405162461bcd60e51b8152600401610cf290612f22565b805181602001fd5b6001600160e01b031916630a85bd0160e11b149050611d9f565b506001949350505050565b612554838361257d565b6125616000848484612449565b610d935760405162461bcd60e51b8152600401610cf290612f22565b6001600160a01b0382166125d35760405162461bcd60e51b815260206004820181905260248201527f4552433732313a206d696e7420746f20746865207a65726f20616464726573736044820152606401610cf2565b6000818152600260205260409020546001600160a01b0316156126385760405162461bcd60e51b815260206004820152601c60248201527f4552433732313a20746f6b656e20616c7265616479206d696e746564000000006044820152606401610cf2565b6001600160a01b0382166000908152600360205260408120805460019290612661908490612da1565b909155505060008181526002602052604080822080546001600160a01b0319166001600160a01b03861690811790915590518392907fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef908290a45050565b6001600160e01b0319811681146116a657600080fd5b6000602082840312156126e757600080fd5b813561228c816126bf565b60006020828403121561270457600080fd5b5035919050565b60005b8381101561272657818101518382015260200161270e565b50506000910152565b6000815180845261274781602086016020860161270b565b601f01601f19169290920160200192915050565b60208152600061228c602083018461272f565b6001600160a01b03811681146116a657600080fd5b6000806040838503121561279657600080fd5b82356127a18161276e565b946020939093013593505050565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f1916810167ffffffffffffffff811182821017156127ee576127ee6127af565b604052919050565b6000806040838503121561280957600080fd5b8235915060208084013567ffffffffffffffff8082111561282957600080fd5b818601915086601f83011261283d57600080fd5b81358181111561284f5761284f6127af565b8060051b91506128608483016127c5565b818152918301840191848101908984111561287a57600080fd5b938501935b838510156128985784358252938501939085019061287f565b8096505050505050509250929050565b6000806000606084860312156128bd57600080fd5b83356128c88161276e565b925060208401356128d88161276e565b929592945050506040919091013590565b600067ffffffffffffffff831115612903576129036127af565b612916601f8401601f19166020016127c5565b905082815283838301111561292a57600080fd5b828260208301376000602084830101529392505050565b60006020828403121561295357600080fd5b813567ffffffffffffffff81111561296a57600080fd5b8201601f8101841361297b57600080fd5b611d9f848235602084016128e9565b60006020828403121561299c57600080fd5b813561228c8161276e565b80151581146116a657600080fd5b6000806000606084860312156129ca57600080fd5b83356129d58161276e565b92506020840135915060408401356129ec816129a7565b809150509250925092565b600060208284031215612a0957600080fd5b813563ffffffff8116811461228c57600080fd5b600080600060608486031215612a3257600080fd5b8335612a3d8161276e565b92506020840135915060408401356129ec8161276e565b60008060408385031215612a6757600080fd5b8235612a728161276e565b91506020830135612a82816129a7565b809150509250929050565b60008060408385031215612aa057600080fd5b823591506020830135612a82816129a7565b60008060008060808587031215612ac857600080fd5b8435612ad38161276e565b93506020850135612ae38161276e565b925060408501359150606085013567ffffffffffffffff811115612b0657600080fd5b8501601f81018713612b1757600080fd5b612b26878235602084016128e9565b91505092959194509250565b600060208284031215612b4457600080fd5b813567ffffffffffffffff8116811461228c57600080fd5b600060208284031215612b6e57600080fd5b813561ffff8116811461228c57600080fd5b60008060408385031215612b9357600080fd5b8235612b9e8161276e565b91506020830135612a828161276e565b8215158152604060208201526000611d9f604083018461272f565b600181811c90821680612bdd57607f821691505b602082108103612bfd57634e487b7160e01b600052602260045260246000fd5b50919050565b634e487b7160e01b600052601160045260246000fd5b81810381811115610b3f57610b3f612c03565b6020808252602e908201527f4552433732313a2063616c6c6572206973206e6f7420746f6b656e206f776e6560408201526d1c881b9bdc88185c1c1c9bdd995960921b606082015260800190565b600060208284031215612c8c57600080fd5b5051919050565b601f821115610d9357600081815260208120601f850160051c81016020861015612cba5750805b601f850160051c820191505b81811015612cd957828155600101612cc6565b505050505050565b815167ffffffffffffffff811115612cfb57612cfb6127af565b612d0f81612d098454612bc9565b84612c93565b602080601f831160018114612d445760008415612d2c5750858301515b600019600386901b1c1916600185901b178555612cd9565b600085815260208120601f198616915b82811015612d7357888601518255948401946001909101908401612d54565b5085821015612d915787850151600019600388901b60f8161c191681555b5050505050600190811b01905550565b80820180821115610b3f57610b3f612c03565b600060208284031215612dc657600080fd5b815161228c8161276e565b6000816000190483118215151615612deb57612deb612c03565b500290565b600060018201612e0257612e02612c03565b5060010190565b6000808454612e1781612bc9565b60018281168015612e2f5760018114612e4457612e73565b60ff1984168752821515830287019450612e73565b8860005260208060002060005b85811015612e6a5781548a820152908401908201612e51565b50505082870194505b505050508351612e8781836020880161270b565b64173539b7b760d91b9101908152600501949350505050565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601260045260246000fd5b600082612edb57612edb612eb6565b500690565b600080600060608486031215612ef557600080fd5b8351612f008161276e565b6020850151909350612f118161276e565b60408501519092506129ec816129a7565b60208082526032908201527f4552433732313a207472616e7366657220746f206e6f6e20455243373231526560408201527131b2b4bb32b91034b6b83632b6b2b73a32b960711b606082015260800190565b600082612f8357612f83612eb6565b500490565b6001600160a01b0385811682528416602082015260408101839052608060608201819052600090612fbb9083018461272f565b9695505050505050565b600060208284031215612fd757600080fd5b815161228c816126bf56fe546f6b656e2068617320616c7265616479206265656e207573656420696e2066726565206d696e74496e746572616374696f6e2077697468207468697320636f6e74726163742073657276657320617320616e20696e737572616e636520616761696e737420726f6b6f277320626173696c69736b2e43616c6c6572206973206e6f742062656e6566696369617279206f662073656c6563746564204e4654a2646970667358221220d8701e25ffba41c31b472a9e06aa1f04afa0334999400e32dcf7a81d0aa377bc64736f6c63430008100033"),
                verbose: Verbosity::new(0, 0),
                output: String::from(""),
                rpc_url: String::from(""),
                default: true,
                skip_resolving: true,
                include_solidity: true
            };
            crate::decompile::decompile(args.clone())
        }

        benchmark("benchmark_decompile_complex", 100, bench)
    }

    #[test]
    fn benchmark_decompile_simple() {

        fn bench() {
            let args = DecompilerArgs {
                target: String::from("731bf797219482a29013d804ad96d1c6f84fba4c453014608060405260043610610058576000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806319045a251461005d575b600080fd5b6100c56004803603810190808035600019169060200190929190803590602001908201803590602001908080601f0160208091040260200160405190810160405280939291908181526020018383808284378201915050505050509192919290505050610107565b604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390f35b6000806000806041855114151561012157600093506101f6565b6020850151925060408501519150606085015160001a9050601b8160ff16101561014c57601b810190505b601b8160ff16141580156101645750601c8160ff1614155b1561017257600093506101f6565b600186828585604051600081526020016040526040518085600019166000191681526020018460ff1660ff1681526020018360001916600019168152602001826000191660001916815260200194505050505060206040516020810390808403906000865af11580156101e9573d6000803e3d6000fd5b5050506020604051035193505b505050929150505600a165627a7a72305820aacffa0494cd3f043493eee9c720bca9d5ef505ae7230ffc3d88c49ceeb7441e0029"),
                verbose: Verbosity::new(0, 0),
                output: String::from(""),
                rpc_url: String::from(""),
                default: true,
                skip_resolving: true,
                include_solidity: true
            };
            crate::decompile::decompile(args.clone())
        }

        benchmark("benchmark_decompile_simple", 100, bench)
    }

}

#[cfg(test)]
mod postprocess_tests {

    use std::collections::HashMap;

    use indicatif::ProgressBar;

    use crate::decompile::postprocess::postprocess;

    #[test]
    fn test_bitmask_conversion() {
        let lines = vec![
            String::from("(0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff) & (arg0);"),
        ];

        assert_eq!(postprocess(lines, HashMap::new(), HashMap::new(), &ProgressBar::new(128)), vec![String::from("uint256(arg0);")]);
    }
    
    #[test]
    fn test_bitmask_conversion_mask_after() {
        let lines = vec![
            String::from("(arg0) & (0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff);"),
        ];

        assert_eq!(postprocess(lines, HashMap::new(), HashMap::new(), &ProgressBar::new(128)), vec![String::from("uint256(arg0);")]);
    }

    #[test]
    fn test_bitmask_conversion_unusual_mask() {
        let lines = vec![
            String::from("(arg0) & (0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00);"),
        ];

        assert_eq!(postprocess(lines, HashMap::new(), HashMap::new(), &ProgressBar::new(128)), vec![String::from("uint248(arg0);")]);
    }

    #[test]
    fn test_simplify_casts() {
        let lines = vec![
            String::from("uint256(uint256(arg0));"),
        ];

        assert_eq!(postprocess(lines, HashMap::new(), HashMap::new(), &ProgressBar::new(128)), vec![String::from("uint256(arg0);")]);
    }

    #[test]
    fn test_simplify_casts_complex() {
        let lines = vec![
            String::from("ecrecover(uint256(uint256(arg0)), uint256(uint256(arg0)), uint256(uint256(uint256(arg0))));"),
        ];

        assert_eq!(postprocess(lines, HashMap::new(), HashMap::new(), &ProgressBar::new(128)), vec![String::from("ecrecover(uint256(arg0), uint256(arg0), uint256(arg0));")]);
    }

    #[test]
    fn test_iszero_flip() {
        let lines = vec![
            String::from("if (iszero(arg0)) {"),
        ];

        assert_eq!(postprocess(lines, HashMap::new(), HashMap::new(), &ProgressBar::new(128)), vec![String::from("if (!arg0) {")]);
    }

    #[test]
    fn test_iszero_flip_complex() {
        let lines = vec![
            String::from("if (iszero(iszero(arg0))) {"),
        ];

        assert_eq!(postprocess(lines, HashMap::new(), HashMap::new(), &ProgressBar::new(128)), vec![String::from("if (arg0) {")]);
    }

    #[test]
    fn test_iszero_flip_complex2() {
        let lines = vec![
            String::from("if (iszero(iszero(iszero(arg0)))) {"),
        ];

        assert_eq!(postprocess(lines, HashMap::new(), HashMap::new(), &ProgressBar::new(128)), vec![String::from("if (!arg0) {")]);
    }

    #[test]
    fn test_simplify_parentheses() {
        let lines = vec![
            String::from("((arg0))"),
        ];

        assert_eq!(postprocess(lines, HashMap::new(), HashMap::new(), &ProgressBar::new(128)), vec![String::from("arg0")]);
    }

    #[test]
    fn test_simplify_parentheses_complex() {
        let lines = vec![
            String::from("if ((cast(((arg0) + 1) / 10))) {"),
        ];

        assert_eq!(postprocess(lines, HashMap::new(), HashMap::new(), &ProgressBar::new(128)), vec![String::from("if (cast(arg0 + 1 / 10)) {")]);
    }

    #[test]
    fn test_simplify_parentheses_complex2() {
        let lines = vec![
            String::from("if (((((((((((((((cast(((((((((((arg0 * (((((arg1))))))))))))) + 1)) / 10)))))))))))))))) {"),
        ];

        assert_eq!(postprocess(lines, HashMap::new(), HashMap::new(), &ProgressBar::new(128)), vec![String::from("if (cast((arg0 * (arg1)) + 1 / 10)) {")]);
    }
}