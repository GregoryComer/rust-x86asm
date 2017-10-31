use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpexpandq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 139, 137, 231], OperandSize::Dword)
}

#[test]
fn vpexpandq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 383346990, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 139, 137, 148, 199, 46, 105, 217, 22], OperandSize::Dword)
}

#[test]
fn vpexpandq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 50, 253, 141, 137, 253], OperandSize::Qword)
}

#[test]
fn vpexpandq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(XMM31)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 253, 141, 137, 57], OperandSize::Qword)
}

#[test]
fn vpexpandq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 172, 137, 203], OperandSize::Dword)
}

#[test]
fn vpexpandq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexed(ESI, ECX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 171, 137, 28, 206], OperandSize::Dword)
}

#[test]
fn vpexpandq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 146, 253, 173, 137, 227], OperandSize::Qword)
}

#[test]
fn vpexpandq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(YMM23)), operand2: Some(IndirectScaledIndexed(RDI, RDI, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 253, 171, 137, 60, 191], OperandSize::Qword)
}

#[test]
fn vpexpandq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 137, 192], OperandSize::Dword)
}

#[test]
fn vpexpandq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 1695409318, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 137, 20, 133, 166, 228, 13, 101], OperandSize::Dword)
}

#[test]
fn vpexpandq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 34, 253, 202, 137, 231], OperandSize::Qword)
}

#[test]
fn vpexpandq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(ZMM26)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 42709000, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 253, 204, 137, 20, 253, 8, 176, 139, 2], OperandSize::Qword)
}

