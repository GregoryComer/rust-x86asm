use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpblendmd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 141, 100, 212], OperandSize::Dword)
}

#[test]
fn vpblendmd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EDI, 786758955, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 117, 138, 100, 159, 43, 253, 228, 46], OperandSize::Dword)
}

#[test]
fn vpblendmd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1661662064, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 156, 100, 20, 77, 112, 243, 10, 99], OperandSize::Dword)
}

#[test]
fn vpblendmd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 210, 69, 129, 100, 195], OperandSize::Qword)
}

#[test]
fn vpblendmd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Two, 1037147774, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 109, 133, 100, 164, 121, 126, 158, 209, 61], OperandSize::Qword)
}

#[test]
fn vpblendmd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 1328506237, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 69, 158, 100, 148, 146, 125, 101, 47, 79], OperandSize::Qword)
}

#[test]
fn vpblendmd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 109, 173, 100, 234], OperandSize::Dword)
}

#[test]
fn vpblendmd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 100, 28, 81], OperandSize::Dword)
}

#[test]
fn vpblendmd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 141597705, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 109, 185, 100, 36, 221, 9, 156, 112, 8], OperandSize::Dword)
}

#[test]
fn vpblendmd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 2, 5, 170, 100, 196], OperandSize::Qword)
}

#[test]
fn vpblendmd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 369273686, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 69, 166, 100, 188, 136, 86, 171, 2, 22], OperandSize::Qword)
}

#[test]
fn vpblendmd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 13, 191, 100, 52, 251], OperandSize::Qword)
}

#[test]
fn vpblendmd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 85, 202, 100, 192], OperandSize::Dword)
}

#[test]
fn vpblendmd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 100, 20, 191], OperandSize::Dword)
}

#[test]
fn vpblendmd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 77, 219, 100, 41], OperandSize::Dword)
}

#[test]
fn vpblendmd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM19)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 101, 197, 100, 251], OperandSize::Qword)
}

#[test]
fn vpblendmd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectDisplaced(RDI, 629059169, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 77, 198, 100, 175, 97, 174, 126, 37], OperandSize::Qword)
}

#[test]
fn vpblendmd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(RSI, 1879632121, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 69, 217, 100, 134, 249, 232, 8, 112], OperandSize::Qword)
}

