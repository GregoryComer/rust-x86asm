use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpxord_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 141, 239, 244], OperandSize::Dword)
}

#[test]
fn vpxord_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Two, 8020978, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 101, 143, 239, 180, 118, 242, 99, 122, 0], OperandSize::Dword)
}

#[test]
fn vpxord_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EDI, 1926878394, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 125, 153, 239, 191, 186, 212, 217, 114], OperandSize::Dword)
}

#[test]
fn vpxord_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 49, 85, 135, 239, 248], OperandSize::Qword)
}

#[test]
fn vpxord_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectDisplaced(RDI, 585306594, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 37, 141, 239, 183, 226, 17, 227, 34], OperandSize::Qword)
}

#[test]
fn vpxord_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 45, 147, 239, 12, 187], OperandSize::Qword)
}

#[test]
fn vpxord_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 169, 239, 201], OperandSize::Dword)
}

#[test]
fn vpxord_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 117, 174, 239, 27], OperandSize::Dword)
}

#[test]
fn vpxord_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 1719906028, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 85, 185, 239, 12, 205, 236, 174, 131, 102], OperandSize::Dword)
}

#[test]
fn vpxord_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM20)), operand3: Some(Direct(YMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 17, 93, 164, 239, 198], OperandSize::Qword)
}

#[test]
fn vpxord_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectDisplaced(RSI, 1436406592, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 53, 167, 239, 158, 64, 211, 157, 85], OperandSize::Qword)
}

#[test]
fn vpxord_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectDisplaced(RCX, 1622018254, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 53, 177, 239, 177, 206, 8, 174, 96], OperandSize::Qword)
}

#[test]
fn vpxord_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 69, 204, 239, 233], OperandSize::Dword)
}

#[test]
fn vpxord_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(EDI, 1978736887, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 77, 201, 239, 143, 247, 32, 241, 117], OperandSize::Dword)
}

#[test]
fn vpxord_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 93, 223, 239, 58], OperandSize::Dword)
}

#[test]
fn vpxord_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM30)), operand3: Some(Direct(ZMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 49, 13, 194, 239, 200], OperandSize::Qword)
}

#[test]
fn vpxord_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 45, 195, 239, 36, 123], OperandSize::Qword)
}

#[test]
fn vpxord_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 826684261, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 225, 125, 219, 239, 4, 221, 101, 51, 70, 49], OperandSize::Qword)
}

