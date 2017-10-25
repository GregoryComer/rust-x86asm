use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vreducepd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 140, 86, 197, 51], OperandSize::Dword)
}

#[test]
fn vreducepd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Two, 420788219, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 141, 86, 188, 75, 251, 183, 20, 25, 91], OperandSize::Dword)
}

#[test]
fn vreducepd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(EDX, EDX, Four, Some(OperandSize::Qword), None)), operand3: Some(Literal8(69)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 159, 86, 60, 146, 69], OperandSize::Dword)
}

#[test]
fn vreducepd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(79)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 138, 86, 240, 79], OperandSize::Qword)
}

#[test]
fn vreducepd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 335165036, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 143, 86, 4, 125, 108, 54, 250, 19, 119], OperandSize::Qword)
}

#[test]
fn vreducepd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(XMM29)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 2099277433, Some(OperandSize::Qword), None)), operand3: Some(Literal8(58)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 99, 253, 156, 86, 172, 66, 121, 110, 32, 125, 58], OperandSize::Qword)
}

#[test]
fn vreducepd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(125)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 174, 86, 209, 125], OperandSize::Dword)
}

#[test]
fn vreducepd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 174, 86, 23, 22], OperandSize::Dword)
}

#[test]
fn vreducepd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 773345945, Some(OperandSize::Qword), None)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 185, 86, 36, 117, 153, 82, 24, 46, 104], OperandSize::Dword)
}

#[test]
fn vreducepd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM8)), operand3: Some(Literal8(125)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 83, 253, 170, 86, 200, 125], OperandSize::Qword)
}

#[test]
fn vreducepd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(YMM13)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Two, 1526486660, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 115, 253, 172, 86, 172, 126, 132, 86, 252, 90, 88], OperandSize::Qword)
}

#[test]
fn vreducepd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(YMM25)), operand2: Some(IndirectDisplaced(RAX, 692083324, Some(OperandSize::Qword), None)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 99, 253, 185, 86, 136, 124, 90, 64, 41, 80], OperandSize::Qword)
}

#[test]
fn vreducepd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(Literal8(52)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 159, 86, 207, 52], OperandSize::Dword)
}

#[test]
fn vreducepd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Four, 543031671, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 207, 86, 164, 185, 119, 1, 94, 32, 76], OperandSize::Dword)
}

#[test]
fn vreducepd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectDisplaced(EDI, 1829328166, Some(OperandSize::Qword), None)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 220, 86, 135, 38, 85, 9, 109, 112], OperandSize::Dword)
}

#[test]
fn vreducepd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM30)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 19, 253, 155, 86, 254, 62], OperandSize::Qword)
}

#[test]
fn vreducepd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Eight, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 201, 86, 44, 248, 26], OperandSize::Qword)
}

#[test]
fn vreducepd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(ZMM21)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Two, 1571619389, Some(OperandSize::Qword), None)), operand3: Some(Literal8(85)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 227, 253, 223, 86, 172, 122, 61, 2, 173, 93, 85], OperandSize::Qword)
}

