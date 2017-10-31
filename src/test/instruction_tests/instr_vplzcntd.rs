use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vplzcntd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 68, 201], OperandSize::Dword)
}

#[test]
fn vplzcntd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(EDI, EDX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 68, 12, 215], OperandSize::Dword)
}

#[test]
fn vplzcntd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(ESI, 1594641610, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 154, 68, 158, 202, 76, 12, 95], OperandSize::Dword)
}

#[test]
fn vplzcntd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 125, 142, 68, 197], OperandSize::Qword)
}

#[test]
fn vplzcntd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RSI, RSI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 68, 4, 118], OperandSize::Qword)
}

#[test]
fn vplzcntd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(XMM11)), operand2: Some(IndirectScaledIndexed(RDI, RSI, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 125, 154, 68, 28, 119], OperandSize::Qword)
}

#[test]
fn vplzcntd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 68, 236], OperandSize::Dword)
}

#[test]
fn vplzcntd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Four, 497490643, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 68, 188, 129, 211, 26, 167, 29], OperandSize::Dword)
}

#[test]
fn vplzcntd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 1345425007, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 185, 68, 60, 245, 111, 142, 49, 80], OperandSize::Dword)
}

#[test]
fn vplzcntd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 82, 125, 169, 68, 247], OperandSize::Qword)
}

#[test]
fn vplzcntd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(YMM8)), operand2: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 125, 174, 68, 1], OperandSize::Qword)
}

#[test]
fn vplzcntd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(YMM8)), operand2: Some(IndirectDisplaced(RDI, 279898758, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 125, 188, 68, 135, 134, 234, 174, 16], OperandSize::Qword)
}

#[test]
fn vplzcntd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 68, 204], OperandSize::Dword)
}

#[test]
fn vplzcntd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectDisplaced(ESI, 830183201, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 68, 142, 33, 151, 123, 49], OperandSize::Dword)
}

#[test]
fn vplzcntd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 492636639, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 219, 68, 36, 221, 223, 9, 93, 29], OperandSize::Dword)
}

#[test]
fn vplzcntd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 2, 125, 204, 68, 250], OperandSize::Qword)
}

#[test]
fn vplzcntd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(ZMM8)), operand2: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 125, 205, 68, 0], OperandSize::Qword)
}

#[test]
fn vplzcntd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(ZMM20)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 125, 219, 68, 35], OperandSize::Qword)
}

