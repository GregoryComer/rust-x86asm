use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psubusw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 217, 236], OperandSize::Dword)
}

#[test]
fn psubusw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexed(EDI, EAX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 217, 44, 135], OperandSize::Dword)
}

#[test]
fn psubusw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 217, 251], OperandSize::Qword)
}

#[test]
fn psubusw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(MM0)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 217, 2], OperandSize::Qword)
}

#[test]
fn psubusw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 217, 195], OperandSize::Dword)
}

#[test]
fn psubusw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 856347951, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 217, 12, 149, 47, 213, 10, 51], OperandSize::Dword)
}

#[test]
fn psubusw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 217, 192], OperandSize::Qword)
}

#[test]
fn psubusw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RDX, 697037250, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 217, 138, 194, 241, 139, 41], OperandSize::Qword)
}

