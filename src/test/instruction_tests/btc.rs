use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn btc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(CX)), operand2: Some(Literal8(120)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 249, 120], OperandSize::Word)
}

fn btc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectDisplaced(DI, 210, Some(OperandSize::Word), None)), operand2: Some(Literal8(101)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 189, 210, 0, 101], OperandSize::Word)
}

fn btc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(BP)), operand2: Some(Literal8(27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 253, 27], OperandSize::Dword)
}

fn btc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledIndexed(ESI, EAX, Four, Some(OperandSize::Word), None)), operand2: Some(Literal8(76)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 60, 134, 76], OperandSize::Dword)
}

fn btc_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(SP)), operand2: Some(Literal8(97)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 252, 97], OperandSize::Qword)
}

fn btc_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledDisplaced(RDI, Four, 1993573642, Some(OperandSize::Word), None)), operand2: Some(Literal8(11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 60, 189, 10, 133, 211, 118, 11], OperandSize::Qword)
}

fn btc_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(EBX)), operand2: Some(Literal8(61)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 251, 61], OperandSize::Word)
}

fn btc_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 5047, Some(OperandSize::Dword), None)), operand2: Some(Literal8(107)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 186, 183, 19, 107], OperandSize::Word)
}

fn btc_9() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(EBP)), operand2: Some(Literal8(59)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 253, 59], OperandSize::Dword)
}

fn btc_10() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand2: Some(Literal8(106)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 62, 106], OperandSize::Dword)
}

fn btc_11() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(EDX)), operand2: Some(Literal8(105)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 250, 105], OperandSize::Qword)
}

fn btc_12() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledDisplaced(RDI, Two, 1537738013, Some(OperandSize::Dword), None)), operand2: Some(Literal8(115)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 60, 125, 29, 5, 168, 91, 115], OperandSize::Qword)
}

fn btc_13() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(RDI)), operand2: Some(Literal8(51)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 255, 51], OperandSize::Qword)
}

fn btc_14() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledIndexed(RDI, RCX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Literal8(8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 60, 207, 8], OperandSize::Qword)
}

fn btc_15() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 187, 238], OperandSize::Word)
}

fn btc_16() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectDisplaced(BP, 25409, Some(OperandSize::Word), None)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 187, 142, 65, 99], OperandSize::Word)
}

fn btc_17() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(SP)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 187, 220], OperandSize::Dword)
}

fn btc_18() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Indirect(EAX, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 187, 32], OperandSize::Dword)
}

fn btc_19() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(BP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 187, 237], OperandSize::Qword)
}

fn btc_20() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectDisplaced(RBX, 1726991759, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 187, 155, 143, 205, 239, 102], OperandSize::Qword)
}

fn btc_21() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 187, 225], OperandSize::Word)
}

fn btc_22() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectDisplaced(SI, 147, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 187, 172, 147, 0], OperandSize::Word)
}

fn btc_23() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 187, 254], OperandSize::Dword)
}

fn btc_24() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectDisplaced(EDX, 771748522, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 187, 146, 170, 242, 255, 45], OperandSize::Dword)
}

fn btc_25() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 187, 225], OperandSize::Qword)
}

fn btc_26() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledDisplaced(RBX, Eight, 496228417, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 187, 28, 221, 65, 216, 147, 29], OperandSize::Qword)
}

fn btc_27() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(RSI)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 187, 214], OperandSize::Qword)
}

fn btc_28() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectDisplaced(RBX, 215074100, Some(OperandSize::Qword), None)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 187, 147, 52, 197, 209, 12], OperandSize::Qword)
}

