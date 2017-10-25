use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn paddsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 237, 243], OperandSize::Dword)
}

#[test]
fn paddsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(MM6)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 237, 49], OperandSize::Dword)
}

#[test]
fn paddsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 237, 232], OperandSize::Qword)
}

#[test]
fn paddsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 770413775, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 237, 60, 157, 207, 148, 235, 45], OperandSize::Qword)
}

#[test]
fn paddsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 237, 237], OperandSize::Dword)
}

#[test]
fn paddsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 1949908015, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 237, 12, 189, 47, 60, 57, 116], OperandSize::Dword)
}

#[test]
fn paddsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 237, 212], OperandSize::Qword)
}

#[test]
fn paddsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 1235838735, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 237, 132, 194, 15, 103, 169, 73], OperandSize::Qword)
}

