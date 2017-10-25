use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn btc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(DX)), operand2: Some(Literal8(2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 250, 2], OperandSize::Word)
}

#[test]
fn btc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand2: Some(Literal8(86)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 58, 86], OperandSize::Word)
}

#[test]
fn btc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(BX)), operand2: Some(Literal8(127)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 251, 127], OperandSize::Dword)
}

#[test]
fn btc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 1076986913, Some(OperandSize::Word), None)), operand2: Some(Literal8(104)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 188, 207, 33, 132, 49, 64, 104], OperandSize::Dword)
}

#[test]
fn btc_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(DX)), operand2: Some(Literal8(103)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 250, 103], OperandSize::Qword)
}

#[test]
fn btc_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectDisplaced(RDI, 1162862143, Some(OperandSize::Word), None)), operand2: Some(Literal8(49)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 191, 63, 222, 79, 69, 49], OperandSize::Qword)
}

#[test]
fn btc_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(ESI)), operand2: Some(Literal8(89)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 254, 89], OperandSize::Word)
}

#[test]
fn btc_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 30138, Some(OperandSize::Dword), None)), operand2: Some(Literal8(90)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 185, 186, 117, 90], OperandSize::Word)
}

#[test]
fn btc_9() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(ESI)), operand2: Some(Literal8(53)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 254, 53], OperandSize::Dword)
}

#[test]
fn btc_10() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectDisplaced(EDI, 362989046, Some(OperandSize::Dword), None)), operand2: Some(Literal8(81)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 191, 246, 197, 162, 21, 81], OperandSize::Dword)
}

#[test]
fn btc_11() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(EDI)), operand2: Some(Literal8(63)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 255, 63], OperandSize::Qword)
}

#[test]
fn btc_12() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledDisplaced(RSI, Four, 2110307495, Some(OperandSize::Dword), None)), operand2: Some(Literal8(72)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 60, 181, 167, 188, 200, 125, 72], OperandSize::Qword)
}

#[test]
fn btc_13() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(RCX)), operand2: Some(Literal8(36)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 249, 36], OperandSize::Qword)
}

#[test]
fn btc_14() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectDisplaced(RCX, 1422180567, Some(OperandSize::Qword), None)), operand2: Some(Literal8(81)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 185, 215, 192, 196, 84, 81], OperandSize::Qword)
}

#[test]
fn btc_15() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(BP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 187, 229], OperandSize::Word)
}

#[test]
fn btc_16() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Indirect(BX, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 187, 23], OperandSize::Word)
}

#[test]
fn btc_17() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(SI)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 187, 214], OperandSize::Dword)
}

#[test]
fn btc_18() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Indirect(ECX, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 187, 57], OperandSize::Dword)
}

#[test]
fn btc_19() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(SI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 187, 246], OperandSize::Qword)
}

#[test]
fn btc_20() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectDisplaced(RBX, 555461385, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 187, 163, 9, 171, 27, 33], OperandSize::Qword)
}

#[test]
fn btc_21() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 187, 245], OperandSize::Word)
}

#[test]
fn btc_22() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 43, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 187, 115, 43], OperandSize::Word)
}

#[test]
fn btc_23() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 187, 243], OperandSize::Dword)
}

#[test]
fn btc_24() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 187, 60, 193], OperandSize::Dword)
}

#[test]
fn btc_25() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 187, 241], OperandSize::Qword)
}

#[test]
fn btc_26() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Two, 569590720, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 187, 172, 113, 192, 67, 243, 33], OperandSize::Qword)
}

#[test]
fn btc_27() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(RCX)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 187, 233], OperandSize::Qword)
}

#[test]
fn btc_28() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 753682947, Some(OperandSize::Qword), None)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 187, 20, 205, 3, 74, 236, 44], OperandSize::Qword)
}

