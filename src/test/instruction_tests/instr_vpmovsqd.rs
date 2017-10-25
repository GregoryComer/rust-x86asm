use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsqd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 142, 37, 213], OperandSize::Dword)
}

#[test]
fn vpmovsqd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 37, 30], OperandSize::Dword)
}

#[test]
fn vpmovsqd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 130, 126, 139, 37, 207], OperandSize::Qword)
}

#[test]
fn vpmovsqd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(IndirectScaledIndexed(RBX, RBX, Two, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 8, 37, 52, 91], OperandSize::Qword)
}

#[test]
fn vpmovsqd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 174, 37, 236], OperandSize::Dword)
}

#[test]
fn vpmovsqd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(IndirectDisplaced(EBX, 300592891, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 37, 131, 251, 174, 234, 17], OperandSize::Dword)
}

#[test]
fn vpmovsqd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(YMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 130, 126, 170, 37, 195], OperandSize::Qword)
}

#[test]
fn vpmovsqd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 40, 37, 26], OperandSize::Qword)
}

#[test]
fn vpmovsqd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 206, 37, 218], OperandSize::Dword)
}

#[test]
fn vpmovsqd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 1800304357, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 37, 156, 243, 229, 118, 78, 107], OperandSize::Dword)
}

#[test]
fn vpmovsqd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(ZMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 162, 126, 207, 37, 228], OperandSize::Qword)
}

#[test]
fn vpmovsqd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Eight, 40530993, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 72, 37, 172, 251, 49, 116, 106, 2], OperandSize::Qword)
}

