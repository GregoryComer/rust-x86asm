use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn phaddsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 3, 238], OperandSize::Dword)
}

#[test]
fn phaddsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 1674724382, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 3, 20, 189, 30, 68, 210, 99], OperandSize::Dword)
}

#[test]
fn phaddsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 3, 198], OperandSize::Qword)
}

#[test]
fn phaddsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 3, 4, 216], OperandSize::Qword)
}

#[test]
fn phaddsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 3, 208], OperandSize::Dword)
}

#[test]
fn phaddsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 3, 15], OperandSize::Dword)
}

#[test]
fn phaddsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 3, 195], OperandSize::Qword)
}

#[test]
fn phaddsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 3, 15], OperandSize::Qword)
}

