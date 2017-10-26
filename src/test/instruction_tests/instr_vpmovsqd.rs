use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsqd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 126, 138, 37, 196], OperandSize::Dword)
}

#[test]
fn vpmovsqd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(IndirectScaledIndexed(EDI, EDI, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 37, 60, 191], OperandSize::Dword)
}

#[test]
fn vpmovsqd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 130, 126, 139, 37, 207], OperandSize::Qword)
}

#[test]
fn vpmovsqd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(IndirectDisplaced(RDX, 68106924, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 8, 37, 138, 172, 58, 15, 4], OperandSize::Qword)
}

#[test]
fn vpmovsqd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 169, 37, 250], OperandSize::Dword)
}

#[test]
fn vpmovsqd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Four, 1018550830, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 37, 156, 155, 46, 218, 181, 60], OperandSize::Dword)
}

#[test]
fn vpmovsqd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 146, 126, 171, 37, 206], OperandSize::Qword)
}

#[test]
fn vpmovsqd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(IndirectScaledDisplaced(RSI, Eight, 1484946001, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 40, 37, 52, 245, 81, 122, 130, 88], OperandSize::Qword)
}

#[test]
fn vpmovsqd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 206, 37, 234], OperandSize::Dword)
}

#[test]
fn vpmovsqd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(IndirectDisplaced(ECX, 1260970535, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 37, 169, 39, 226, 40, 75], OperandSize::Dword)
}

#[test]
fn vpmovsqd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Direct(YMM16)), operand2: Some(Direct(ZMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 162, 126, 206, 37, 248], OperandSize::Qword)
}

#[test]
fn vpmovsqd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(IndirectDisplaced(RBX, 1775826833, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 37, 187, 145, 247, 216, 105], OperandSize::Qword)
}

