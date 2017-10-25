use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsqw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 141, 36, 241], OperandSize::Dword)
}

#[test]
fn vpmovsqw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(IndirectScaledIndexed(EAX, EBX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 36, 52, 216], OperandSize::Dword)
}

#[test]
fn vpmovsqw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 142, 36, 250], OperandSize::Qword)
}

#[test]
fn vpmovsqw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(IndirectDisplaced(RDI, 725391966, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 8, 36, 191, 94, 154, 60, 43], OperandSize::Qword)
}

#[test]
fn vpmovsqw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 174, 36, 253], OperandSize::Dword)
}

#[test]
fn vpmovsqw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(IndirectScaledIndexed(EBX, ESI, Two, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 36, 12, 115], OperandSize::Dword)
}

#[test]
fn vpmovsqw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Direct(XMM14)), operand2: Some(Direct(YMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 194, 126, 173, 36, 214], OperandSize::Qword)
}

#[test]
fn vpmovsqw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 36, 50], OperandSize::Qword)
}

#[test]
fn vpmovsqw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 203, 36, 192], OperandSize::Dword)
}

#[test]
fn vpmovsqw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(IndirectScaledDisplaced(ESI, Two, 754991265, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 36, 28, 117, 161, 64, 0, 45], OperandSize::Dword)
}

#[test]
fn vpmovsqw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Direct(XMM19)), operand2: Some(Direct(ZMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 34, 126, 205, 36, 203], OperandSize::Qword)
}

#[test]
fn vpmovsqw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(IndirectScaledDisplaced(RBX, Four, 72090014, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 72, 36, 52, 157, 158, 1, 76, 4], OperandSize::Qword)
}

