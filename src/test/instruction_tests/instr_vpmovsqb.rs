use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsqb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 137, 34, 211], OperandSize::Dword)
}

#[test]
fn vpmovsqb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 34, 22], OperandSize::Dword)
}

#[test]
fn vpmovsqb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 50, 126, 143, 34, 213], OperandSize::Qword)
}

#[test]
fn vpmovsqb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 1886520494, Some(OperandSize::Word), None)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 8, 34, 36, 213, 174, 4, 114, 112], OperandSize::Qword)
}

#[test]
fn vpmovsqb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 175, 34, 207], OperandSize::Dword)
}

#[test]
fn vpmovsqb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Eight, 2098916306, Some(OperandSize::Dword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 34, 172, 255, 210, 235, 26, 125], OperandSize::Dword)
}

#[test]
fn vpmovsqb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Direct(XMM12)), operand2: Some(Direct(YMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 82, 126, 172, 34, 244], OperandSize::Qword)
}

#[test]
fn vpmovsqb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(IndirectDisplaced(RCX, 1067728892, Some(OperandSize::Dword), None)), operand2: Some(Direct(YMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 40, 34, 177, 252, 63, 164, 63], OperandSize::Qword)
}

#[test]
fn vpmovsqb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 207, 34, 192], OperandSize::Dword)
}

#[test]
fn vpmovsqb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 34, 60, 214], OperandSize::Dword)
}

#[test]
fn vpmovsqb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Direct(XMM10)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 210, 126, 204, 34, 242], OperandSize::Qword)
}

#[test]
fn vpmovsqb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Eight, 2071276436, Some(OperandSize::Qword), None)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 34, 156, 207, 148, 43, 117, 123], OperandSize::Qword)
}

