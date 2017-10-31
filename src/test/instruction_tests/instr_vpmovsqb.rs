use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsqb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 137, 34, 193], OperandSize::Dword)
}

#[test]
fn vpmovsqb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Four, 1143916607, Some(OperandSize::Word), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 34, 180, 190, 63, 200, 46, 68], OperandSize::Dword)
}

#[test]
fn vpmovsqb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 137, 34, 218], OperandSize::Qword)
}

#[test]
fn vpmovsqb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Word), None)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 8, 34, 20, 194], OperandSize::Qword)
}

#[test]
fn vpmovsqb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 169, 34, 222], OperandSize::Dword)
}

#[test]
fn vpmovsqb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 34, 16], OperandSize::Dword)
}

#[test]
fn vpmovsqb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Direct(XMM22)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 178, 126, 169, 34, 198], OperandSize::Qword)
}

#[test]
fn vpmovsqb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(IndirectDisplaced(RCX, 334045057, Some(OperandSize::Dword), None)), operand2: Some(Direct(YMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 40, 34, 145, 129, 31, 233, 19], OperandSize::Qword)
}

#[test]
fn vpmovsqb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 126, 204, 34, 245], OperandSize::Dword)
}

#[test]
fn vpmovsqb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 34, 32], OperandSize::Dword)
}

#[test]
fn vpmovsqb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Direct(XMM8)), operand2: Some(Direct(ZMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 82, 126, 201, 34, 240], OperandSize::Qword)
}

#[test]
fn vpmovsqb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(IndirectScaledDisplaced(RDI, Four, 584946736, Some(OperandSize::Qword), None)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 34, 52, 189, 48, 148, 221, 34], OperandSize::Qword)
}

