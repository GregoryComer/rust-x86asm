use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psubsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 232, 226], OperandSize::Dword)
}

#[test]
fn psubsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Eight, 1853298750, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 232, 140, 201, 62, 24, 119, 110], OperandSize::Dword)
}

#[test]
fn psubsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 232, 207], OperandSize::Qword)
}

#[test]
fn psubsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 192397967, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 232, 20, 85, 143, 194, 119, 11], OperandSize::Qword)
}

#[test]
fn psubsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 232, 249], OperandSize::Dword)
}

#[test]
fn psubsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EDI, EDI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 232, 44, 255], OperandSize::Dword)
}

#[test]
fn psubsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 232, 229], OperandSize::Qword)
}

#[test]
fn psubsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RCX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 232, 36, 201], OperandSize::Qword)
}

