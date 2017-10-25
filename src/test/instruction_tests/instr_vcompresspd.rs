use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcompresspd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 139, 138, 254], OperandSize::Dword)
}

#[test]
fn vcompresspd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(IndirectScaledIndexed(EDI, ECX, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 253, 8, 138, 36, 79], OperandSize::Dword)
}

#[test]
fn vcompresspd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 18, 253, 139, 138, 202], OperandSize::Qword)
}

#[test]
fn vcompresspd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 979259700, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 253, 8, 138, 12, 197, 52, 81, 94, 58], OperandSize::Qword)
}

#[test]
fn vcompresspd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 169, 138, 196], OperandSize::Dword)
}

#[test]
fn vcompresspd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(IndirectScaledIndexed(EDX, ESI, Two, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 253, 40, 138, 28, 114], OperandSize::Dword)
}

#[test]
fn vcompresspd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 253, 170, 138, 210], OperandSize::Qword)
}

#[test]
fn vcompresspd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(IndirectDisplaced(RBX, 1884872042, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 253, 40, 138, 179, 106, 221, 88, 112], OperandSize::Qword)
}

#[test]
fn vcompresspd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 138, 232], OperandSize::Dword)
}

#[test]
fn vcompresspd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(IndirectScaledDisplaced(EDI, Four, 1222269151, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 253, 72, 138, 12, 189, 223, 88, 218, 72], OperandSize::Dword)
}

#[test]
fn vcompresspd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 18, 253, 201, 138, 223], OperandSize::Qword)
}

#[test]
fn vcompresspd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(IndirectDisplaced(RDI, 1443295707, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 253, 72, 138, 175, 219, 241, 6, 86], OperandSize::Qword)
}

