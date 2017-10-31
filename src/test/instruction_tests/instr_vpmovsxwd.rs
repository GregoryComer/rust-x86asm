use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsxwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 35, 226], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Two, 687785575, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 35, 180, 81, 103, 198, 254, 40], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 35, 244], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RCX, RCX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 35, 60, 201], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 35, 221], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 1950810691, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 35, 36, 245, 67, 2, 71, 116], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 35, 250], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 1736858583, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 35, 4, 133, 215, 91, 134, 103], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 35, 195], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EDI, EAX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 35, 52, 71], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 18, 125, 141, 35, 253], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM25)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 1164074833, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 125, 139, 35, 12, 149, 81, 95, 98, 69], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 35, 212], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 35, 6], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM13)), operand2: Some(Direct(XMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 18, 125, 172, 35, 237], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM30)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 125, 172, 35, 52, 145], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 35, 244], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(ZMM7)), operand2: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 35, 63], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(YMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 2, 125, 202, 35, 219], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(ZMM15)), operand2: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 125, 203, 35, 60, 115], OperandSize::Qword)
}

