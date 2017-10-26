use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn phaddsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 3, 219], OperandSize::Dword)
}

#[test]
fn phaddsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 3, 52, 126], OperandSize::Dword)
}

#[test]
fn phaddsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 3, 215], OperandSize::Qword)
}

#[test]
fn phaddsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(MM6)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 3, 48], OperandSize::Qword)
}

#[test]
fn phaddsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 3, 238], OperandSize::Dword)
}

#[test]
fn phaddsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 3, 4, 216], OperandSize::Dword)
}

#[test]
fn phaddsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 3, 240], OperandSize::Qword)
}

#[test]
fn phaddsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RDX, 1191198464, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 3, 154, 0, 63, 0, 71], OperandSize::Qword)
}

