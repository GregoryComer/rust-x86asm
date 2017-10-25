use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pavgw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 227, 203], OperandSize::Dword)
}

#[test]
fn pavgw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(MM4)), operand2: Some(IndirectDisplaced(ESI, 499865013, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 227, 166, 181, 85, 203, 29], OperandSize::Dword)
}

#[test]
fn pavgw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 227, 252], OperandSize::Qword)
}

#[test]
fn pavgw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(MM0)), operand2: Some(IndirectDisplaced(RDX, 686536219, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 227, 130, 27, 182, 235, 40], OperandSize::Qword)
}

#[test]
fn pavgw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 227, 254], OperandSize::Dword)
}

#[test]
fn pavgw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(EDX, 429244145, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 227, 186, 241, 190, 149, 25], OperandSize::Dword)
}

#[test]
fn pavgw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 227, 247], OperandSize::Qword)
}

#[test]
fn pavgw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 227, 0], OperandSize::Qword)
}

