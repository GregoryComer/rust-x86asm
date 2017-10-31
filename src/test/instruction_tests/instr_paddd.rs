use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn paddd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 254, 195], OperandSize::Dword)
}

#[test]
fn paddd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 254, 12, 214], OperandSize::Dword)
}

#[test]
fn paddd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 254, 225], OperandSize::Qword)
}

#[test]
fn paddd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Eight, 1267064114, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 254, 180, 201, 50, 221, 133, 75], OperandSize::Qword)
}

#[test]
fn paddd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 254, 216], OperandSize::Dword)
}

#[test]
fn paddd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(EDX, ESI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 254, 28, 242], OperandSize::Dword)
}

#[test]
fn paddd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 254, 198], OperandSize::Qword)
}

#[test]
fn paddd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 254, 63], OperandSize::Qword)
}

